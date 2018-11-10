//! Module containig all the temperature controller logic

use std::time::Duration;
use std::time::Instant;
use std::string::ToString;
use std::fmt::Display;
use std::fmt;
use std::thread;
use std::sync::Mutex;
use std::sync::Arc;

use rocket::Request;
use rocket::Data;

use serde::{Serialize, Deserialize};
use serde::de::DeserializeOwned;

pub mod sensor;
pub mod output;
pub mod mock;

//#[cfg(target = "armv7-unknown-linux-gnueabihf")]
pub mod ds18b20;
//#[cfg(target = "armv7-unknown-linux-gnueabihf")]
pub mod led;

pub mod pid;

use self::sensor::Sensor;
use self::output::Output;
use self::pid::*;
use super::log::{Logger, LogEntry, LoggerChannel};

#[derive(Clone, Serialize, Deserialize)]
pub struct ReferenceSeries( pub Vec<Reference> );

impl ReferenceSeries {
    pub fn new(references: Vec<Reference>) -> ReferenceSeries {
        ReferenceSeries(references)
    }
}

impl Display for ReferenceSeries {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for Reference{duration, temp} in &self.0 {
            result.push_str(&duration.to_string());
            result.push_str(": ");
            result.push_str(&temp.to_string());
            result.push_str(", ");
        }
        write!(f, "{}", result)
    }
}

enum ControllerCommand {
    StartControlling(Mutex<ReferenceSeries>),
    StopControlling
}

#[derive(Clone)]
pub struct Controller {
    last_log_entry: Arc<Mutex<Option<LogEntry>>>,
    reference_series: Arc<Mutex<Option<ReferenceSeries>>>,
    logger: Option<Logger>,
    parameters: PidParameters,
}

impl Controller {
    // TODO: Complete this function
    /// Spawns a thread that lasts forever
    pub fn new<S, O>(sensor: S, mut output: O) -> Controller
    where S: 'static + Sensor + Send,
          O: 'static + Output + Send {
        let controller = Controller{
            last_log_entry: Arc::new(Mutex::new(None)),
            reference_series: Arc::new(Mutex::new(None)),
            logger: None,
            parameters: PidParameters::new(1.0, 0.0, 0.0),
        };
        let reference_series = controller.reference_series.clone();
        let last_log_entry = controller.last_log_entry.clone();
        let parameters = controller.parameters.clone();
        let mut pid = Pid::new(&controller.parameters);
        thread::spawn(move || {
            let start = Instant::now();
            let mut current_reference = None;
            loop {
                if let Some(ref referance_series) = *reference_series.lock()
                    .expect("Unable to lock reference series mutex") {
                        let mut elapsed = start.elapsed();
                    for Reference{duration, temp} in &referance_series.0 {
                        if let Some(difference) =
                            elapsed.checked_sub(Duration::from_secs(*duration)) {
                            elapsed = difference;
                        } else {
                            current_reference = Some(temp.clone());
                            break; // We have found our current reference
                        }
                    }
                    if let Some(reference) = current_reference {
                        let x = sensor.read();
                        let u = pid.pid(x, reference as f32);
                        output.set(x);
                        // logger
                    }
                    // do logging
                } else {
                    // Turn off, notify user?
                }
            }
        });
        controller
    }

    // name should include the name of the resource used, and be URL friendly
    pub fn start(self, name: String, referance: ReferenceSeries) {
        let logger = Logger::new(name, referance.clone());
        // Start controlling the temperature
        // return a channel used to get status updates from the logger
        *self.reference_series.lock()
            .expect("Unable to lock reference series mutex") = Some(referance);
    }

    pub fn get_last_log_entry(&self) -> Option<LogEntry> {
        self.last_log_entry
            .lock()
            .expect("Unable to lock last_log_entry mutex")
            .clone()
    }

    pub fn get_name(&self) -> Option<String> {
        match &self.logger {
            Some(logger) => Some(logger.get_name()),
            None => None
        }
    }
}
