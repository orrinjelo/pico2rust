// ST7796S Instructions
#[repr(u8)]
pub enum Command {
    NOP           = 0x00,   // No operation
    SWRESET       = 0x01,   // Software reset

    RDDID         = 0x04,   // Read display ID
    RNEDSI        = 0x05,   // Read number of errors on DSI/Read DSI
    RDDST         = 0x09,   // Read display status
    RDDPM         = 0x0A,   // Read display power
    RDDMADCTL     = 0x0B,   // Read display
    RDDPIXFMT     = 0x0C,   // Read display pixel
    RDDIM         = 0x0D,   // Read display image
    RDDSM         = 0x0E,   // Read display signal
    RDDSDR        = 0x0F,   // Read display self-diagnostic result

    SLPIN         = 0x10,   // Sleep in
    SLPOUT        = 0x11,   // Sleep out
    PTLON         = 0x12,   // Partial mode on
    NORON         = 0x13,   // Partial mode off (normal)

    INVOFF        = 0x20,   // Display inversion off
    INVON         = 0x21,   // Display inversion on 
    DISPOFF       = 0x28,   // Display off
    DISPON        = 0x29,   // Display on

    CASET         = 0x2A,   // Column address set
    RASET         = 0x2B,   // Row address set
    RAMWR         = 0x2C,   // Memory write
    RAMRD         = 0x2E,   // Memory read

    PTLAR         = 0x30,   // Partial start/end address set
    VSCRDER       = 0x33,   // Vertical scrolling definition
    TEOFF         = 0x34,   // Tearing effect line off
    TEON          = 0x35,   // Tearing effect line on
    MADCTL        = 0x36,   // Memory data access control
    VSCRSADD      = 0x37,   // Vertical scrolling start address
    IDMOFF        = 0x38,   // Idle mode off
    IDMON         = 0x39,   // Idle mode on
    PIXFMT        = 0x3A,   // Interface pixel format
    RAMWRC        = 0x3C,   // Memory write continue
    RAMRDC        = 0x3E,   // Memory read continue

    TESCAN        = 0x44,   // Set tear scanline
    RDTESCAN      = 0x45,   // Get scanline

    WRDISBV       = 0x51,   // Write display brightness
    RDDISBV       = 0x52,   // Read display brightness value
    WRCTRLD       = 0x53,   // Write CTRL display
    RDCTRLD       = 0x54,   // Read CTRL display value
    WRCABC        = 0x55,   // Write content adaptive brightness control
    RDCABC        = 0x56,   // Read content adaptive brightness control
    WRCABCMB      = 0x5E,   // Write CABC minimum brightness
    RDCABCMB      = 0x5F,   // Read CABC minimum brightness

    RDFCHKSUM     = 0xAA,   // Read first checksum
    RDCCHKSUM     = 0xAF,   // Read continue checksum
    
    RDID1         = 0xDA,   // Read ID1
    RDID2         = 0xDB,   // Read ID2
    RDID3         = 0xDC,   // Read ID3
    // RDID4         = 0xDD,   // Read ID4

    IFMODE        = 0xB0,   // Interface mode control
    FRMCTR1       = 0xB1,   // Frame rate control (in normal mode/full colors)
    FRMCTR2       = 0xB2,   // Frame rate control (in idle mode/8 colors)
    FRMCTR3       = 0xB3,   // Frame rate control (in partial mode/full colors)
    INVCTR        = 0xB4,   // Display inversion control
    BPC           = 0xB5,   // Blanking porch control
    DFC           = 0xB6,   // Display function control
    EM            = 0xB7,   // Entry mode set

    PWCTR1        = 0xC0,   // Power control 1
    PWCTR2        = 0xC1,   // Power control 2
    PWCTR3        = 0xC2,   // Power control 3
    // PWCTR4        = 0xC3,   // Power control 4
    // PWCTR5        = 0xC4,   // Power control 5
    VCMPCTR       = 0xC5,   // VCom control
    VCMOFFS       = 0xC6,   // Vcom offset register

    NVMADW        = 0xD0,   // NVM address/data
    NVMBPROG      = 0xD1,   // NVM byte program control
    NVMSTRD       = 0xD2,   // NVM status read
    RDID4         = 0xD3,   // Read ID4

    PGC           = 0xE0,   // Positive gamma control
    NGC           = 0xE1,   // Negative gamma control
    DGC1          = 0xE2,   // Digital gamma control 1
    DGC2          = 0xE3,   // Digital gamma control 2
    DOCA          = 0xE8,   // Display output CTRL adjust

    CSCON         = 0xF0,   // Command set control
    SPIRC         = 0xFB,   // SPI read control
}

impl Into<Command> for u8 {
    fn into(self) -> Command {
        match self {
            0x00 => Command::NOP,
            0x01 => Command::SWRESET,
            0x04 => Command::RDDID,
            0x05 => Command::RNEDSI,
            0x09 => Command::RDDST,
            0x0A => Command::RDDPM,
            0x0B => Command::RDDMADCTL,
            0x0C => Command::RDDPIXFMT,
            0x0D => Command::RDDIM,
            0x0E => Command::RDDSM,
            0x0F => Command::RDDSDR,
            0x10 => Command::SLPIN,
            0x11 => Command::SLPOUT,
            0x12 => Command::PTLON,
            0x13 => Command::NORON,
            0x20 => Command::INVOFF,
            0x21 => Command::INVON,
            0x28 => Command::DISPOFF,
            0x29 => Command::DISPON,
            0x2A => Command::CASET,
            0x2B => Command::RASET,
            0x2C => Command::RAMWR,
            0x2E => Command::RAMRD,
            0x30 => Command::PTLAR,
            0x33 => Command::VSCRDER,
            0x34 => Command::TEOFF,
            0x35 => Command::TEON,
            0x36 => Command::MADCTL,
            0x37 => Command::VSCRSADD,
            0x38 => Command::IDMOFF,
            0x39 => Command::IDMON,
            0x3A => Command::PIXFMT,
            0x3C => Command::RAMWRC,
            0x3E => Command::RAMRDC,
            0x44 => Command::TESCAN,
            0x45 => Command::RDTESCAN,
            0x51 => Command::WRDISBV,
            0x52 => Command::RDDISBV,
            0x53 => Command::WRCTRLD,
            0x54 => Command::RDCTRLD,
            0x55 => Command::WRCABC,
            0x56 => Command::RDCABC,
            0x5E => Command::WRCABCMB,
            0x5F => Command::RDCABCMB,
            0xAA => Command::RDFCHKSUM,
            0xAF => Command::RDCCHKSUM,
            0xDA => Command::RDID1,
            0xDB => Command::RDID2,
            0xDC => Command::RDID3,
            0xB0 => Command::IFMODE,
            0xB1 => Command::FRMCTR1,
            0xB2 => Command::FRMCTR2,
            0xB3 => Command::FRMCTR3,
            0xB4 => Command::INVCTR,
            0xB5 => Command::BPC,
            0xB6 => Command::DFC,
            0xB7 => Command::EM,
            0xC0 => Command::PWCTR1,
            0xC1 => Command::PWCTR2,
            0xC2 => Command::PWCTR3,
            0xC5 => Command::VCMPCTR,
            0xC6 => Command::VCMOFFS,
            0xD0 => Command::NVMADW,
            0xD1 => Command::NVMBPROG,
            0xD2 => Command::NVMSTRD,
            0xD3 => Command::RDID4,
            0xE0 => Command::PGC,
            0xE1 => Command::NGC,
            0xE2 => Command::DGC1,
            0xE3 => Command::DGC2,
            0xE8 => Command::DOCA,
            0xF0 => Command::CSCON,
            0xFB => Command::SPIRC,
            _ => Command::NOP,
        }
    }
}

impl Into<u8> for Command {
    fn into(self) -> u8 {
        match self {
            Command::NOP => 0x00,
            Command::SWRESET => 0x01,
            Command::RDDID => 0x04,
            Command::RNEDSI => 0x05,
            Command::RDDST => 0x09,
            Command::RDDPM => 0x0A,
            Command::RDDMADCTL => 0x0B,
            Command::RDDPIXFMT => 0x0C,
            Command::RDDIM => 0x0D,
            Command::RDDSM => 0x0E,
            Command::RDDSDR => 0x0F,
            Command::SLPIN => 0x10,
            Command::SLPOUT => 0x11,
            Command::PTLON => 0x12,
            Command::NORON => 0x13,
            Command::INVOFF => 0x20,
            Command::INVON => 0x21,
            Command::DISPOFF => 0x28,
            Command::DISPON => 0x29,
            Command::CASET => 0x2A,
            Command::RASET => 0x2B,
            Command::RAMWR => 0x2C,
            Command::RAMRD => 0x2E,
            Command::PTLAR => 0x30,
            Command::VSCRDER => 0x33,
            Command::TEOFF => 0x34,
            Command::TEON => 0x35,
            Command::MADCTL => 0x36,
            Command::VSCRSADD => 0x37,
            Command::IDMOFF => 0x38,
            Command::IDMON => 0x39,
            Command::PIXFMT => 0x3A,
            Command::RAMWRC => 0x3C,
            Command::RAMRDC => 0x3E,
            Command::TESCAN => 0x44,
            Command::RDTESCAN => 0x45,
            Command::WRDISBV => 0x51,
            Command::RDDISBV => 0x52,
            Command::WRCTRLD => 0x53,
            Command::RDCTRLD => 0x54,
            Command::WRCABC => 0x55,
            Command::RDCABC => 0x56,
            Command::WRCABCMB => 0x5E,
            Command::RDCABCMB => 0x5F,
            Command::RDFCHKSUM => 0xAA,
            Command::RDCCHKSUM => 0xAF,
            Command::RDID1 => 0xDA,
            Command::RDID2 => 0xDB,
            Command::RDID3 => 0xDC,
            Command::IFMODE => 0xB0,
            Command::FRMCTR1 => 0xB1,
            Command::FRMCTR2 => 0xB2,
            Command::FRMCTR3 => 0xB3,
            Command::INVCTR => 0xB4,
            Command::BPC => 0xB5,
            Command::DFC => 0xB6,
            Command::EM => 0xB7,
            Command::PWCTR1 => 0xC0,
            Command::PWCTR2 => 0xC1,
            Command::PWCTR3 => 0xC2,
            Command::VCMPCTR => 0xC5,
            Command::VCMOFFS => 0xC6,
            Command::NVMADW => 0xD0,
            Command::NVMBPROG => 0xD1,
            Command::NVMSTRD => 0xD2,
            Command::RDID4 => 0xD3,
            Command::PGC => 0xE0,
            Command::NGC => 0xE1,
            Command::DGC1 => 0xE2,
            Command::DGC2 => 0xE3,
            Command::DOCA => 0xE8,
            Command::CSCON => 0xF0,
            Command::SPIRC => 0xFB,
            // _ => 0x00,
        }
    }
}