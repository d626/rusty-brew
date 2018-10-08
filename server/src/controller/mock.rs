//! Module implementing a mock temperature sensor used for testing on devices
//! that is not connected to real hw.

use std::time;
use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

use super::sensor::Sensor;
use super::output::Output;

const ROOM_TEMPERATURE: f32 = 20.0;

/// Struct representing the system, used for simulation
#[derive(Clone)]
pub struct MockInternalState {
    kill_tx: mpsc::Sender<()>,
    temperature_mutex: Arc<Mutex<f32>>,
    output_mutex: Arc<Mutex<f32>>
}

impl MockInternalState {
    /// Makes a new default MockInternalState
    /// Uses Newton's law of cooling to simulate the system
    pub fn new() -> Self {
        #![allow(non_snake_case)]
        let (kill_tx, kill_rx) = mpsc::channel::<()>();
        let temperature_mutex = Arc::new(Mutex::new(20.0));
        let output_mutex = Arc::new(Mutex::new(20.0));
        let mock_internal = MockInternalState {
            kill_tx,
            temperature_mutex: temperature_mutex.clone(),
            output_mutex: output_mutex.clone(),
        };
        thread::spawn(move || {
            let m = 25.0; // mass [kg]
            let cp = 4185.5; // specific heat capacity [J/kg*K]
            let A = 0.3807; // surface area [m^2]
            let h = 1.0; // heat transfer coefficient
            let r = (h*A) / (m*cp); // [1/s]
            let sleep_period = 500;
            loop {
                match kill_rx.try_recv() {
                    Err(mpsc::TryRecvError::Disconnected) => break,
                    _ => {}
                }
                {
                    let T_env = output_mutex.lock()
                        .expect("Unable to lock output_mutex");
                    let mut T = temperature_mutex.lock()
                        .expect("Unable to lock temperature_mutex");
                    let dt = -r * (*T - *T_env);
                    *T = *T + (sleep_period as f32)/1000.0 * dt;
                } // Mutexes are unlocked here
                thread::sleep(time::Duration::from_millis(sleep_period));
            }
            println!("Simulator finished");
        });
        mock_internal
    }
}

/// Struct representing a mock temperature sensor. Used for testing the system
/// on devices that is not connected to real hw.
pub struct MockTemperatureSensor {
    internal_state: MockInternalState
}

impl MockTemperatureSensor {
    /// Makes a new default MockTemperatureSensor
    pub fn new(internal: MockInternalState)
               -> MockTemperatureSensor {
        MockTemperatureSensor {
            internal_state: internal
        }
    }
}

impl Sensor for MockTemperatureSensor {
    fn read(&self) -> f32 {
        *self.internal_state.temperature_mutex.lock()
            .expect("Unable to read temperature_mutex")
    }
}

/// Struct representing a mock output to controll the temperature of the system.
/// Used for testing on devices that is not connectet to real hw.
pub struct MockOutput(MockInternalState);

impl MockOutput {
    /// Makes a new default MockOutput
    pub fn new(internal: MockInternalState) -> Self {
        MockOutput(internal)
    }
}

impl Output for MockOutput {
    fn set(&mut self, output: f32) {
        *self.0.output_mutex.lock()
            .expect("Unable to set output_mutex") = output;
    }
}
