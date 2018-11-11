//! Module containig types and functionality for reading sensor values of the temperature sensors

use std::marker::{Send, Sync};

pub trait Sensor : Send + Sync {
    /// Function for reading from a Sensor
    fn read(&self) -> f32;
}
