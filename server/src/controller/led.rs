//! Module containing a output that blinks a LED for increasing output,
//! and another for decreasing output.
use std::cmp::Ordering;
use std::thread::sleep;
use std::time::Duration;

use linux_embedded_hal::Pin;
use embedded_hal::digital::OutputPin;
use sysfs_gpio::Direction;

use super::output::Output;

/// A struct representing the LEDs used as an output.
pub struct LedOutput {
    /// Pin connected to the LED signifying an increase in output.
    up_pin: Pin,
    /// Pin connected to the LED signifying an decrease in output.
    down_pin: Pin,
    /// Field used to keep track of what the current output is.
    state: u32,
    /// Field used to configure how long an LED blink is.
    pulse_width: u64,
}

impl LedOutput {
    /// Constructor for LedOutput.
    /// up_pin and down_pin is the pin number (as seen in Linux, not the physica
    /// pin number) connected to the up LED an down LED.
    /// pulse_width is the duration the LEDs are on when "increasing" or
    /// "decreasing" the output in milliseconds.
    pub fn new(up_pin: u64, down_pin: u64, pulse_width: u64) -> LedOutput {
        let up_pin = Pin::new(up_pin);
        let down_pin = Pin::new(down_pin);

        up_pin.export().expect("Unable to export up_pin");
        down_pin.export().expect("Unable to export down_pin");

        up_pin.set_direction(Direction::Low).expect("Unable to set direction of up_pin");
        down_pin.set_direction(Direction::Low).expect("Unable to set direction of down_pin");

        LedOutput{up_pin, down_pin, pulse_width, state: 0}
    }

    /// Helper function to increase the output
    fn increase_output(&mut self) {
        self.up_pin.set_high();
        sleep(Duration::from_millis(self.pulse_width));
        self.up_pin.set_low();
        sleep(Duration::from_millis(self.pulse_width));
        self.state += 1;
    }

    /// Helper function to decrease the output
    fn decrease_output(&mut self) {
        self.down_pin.set_high();
        sleep(Duration::from_millis(self.pulse_width));
        self.down_pin.set_low();
        sleep(Duration::from_millis(self.pulse_width));
        self.state -= 1;
    }
}

impl Output for LedOutput {
    fn set(&mut self, output: f32) {
        let output = if output < 0.0 { 0 } else { output as u32 };
        let output = if output > 100 { 100 } else { output };
        let output = output / 10;

        match output.cmp(&self.state) {
            Ordering::Less => {
                for _ in output..self.state {
                    self.decrease_output();
                }
            }
            Ordering::Greater => {
                for _ in self.state..output {
                    self.increase_output();
                }
            }
            Ordering::Equal => {
                // No operation needed, as we already are on the right level
            }
        }
    }

    fn turn_off(&mut self) {
        for _ in self.state..0 {
            self.decrease_output();
        }
    }
}
