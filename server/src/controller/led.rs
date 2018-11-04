use std::cmp::Ordering;
use std::thread::sleep;
use std::time::Duration;

use linux_embedded_hal::Pin;
use embedded_hal::digital::OutputPin;

use super::output::Output;

pub struct LedOutput {
    up_pin: Pin,
    down_pin: Pin,
    state: u32,
}

impl LedOutput {
    pub fn new(up_pin: u64, down_pin: u64) -> LedOutput {
        let up_pin = Pin::new(up_pin);
        let down_pin = Pin::new(down_pin);

        up_pin.export().expect("Unable to export up_pin");
        down_pin.export().expect("Unable to export down_pin");

        LedOutput{up_pin, down_pin, state: 0}
    }
}

impl Output for LedOutput {
    fn set(&mut self, output: f32) {
        let output = (output / 100.0) as u32;
        let output = output / 10;

        match output.cmp(&self.state) {
            Ordering::Less => {
                for _ in output..self.state {
                    self.down_pin.set_high();
                    sleep(Duration::from_millis(50));
                    self.down_pin.set_low();
                    sleep(Duration::from_millis(50));
                }
            }
            Ordering::Greater => {
                for _ in output..self.state {
                    self.up_pin.set_high();
                    sleep(Duration::from_millis(50));
                    self.up_pin.set_low();
                    sleep(Duration::from_millis(50));
                }
            }
            Ordering::Equal => {
                // No operation needed, as we already are on the right level
            }
        }
    }
}
