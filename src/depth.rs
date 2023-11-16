use std::thread::sleep;
use std::time::{Duration, Instant};

use anyhow::Result;
use rppal::gpio::{Gpio, InputPin, OutputPin};

// ultrasonic pins
const TRIG_PIN: u8 = 27; // D2 (robot-hat)
const ECHO_PIN: u8 = 22; // D3 (robot-hat)

pub struct Ultrasonic {
    trig: OutputPin,
    echo: InputPin,
}

impl Ultrasonic {
    pub fn new() -> Result<Self> {
        let trig = Gpio::new()?.get(TRIG_PIN)?.into_output();
        let echo = Gpio::new()?.get(ECHO_PIN)?.into_input();

        Ok(Ultrasonic { trig, echo })
    }

    pub fn read(&mut self) -> u64 {
        // Set trigger pin low for 5 us
        self.trig.set_low();
        sleep(Duration::from_micros(5));

        // Generate a 10us pulse on trigger pin
        self.trig.set_high();
        sleep(Duration::from_micros(10));
        self.trig.set_low();

        // Wait for the echo pin to go high
        while !self.echo.is_high() {}

        let pulse_start = Instant::now();
        // Wait for the echo pin to go low
        while !self.echo.is_low() {}

        let time_taken = pulse_start.elapsed().as_micros();
        // Distance in cm
        (time_taken / 58) as u64
    }
}
