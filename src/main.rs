#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;

mod controller;
use controller::sensor::Sensor;
use controller::mock::*;

use std::cell::RefCell;
use std::rc::Rc;

#[get("/")]
fn index() -> &'static str {
    "Hello, World!"
}

fn main() {
    let mock_state = Rc::new(RefCell::new(MockInternalState::new()));
    let foo = controller::mock::MockTemperatureSensor::new(mock_state);

    println!("Sensor value: {}", foo.read());
    //rocket::ignite().mount("/", routes![index]).launch();
}
