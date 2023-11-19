//! Motor Module

use anyhow::{Context, Result};
use rppal::gpio::{Level, OutputPin};

use crate::{
    pin::{PinType, RHPin},
    pwm::PWM,
};

// Motor Constants
const PERIOD: u16 = 4095;
const PRESCALER: u16 = 10;

struct Motor {
    pwm: PWM,
    dir: OutputPin,
}

impl Motor {
    fn new(pwm_pin: PinType, dir_pin: PinType) -> Result<Self> {
        let mut pwm = PWM::new(pwm_pin)?;
        let dir_pin = RHPin::new(dir_pin)?;
        let mut dir = dir_pin.gpio_pin.into_output();

        pwm.period(PERIOD)?;
        pwm.prescaler(PRESCALER)?;
        // Set motor to zero
        pwm.pulse_width_percent(0)?;
        dir.write(Level::Low);
        Ok(Self { pwm, dir })
    }

    fn speed(&mut self, speed: i8) -> Result<()> {
        let dir: Level = if speed > 0 { Level::High } else { Level::Low };

        self.pwm.pulse_width_percent(speed.unsigned_abs())?;
        self.dir.write(dir);

        Ok(())
    }
}

/// A robot-hat Motors
pub struct Motors {
    /// Left Motor is created using Pwm pin `P12` and direction pin `D4`
    left_motor: Motor,
    /// Right Motor is created using Pwm pin `P13` and direction pin `D5`
    right_motor: Motor,
}

impl Motors {
    /// Create motors using following config as per robot-hat (Python)
    ///
    /// Left Motor is created using Pwm pin `P12` and direction pin `D4`
    ///
    /// Right Motor is created using Pwm pin `P13` and direction pin `D5`
    pub fn new() -> Result<Self> {
        let left_motor_pwm_pin = PinType::P12;
        let left_motor_dir_pin = PinType::D4;
        let right_motor_pwm_pin = PinType::P13;
        let right_motor_dir_pin = PinType::D5;

        let left_motor =
            Motor::new(left_motor_pwm_pin, left_motor_dir_pin).context("LEFT MOTOR INIT FAILED")?;
        let right_motor = Motor::new(right_motor_pwm_pin, right_motor_dir_pin)
            .context("RIGHT MOTOR INIT FAILED")?;

        Ok(Self {
            left_motor,
            right_motor,
        })
    }

    /// Stop all motors
    pub fn stop(&mut self) {
        let _ = self.left_motor.speed(0);
        let _ = self.right_motor.speed(0);
    }

    /// Set motor speed
    ///
    /// Range --> (0 - 100)
    fn speed(&mut self, left_speed: i8, right_speed: i8) {
        let _ = self.left_motor.speed(left_speed);
        let _ = self.right_motor.speed(-right_speed); // Negating as per robot-hat python module
    }

    /// Move motors forward with `speed`
    ///
    /// Range --> (0 - 100)
    pub fn forward(&mut self, speed: i8) {
        self.speed(speed, speed);
    }

    /// Move motors backward with `speed`
    ///
    /// Range --> (0 - 100)
    pub fn backward(&mut self, speed: i8) {
        self.speed(-speed, -speed);
    }

    /// Move motors left with `speed`
    ///
    /// Range --> (0 - 100)
    pub fn turn_left(&mut self, speed: i8) {
        self.speed(-speed, speed);
    }

    /// Move motors right with `speed`
    ///
    /// Range --> (0 - 100)
    pub fn turn_right(&mut self, speed: i8) {
        self.speed(speed, -speed);
    }
}
