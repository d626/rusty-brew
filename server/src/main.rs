#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
extern crate rocket_contrib;

extern crate serde;
#[macro_use] extern crate serde_derive;
extern crate serde_json;

extern crate chrono;

extern crate linux_embedded_hal;
extern crate embedded_hal;
extern crate sysfs_gpio;

use std::collections::HashMap;
use std::sync::Mutex;
// use std::f32;

pub mod controller;
use controller::Controller;
use controller::sensor::Sensor;
use controller::output::Output;
use controller::mock::*;

use controller::ds18b20;
use controller::led;

use controller::pid::{Reference, PidParameters};
use controller::ReferenceSeries;

pub mod log;

pub mod interface;

/// Program entry point
fn main() {
    log();
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

/// Function for testing HW sensor and output
fn test_physical() {
    let sensor1 = ds18b20::DS18B20::new("28-000009eab19f".to_owned());
    let sensor2 = ds18b20::DS18B20::new("28-000009eb40fe".to_owned());
    let mut output = led::LedOutput::new(20, 21, 50);
    for _ in 0..10 {
        output.set(30.0);
        println!("Temp1: {}", sensor1.read());
        println!("Temp2: {}", sensor2.read());
        output.set(0.0);
    }
    output.set(100.0);
}

/// Function for testing the server, using mocked input and output
fn test_start_interface() {
    let mock_state = MockInternalState::new();
    let mock_sensor = MockTemperatureSensor::new(mock_state.clone());
    let mock_output = MockOutput::new(mock_state.clone());
    let mock_controller = Controller::new(mock_sensor,
                                           mock_output,
                                           PidParameters::new(7.0, 0.6, 0.0, 35.0),
                                           60);

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
        let _mock_controller = Controller::new(mock_sensor,
                                               mock_output,
                                               PidParameters::default(),
                                               60);
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

fn test_log() {
    let sensor = ds18b20::DS18B20::new("28-000006739a01".to_owned());
    let mock_state = MockInternalState::new();
    let mock_output = MockOutput::new(mock_state.clone());
    let reference_series = vec![
        Reference{duration: 60*60*24, temp: 100},
    ];
    let reference_series = ReferenceSeries::new(reference_series);
    let parameters = controller::pid::PidParameters::default();

    let mut controller = controller::Controller::new(sensor, mock_output, parameters, 1);
    controller.start("response".to_owned(), reference_series).expect("Unable to start logging");
}

fn log() {
    use std::sync::mpsc::channel;
    use std::thread;
    use std::time::Duration;
    use std::io;

    // let sensor = ds18b20::DS18B20::new("28-000006739a01".to_owned());
    let sensor = MockTemperatureSensor::new(MockInternalState::new());
    let mut log = log::Logger::new("step_response".to_owned());
    let (tx, rx) = channel();

    let handle = thread::spawn(move || {
        loop {
            match rx.try_recv() {
                Err(std::sync::mpsc::TryRecvError::Disconnected) => {
                    return;
                }
                _ => {
                    println!("Logged");
                    log.add_entry(0., sensor.read(), 0.);
                    thread::sleep(Duration::from_secs(10));
                }
            }
        }
    });
    thread::spawn(move || {
        loop {
            let mut input = String::new();
            print!("Stop logging? ");
            io::stdin().read_line(&mut input).expect("Failed to read input");
            if input.trim() == "y" {
                return;
            } else {
                tx.send(()).expect("Unable to send");
            }
        }
    });
    handle.join().expect("Error while joining");
}
