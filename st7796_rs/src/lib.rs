#![no_std]
// use core::cell::RefCell;

/**/
use embedded_hal::spi::{
    SpiBus,
    // MODE_0
};
use embedded_hal::digital::OutputPin;

// use fugit::RateExtU32;
use rp235x_hal::{
    self as hal, 
    // gpio::{FunctionSpi, Pins}, 
    pac, 
    spi::{
        // Disabled, 
        Enabled, 
        // Spi, 
        // SpiDevice, 
        // State, 
        ValidSpiPinout
    }, 
    // Sio,
    Timer,
};

use embedded_hal::delay::DelayNs;

// Some things we need
// use embedded_hal_0_2::prelude::*;
use hal::clocks::Clock;
use hal::fugit::RateExtU32;

pub mod instruction;

use crate::instruction::Command;

/// External high-speed crystal on the Raspberry Pi Pico 2 board is 12 MHz.
/// Adjust if your board has a different frequency
// const XTAL_FREQ_HZ: u32 = 12_000_000u32;

pub struct ST7796S<P, CS, DC, T>
where 
    P: ValidSpiPinout<pac::SPI0>,
    CS: OutputPin,
    DC: OutputPin,
    T: embedded_hal::delay::DelayNs
{
    interface: hal::spi::Spi<Enabled, pac::SPI0, P>,
    cs: CS,
    dc: DC,
    timer: T,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct RDDIDResult {
    lcd_manufacturer_id: u8,
    lcd_driver_version: u8,
    lcd_driver_id: u8,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct RNEDSIResult {
    num_errors: u8,
    has_overflow: bool,
    additional: u8,
}

#[derive(Debug)]
pub enum BoosterVoltageStatus {
    OFF,
    ON,
}

#[derive(Debug)]
pub enum AddressOrder {
    INCREMENT,
    DECREMENT,
}

#[derive(Debug)]
pub enum RowColumnExchange {
    NORMAL,
    EXCHANGE,
}

#[derive(Debug)]
pub enum RgbOrder {
    RGB,
    BGR,
}

#[derive(Debug)]
pub enum PixelFormat {
    Undefined,
    Bit16,
    Bit18,
    Bit24,
}

#[derive(Debug)]
pub enum OnOff {
    OFF,
    ON,
}
#[derive(Debug)]
pub enum InOut {
    IN,
    OUT,
}

#[derive(Debug)]
pub enum DisplayMode {
    PARTIAL,
    NORMAL,
}

#[derive(Debug)]
pub enum GammaCurveSelect {
    Undefined,
    GC0,
    GC1,
    GC2,
    GC3,
}

#[derive(Debug)]
pub enum TearingEffect {
    MODE1,
    MODE2,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct RDDSTResult {
    bston: BoosterVoltageStatus, // Booster voltage status
    my: AddressOrder,            // Row address order
    mx: AddressOrder,            // Column address order
    mv: RowColumnExchange,       // Row/column exchange
    ml: AddressOrder,            // Scan address order
    rgb: RgbOrder,               // RGB order
    ifpf: PixelFormat,           // Interface color pixel format definition
    idmon: OnOff,                // Idle mode on/off
    ptlon: OnOff,                // Partial mode on/off
    slpout: InOut,               // Sleep in/out
    noron: DisplayMode,          // Display normal mode on/off,
    st: OnOff,                   // Vertical scrolling status,
    invon: OnOff,                // Inversion status
    dison: OnOff,                // Display on/off
    teon: OnOff,                 // Tearing effect line on/off
    gcsel: GammaCurveSelect,     // Gamma curve selection
    tem: TearingEffect,          // Tearing effect line mode
}

pub enum InstructionInput {
    NoInput,
}

pub enum InstructionResult {
    NoReturn,
    RDDIDReturn(RDDIDResult),
    RNEDSIReturn(RNEDSIResult),
    RDDSTReturn(RDDSTResult),
}

// Callback for printing debug info
pub fn dummy_debug_info(_s: &str) {
    // Do nothing.
}

impl<P, CS, DC, T> ST7796S<P, CS, DC, T> 
where 
    P: ValidSpiPinout<pac::SPI0>,
    CS: OutputPin,
    DC: OutputPin,
    T: embedded_hal::delay::DelayNs,
{
    pub fn new<F: FnMut(&str)>(
            pins: P,
            clocks: rp235x_hal::clocks::ClocksManager,
            spi0: pac::SPI0,
            resets: &mut pac::RESETS,
            cs: CS,
            dc: DC,
            timer: T,
            mut debug_cb: F) -> Self {

        debug_cb("Initializing SPI device.\r\n");
        // These are implicitly used by the spi driver if they are in the correct mode
        let s = hal::spi::Spi::new(spi0, pins).init(
            resets,
            clocks.peripheral_clock.freq(),
            // 62500.kHz(),
            // 16.MHz(),
            8.MHz(),
            embedded_hal::spi::MODE_0,
        );

        ST7796S {
            interface: s,
            cs: cs,
            dc: dc,
            timer: timer,
        }
    }

    pub fn init(&mut self) {
        // self.write_command(&[0xCF, 0x00, 0x83, 0x30]);               // ?
        // self.write_command(&[0xED, 0x64, 0x03, 0x12, 0x81]);         // DOCA: display output ctrl adjust
        // self.write_command(&[0xE8, 0x85, 0x01, 0x79]);               // DOCA: display output ctrl adjust
        // self.write_command(&[0xCB, 0x39, 0x2C, 0x00, 0x34, 0x02]);   // ?
        // self.write_command(&[0xF7, 0x20]);                           // ?
        // self.write_command(&[0xEA, 0x00, 0x00]);                     // DOCA: display output ctrl adjust
        // self.write_command(&[0xC0, 0x26]);	                  	     // Power control
        // self.write_command(&[0xC1, 0x11]);	                  	     // Power control 
        // self.write_command(&[0xC5, 0x35, 0x3E]);                     // VCOM control
        // self.write_command(&[0xC7, 0xBE]);		                     // VCM Offset: vcom offset register
        // self.write_command(&[0x36, 0x28]);		                     // Memory Access Control
        // self.write_command(&[0x3A, 0x05]);		                     // COLMOD: Interface pixel format
        // self.write_command(&[0xB1, 0x00, 0x1B]);                     // FRMCTR1: frame rate control
        // self.write_command(&[0xB1, 0x00, 0x00]);                     // FRMCTR1: frame rate control
        // self.write_command(&[0xF2, 0x08]);                           // ?
        // self.write_command(&[0x26, 0x01]);                           // ?
        // self.write_command(&[0xE0, 0x1F, 0x1A, 0x18, 0x0A, 0x0F, 0x06, 0x45, 0x87, 0x32, 0x0A, 0x07, 0x02, 0x07, 0x05, 0x00]); // PGC: positive gamma control
        // self.write_command(&[0xE1, 0x00, 0x25, 0x27, 0x05, 0x10, 0x09, 0x3A, 0x78, 0x4D, 0x05, 0x18, 0x0D, 0x38, 0x3A, 0x1F]); // NGC: negative gamma control
        // self.write_command(&[0x2A, 0x00, 0x00, 0x00, 0xEF]);         // CASET: column address set
        // self.write_command(&[0x2B, 0x00, 0x00, 0x01, 0x3f]);         // RASET: row address set
        // self.write_command(&[0x2C, 0]);                              // RAMWR: memory write
        // self.write_command(&[0xB7, 0x07]);                           // EM: entry mode wet
        // self.write_command(&[0xB6, 0x0A, 0x82, 0x27, 0x00]);         // DFC: display function control
        // self.write_command(&[0x11, 0]);                              // SLP: sleep out
        // self.write_command(&[0x29, 0]);                              // DISPON: display on
        // self.write_command(&[0, 0]);                                 // NOP: no operation

        self.write_command(0x11);          // Sleep out
        self.timer.delay_ms(120);

        self.write_command(0x3A);          // Set pixel format
        self.write_data(&[0x55]);              // 16-bit color
        self.timer.delay_ms(120);

        self.write_command(0x36);          // Memory data access control
        self.write_data(&[0x00]);               // Row/col order
        self.timer.delay_ms(120);

        self.write_command(0x29);          // Display on
        self.timer.delay_ms(10);
        self.timer.delay_ms(120);
    }

    pub fn write_command(&mut self, cmd: u8) {
        self.cs.set_low().unwrap();
        self.interface.write(&[cmd]).unwrap();
        self.cs.set_high().unwrap();
    }

    fn write_data(&mut self, data: &[u8]) {
        self.dc.set_high().unwrap();        // Data mode
        self.interface.write(data).unwrap();
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

    pub fn loopback_test(&mut self) -> Result<InstructionResult, u8> {
        let tx_buffer: [u8; 4]  = [0x00, 0xAA, 0xFF, 0x55];
        let mut rx_buffer: [u8; 4] = [0u8; 4];
        match self.interface.transfer(&mut rx_buffer, &tx_buffer) {
            Ok(_x) => {
                if tx_buffer == rx_buffer {
                    Ok(InstructionResult::NoReturn)
                } else {
                    Err(1)
                }
            },
            _ => Err(0),
        }
    }

    // No operation
    pub fn nop(&mut self) -> Result<InstructionResult, u8> {
        self.cs.set_low().unwrap();
        let ret = match self.interface.write(&[0x00]) {
            Ok(_x) => { Ok(InstructionResult::NoReturn) },
            _ => { Err(0) },
        };
        self.cs.set_high().unwrap();
        ret
    }

    // Software reset
    pub fn swreset(&mut self) -> Result<InstructionResult, u8> {
        self.cs.set_low().unwrap();
        let ret = match self.interface.write(&[0x01]) {
            Ok(_x) => { Ok(InstructionResult::NoReturn) },
            _ => { Err(1) },
        };
        self.cs.set_high().unwrap();
        ret
    }

    // Read Display ID
    pub fn rddid(&mut self) -> Result<InstructionResult, u8> {
        self.cs.set_low().unwrap();
        self.dc.set_low().unwrap();
        let mut ret_words: [u8; 5] = [0x04, 0, 0, 0, 0];
        let ret = match self.interface.write(&[0x04]) {
            Ok(_x) => { 
                let mut buffer: [u8; 4] = [0, 0, 0, 0];
                self.dc.set_high().unwrap();
                // match self.interface.read(&mut ret_words) {
                match self.interface.transfer(&mut buffer, &[0u8]) {
                    Ok(_x) => Ok(
                        InstructionResult::RDDIDReturn(
                            RDDIDResult {
                                lcd_manufacturer_id: ret_words[1],
                                lcd_driver_version: ret_words[2],
                                lcd_driver_id: ret_words[3],
                            }
                        )
                    ),
                    Err(_) => Err(2),
                }
            },
            _ => { Err(3) },
        };
        self.cs.set_high().unwrap();
        ret
    }

    pub fn read_id(&mut self) -> [u8; 4] {
        let mut buffer = [0u8; 4];

        // Select the display
        self.cs.set_low().unwrap();
        
        // Send the RDDID command
        self.dc.set_low().unwrap();  // Command mode
        self.interface.write(&[0x04]).unwrap();
        self.timer.delay_ms(10);

        // Read 4 bytes of ID
        self.dc.set_high().unwrap();  // Data mode
        self.interface.transfer(&mut buffer, &[0x00, 0x00, 0x00, 0x00]).unwrap();

        // Deselect the display
        self.cs.set_high().unwrap();
        self.timer.delay_ms(10);

        buffer
    }

    // Read Number of the Errors on DSI
    pub fn rnedsi(&mut self) -> Result<InstructionResult, u8> {
        match self.interface.write(&[0x05]) {
            Ok(_x) => { 
                let mut ret_words: [u8; 2] = [0, 0];
                match self.interface.read(&mut ret_words) {
                    Ok(_x) => Ok(
                        InstructionResult::RNEDSIReturn(
                            RNEDSIResult {
                                num_errors: ret_words[1] & 0x7f,
                                has_overflow: ( ret_words[1] & 0x80 ) > 0,
                                additional: 0,
                            }
                        )
                    ),
                    Err(_) => Err(4),
                }
            },
            _ => { Err(5) },
        }
    }

    // Read Number of the Errors on DSI
    pub fn rddst(&mut self) -> Result<InstructionResult, u8> {
        match self.interface.write(&[0x05]) {
            Ok(_x) => { 
                let mut ret_words: [u8; 5] = [0, 0, 0, 0, 0];
                match self.interface.read(&mut ret_words) {
                    Ok(_x) => Ok(
                        InstructionResult::RDDSTReturn(
                            RDDSTResult {
                                bston: match ret_words[1] & 0x80 {
                                    0 => BoosterVoltageStatus::OFF,
                                    _ => BoosterVoltageStatus::ON,
                                },
                                my: match ret_words[1] & 0x40 {
                                    0 => AddressOrder::INCREMENT,
                                    _ => AddressOrder::DECREMENT,
                                },           
                                mx: match ret_words[1] & 0x20 {
                                    0 => AddressOrder::INCREMENT,
                                    _ => AddressOrder::DECREMENT,
                                },           
                                mv: match ret_words[1] & 0x10 {
                                    0 => RowColumnExchange::NORMAL,
                                    _ => RowColumnExchange::EXCHANGE,
                                },      
                                ml: match ret_words[1] & 0x08 {
                                    0 => AddressOrder::INCREMENT,
                                    _ => AddressOrder::DECREMENT,
                                },           
                                rgb: match ret_words[1] & 0x04 {
                                    0 => RgbOrder::RGB,
                                    _ => RgbOrder::BGR,
                                },              
                                ifpf: match ret_words[2] & 0x70 {
                                    0x50 => PixelFormat::Bit16,
                                    0x60 => PixelFormat::Bit18,
                                    0x70 => PixelFormat::Bit24,
                                    _ => PixelFormat::Undefined,
                                },          
                                idmon: match ret_words[2] & 0x08 {
                                    0 => OnOff::OFF,
                                    _ => OnOff::ON,
                                },               
                                ptlon: match ret_words[2] & 0x04 {
                                    0 => OnOff::OFF,
                                    _ => OnOff::ON,
                                },               
                                slpout: match ret_words[2] & 0x02 {
                                    0 => InOut::IN,
                                    _ => InOut::OUT,
                                },              
                                noron: match ret_words[2] & 0x01 {
                                    0 => DisplayMode::PARTIAL,
                                    _ => DisplayMode::NORMAL,
                                },         
                                st: match ret_words[3] & 0x80 {
                                    0 => OnOff::OFF,
                                    _ => OnOff::ON,
                                },                  
                                invon: match ret_words[3] & 0x20 {
                                    0 => OnOff::OFF,
                                    _ => OnOff::ON,
                                },               
                                dison: match ret_words[3] & 0x04 {
                                    0 => OnOff::OFF,
                                    _ => OnOff::ON,
                                },               
                                teon: match ret_words[3] & 0x02 {
                                    0 => OnOff::OFF,
                                    _ => OnOff::ON,
                                },                
                                gcsel: match ret_words[3] & 0x01 {
                                    0 => match ret_words[4] & 0xc0 {
                                        0x0 => GammaCurveSelect::GC0,
                                        0x40 => GammaCurveSelect::GC1,
                                        0x80 => GammaCurveSelect::GC2,
                                        0xc0 => GammaCurveSelect::GC3,
                                        _ => GammaCurveSelect::Undefined,
                                    },
                                    _ => GammaCurveSelect::Undefined,
                                },
                                tem: match ret_words[4] & 0x20 {
                                    0 => TearingEffect::MODE1,
                                    _ => TearingEffect::MODE2,
                                }
                            }
                        )
                    ),
                    Err(_) => Err(4),
                }
            },
            _ => { Err(5) },
        }
    }

    // Enter inversion off mode
    pub fn invoff(&mut self) -> Result<InstructionResult, u8> {
        match self.interface.write(&[0x20]) {
            Ok(_x) => { Ok(InstructionResult::NoReturn) },
            _ => { Err(1) },
        }
    }

    // Enter inversion off mode
    pub fn invon(&mut self) -> Result<InstructionResult, u8> {
        match self.interface.write(&[0x21]) {
            Ok(_x) => { Ok(InstructionResult::NoReturn) },
            _ => { Err(1) },
        }
    }

    // Enter display off mode
    pub fn dispoff(&mut self) -> Result<InstructionResult, u8> {
        match self.interface.write(&[0x28]) {
            Ok(_x) => { Ok(InstructionResult::NoReturn) },
            _ => { Err(1) },
        }
    }

    // Enter display on mode
    pub fn dispon(&mut self) -> Result<InstructionResult, u8> {
        match self.interface.write(&[0x29]) {
            Ok(_x) => { Ok(InstructionResult::NoReturn) },
            _ => { Err(1) },
        }
    }

    pub fn wrdisbv(&mut self, val: u8) -> Result<InstructionResult, u8> {
        match self.interface.write(&[Command::WRDISBV as u8, val]) {
            Ok(_x) => { Ok(InstructionResult::NoReturn) },
            _ => { Err(1) },
        }
    }
}