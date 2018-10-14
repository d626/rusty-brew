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
use rocket::data;
use rocket::data::FromData;

pub mod sensor;
pub mod output;
pub mod mock;

//mod pid;

use self::sensor::Sensor;
use self::output::Output;
use super::log::{Logger, LogEntry, LoggerChannel};

#[derive(Clone)]
pub struct ReferenceSeries(Vec<(Duration, f32)>);

impl Display for ReferenceSeries {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();
        for (duration, reference) in &self.0 {
            result.push_str(&duration.as_secs().to_string());
            result.push_str(": ");
            result.push_str(&reference.to_string());
            result.push_str(", ");
        }
        write!(f, "{}", result)
    }
}

// TODO: impl this (mayby change some types)
impl FromData for ReferenceSeries {
    type Error = String;

    fn from_data(req: &Request, data: Data) -> data::Outcome<Self, String> {
        unimplemented!();
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
}

impl Controller {
    // TODO: Complete this function
    pub fn new<S, O>(sensor: S, mut output: O) -> Controller
    where S: 'static + Sensor + Send,
          O: 'static + Output + Send {
        let controller = Controller{
            last_log_entry: Arc::new(Mutex::new(None)),
            reference_series: Arc::new(Mutex::new(None)),
            logger: None,
        };
        let reference_series = controller.reference_series.clone();
        let last_log_entry = controller.last_log_entry.clone();
        thread::spawn(move || {
            let start = Instant::now();
            let mut current_reference = None;
            loop {
                if let Some(ref referance_series) = *reference_series.lock()
                    .expect("Unable to lock reference series mutex") {
                    let mut elapsed = start.elapsed();
                    for (duration, referance) in &referance_series.0 {
                        if let Some(difference) =
                            elapsed.checked_sub(duration.clone()) {
                            elapsed = difference;
                        } else {
                            current_reference = Some(referance.clone());
                            break;
                        }
                    }
                        let x = sensor.read();
                        output.set(pid(current_reference));

                } else {

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

fn pid(reference: Option<f32>) -> f32 {
    unimplemented!();
}
