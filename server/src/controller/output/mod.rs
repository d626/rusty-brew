//! Module defining types used for setting the output.

use std::marker::{Send, Sync};

// TODO: Impl<O: Output> Drop for O>, go to safe state when the output is dropped

/// Trait that must be implemented by types used as output by the controllers.
pub trait Output : Send + Sync {
    /// Function used to set the output.
    fn set(&mut self, f32);

    /// Function used to turn off the output when it is no more used.
    fn turn_off(&mut self);
}
