#![no_std]
/**/
use embedded_hal::spi::{SpiBus, MODE_0};
// use fugit::RateExtU32;
use rp235x_hal::{
    self as hal, gpio::{FunctionSpi, Pins}, pac, spi::{Disabled, Enabled, Spi, SpiDevice, State, ValidSpiPinout}, Sio
};

// Some things we need
// use embedded_hal_0_2::prelude::*;
use hal::clocks::Clock;
use hal::fugit::RateExtU32;

pub mod instruction;

use crate::instruction::Command;

/// External high-speed crystal on the Raspberry Pi Pico 2 board is 12 MHz.
/// Adjust if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

pub struct ST7796S<P>
where 
    P: ValidSpiPinout<pac::SPI0>,
{
    interface: hal::spi::Spi<Enabled, pac::SPI0, P>,
}

pub struct RDDIDResult {
    lcd_manufacturer_id: u8,
    lcd_driver_version: u8,
    lcd_driver_id: u8,
}

pub enum InstructionInput {
    NoInput,
}

pub enum InstructionResult {
    NoReturn,
    RDDIDReturn(RDDIDResult),
}

impl<P> ST7796S<P> 
where 
    P: ValidSpiPinout<pac::SPI0>,
{
    pub fn new(pins: P) -> Self {
        let mut pac = hal::pac::Peripherals::take().unwrap();

        // Set up the watchdog driver - needed by the clock setup code
        let mut watchdog = hal::Watchdog::new(pac.WATCHDOG);
        
        // Configure the clocks
        let clocks = hal::clocks::init_clocks_and_plls(
            XTAL_FREQ_HZ,
            pac.XOSC,
            pac.CLOCKS,
            pac.PLL_SYS,
            pac.PLL_USB,
            &mut pac.RESETS,
            &mut watchdog,
        )
        .unwrap();


        // These are implicitly used by the spi driver if they are in the correct mode
        let s = hal::spi::Spi::new(pac.SPI0, pins).init(
            &mut pac.RESETS,
            clocks.peripheral_clock.freq(),
            16.MHz(),
            embedded_hal::spi::MODE_0,
        );

        // // Exchange the uninitialised SPI driver for an initialised one
        // let mut spi_bus = s.init(
        //     &mut pac.RESETS,
        //     clocks.peripheral_clock.freq(),
        //     16.MHz(),
        //     embedded_hal::spi::MODE_0,
        // );

        ST7796S {
            interface: s
        }
    }

    pub fn init(&mut self) {

    }

    pub fn exec(&mut self, command: Command, _inp: InstructionInput) -> Result<InstructionResult, u8> {
        match command {
            Command::NOP => self.nop(),
            Command::SWRESET => self.swreset(),
            Command::RDDID => {
                self.rddid()
            },
            _ => Ok(InstructionResult::NoReturn)
        }
    }

    // No operation
    pub fn nop(&mut self) -> Result<InstructionResult, u8> {
        match self.interface.write(&[0x00]) {
            Ok(_x) => { Ok(InstructionResult::NoReturn) },
            _ => { Err(0) },
        }
    }

    // Software reset
    pub fn swreset(&mut self) -> Result<InstructionResult, u8> {
        match self.interface.write(&[0x01]) {
            Ok(_x) => { Ok(InstructionResult::NoReturn) },
            _ => { Err(1) },
        }
    }

    // Read Display ID
    pub fn rddid(&mut self) -> Result<InstructionResult, u8> {
        match self.interface.write(&[0x01]) {
            Ok(_x) => { 
                let mut ret_words: [u8; 4] = [0, 0, 0, 0];
                match self.interface.read(&mut ret_words) {
                    Ok(_x) => Ok(
                        InstructionResult::RDDIDReturn(
                            RDDIDResult {
                                lcd_manufacturer_id: ret_words[1],
                                lcd_driver_version: ret_words[2],
                                lcd_driver_id: ret_words[3],
                            }
                        )
                    ),
                    Err(_) => Err(1),
                }
            },
            _ => { Err(1) },
        }
    }
}