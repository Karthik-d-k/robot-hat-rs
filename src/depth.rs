//! Ultrasonic module implementation

use std::thread::sleep;
use std::time::{Duration, Instant};

use anyhow::{bail, Context, Result};
use rppal::gpio::{InputPin, OutputPin};

use crate::pin::{PinType, RHPin};

/// Ultrasonic ranging sensor
pub struct Ultrasonic {
    trig: OutputPin,
    echo: InputPin,
}

impl Ultrasonic {
    /// Create ultrasonic ranging sensor using trigger and echo pins with [`PinType`]  *(D0-D16)*
    pub fn new(trig_pin: PinType, echo_pin: PinType) -> Result<Self> {
        // check if digital pins are passed
        if !trig_pin.is_digital_pin() {
            bail!(
                "trig_pin should be one of PinType::D0-D16, but passed {:?}",
                trig_pin
            )
        }
        if !echo_pin.is_digital_pin() {
            bail!(
                "echo_pin should be one of PinType::D0-D16, but passed {:?}",
                echo_pin
            )
        }

        let trig_pin = RHPin::new(trig_pin)
            .with_context(|| format!("Creating trigger pin using {:?} failed", trig_pin))?;
        let echo_pin = RHPin::new(echo_pin)
            .with_context(|| format!("Creating echo pin using {:?} failed", echo_pin))?;
        let trig = trig_pin.gpio_pin.into_output();
        let echo = echo_pin.gpio_pin.into_input();

        Ok(Ultrasonic { trig, echo })
    }

    /// Read distance values in `cm`
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
