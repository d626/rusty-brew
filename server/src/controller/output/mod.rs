//! Module defining types used for setting the output

use std::marker::{Send, Sync};

// TODO: Impl<O: Output> Drop for O>, go to safe state when the output is dropped
pub trait Output : Send + Sync {
    /// Function used to set the output.
    fn set(&mut self, f32);

    fn turn_off(&mut self);
}
