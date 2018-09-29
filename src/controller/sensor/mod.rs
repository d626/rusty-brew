//! Module containig types and functionality for reading sensor values of the temperature sensors

pub trait Sensor {
    /// Function for reading from a Sensor
    fn read(&self) -> f32;
}
