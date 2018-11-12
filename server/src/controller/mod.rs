//! Module containig all the temperature controller logic

use std::time::Duration;
use std::time::Instant;
use std::string::ToString;
use std::fmt::Display;
use std::fmt;
use std::thread;
use std::sync::Mutex;
use std::sync::Arc;

use std::marker::PhantomData;

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

use std::sync::mpsc::channel;
use std::fs;

pub struct Controller {
    logger: Arc<Mutex<Option<Logger>>>,
    sensor: Arc<Mutex<Box<'static + Sensor>>>,
    output: Arc<Mutex<Box<'static + Output>>>,
    pid_parameters: PidParameters,
}

impl Controller {
    pub fn new<S, O>(sensor: S, output: O, pid_parameters: PidParameters) -> Controller
    where S: 'static + Sensor + Sync + Send,
          O: 'static + Output + Sync + Send,
    {
        Controller {
            pid_parameters,
            sensor: Arc::new(Mutex::new(Box::new(sensor))),
            output: Arc::new(Mutex::new(Box::new(output))),
            logger: Arc::new(Mutex::new(None)),
        }
    }

    // TODO: Document this function ALOT!
    pub fn start(&mut self, reference_name: String, reference_series: ReferenceSeries) -> std::io::Result<()> {
        // Make new thread to make function return immediately
        let logger = Logger::new(reference_name.clone()); // TODO: This can fail if file exists, add date to name
        *self.logger.lock().expect("Unable to lock logger") = Some(logger);


        let logger = Arc::clone(&self.logger);
        let output = Arc::clone(&self.output);
        let sensor = Arc::clone(&self.sensor);
        let parameters = self.pid_parameters.clone();

        thread::spawn(move || {
            let (r_tx, r_rx) = channel();
            thread::spawn(move || {
                for reference in reference_series.0 {
                    r_tx.send(reference.temp).expect("r_tx failed"); // This should always work
                    thread::sleep(Duration::from_secs(reference.duration));
                }
            });

            let logger_ref = Arc::clone(&logger);
            let output_ref = Arc::clone(&output);

            // TODO: Make calculate output at a given interval, rather than when we get a new reference
            // TODO: Figure out why this crashes:thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: Error("invalid type: null, expected f32", line: 1, column: 148)', libcore/result.rs:1009:5
            // note: Run with `RUST_BACKTRACE=1` for a backtrace.
                // thread '<unnamed>' panicked at 'Unable to join pid thread: Any', libcore/result.rs:1009:5

            thread::spawn(move || {
                let mut pid = Pid::new(&parameters);
                loop {
                    if let Ok(r) = r_rx.recv()  {
                        println!("r: {}", r);
                        let y = sensor.lock().expect("Unable to lock sensor").read();
                        let u = pid.pid(y, r as f32);
                        output_ref.lock().expect("Unable to lock output").set(u);
                        let logger = &mut *logger_ref.lock().expect("Unable to lock logger");
                        logger.as_mut().expect("Unable to take logger as mut").add_entry(r as f32, y, u);
                    } else {
                        return;
                    }
                };
            }).join().expect("Unable to join pid thread"); // The thread should not panic, unless something has gone horribly wrong

            output.lock().expect("Unable to lock output").turn_off();
            *logger.lock().expect("Unable to lock logger") = None;
        });

        Ok(())
    }

    pub fn get_last_log_entry(&self) -> Option<LogEntry> {
        match *self.logger.lock().expect("Unable to lock logger") {
            Some(ref logger) => logger.get_last_entry(),
            None => None
        }
    }

    pub fn get_name_of_current_process(&self) -> Option<String> {
        match *self.logger.lock().expect("Unable to lock logger") {
            Some(ref logger) => Some(logger.get_name()),
            None => None
        }
    }
}
