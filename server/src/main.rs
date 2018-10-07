#![allow(unreachable_code)]
#![allow(unused_variables)]
#![allow(dead_code)]

#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate chrono;

mod controller;
use controller::sensor::Sensor;
use controller::mock::*;

mod log;

mod interface;

use std::cell::RefCell;
use std::rc::Rc;


fn main() {
    let mock_state = Rc::new(RefCell::new(MockInternalState::new()));
    let mock_sensor = controller::mock::MockTemperatureSensor::new(Rc::clone(&mock_state));
    let mock_output = controller::mock::MockOutput::new(Rc::clone(&mock_state));
    println!("Sensor value: {}", mock_sensor.read());
    let mock_controller = controller::Controller::new(mock_sensor, mock_output);


    let controllers = vec![mock_controller];
}
