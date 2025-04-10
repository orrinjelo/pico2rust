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
    // Timer,
};

// use embedded_hal::delay::DelayNs;

// Some things we need
// use embedded_hal_0_2::prelude::*;
use hal::clocks::Clock;
use hal::fugit::RateExtU32;

pub mod instruction;

use crate::instruction::Command;

/// External high-speed crystal on the Raspberry Pi Pico 2 board is 12 MHz.
/// Adjust if your board has a different frequency
// const XTAL_FREQ_HZ: u32 = 12_000_000u32;

pub struct ST7796S<P, CS, DC, RST, T>
where 
    P: ValidSpiPinout<pac::SPI0>,
    CS: OutputPin,
    DC: OutputPin,
    RST: OutputPin,
    T: embedded_hal::delay::DelayNs
{
    interface: hal::spi::Spi<Enabled, pac::SPI0, P>,
    cs: CS,
    dc: DC,
    rst: RST,
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

#[derive(Debug)]
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

impl<P, CS, DC, RST, T> ST7796S<P, CS, DC, RST, T> 
where 
    P: ValidSpiPinout<pac::SPI0>,
    CS: OutputPin,
    DC: OutputPin,
    RST: OutputPin,
    T: embedded_hal::delay::DelayNs,
{
    pub fn new<F: FnMut(&str)>(
            pins: P,
            clocks: rp235x_hal::clocks::ClocksManager,
            spi0: pac::SPI0,
            resets: &mut pac::RESETS,
            cs: CS,
            dc: DC,
            rst: RST,
            timer: T,
            mut debug_cb: F) -> Self {

        debug_cb("Initializing SPI device.\r\n");
        // These are implicitly used by the spi driver if they are in the correct mode
        let s = hal::spi::Spi::new(spi0, pins).init(
            resets,
            clocks.peripheral_clock.freq(),
            62500.kHz(),
            // 16.MHz(),
            // 100_000.kHz(),
            embedded_hal::spi::MODE_0,
        );

        ST7796S {
            interface: s,
            cs: cs,
            dc: dc,
            rst: rst,
            timer: timer,
        }
    }

    pub fn init(&mut self) {

        self.reset_pin();

        // 2. Software Reset
        self.swreset().unwrap();
        self.timer.delay_ms(10);

        // 3. Sleep Out
        self.write_command(0x11); // SLPOUT
        self.timer.delay_ms(120);

        // 4. Display ON
        self.write_command(0x29); // DISPON
        self.timer.delay_ms(20);

        // 5. Set pixel format
        self.write_command(0x3A); 
        self.write_data(&[0x55]); // 16-bit color (RGB565)

        // 6. Set memory access control (optional, orientation)
        self.write_command(0x36); 
        self.write_data(&[0x00]); // Row/column order (adjust as needed)

        self.write_command(0x2A);
        self.write_data(&[0x00, 0x00, 0x00, 0xEF]);         // CASET: column address set
        self.write_command(0x2B);
        self.write_data(&[0x00, 0x00, 0x01, 0xDF]);         // RASET: row address set
        self.write_command(0x2C);
        for _ in 0..(479*319) {
            // self.write_data(&[0xff, 0xff])
            self.write_data(&[0x00, 0x00])
        }

        // self.write_command(0x29);          // Display on
        // self.timer.delay_ms(10);
        // self.timer.delay_ms(120);
    }

    // Helper function to write a command to the SPI interface
    fn write_command(&mut self, cmd: u8) {
        self.cs.set_low().unwrap();
        self.dc.set_low().unwrap();
        self.interface.write(&[cmd]).unwrap();
        self.cs.set_high().unwrap();
    }

    // Helper function to write data to the SPI interface
    fn write_data(&mut self, data: &[u8]) {
        self.cs.set_low().unwrap();
        self.dc.set_high().unwrap();
        self.interface.write(data).unwrap();
        self.cs.set_high().unwrap();
    }

    #[allow(dead_code)]
    // Helper function to read data from the SPI interface
    fn read_data(&mut self, data: &mut [u8]) {
        self.cs.set_low().unwrap();
        self.dc.set_high().unwrap();
        self.interface.read(data).unwrap();
        self.cs.set_high().unwrap();
    }

    // Toggle the reset pin
    fn reset_pin(&mut self) {
        self.rst.set_high().unwrap();
        self.timer.delay_ms(50);
        self.rst.set_low().unwrap();
        self.timer.delay_ms(50);
        self.rst.set_high().unwrap();
        self.timer.delay_ms(120);
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

    #[cold]
    // Loop back test. Kind of doesn't work.
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
        self.write_command(Command::NOP.into());
        Ok(InstructionResult::NoReturn)
    }

    // Software reset
    pub fn swreset(&mut self) -> Result<InstructionResult, u8> {
        self.write_command(Command::SWRESET.into());
        Ok(InstructionResult::NoReturn)
    }

    // Read Display ID
    #[cold]
    pub fn rddid(&mut self) -> Result<InstructionResult, u8> {
        self.write_command(Command::RDDID.into());
        let mut buffer: [u8; 4] = [0, 0, 0, 0];
        self.read_data(&mut buffer);
        Ok(
            InstructionResult::RDDIDReturn(
                RDDIDResult {
                    lcd_manufacturer_id: buffer[1],
                    lcd_driver_version: buffer[2],
                    lcd_driver_id: buffer[3],
                }
            )
        )
    }

    // // Alternative implementation of rddid
    // pub fn read_id(&mut self) -> [u8; 4] {
    //     let mut buffer = [0u8; 4];

    //     // Select the display
    //     self.cs.set_low().unwrap();
    //     self.dc.set_low().unwrap();  // Command mode
    //     self.timer.delay_ms(10);
        
    //     // Send the RDDID command
    //     self.interface.write(&[0x04]).unwrap();
    //     self.timer.delay_ms(10);
    //     self.cs.set_high().unwrap();

    //     // Read 4 bytes of ID
    //     self.dc.set_high().unwrap();  // Data mode
    //     self.cs.set_low().unwrap();
    //     self.timer.delay_ms(100);

    //     self.interface.transfer(&mut buffer, &[0x00, 0x00, 0x00, 0x00]).unwrap();
    //     // self.interface.read(&mut buffer).unwrap();

    //     self.timer.delay_ms(10);

    //     // Deselect the display
    //     self.cs.set_high().unwrap();

    //     buffer
    // }

    // pub fn read_disp(&mut self) -> [u8; 4] {
    //     let mut buffer = [0x00; 4]; // Buffer to store response

    //     self.dc.set_low().unwrap();    // Set DC LOW for command mode
    //     self.cs.set_low().unwrap();    // Select the display
    //     self.timer.delay_ms(10);

    //     self.interface.write(&mut [0x09]).unwrap();  // Send RDDST command (0x09)
    //     self.timer.delay_ms(10);
    //     self.cs.set_high().unwrap();   // Deselect display
    //     self.timer.delay_ms(10);

    //     self.dc.set_high().unwrap();   // Set DC HIGH for data mode
    //     self.cs.set_low().unwrap();    // Select display
    //     self.timer.delay_ms(100);

    //     self.interface.transfer(&mut buffer, &[0x00, 0x00, 0x00, 0x00]).unwrap();
    //     // self.interface.read(&mut buffer).unwrap();  // Read 4 bytes of status data
    //     self.timer.delay_ms(10);
    //     self.cs.set_high().unwrap();   // Deselect display

    //     buffer
    // }

    // Read Number of the Errors on DSI
    #[cold]
    pub fn rnedsi(&mut self) -> Result<InstructionResult, u8> {
        self.write_command(Command::RNEDSI.into());
        let mut ret_words: [u8; 2] = [0, 0];
        self.read_data(&mut ret_words);
        Ok(
            InstructionResult::RNEDSIReturn(
                RNEDSIResult {
                    num_errors: ret_words[1] & 0x7f,
                    has_overflow: ( ret_words[1] & 0x80 ) > 0,
                    additional: 0,
                }
            )
        )
    }

    // Read Display Status
    #[cold]
    pub fn rddst(&mut self) -> Result<InstructionResult, u8> {
        self.write_command(Command::RDDST.into());
        let mut ret_words: [u8; 5] = [0, 0, 0, 0, 0];
        self.read_data(&mut ret_words);
        Ok(
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
        )
    }

    // Enter inversion off mode
    pub fn invoff(&mut self) -> Result<InstructionResult, u8> {
        self.write_command(Command::INVOFF.into());
        Ok(InstructionResult::NoReturn)
    }

    // Enter inversion on mode
    pub fn invon(&mut self) -> Result<InstructionResult, u8> {
        self.write_command(Command::INVON.into());
        Ok(InstructionResult::NoReturn)
    }

    // Enter display off mode
    pub fn dispoff(&mut self) -> Result<InstructionResult, u8> {
        self.write_command(Command::DISPOFF.into());
        Ok(InstructionResult::NoReturn)
    }

    // Enter display on mode
    pub fn dispon(&mut self) -> Result<InstructionResult, u8> {
        self.write_command(Command::DISPON.into());
        Ok(InstructionResult::NoReturn)
    }

    pub fn wrdisbv(&mut self, _val: u8) -> Result<InstructionResult, u8> {
        self.write_command(Command::WRDISBV.into());
        Ok(InstructionResult::NoReturn)
    }
}