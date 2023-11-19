//! Utilities for robot-hat

use std::{thread::sleep, time::Duration};

use anyhow::{Context, Result};
use rppal::i2c::I2c;

const I2C_BUS: u8 = 1;
const SLAVE_ADDR: u16 = 0x14;

/// Initialize robot-hat I2C
pub fn init_i2c() -> Result<I2c> {
    let mut i2c = I2c::with_bus(I2C_BUS).context("Constructing new I2C failed")?;
    // wait after I2C init to avopid 121 IO error
    sleep(Duration::from_secs(1));

    i2c.set_slave_address(SLAVE_ADDR)
        .context("Setting SLAVE addr failed")?;
    i2c.smbus_send_byte(0x2C)
        .context("Sending byte 0x2c failed")?;
    i2c.smbus_send_byte(0x00)
        .context("Sending byte 0x00 failed")?;
    i2c.smbus_send_byte(0x00)
        .context("Sending byte 0x00 failed")?;

    Ok(i2c)
}

/// Map value from one range (`from_range`) to another range (`to_range`)
pub fn map_range(value: f32, from_range: (f32, f32), to_range: (f32, f32)) -> f32 {
    to_range.0 + (value - from_range.0) * (to_range.1 - to_range.0) / (from_range.1 - from_range.0)
}
