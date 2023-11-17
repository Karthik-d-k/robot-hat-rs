//! Pin abstraction layer for robot-hat

use anyhow::{Context, Result};
use rppal::gpio::{self, Gpio};

const BOARD_TYPE: u8 = 12;

fn check_board_type() -> Result<bool> {
    let type_pin = Gpio::new()?.get(BOARD_TYPE)?.into_input();

    Ok(type_pin.is_low())
}

/// An explicit allowable types for [`RHPin`]
#[derive(Copy, Clone, Debug)]
pub enum PinType {
    /// The Digital pin 0
    D0,
    /// The Digital pin 1
    D1,
    /// The Digital pin 2
    D2,
    /// The Digital pin 3
    D3,
    /// The Digital pin 4
    D4,
    /// The Digital pin 5
    D5,
    /// The Digital pin 6
    D6,
    /// The Digital pin 7
    D7,
    /// The Digital pin 8
    D8,
    /// The Digital pin 9
    D9,
    /// The Digital pin 10
    D10,
    /// The Digital pin 11
    D11,
    /// The Digital pin 12
    D12,
    /// The Digital pin 13
    D13,
    /// The Digital pin 14
    D14,
    /// The Digital pin 15
    D15,
    /// The Digital pin 16
    D16,
    /// The USR button
    SW,
    /// The USR button
    User,
    /// The LED on the board
    Led,
    /// The board type pin
    BoardType,
    /// The Reset pin
    Rst,
    /// The Ble interrupt pin ?
    BleInt,
    /// The Ble reset pin ?
    BleRst,
    /// The MCU reset pin
    McuRst,
}

impl PinType {
    fn bcm_num(&self, board_type: bool) -> u8 {
        match self {
            PinType::D0 => 17,
            PinType::D1 => {
                if board_type {
                    18
                } else {
                    4
                }
            }
            PinType::D2 => 27,
            PinType::D3 => 22,
            PinType::D4 => 23,
            PinType::D5 => 24,
            PinType::D6 => 25,
            PinType::D7 => 4,
            PinType::D8 => 5,
            PinType::D9 => 6,
            PinType::D10 => 12,
            PinType::D11 => 13,
            PinType::D12 => 19,
            PinType::D13 => 16,
            PinType::D14 => 26,
            PinType::D15 => 20,
            PinType::D16 => 21,
            PinType::SW => {
                if board_type {
                    19
                } else {
                    25
                }
            }
            PinType::User => {
                if board_type {
                    19
                } else {
                    25
                }
            }
            PinType::Led => 26,
            PinType::BoardType => 12,
            PinType::Rst => 16,
            PinType::BleInt => 13,
            PinType::BleRst => 20,
            PinType::McuRst => {
                if board_type {
                    21
                } else {
                    5
                }
            }
        }
    }
}

/// A robot-hat pin
#[derive(Debug)]
pub struct RHPin {
    /// A [`gpio::Pin`] from [`rppal`] crate
    pub gpio_pin: gpio::Pin,
}

impl RHPin {
    /// Create a new robot-hat pin using any [`PinType`]
    pub fn new(pin_type: PinType) -> Result<Self> {
        let board_type = check_board_type().context("Checking Board type failed")?;
        let bcm_num = pin_type.bcm_num(board_type);
        let gpio_pin = Gpio::new()?.get(bcm_num)?;

        Ok(Self { gpio_pin })
    }
}
