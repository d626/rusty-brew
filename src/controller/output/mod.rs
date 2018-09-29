//! Module defining types used for setting the output

pub trait Output {
    /// Function used to set the output. 
    fn set(&mut self, f32);
}
