//! Module containig data types needed for duing PID controlling.

use std::default::Default;
use std::f32;

/// Struct representing a reference, and how long that reference should be held.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reference {
    pub duration: u64,
    pub temp: u32
}

/// Struct containing tuning parameters for the PID controller.
#[derive(Clone)]
pub struct PidParameters {
    kp: f32,
    ki: f32,
    kd: f32,
    max_integrator: f32,
}

impl PidParameters {
    /// Constructor for the PidParameters struct. max_integrator is the maximum value that
    /// the integrator can have. This is used to proved anti-windup.
    pub fn new(kp: f32, ki: f32, kd: f32, max_integrator: f32) -> PidParameters {
        PidParameters{
            kp,
            ki,
            kd,
            max_integrator,
        }
    }

    pub fn get_parameters(&self) -> (f32, f32, f32) {
        (self.kp, self.ki, self.kd)
    }
}

/// kp = 1, ki = 0, kd = 0, max_integrator = f32::INFINITY
impl Default for PidParameters {
    fn default() -> PidParameters {
        PidParameters {
            kp: 1.0,
            ki: 0.0,
            kd: 0.0,
            max_integrator: f32::INFINITY,
        }
    }
}

/// Struct providing PID functionality.
pub struct Pid {
    parameters: PidParameters,
    accumulator: f32,
    previous_input: f32,
}

impl Pid {
    /// Constructor.
    pub fn new(parameters: &PidParameters) -> Pid {
        Pid {
            parameters: parameters.clone(),
            accumulator: 0.0,
            previous_input: 0.0,
        }
    }

    /// Function for calculationg a new output.
    pub fn pid(&mut self, input: f32, reference: f32) -> f32 {
        let (kp, ki, kd) = self.parameters.get_parameters();

        let error = reference - input;

        let tmp_accumulator = self.accumulator + error;
        self.accumulator = if tmp_accumulator < self.parameters.max_integrator {
            tmp_accumulator
        } else {
            self.parameters.max_integrator
        };

        let derivative = input - self.previous_input;

        self.previous_input = input;
        kp*error + ki*self.accumulator + kd*derivative
    }
}
