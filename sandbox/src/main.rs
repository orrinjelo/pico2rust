#![no_std]
#![no_main]

use core::fmt::{
    // self, 
    Write
};

// use cortex_m::prelude::_embedded_hal_blocking_spi_Transfer;
// Ensure we halt the program on panic (if we don't mention this crate it won't
// be linked)
use panic_halt as _;

// use rp235x_hal::gpio::bank0::{Gpio2, Gpio3, Gpio4};
// use rp235x_hal::gpio::{Pin, PullDown};
// use rp235x_hal::pac::SPI0;
// use rp235x_hal::spi::Enabled;
// Alias for our HAL crate
use rp235x_hal as hal;
use rp235x_hal::{
    // gpio::Pins,
    uart::{DataBits, StopBits, UartConfig, UartPeripheral},
    // fugit::RateExtU32,
    fugit::Rate,
    clocks::Clock,
};
// use hal::clocks::Clock;


// Some things we need
use embedded_hal::{
    delay::DelayNs,
    // spi::SpiBus
};
// use embedded_hal::digital::OutputPin;

// use embedded_hal::spi::MODE_0;
// use fugit::RateExtU32;
// use rp235x_hal::{
//     gpio::{FunctionSpi, Pins},
//     spi::Spi,
//     Sio,
// };

// Starting display
use st7796_rs::*;

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

     // Configure UART pins (GPIO0 for TX, GPIO1 for RX)
    let uart_pins = (pins.gpio0.into_function(), pins.gpio1.into_function());

    // Configure UART peripheral
    let mut uart = UartPeripheral::new(pac.UART0, uart_pins, &mut pac.RESETS)
        .enable(
            UartConfig::new(Rate::<u32, 1, 1>::from_raw(115_200u32), DataBits::Eight, None, StopBits::One),
            clocks.peripheral_clock.freq(),
        )
        .unwrap();
    uart.write_str("UART initialized\r\n").unwrap();

    // These are implicitly used by the spi driver if they are in the correct mode
    let spi_mosi = pins.gpio3.into_function::<hal::gpio::FunctionSpi>();
    let spi_miso = pins.gpio4.into_function::<hal::gpio::FunctionSpi>();
    let spi_sclk = pins.gpio2.into_function::<hal::gpio::FunctionSpi>();
    let cs: rp235x_hal::gpio::Pin<_, _, _> = pins.gpio5.into_push_pull_output();
    let dc = pins.gpio6.into_push_pull_output();
    let rst = pins.gpio7.into_push_pull_output();

    let spi_pins = (spi_mosi, spi_miso, spi_sclk);
    // let led_pin_d1 = pins.gpio16.into_push_pull_output();

    uart.write_str("Creating display SPI...\r\n").unwrap();
    let mut display = ST7796S::new(spi_pins, clocks, pac.SPI0, &mut pac.RESETS, cs, dc, rst, timer, |s: &str| { uart.write_str(s).unwrap(); });
    display.init();

    let rdid = display.rddid().unwrap();
    uart.write_fmt(format_args!("Display ID: {:?}\r\n", rdid)).unwrap();

    let rdst = display.rddst().unwrap();
    uart.write_fmt(format_args!("Display Status: {:?}\r\n", rdst)).unwrap();

    // match display.loopback_test() {
    //     Ok(_) => {
    //         uart.write_str("Loopback test succeeded. \r\n").unwrap();
    //     },
    //     Err(_) => {
    //         uart.write_str("Loopback test failed. \r\n").unwrap();
    //     }
    // }

    // cs.set_low().unwrap();
        
    // Send the RDDID command
    // self.dc.set_low().unwrap();  // Command mode
    // self.interface.write(&[0x04]).unwrap();
    // self.timer.delay_ms(10);

    // // Read 4 bytes of ID
    // self.dc.set_high().unwrap();  // Data mode
    // self.interface.transfer(&mut buffer, &[0x00, 0x00, 0x00, 0x00]).unwrap();

    // // Deselect the display
    // self.cs.set_high().unwrap();
    // self.timer.delay_ms(10);

    loop {
        // hal::arch::wfi();
        // uart.write_str("LED blink!\r\n").unwrap();
        // display.wrdisbv(0xff).unwrap();
        // led_pin_d1.set_high().unwrap();
        // dc.set_low().unwrap();
        display.dispoff().unwrap();
        timer.delay_ms(500);
        // display.wrdisbv(0x00).unwrap();
        // led_pin_d1.set_low().unwrap();
        // dc.set_high().unwrap();
        display.dispon().unwrap();
        timer.delay_ms(500);
    }
}

/// Program metadata for `picotool info`
#[link_section = ".bi_entries"]
#[used]
pub static PICOTOOL_ENTRIES: [hal::binary_info::EntryAddr; 5] = [
    hal::binary_info::rp_cargo_bin_name!(),
    hal::binary_info::rp_cargo_version!(),
    hal::binary_info::rp_program_description!(c"Sandbox"),
    hal::binary_info::rp_cargo_homepage_url!(),
    hal::binary_info::rp_program_build_attribute!(),
];

// End of file
