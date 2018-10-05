//! Module containig all the temperature controller logic

use std::time::Duration;
use std::string::ToString;
use std::fmt::Display;
use std::fmt;

pub mod sensor;
pub mod output;
pub mod mock;

use self::sensor::Sensor;
use self::output::Output;
use super::log::{Logger, LoggerChannel};

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

pub struct Controller<S: Sensor, O: Output> {
    sensor: S,
    output: O,
}

impl<S: Sensor, O: Output> Controller<S, O> {
    pub fn new(sensor: S, output: O) -> Controller<S, O> {
        unimplemented!();
        Controller{sensor, output}
    }

    pub fn start(&self, name: String, referance: ReferenceSeries) -> LoggerChannel {
        let logger = Logger::new(name, referance);
        // Start controlling the temperature
        // return a channel used to get status updates from the logger
        unimplemented!();
    }
}
