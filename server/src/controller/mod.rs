//! Module containig all the temperature controller logic

pub mod sensor;
pub mod output;
pub mod mock;

use self::sensor::Sensor;
use self::output::Output;

pub struct Controller<S: Sensor, O: Output> {
    sensor: S,
    output: O,
}

impl<S, O> Controller<S, O> {
    pub fn new(sensor: S, output: O) -> Controller<S, O> {
        unimplemented!();
        Controller{sensor, output}
    }
}
