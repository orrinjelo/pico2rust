#![no_std]
/*

*/
use embedded_hal::spi::MODE_0;
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

impl<P> ST7796S<P> 
where 
    P: ValidSpiPinout<pac::SPI0>,
{
    pub fn new(pins: P) -> Self {
        let mut pac = hal::pac::Peripherals::take().unwrap();
        // The single-cycle I/O block controls our GPIO pins
        // let sio = hal::Sio::new(pac.SIO);

        // // Set the pins to their default state
        // let pins = hal::gpio::Pins::new(
        //     pac.IO_BANK0,
        //     pac.PADS_BANK0,
        //     sio.gpio_bank0,
        //     &mut pac.RESETS,
        // );

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
        // let spi_mosi = pins.gpio7.into_function::<hal::gpio::FunctionSpi>();
        // let spi_miso = pins.gpio4.into_function::<hal::gpio::FunctionSpi>();
        // let spi_sclk = pins.gpio6.into_function::<hal::gpio::FunctionSpi>();
        let s = hal::spi::Spi::new(pac.SPI0, pins).init(
            &mut pac.RESETS,
            clocks.peripheral_clock.freq(),
            16.MHz(),
            embedded_hal::spi::MODE_0,
        );

        ST7796S {
            interface: s
        }
    }

    // pub fn nop(&mut self) {
    //     self.interface.send(&[0x00]);
    // }
}