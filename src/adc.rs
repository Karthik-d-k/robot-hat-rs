//! ADC Module

use anyhow::{Context, Result};
use rppal::i2c::I2c;

use crate::{pin::PinType, utils::init_i2c};

/// A robot-hat ADC
#[derive(Debug)]
pub struct ADC {
    reg: u8,
    bus: I2c,
}

impl ADC {
    /// Create a new robot-hat adc pin with [`PinType`]  *(A0-A7)*
    pub fn new(adc_pin: PinType) -> Result<Self> {
        let channel = 7 - adc_pin.adc_channel();
        let reg = channel | 16; // 0x10

        let bus = init_i2c().context("I2C INIT FAILED")?;
        let adc = Self { reg, bus };

        Ok(adc)
    }

    /// Read the adc channel value
    ///
    /// Range --> (0 - 4095)
    pub fn read_value(&mut self) -> Result<u16> {
        self.bus
            .smbus_write_word(self.reg, 0)
            .context("ADC READ INIT FAILED")?;

        let value_h = self
            .bus
            .smbus_read_byte(self.reg)
            .context("ADC READ (MSByte) FAILED")? as u16;
        let value_l = self
            .bus
            .smbus_read_byte(self.reg)
            .context("ADC READ (LSByte) FAILED")? as u16;

        let value = ((value_h) << 8) + value_l;

        Ok(value)
    }

    /// Read the adc channel voltage
    ///
    /// Range --> (0 - 3.3V)
    pub fn read_voltage(&mut self) -> Result<f32> {
        let value = self.read_value()?;
        let value = (value as f32) * 3.3 / 4095.0;

        Ok(value)
    }
}
