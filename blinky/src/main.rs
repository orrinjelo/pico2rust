//! # GPIO 'Blinky' Example
//!
//! This application demonstrates how to control a GPIO pin on the rp235x.
//!
//! It may need to be adapted to your particular board layout and/or pin assignment.
//!
//! See the `Cargo.toml` file for Copyright and license details.

#![no_std]
#![no_main]

use cortex_m::prelude::_embedded_hal_blocking_spi_Transfer;
// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

use rp235x_hal::gpio::bank0::{Gpio2, Gpio3, Gpio4};
use rp235x_hal::gpio::{Pin, PullDown};
use rp235x_hal::pac::SPI0;
use rp235x_hal::spi::Enabled;
// Alias for our HAL crate
use rp235x_hal as hal;
use hal::clocks::Clock;

// Some things we need
use embedded_hal::{delay::DelayNs, spi::SpiBus};
use embedded_hal::digital::OutputPin;

use embedded_hal::spi::MODE_0;
use fugit::RateExtU32;
use rp235x_hal::{
    gpio::{FunctionSpi, Pins},
    spi::Spi,
    Sio,
};

// Starting display
use st7796s::*;

/// Tell the Boot ROM about our application
#[link_section = ".start_block"]
#[used]
pub static IMAGE_DEF: hal::block::ImageDef = hal::block::ImageDef::secure_exe();

/// External high-speed crystal on the Raspberry Pi Pico 2 board is 12 MHz.
/// Adjust if your board has a different frequency
const XTAL_FREQ_HZ: u32 = 12_000_000u32;

/// Entry point to our bare-metal application.
///
/// The `#[hal::entry]` macro ensures the Cortex-M start-up code calls this function
/// as soon as all global variables and the spinlock are initialised.
///
/// The function configures the rp235x peripherals, then toggles a GPIO pin in
/// an infinite loop. If there is an LED connected to that pin, it will blink.
#[hal::entry]
fn main() -> ! {
    // Grab our singleton objects
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

    let mut timer = hal::Timer::new_timer0(pac.TIMER0, &mut pac.RESETS, &clocks);

    // The single-cycle I/O block controls our GPIO pins
    let sio = hal::Sio::new(pac.SIO);

    // Set the pins to their default state
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    // let mut sclk = pins.gpio2.into_push_pull_output();
    // let mut mosi = pins.gpio3.into_push_pull_output();
    // let mut miso = pins.gpio4.into_push_pull_output();
    // let mut cs = pins.gpio5.into_push_pull_output();
    // let mut dc = pins.gpio6.into_push_pull_output();
    // let mut rst = pins.gpio7.into_push_pull_output();
    // let mut i2c_0 = pins.gpio8.into_push_pull_output();
    // let mut i2c_1 = pins.gpio9.into_push_pull_output();
    // let mut tprst = pins.gpio10.into_push_pull_output();
    // let mut tpint = pins.gpio11.into_push_pull_output();

    // let mut led_rgb = pins.gpio12.into_push_pull_output();

    let sclk = pins.gpio2.into_function::<FunctionSpi>();
    let mosi = pins.gpio3.into_function::<FunctionSpi>();
    let miso = pins.gpio4.into_function::<FunctionSpi>();
    // let spi_device = pac.SPI0;
    // let spi_pin_layout = (mosi, sclk);
    let spi_bus = hal::spi::Spi::<_, _, _, 8>::new(
        pac.SPI0, (mosi, miso, sclk));

    // Configure some LED pins
    let mut led_pin_d1 = pins.gpio16.into_push_pull_output();
    let mut led_pin_d2 = pins.gpio17.into_push_pull_output();
    
    // LCD
    // let mut lcd = ST7796::new(Some(din), Some(rst), None, 480, 320);
    // Exchange the uninitialised SPI driver for an initialised one
    let mut spi_bus = spi_bus.init(
        &mut pac.RESETS,
        clocks.peripheral_clock.freq(),
        16.MHz(),
        embedded_hal::spi::MODE_0,
    );

    // Do a read+write at the same time. Data in `buffer` will be replaced with
    // the data read from the SPI device.
    // let mut buffer: [u8; 4] = [1, 2, 3, 4];
    display_send(spi_bus, &[1u8,2u8,3u8,4u8,0u8,0u8,0u8,0u8]);

    // loop {
    //     hal::arch::wfi();
    // }
    loop {
        led_pin_d1.set_high().unwrap();
        led_pin_d2.set_low().unwrap();
        timer.delay_ms(500);
        led_pin_d2.set_high().unwrap();
        led_pin_d1.set_low().unwrap();
        timer.delay_ms(500);
    }
}

pub fn display_send(mut spi_bus: Spi<Enabled, SPI0, (Pin<Gpio3, FunctionSpi, PullDown>, Pin<Gpio4, FunctionSpi, PullDown>, Pin<Gpio2, FunctionSpi, PullDown>)>, &buffer: &[u8; 8]) {
    let outp: [u8; 8] = [0; 8];
    let mut buff = buffer;
    let transfer_success = SpiBus::transfer(&mut spi_bus, &mut buff, &outp);
    #[allow(clippy::single_match)]
    match transfer_success {
        Ok(_) => {}  // Handle success
        Err(_) => {} // handle errors
    };

}

/// Program metadata for `picotool info`
#[link_section = ".bi_entries"]
#[used]
pub static PICOTOOL_ENTRIES: [hal::binary_info::EntryAddr; 5] = [
    hal::binary_info::rp_cargo_bin_name!(),
    hal::binary_info::rp_cargo_version!(),
    hal::binary_info::rp_program_description!(c"Blinky Example"),
    hal::binary_info::rp_cargo_homepage_url!(),
    hal::binary_info::rp_program_build_attribute!(),
];

// End of file
