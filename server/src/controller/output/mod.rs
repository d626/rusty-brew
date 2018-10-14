//! Module defining types used for setting the output

// TODO: Impl<O: Output> Drop for O>, go to safe state when the output is dropped
pub trait Output {
    /// Function used to set the output. 
    fn set(&mut self, f32);
}
