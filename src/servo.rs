//! Servo Module

use anyhow::Result;

use crate::{pin::PinType, pwm::PWM, utils::map_range};

// Servo Constants
const FREQ: u32 = 50;
const PERIOD: u32 = 4095;
const MIN_PW: u16 = 500;
const MAX_PW: u16 = 2500;
const CPU_CLOCK: u32 = 72_000_000;

/// A robot-hat Servo
pub struct Servo {
    pwm: PWM,
}

impl Servo {
    /// Create a new robot-hat servo pin with [`PinType`]  *(P0-P13)*
    pub fn new(pwm_pin: PinType) -> Result<Self> {
        let mut pwm = PWM::new(pwm_pin)?;
        pwm.period(PERIOD as u16)?;
        let prescaler = ((CPU_CLOCK / FREQ) / PERIOD) as u16;
        pwm.prescaler(prescaler)?;

        Ok(Self { pwm })
    }

    /// Set the pulse width of the servo motor
    ///
    /// Range --> (500 - 2500)
    pub fn pulse_width_time(&mut self, pw_time: i32) -> Result<()> {
        let value = ((pw_time * 4095) / 20000) as u16; // 20,000 us --> 20ms (50Hz signal for servo)
        self.pwm.pulse_width(value)?;

        Ok(())
    }

    /// Set the angle of the servo motor
    ///
    /// Range --> (-90.0 - 90.0)
    pub fn angle(&mut self, angle: f32) {
        let angle = angle.clamp(-90.0, 90.0);
        let pw_time = map_range(angle, (-90.0, 90.0), (MIN_PW.into(), MAX_PW.into()));
        let pw_time = pw_time.clamp(MIN_PW.into(), MAX_PW.into()) as i32;
        let _ = self.pulse_width_time(pw_time);
    }
}
