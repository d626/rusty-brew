use std::fs;

use super::sensor::Sensor;

struct DS18B20 {
    id: String,
}

impl DS18B20 {
    pub fn new(id: String) -> DS18B20 {
        // TODO: Scan for devices?
        DS18B20{id}
    }
}

impl Sensor for DS18B20 {
    fn read(&self) -> f32 {
        fs::read_to_string(format!("/sys/bus/w1/devices/{}/w1_slave", self.id))
            .expect("Unable to read temperature")
            .to_owned() // &str -> String
            .split_whitespace()
            .filter( |w| w.contains("t=") ) // The temperature is preceded by t=
            .map( |w| w.trim_left_matches("t=").to_owned() )
            .map( |s| s.parse().unwrap() )
            .map( |t: i32| t as f32 / 1000.0 )
            .nth(0).unwrap()
    }
}
