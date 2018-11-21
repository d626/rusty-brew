//! Module containing a representation of the DS18B20 onewire temperature sensor.
use std::fs;

use super::sensor::Sensor;

/// Struct representing a DS18B20 onewire temperature sensor.
pub struct DS18B20 {
    id: String,
}

impl DS18B20 {
    /// Constructor for DS18B20.
    /// id is the name of the device, as found in /sys/bus/w1/devices.
    pub fn new(id: String) -> DS18B20 {
        // TODO: Scan for devices?
        DS18B20{id}
    }
}

/// Sensor implementation for DS18B20.
/// This implementation uses the Linux kernel module, and its file abstraction.
/// This is because bit banging onewire from a userspace thread is not able to
/// meet the timing demands of the bus.
impl Sensor for DS18B20 {
    fn read(&self) -> f32 {
        fs::read_to_string(format!("/sys/bus/w1/devices/{}/w1_slave", self.id))
            .expect("Unable to read temperature")
            .split_whitespace()
            .filter( |w| w.contains("t=") ) // The temperature is preceded by t=
            .map( |w| w.trim_left_matches("t=").to_owned() ) // Remove t=
            .map( |s| s.parse().unwrap() ) // Parse to i32
            .map( |t: i32| t as f32 / 1000.0 ) // Convert to deg C
            .nth(0).unwrap() // Take first (and only) element
    }
}
