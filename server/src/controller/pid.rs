struct Reference(i32);

fn pid(kp: f32, ki: f32, kd: f32) -> f32 {
    unimplemented!();
}

#[cfg(test)]
mod test {
    use super::mock::*;

    fn test() {
        let mock_sensor = MockTemperatureSensor::new();
        let mock_output = MockOutput::new();

        mock_sensor.read();
        mock_output.set();
    }
}
