#![allow(unreachable_code)]
#![allow(unused_variables)]
#![allow(dead_code)]

#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

extern crate chrono;
use chrono::Local;

mod controller;
use controller::sensor::Sensor;
use controller::mock::*;

mod log;

mod interface;

fn main() {
    let date = Local::now();
    println!("Date: {}", date.format("%Y-%m-%d"));
    let mock_state = MockInternalState::new();
    let mock_sensor = controller::mock::MockTemperatureSensor::new(mock_state.clone());
    let mock_output = controller::mock::MockOutput::new(mock_state.clone());
    println!("Sensor value: {}", mock_sensor.read());
    let mock_controller = controller::Controller::new(mock_sensor, mock_output);


    let controllers = vec![mock_controller];
}
