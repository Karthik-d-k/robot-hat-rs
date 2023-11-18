//! PWM Module

use anyhow::{Context, Result};
use rppal::i2c::I2c;

use crate::{pin::PinType, utils::init_i2c};

const REG_PW: u8 = 0x20; // REG_CHN
const REG_PSC: u8 = 0x40; // REG_PSC
const REG_PER: u8 = 0x44; // REG_ARR

/// A robot-hat PWM
pub struct PWM {
    channel: u8,
    period: Vec<u16>,
    bus: I2c,
}

impl PWM {
    /// Create a new robot-hat pwm pin with [`PinType`]  *(P0-P13)*
    pub fn new(pwm_pin: PinType) -> Result<Self> {
        let channel = pwm_pin.pwm_channel();
        let period = vec![0, 0, 0, 0];
        let bus = init_i2c().context("I2C INIT FAILED")?;
        let mut pwm = Self {
            channel,
            period,
            bus,
        };

        pwm.freq(50).context("PWM FREQ INIT FAILED")?;

        Ok(pwm)
    }

    /// Set the frequency of the pwm channel
    pub fn freq(&mut self, freq: u16) -> Result<()> {
        /*  Buggy code: For now, we hardcode the values
                let mut result_psc = Vec::with_capacity(12); // Create a vector for prescaler
                let mut result_per = Vec::with_capacity(12); // Create a vector for period
                let mut result_acy = Vec::with_capacity(12); // Create a vector for accuracy

                let st = ((CLOCK as f32 / freq as f32).sqrt() as u16) - 5;

                for psc in st..st + 10 {
                    let per = (CLOCK / (freq * psc) as u32) as u16;
                    result_psc.push(psc);
                    result_per.push(per);
                    result_acy.push(f32::abs(freq as f32 - CLOCK as f32 / (psc * per) as f32));
                }

                let i = result_acy
                    .iter()
                    .position(|&x| x == result_acy.iter().cloned().fold(f32::INFINITY, f32::min))
                    .unwrap();
                let psc = result_psc[i];
                let per = result_per[i];
        */
        let psc: u16 = freq * 24; // 1200
        let per: u16 = freq * 24; // 1200

        self.prescaler(psc).context("PWM PRESCALER INIT FAILED")?;
        self.period(per).context("PWM PERIOD INIT FAILED")?;

        Ok(())
    }

    /// Set the prescaler for the pwm channel
    pub fn prescaler(&mut self, prescaler: u16) -> Result<()> {
        let timer = self.channel / 4_u8;
        let reg = REG_PSC + timer;
        self.bus
            .smbus_write_word(reg, (prescaler - 1).swap_bytes())
            .context("PWM PRESCALER SEND FAILED")?;

        Ok(())
    }

    /// Set the period for the pwm channel
    pub fn period(&mut self, per: u16) -> Result<()> {
        let timer = self.channel / 4_u8;
        let reg = REG_PER + timer;
        self.period[timer as usize] = per - 1;
        self.bus
            .smbus_write_word(reg, self.period[timer as usize].swap_bytes())
            .context("PWM PERIOD SEND FAILED")?;

        Ok(())
    }

    /// Set the pulse width for the pwm channel
    pub fn pulse_width(&mut self, pw: u16) -> Result<()> {
        let reg = REG_PW + self.channel;
        self.bus
            .smbus_write_word(reg, pw.swap_bytes())
            .context("PWM PULSE WIDTH SEND FAILED")?;

        Ok(())
    }

    /// Set the pulse width percentage for the pwm channel
    pub fn pulse_width_percent(&mut self, pulse_width_percent: u32) -> Result<()> {
        // Buggy code ? !!
        let timer = self.channel / 4_u8;
        let pulse_width = ((self.period[timer as usize] as u32 * pulse_width_percent) / 100) as u16;
        self.pulse_width(pulse_width)?;

        Ok(())
    }
}
