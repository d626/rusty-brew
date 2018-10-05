//! Module implementing a mock temperature sensor used for testing on devices
//! that is not connected to real hw.

use std::time;
use std::cell::RefCell;
use std::rc::Rc;

use super::sensor::Sensor;
use super::output::Output;

const ROOM_TEMPERATURE: f32 = 20.0;

/// Struct representing the system, used for simulation
#[derive(Copy, Clone)]
pub struct MockInternalState {
    temp: f32,
    output: f32,
    last_updated: time::Instant
}

impl MockInternalState {
    /// Makes a new default MockInternalState
    pub fn new() -> Self {
        MockInternalState{
            temp: ROOM_TEMPERATURE,
            last_updated: time::Instant::now(),
            output: ROOM_TEMPERATURE,
        }
    }
}

/// Struct representing a mock temperature sensor. Used for testing the system
/// on devices that is not connected to real hw.
pub struct MockTemperatureSensor {
    internal_state: Rc<RefCell<MockInternalState>>
}

impl MockTemperatureSensor {
    /// Makes a new default MockTemperatureSensor
    pub fn new(internal: Rc<RefCell<MockInternalState>>)
               -> MockTemperatureSensor {
        MockTemperatureSensor {
            internal_state: internal
        }
    }
}

impl Sensor for MockTemperatureSensor {
    /// Uses Newton's law of cooling to simulate the system
    fn read(&self) -> f32 {
        #![allow(non_snake_case)]
        let mut state = self.internal_state.borrow_mut();
        let T0 = state.temp + 273.15; // Initial temperature [K]
        let m = 25.0; // mass [kg]
        let cp = 4185.5; // specific heat capacity [J/kg*K]
        let A = 0.3807; // surface area [m^2]
        let h = 1.0; // heat transfer coefficient
        let r = (h*A) / (m*cp); // [1/s]
        state.temp = 20.0;
        unimplemented!();
    }
}

/// Struct representing a mock output to controll the temperature of the system.
/// Used for testing on devices that is not connectet to real hw.
pub struct MockOutput(Rc<RefCell<MockInternalState>>);

impl MockOutput {
    /// Makes a new default MockOutput
    pub fn new(internal: Rc<RefCell<MockInternalState>>) -> Self {
        MockOutput(internal)
    }
}

impl Output for MockOutput {
    fn set(&mut self, output: f32) {
        let mut internal = self.0.borrow_mut();
        internal.output = output;
    }
}
