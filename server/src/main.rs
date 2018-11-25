#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

extern crate chrono;

use std::collections::HashMap;
use std::sync::Mutex;

pub mod controller;
use controller::Controller;
use controller::sensor::Sensor;
use controller::output::Output;
use controller::mock::*;

use controller::pid::Reference;
use controller::ReferenceSeries;

pub mod log;

pub mod interface;

/// Program entry point
fn main() {
    test_start_interface();
}

/// Function for testing the controller
fn test_pid() {
    let environment = MockInternalState::new();
    let sensor = MockTemperatureSensor::new(environment.clone());
    let output = MockOutput::new(environment.clone());

    let parameters = controller::pid::PidParameters::default();
    let mut controller = Controller::new(sensor, output, parameters, 60);

    let reference_series = vec![
        Reference{duration: 60, temp: 55},
        Reference{duration: 30, temp: 60},
        Reference{duration: 30, temp: 75},
    ];
    let reference_series = ReferenceSeries::new(reference_series);

    controller.start("foo".to_owned(), reference_series).unwrap();
    std::thread::sleep(std::time::Duration::from_secs(200));
}


/// Function for testing the server, using mocked input and output
fn test_start_interface() {
    let mock_state = MockInternalState::new();
    let mock_sensor = MockTemperatureSensor::new(mock_state.clone());
    let mock_output = MockOutput::new(mock_state.clone());
    let mock_controller = Controller::new(mock_sensor, mock_output, Default::default(), 60);

    let reference = Reference { duration: 23, temp: 59 };
    let series = ReferenceSeries::new(vec![reference]);
    println!("{}", serde_json::to_string(&series).unwrap());
    let mut controllers = HashMap::new();
    controllers.insert("Mock".to_owned(), Mutex::new(mock_controller));

    interface::init_interface(controllers);
}

/// Function for testing mocked input and output
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
        let _mock_controller = Controller::new(mock_sensor, mock_output, Default::default(), 60);
    }
    std::thread::sleep(std::time::Duration::from_millis(10*1000));
    println!("Finished");
}

/// Function for testing if we can get a list of logs
fn test_get_logs() {
    for name in log::get_list_of_logs() {
        println!("{}", name);
    }
}
