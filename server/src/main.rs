#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate chrono;

mod controller;
use controller::sensor::Sensor;
use controller::mock::*;

mod log;

use std::cell::RefCell;
use std::rc::Rc;

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

fn main() {
    let mock_state = Rc::new(RefCell::new(MockInternalState::new()));
    let mock_sensor = controller::mock::MockTemperatureSensor::new(mock_state);
    let mock_output = controller::mock::MockOutput::new();
    let mock_controller = controller::Controller::new(mock_sensor, mock_output);

    println!("Sensor value: {}", mock_controller.read());

    let controllers = vec![mock_controller];

    //rocket::ignite().mount("/", routes![index]).launch();
}
