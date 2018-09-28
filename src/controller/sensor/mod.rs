//! Module containig types and functionality for reading sensor values of the temperature sensors

pub trait Sensor {
    fn read() -> f32;
}
