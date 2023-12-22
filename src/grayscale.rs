//! Grayscale module implementation

use anyhow::{Context, Result};

use crate::adc::ADC;
use crate::pin::PinType;

const GRAYSCALE_REFS: [u16; 3] = [1000, 1000, 1000];

/// 3 channel Grayscale sensor
pub struct Grayscale {
    channels: [ADC; 3],
    refs: [u16; 3],
}

impl Grayscale {
    /// Create a Grayscale sensor with 3 default channels using 3 [`ADC`]` pins with [`PinType`] *(A0-A7)*
    pub fn new(left: PinType, middle: PinType, right: PinType) -> Result<Self> {
        let left = ADC::new(left).context("Creating ADC Pin failed")?;
        let middle = ADC::new(middle).context("Creating ADC Pin failed")?;
        let right = ADC::new(right).context("Creating ADC Pin failed")?;

        let channels = [left, middle, right];

        Ok(Grayscale {
            channels,
            refs: GRAYSCALE_REFS,
        })
    }

    /// Set reference analog values for the channels `(default: 1000)`
    pub fn set_reference_values(mut self, refs: [u16; 3]) {
        self.refs = refs;
    }

    /// Read all 3 ADC channel values
    pub fn read_values(mut self) -> Result<[u16; 3]> {
        let mut values = [0, 0, 0];
        values[0] = self.channels[0]
            .read_value()
            .context("Reading ADC value failed")?;
        values[1] = self.channels[1]
            .read_value()
            .context("Reading ADC value failed")?;
        values[2] = self.channels[2]
            .read_value()
            .context("Reading ADC value failed")?;

        Ok(values)
    }

    /// Read Grayscale sensor statuses
    ///
    /// Array of line status, `true` for `white`, `false` for `black`
    pub fn read_status(self) -> Result<[bool; 3]> {
        let refs = self.refs;
        let values = self.read_values()?;
        let mut status = [false, false, false];
        for (i, v) in values.into_iter().enumerate() {
            status[i] = v > refs[i];
        }

        Ok(status)
    }
}
