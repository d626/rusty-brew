//! Module containig types and functionality for reading sensor values of the temperature sensors

use std::marker::{Send, Sync};

/// Trait that must be implemented by types that are used as inputs by the controllers.
pub trait Sensor : Send + Sync {
    /// Function for reading from a Sensor
    fn read(&self) -> f32;
}
