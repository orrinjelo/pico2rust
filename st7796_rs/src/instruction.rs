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

