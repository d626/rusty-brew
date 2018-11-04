#![allow(unreachable_code)]
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_imports)]

#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

extern crate chrono;
use chrono::Local;

extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

mod controller;
use controller::Controller;
use controller::sensor::Sensor;
use controller::output::Output;
use controller::mock::*;

use controller::pid::Reference;
use controller::ReferenceSeries;

mod log;

mod interface;

fn main() {
    let date = Local::now();
    println!("Date: {}", date.format("%Y-%m-%d"));
    let mock_state = MockInternalState::new();
    let mock_sensor = MockTemperatureSensor::new(mock_state.clone());
    let mock_output = MockOutput::new(mock_state.clone());
    let mock_controller = Controller::new(mock_sensor, mock_output);

    let reference = Reference { duration: 23, temp: 59 };
    let series = ReferenceSeries::new(vec![reference]);
    println!("{}", serde_json::to_string(&series).unwrap());
    interface::init_interface::<MockTemperatureSensor, MockOutput>(vec![mock_controller]);
    loop {}
}

fn test_mock_system() {
    {
        let mock_state = MockInternalState::new();
        let mock_sensor = MockTemperatureSensor::new(mock_state.clone());
        let mut mock_output = MockOutput::new(mock_state.clone());
        println!("Sensor value: {}", mock_sensor.read());
        mock_output.set(100_f32);
        for _ in 0..999 {
            {
                println!("Sensor value: {}", mock_sensor.read());
            }
            std::thread::sleep(std::time::Duration::from_millis(1000));
        }
        // let mock_controller = controller::Controller::new(mock_sensor, mock_output);

        // let controllers = vec![mock_controller];
        let mock_controller = Controller::new(mock_sensor, mock_output);
    }
    std::thread::sleep(std::time::Duration::from_millis(10*1000));
    println!("Finished");
}

fn test_get_logs() {
    for name in log::get_list_of_logs() {
        println!("{}", name);
    }
}
