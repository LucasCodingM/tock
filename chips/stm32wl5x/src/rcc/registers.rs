use kernel::utilities::registers::{register_bitfields, ReadWrite};
use kernel::utilities::StaticRef;


/*The following different clock sources can be used to drive the system clock (SYSCLK):
• HSI16 (high-speed internal) 16 MHz RC oscillator clock
• MSI (multi-speed internal) RC oscillator clock from 100 kHz to 48 MHz
• HSE32 (high-speed external) 32 MHz oscillator clock, with trimming capacitors.
• PLL clock
The MSI is used as system clock source after startup from reset, configured at 4 MHz.
The devices have the following additional clock sources:
• LSI: 32 kHz low-speed internal RC that may drive the independent watchdog and
optionally the RTC used for auto-wakeup from Stop and Standby modes.
• LSE: 32.768 kHz low-speed external crystal that optionally drives the RTC used for
auto-wakeup from Stop, Standby and Shutdown modes, or the real-time clock
(RTCCLK).
Each clock source can be switched on or off independently when it is not used, to optimize
power consumption.
Several prescalers can be used to configure the AHB frequencies (HCLK3/PCLK3, HCLK1,
HCLK2), the high-speed APB2 (PCLK2) and the low-speed APB1 (PCLK1) domains. The
maximum frequency of the AHB (HCLK3, HCLK1, and HCLK2), the PCLK1 and the PCLK2
domains is 48 MHz. */

/// Reset and clock control register block.
#[repr(C)]
pub struct RccRegisters {
    /// Control Register – Enables/disables oscillators (HSI, HSE, MSI, etc.)
    pub cr: ReadWrite<u32, CR::Register>,
    /// Internal Clock Sources Calibration Register – Calibrates internal clocks (e.g., MSI, HSI)
    pub icscr: ReadWrite<u32, ICSCR::Register>,
    /// Clock Configuration Register – Selects system clock source (SYSCLK), AHB/APB prescalers
    pub cfgr: ReadWrite<u32, CFGR::Register>,
    /// PLL Configuration Register – Configures PLL multipliers/dividers (M, N, R, P, Q)
    pub pllcfgr: ReadWrite<u32, PLLCFGR::Register>,
    /// Clock Interrupt Enable Register – Enables interrupts for clock events
    pub cier: ReadWrite<u32, CIER::Register>,
    /// Clock Interrupt Flag Register – Indicates which clock interrupt has occurred
    pub cifr: ReadWrite<u32, CIFR::Register>,
    /// Clock Interrupt Clear Register – Clears clock interrupt flags
    pub cicr: ReadWrite<u32, CICR::Register>,

    /// Reset peripherals on AHB1 bus
    pub ahb1rstr: ReadWrite<u32, AHB1RSTR::Register>,
    /// Reset peripherals on AHB2 bus
    pub ahb2rstr: ReadWrite<u32, AHB2RSTR::Register>,
    ///	Reset peripherals on AHB3 bus
    pub ahb3rstr: ReadWrite<u32, AHB3RSTR::Register>,
    /// Reset APB1 (part 1) peripherals
    pub apb1rstr1: ReadWrite<u32, APB1RSTR1::Register>,
    /// Reset APB1 (part 2) peripherals
    pub apb1rstr2: ReadWrite<u32, APB1RSTR2::Register>,
    /// Reset APB2 peripherals
    pub apb2rstr: ReadWrite<u32, APB2RSTR::Register>,
    /// Reset APB3 peripherals
    pub apb3rstr: ReadWrite<u32, APB3RSTR::Register>,

    /// Enable clock for AHB1 peripherals
    pub ahb1enr: ReadWrite<u32, AHB1ENR::Register>,
    /// Enable clock for AHB2 peripherals
    pub ahb2enr: ReadWrite<u32, AHB2ENR::Register>,
    /// Enable clock for AHB3 peripherals
    pub ahb3enr: ReadWrite<u32, AHB3ENR::Register>,
    /// Enable APB1 (part 1) peripheral clocks
    pub apb1enr1: ReadWrite<u32, APB1ENR1::Register>,
    /// Enable APB1 (part 2) peripheral clocks
    pub apb1enr2: ReadWrite<u32, APB1ENR2::Register>,
    /// Enable APB2 peripheral clocks
    pub apb2enr: ReadWrite<u32, APB2ENR::Register>,
    /// Enable APB3 peripheral clocks
    pub apb3enr: ReadWrite<u32, APB3ENR::Register>,

    /// Keep AHB1 clocks enabled during Sleep
    pub ahb1smenr: ReadWrite<u32, AHB1SMENR::Register>,
    /// Same for AHB2 clocks
    pub ahb2smenr: ReadWrite<u32, AHB2SMENR::Register>,
    /// Same for AHB3 clocks
    pub ahb3smenr: ReadWrite<u32, AHB3SMENR::Register>,
    /// APB1 (part 1) sleep mode clock control
    pub apb1smenr1: ReadWrite<u32, APB1SMENR1::Register>,
    /// APB1 (part 2) sleep mode clock control
    pub apb1smenr2: ReadWrite<u32, APB1SMENR2::Register>,
    /// APB2 sleep mode clock control
    pub apb2smenr: ReadWrite<u32, APB2SMENR::Register>,
    /// APB3 sleep mode clock control
    pub apb3smenr: ReadWrite<u32, APB3SMENR::Register>,

    /// Clock Configuration Independent Peripherals Register 
    /// Selects clock sources for specific peripherals (USARTs, ADCs, RNG, etc.)
    pub ccipr: ReadWrite<u32, CCIPR::Register>,

    /// Backup Domain Control Register – Controls LSE, RTC, backup RAM, and reset
    pub bdcr: ReadWrite<u32, BDCR::Register>,
    /// Control and Status Register 
    /// Flags for resets, IWDG, BOR, etc.
    pub csr: ReadWrite<u32, CSR::Register>,
    /// Extended Clock Configuration Register 
    /// Additional config (e.g., CPU frequency scaling, power saving)

    /// Extended Clock Configuration Register
    /// Additional config (e.g., CPU frequency scaling, power saving)
    pub extcfgr: ReadWrite<u32, EXTCFGR::Register>,

    /// AHB1 enable for CPU2
    pub c2ahb1enr: ReadWrite<u32, C2AHB1ENR::Register>,
    /// AHB2 enable for CPU2
    pub c2ahb2enr: ReadWrite<u32, C2AHB2ENR::Register>,
    /// AHB3 enable for CPU2
    pub c2ahb3enr: ReadWrite<u32, C2AHB3ENR::Register>,
    /// APB1 (part 1) enable for CPU2
    pub c2apb1enr1: ReadWrite<u32, C2APB1ENR1::Register>,
    /// APB1 (part 2) enable for CPU2
    pub c2apb1enr2: ReadWrite<u32, C2APB1ENR2::Register>,
    // APB2 enable for CPU2
    pub c2apb2enr: ReadWrite<u32, C2APB2ENR::Register>,
    /// APB3 enable for CPU2
    pub c2apb3enr: ReadWrite<u32, C2APB3ENR::Register>,
    /// AHB1 sleep mode enable for CPU2
    pub c2ahb1smenr: ReadWrite<u32, C2AHB1SMENR::Register>,
    /// AHB2 sleep mode enable for CPU2
    pub c2ahb2smenr: ReadWrite<u32, C2AHB2SMENR::Register>,
    /// AHB3 sleep mode enable for CPU2
    pub c2ahb3smenr: ReadWrite<u32, C2AHB3SMENR::Register>,
    /// APB1 (part 1) sleep mode enable for CPU2
    pub c2apb1smenr1: ReadWrite<u32, C2APB1SMENR1::Register>,
    /// APB1 (part 2) sleep mode enable for CPU2
    pub c2apb1smenr2: ReadWrite<u32, C2APB1SMENR2::Register>,
    /// APB2 sleep mode enable for CPU2
    pub c2apb2smenr: ReadWrite<u32, C2APB2SMENR::Register>,
    /// APB3 sleep mode enable for CPU2
    pub c2apb3smenr: ReadWrite<u32, C2APB3SMENR::Register>,    
}

register_bitfields![u32,
    pub CR [
        MSION OFFSET(0) NUMBITS(1) [],
        MSIRDY OFFSET(1) NUMBITS(1) [],
        MSIPLLEN OFFSET(2) NUMBITS(1) [],
        MSIRGSEL OFFSET(3) NUMBITS(1) [],
        MSIRANGE OFFSET(4) NUMBITS(4) [
            Range100kHz = 0b0000,
            Range200kHz = 0b0001,
            Range400kHz = 0b0010,
            Range800kHz = 0b0011,
            Range1MHz   = 0b0100,
            Range2MHz   = 0b0101,
            Range4MHz   = 0b0110, // reset value
            Range8MHz   = 0b0111,
            Range16MHz  = 0b1000,
            Range24MHz  = 0b1001,
            Range32MHz  = 0b1010,
            Range48MHz  = 0b1011,
    ],
        HSION OFFSET(8) NUMBITS(1) [],
        HSIKERON OFFSET(9) NUMBITS(1) [],
        HSIRDY OFFSET(10) NUMBITS(1) [],
        HSIASFS OFFSET(11) NUMBITS(1) [],
        HSIKERDY OFFSET(12) NUMBITS(1) [],
        HSEON OFFSET(16) NUMBITS(1) [],
        HSERDY OFFSET(17) NUMBITS(1) [],
        CSSON OFFSET(19) NUMBITS(1) [],
        HSEPRE OFFSET(20) NUMBITS(1) [],
        HSEBYPPWR OFFSET(21) NUMBITS(1) [],
        PLLON OFFSET(24) NUMBITS(1) [],
        PLLRDY OFFSET(25) NUMBITS(1) [],
    ],
    pub ICSCR [
        MSICAL OFFSET(0) NUMBITS(8) [],
        MSITRIM OFFSET(8) NUMBITS(8) [],
        HSICAL OFFSET(16) NUMBITS(8) [],
        HSITRIM OFFSET(24) NUMBITS(8) [],
    ],
    pub CFGR [
    MCOPRE OFFSET(28) NUMBITS(3) [
        Div1 = 0b000,  // MCO divided by 1
        Div2 = 0b001,  // MCO divided by 2
        Div4 = 0b010,  // MCO divided by 4
        Div8 = 0b011,  // MCO divided by 8
        Div16= 0b100,  // MCO divided by 16
    ],

    MCOSEL OFFSET(24) NUMBITS(4) [
        Disabled = 0b0000,      // No output on MCO
        SYSCLK   = 0b0001,      // System clock
        MSI      = 0b0010,      // MSI clock
        HSI16    = 0b0011,      // HSI16 internal oscillator
        HSE32    = 0b0100,      // External HSE clock (32 MHz)
        PLLR     = 0b0101,      // PLL R output
        LSI      = 0b0110,      // Low speed internal clock
        LSE      = 0b1000,      // Low speed external clock
        PLLP     = 0b1101,      // PLL P output
        PLLQ     = 0b1110,      // PLL Q output
],

    PPRE2F OFFSET(18) NUMBITS(1) [],
    PPRE1F OFFSET(17) NUMBITS(1) [],
    
    HPREF OFFSET(16) NUMBITS(1) [],

    STOPWUCK OFFSET(15) NUMBITS(1) [],
    /// APB high-speed prescaler (APB2)
    PPRE2 OFFSET(11) NUMBITS(3) [
        Div2  = 0b100,  // Divide by 2
        Div4  = 0b101,  // Divide by 4
        Div8  = 0b110,  // Divide by 8
        Div16 = 0b111,  // Divide by 16
],
    /// APB Low speed prescaler (APB1)
    PPRE1 OFFSET(8) NUMBITS(3) [
        Div2  = 0b100,  // Divide by 2
        Div4  = 0b101,  // Divide by 4
        Div8  = 0b110,  // Divide by 8
        Div16 = 0b111,  // Divide by 16
    ],
    /// AHB prescaler
    HPRE OFFSET(4) NUMBITS(4) [
        Div1    = 0b0000, // SYSCLK not divided (default)
        Div3    = 0b0001, // SYSCLK divided by 3
        Div5    = 0b0010, // SYSCLK divided by 5
        // 0b0011 and 0b0100 reserved or not defined, treated as Div1
        Div6    = 0b0101, // SYSCLK divided by 6
        Div10   = 0b0110, // SYSCLK divided by 10
        Div32   = 0b0111, // SYSCLK divided by 32
        Div2    = 0b1000, // SYSCLK divided by 2
        Div4    = 0b1001, // SYSCLK divided by 4
        Div8    = 0b1010, // SYSCLK divided by 8
        Div16   = 0b1011, // SYSCLK divided by 16
        Div64   = 0b1100, // SYSCLK divided by 64
        Div128  = 0b1101, // SYSCLK divided by 128
        Div256  = 0b1110, // SYSCLK divided by 256
        Div512  = 0b1111, // SYSCLK divided by 512
    ],
    /// System clock switch status
    SWS OFFSET(2) NUMBITS(2) [],
    /// System clock switch
    SW OFFSET(0) NUMBITS(2) [
        MSI = 0b00,
        HSI16 = 0b01,
        HSE32 = 0b10,
        PLLRCLK = 0b11,        
    ]
    ],
    pub PLLCFGR [
        PLLSRC OFFSET(0) NUMBITS(2) [
            MSI = 0b01,
            HSI16 = 0b01,
            HSE32 = 0b11,
        ],
        PLLM OFFSET(4) NUMBITS(3) [],        
        PLLN OFFSET(8) NUMBITS(7) [],
        PLLPEN OFFSET(16) NUMBITS(1) [],

        PLLP OFFSET(17) NUMBITS(5) [
            DivideBy2 = 0b00001,
            DivideBy4 = 0b00011,
            DivideBy6 = 0b00101,
            DivideBy8 = 0b01000,
        ],

        PLLQEN OFFSET(24) NUMBITS(1) [],
        PLLQ OFFSET(25) NUMBITS(3) [
            Reserved = 0b000, // Reserved
            Div2     = 0b001, // PLLQ = 2
            Div3     = 0b010, // PLLQ = 3
            Div4     = 0b011, // PLLQ = 4
            Div5     = 0b100, // PLLQ = 5
            Div6     = 0b101, // PLLQ = 6
            Div7     = 0b110, // PLLQ = 7
            Div8     = 0b111, // PLLQ = 8
        ],
        PLLREN OFFSET(28) NUMBITS(1) [
            Disabled = 0, // PLLRCLK output disabled
            Enabled  = 1, // PLLRCLK output enabled
        ],
        PLLR OFFSET(29) NUMBITS(3) [
            Div2     = 0b001, // PLLR = 2
            Div3     = 0b010, // PLLR = 3
            Div4     = 0b011, // PLLR = 4
            Div5     = 0b100, // PLLR = 5
            Div6     = 0b101, // PLLR = 6
            Div7     = 0b110, // PLLR = 7
            Div8     = 0b111, // PLLR = 8
        ],           
    ],
    pub CIER [
        LSIRDYIE OFFSET(0) NUMBITS(1) [],
        LSERDYIE OFFSET(1) NUMBITS(1) [],
        MSIRDYIE OFFSET(2) NUMBITS(1) [],
        HSIRDYIE OFFSET(3) NUMBITS(1) [],
        HSERDYIE OFFSET(4) NUMBITS(1) [],
        PLLRDYIE OFFSET(5) NUMBITS(1) [],
        LSECSSIE OFFSET(9) NUMBITS(1) [],
    ],
    pub CIFR [
        LSIRDYF OFFSET(0) NUMBITS(1) [],
        LSERDYF OFFSET(1) NUMBITS(1) [],
        MSIRDYF OFFSET(2) NUMBITS(1) [],
        HSIRDYF OFFSET(3) NUMBITS(1) [],
        HSERDYF OFFSET(4) NUMBITS(1) [],
        PLLRDYF OFFSET(5) NUMBITS(1) [],
        CSSF    OFFSET(8) NUMBITS(1) [],
        LSECSSF OFFSET(9) NUMBITS(1) []
    ],
    pub CICR [
        LSIRDYC OFFSET(0) NUMBITS(1) [],
        LSERDYC OFFSET(1) NUMBITS(1) [],
        MSIDYC OFFSET(2) NUMBITS(1) [],
        HSIRDYC OFFSET(3) NUMBITS(1) [],
        HSERDYC OFFSET(4) NUMBITS(1) [],
        PLLRDYC OFFSET(5) NUMBITS(1) [],
        CSSC    OFFSET(8) NUMBITS(1) [],
        LSECSSC OFFSET(9) NUMBITS(1) []
    ],
    pub AHB1RSTR [
         DMA1RST OFFSET(0) NUMBITS(1) [],
         DMA2RST OFFSET(1) NUMBITS(1) [],
         DMAMUX1RST OFFSET(2) NUMBITS(1) [],
         CRCRST OFFSET(12) NUMBITS(1) [],
    ],
    pub AHB2RSTR [
        GPIOARST OFFSET(0) NUMBITS(1) [],
        GPIOBRST OFFSET(1) NUMBITS(1) [],
        GPIOCRST OFFSET(2) NUMBITS(1) [],
        GPIOHRST OFFSET(7) NUMBITS(1) [],
    ],
    pub AHB3RSTR [
        PKARST OFFSET(16) NUMBITS(1) [],
        AESRST OFFSET(17) NUMBITS(1) [],
        RNGRST OFFSET(18) NUMBITS(1) [],
        HSEMRST OFFSET(19) NUMBITS(1) [],
        IPCCRST OFFSET(20) NUMBITS(1) [],
        FLASHRST OFFSET(25) NUMBITS(1) [],
    ],
    pub APB1RSTR1 [
        TIM2RST OFFSET(0) NUMBITS(1) [],
        SPI2S2RST OFFSET(14) NUMBITS(1) [],
        USART2RST OFFSET(17) NUMBITS(1) [],
        I2C1RST OFFSET(21) NUMBITS(1) [],
        I2C2RST OFFSET(22) NUMBITS(1) [],
        I2C3RST OFFSET(23) NUMBITS(1) [],
        DACRST OFFSET(29) NUMBITS(1) [],
        LPTIM1RST OFFSET(31) NUMBITS(1) [],
    ],
    pub APB1RSTR2 [
        LPUART1RST OFFSET(0) NUMBITS(1) [],
        LPTIM2RST OFFSET(5) NUMBITS(1) [],
        LPTIM3RST OFFSET(6) NUMBITS(1) [],
    ],
    pub APB2RSTR [
        ADCRST OFFSET(9) NUMBITS(1) [],
        TIM1RST OFFSET(11) NUMBITS(1) [],
        SPI1RST OFFSET(12) NUMBITS(1) [],
        USART1RST OFFSET(14) NUMBITS(1) [],
        TIM16RST OFFSET(17) NUMBITS(1) [],
        TIM17RST OFFSET(18) NUMBITS(1) [],
    ],
    pub APB3RSTR [
        SUBGHZSPIRST OFFSET(0) NUMBITS(1) [],
    ],
    pub AHB1ENR [
        DMA1EN OFFSET(0) NUMBITS(1) [],
        DMA2EN OFFSET(1) NUMBITS(1) [],
        DMAMUX1EN OFFSET(2) NUMBITS(1) [],
        CRCEN OFFSET(12) NUMBITS(1) [],
    ],
    pub AHB2ENR [
        GPIOAEN OFFSET(0) NUMBITS(1) [],
        GPIOBEN OFFSET(1) NUMBITS(1) [],
        GPIOCEN OFFSET(2) NUMBITS(1) [],
        GPIOHEN OFFSET(7) NUMBITS(1) [],
    ],
    pub AHB3ENR [
        PKAEN OFFSET(16) NUMBITS(1) [],
        AESEN OFFSET(17) NUMBITS(1) [],
        RNGEN OFFSET(18) NUMBITS(1) [],
        HSEMEN OFFSET(19) NUMBITS(1) [],
        IPCC OFFSET(20) NUMBITS(1) [],
        FLASHEN OFFSET(25) NUMBITS(1) [],
    ],
    pub APB1ENR1 [
        TIM2EN      OFFSET(0) NUMBITS(1) [],
        RTCAPBEN    OFFSET(10) NUMBITS(1) [],
        WWDGEN      OFFSET(11) NUMBITS(1) [],
        SPI2S2EN    OFFSET(14) NUMBITS(1) [],
        USART2EN    OFFSET(17) NUMBITS(1) [],
        I2C1EN      OFFSET(21) NUMBITS(1) [],
        I2C2EN      OFFSET(22) NUMBITS(1) [],
        I2C3EN      OFFSET(23) NUMBITS(1) [],
        DACEN       OFFSET(29) NUMBITS(1) [],
        LPTIM1EN    OFFSET(31) NUMBITS(1) [],
    ],
    pub APB1ENR2 [
        LPUART1EN     OFFSET(0) NUMBITS(1) [],
        LPTIM2EN      OFFSET(5) NUMBITS(1) [],
        LPTIM3EN      OFFSET(6) NUMBITS(1) [],
    ],
    pub APB2ENR [
        ADCEN       OFFSET(9) NUMBITS(1) [],
        TIM1EN       OFFSET(11) NUMBITS(1) [],
        SPI1EN       OFFSET(12) NUMBITS(1) [],
        USART1EN       OFFSET(14) NUMBITS(1) [],
        TIM16EN       OFFSET(17) NUMBITS(1) [],
        TIM17EN       OFFSET(18) NUMBITS(1) [],
    ],
    pub APB3ENR [
        SUBGHZSPIEN       OFFSET(0) NUMBITS(1) [],
    ],
    pub AHB1SMENR [
        DMA1SMEN OFFSET(0) NUMBITS(1) [],
        DMA2SMEN OFFSET(1) NUMBITS(1) [],
        DMAMUX1SMEN OFFSET(2) NUMBITS(1) [],
        CRCSMEN OFFSET(12) NUMBITS(1) [],
    ],
    pub AHB2SMENR [
        GPIOASMEN OFFSET(0) NUMBITS(1) [],
        GPIOBSMEN OFFSET(1) NUMBITS(1) [],
        GPIOCSMEN OFFSET(2) NUMBITS(1) [],
        GPIOHSMEN OFFSET(7) NUMBITS(1) [],
    ],
    pub AHB3SMENR [
        PKASMEN OFFSET(16) NUMBITS(1) [],
        AESSMEN OFFSET(17) NUMBITS(1) [],
        RNGSMEN OFFSET(18) NUMBITS(1) [],
        SRAM1SMEN OFFSET(23) NUMBITS(1) [],
        SRAM2SMEN OFFSET(24) NUMBITS(1) [],
        FLASHSMEN OFFSET(25) NUMBITS(1) [],
    ],
    pub APB1SMENR1 [
        TIM2SMEN OFFSET(0) NUMBITS(1) [],
        RTCAPBSMEN OFFSET(10) NUMBITS(1) [],
        WWDGSMEN OFFSET(11) NUMBITS(1) [],
        SPI2S2SMEN OFFSET(14) NUMBITS(1) [],
        USART2SMEN OFFSET(17) NUMBITS(1) [],
        I2C1SMEN OFFSET(21) NUMBITS(1) [],
        I2C2SMEN OFFSET(22) NUMBITS(1) [],
        I2C3SMEN OFFSET(23) NUMBITS(1) [],
        DACSMEN OFFSET(29) NUMBITS(1) [],
        LPTIM1SMEN OFFSET(31) NUMBITS(1) [],
    ],
    pub APB1SMENR2 [
        LPUART1SMEN OFFSET(0) NUMBITS(1) [],
        LPTIM2SMEN OFFSET(5) NUMBITS(1) [],
        LPTIM3SMEN OFFSET(6) NUMBITS(1) [],
    ],
    pub APB2SMENR [
        ADCSMEN OFFSET(9) NUMBITS(1) [],
        TIM1SMEN OFFSET(11) NUMBITS(1) [],
        SPI1SMEN OFFSET(12) NUMBITS(1) [],
        USART1SMEN OFFSET(14) NUMBITS(1) [],
        TIM16SMEN OFFSET(17) NUMBITS(1) [],
        TIM17SMEN OFFSET(18) NUMBITS(1) [],
    ],
    pub APB3SMENR [
        SUBGHZSPISMEN OFFSET(0) NUMBITS(1) [],
    ],
    pub CCIPR [
        USART1SEL OFFSET(0) NUMBITS(2) [
            PCLK = 0b00,
            SYSCLK = 0b01,
            HSI16 = 0b10,
            LSE = 0b11,
        ],
        USART2SEL OFFSET(2) NUMBITS(2) [
            PCLK = 0b00,
            SYSCLK = 0b01,
            PLLRCLK = 0b10,
            LSE = 0b11,
        ],
        SPI2S2SEL OFFSET(8) NUMBITS(2) [
            PLLQCLK = 0b01,
            HSI16 = 0b10,
            I2S_CKIN = 0b11,
        ],
        LPUART1SEL OFFSET(10) NUMBITS(2) [
            PCLK = 0b00,
            SYSCLK = 0b01,
            HSI16 = 0b10,
            LSE = 0b11,
        ],
        I2C1SEL OFFSET(12) NUMBITS(2) [
            PCLK = 0b00,
            SYSCLK = 0b01,
            HSI16 = 0b10,
        ],
        I2C2SEL OFFSET(14) NUMBITS(2) [
            PCLK = 0b00,
            SYSCLK = 0b01,
            HSI16 = 0b10,
        ],  
        I2C3SEL OFFSET(16) NUMBITS(2) [
            PCLK = 0b00,
            SYSCLK = 0b01,
            HSI16 = 0b10,
        ],
        LPTIM1SEL OFFSET(18) NUMBITS(2) [
            PCLK = 0b00,
            LSI = 0b01,
            HSI16 = 0b10,
            LSE = 0b11,
        ],
        LPTIM2SEL OFFSET(20) NUMBITS(2) [
            PCLK = 0b00,
            LSI = 0b01,
            HSI16 = 0b10,
            LSE = 0b11,
        ],
        LPTIM3SEL OFFSET(22) NUMBITS(2) [
            PCLK = 0b00,
            LSI = 0b01,
            HSI16 = 0b10,
            LSE = 0b11,
        ],
        ADCSEL OFFSET(28) NUMBITS(2) [
            HSI16 = 0b01,
            PLLPCLK = 0b10,
            SYSCLK = 0b11,
        ],
        RNGSEL OFFSET(30) NUMBITS(2) [
            PLLQCLK = 0b00,
            LSI = 0b01,
            LSE = 0b10,
            MSI = 0b11,
        ],
    ],
    pub BDCR [
        LSEON       OFFSET(0)   NUMBITS(1) [],
        LSERDY      OFFSET(1)   NUMBITS(1) [],
        LSEBYP      OFFSET(2)   NUMBITS(1) [],
        LSEDRV      OFFSET(3)   NUMBITS(2) [],
        LSECSSON    OFFSET(5)   NUMBITS(1) [],
        LSECSSD     OFFSET(6)   NUMBITS(1) [],
        LSESYSEN    OFFSET(7)   NUMBITS(1) [],
        RTCSEL      OFFSET(8)   NUMBITS(2) [
            LSE = 0b01,
            LSI = 0b10,
            HSE32 = 0b11,
        ],
        LSESYSRDY   OFFSET(11)  NUMBITS(1) [],
        RTCEN       OFFSET(15)  NUMBITS(1) [],
        BDRST       OFFSET(16)  NUMBITS(1) [],
        LSCOEN      OFFSET(24)  NUMBITS(1) [],
        LSCOSEL     OFFSET(25)  NUMBITS(1) [],
    ],   
    pub CSR [
        LSION OFFSET(0) NUMBITS(1) [],
        LSIRDY OFFSET(1) NUMBITS(1) [],
        LSIPRE OFFSET(4) NUMBITS(1) [],
        MSISRANGE OFFSET(8) NUMBITS(4) [
            RANGE4 = 0b0100,
            RANGE5 = 0b0101,
            RANGE6 = 0b0110,
            RANGE7 = 0b0111,
        ],
        RFRSTF OFFSET(14) NUMBITS(1) [],
        RFRST OFFSET(15) NUMBITS(1) [],
        RMVF OFFSET(23) NUMBITS(1) [],
        RFILARSTF OFFSET(24) NUMBITS(1) [],
        OBLRSTF OFFSET(25) NUMBITS(1) [],
        PINRSTF OFFSET(26) NUMBITS(1) [],
        BORRSTF OFFSET(27) NUMBITS(1) [],
        SFTRSTF OFFSET(28) NUMBITS(1) [],
        IWDGRSTF OFFSET(29) NUMBITS(1) [],
        WWDGRSTF OFFSET(30) NUMBITS(1) [],
        LPWRRSTF OFFSET(31) NUMBITS(1) [],
    ],
    pub EXTCFGR [
        SHDHPRE OFFSET(0) NUMBITS(4) [
            DIV2 = 0b1000,
            DIV3 = 0b0001,
            DIV4 = 0b1001,
            DIV5 = 0b0010,
            DIV6 = 0b0101,
            DIV8 = 0b1010,
            DIV10 = 0b0110,
            DIV16 = 0b1011,           
            DIV32 = 0b0111,
            DIV64 = 0b1100,
            DIV128 = 0b1101,
            DIV256 = 0b1110,
            DIV512 = 0b1111,
        ],
        C2HPRE OFFSET(4) NUMBITS(4) [
            DIV2 = 0b1000,
            DIV3 = 0b0001,
            DIV4 = 0b1001,
            DIV5 = 0b0010,
            DIV6 = 0b0101,
            DIV8 = 0b1010,
            DIV10 = 0b0110,
            DIV16 = 0b1011,           
            DIV32 = 0b0111,
            DIV64 = 0b1100,
            DIV128 = 0b1101,
            DIV256 = 0b1110,
            DIV512 = 0b1111,
        ],
        SHDHPREF OFFSET(16) NUMBITS(1) [],
        C2HPREF OFFSET(17) NUMBITS(1) [],
    ],
    pub C2AHB1ENR [
        DMA1EN OFFSET(0) NUMBITS(1) [],
        DMA2EN OFFSET(1) NUMBITS(1) [],
        DMAMUX1EN OFFSET(2) NUMBITS(1) [],
        CRCEN OFFSET(12) NUMBITS(1) [],
    ],
    pub C2AHB2ENR [
        GPIOAEN OFFSET(0) NUMBITS(1) [],
        GPIOBEN OFFSET(1) NUMBITS(1) [],
        GPIOCEN OFFSET(2) NUMBITS(1) [],
        GPIOHEN OFFSET(7) NUMBITS(1) [],
    ],
    pub C2AHB3ENR [
        PKAEN OFFSET(16) NUMBITS(1) [],
        AESEN OFFSET(17) NUMBITS(1) [],
        RNGEN OFFSET(18) NUMBITS(1) [],
        HSEMEN OFFSET(19) NUMBITS(1) [],
        IPCC OFFSET(20) NUMBITS(1) [],
        FLASHEN OFFSET(25) NUMBITS(1) [],
    ],
    pub C2APB1ENR1 [
        TIM2EN      OFFSET(0) NUMBITS(1) [],
        RTCAPBEN    OFFSET(10) NUMBITS(1) [],
        SPI2S2EN    OFFSET(14) NUMBITS(1) [],
        USART2EN    OFFSET(17) NUMBITS(1) [],
        I2C1EN      OFFSET(21) NUMBITS(1) [],
        I2C2EN      OFFSET(22) NUMBITS(1) [],
        I2C3EN      OFFSET(23) NUMBITS(1) [],
        DACEN       OFFSET(29) NUMBITS(1) [],
        LPTIM1EN    OFFSET(31) NUMBITS(1) [],
    ],
    pub C2APB1ENR2 [
        LPUART1EN     OFFSET(0) NUMBITS(1) [],
        LPTIM2EN      OFFSET(5) NUMBITS(1) [],
        LPTIM3EN      OFFSET(6) NUMBITS(1) [],
    ],
    pub C2APB2ENR [
        ADCEN       OFFSET(9) NUMBITS(1) [],
        TIM1EN       OFFSET(11) NUMBITS(1) [],
        SPI1EN       OFFSET(12) NUMBITS(1) [],
        USART1EN       OFFSET(14) NUMBITS(1) [],
        TIM16EN       OFFSET(17) NUMBITS(1) [],
        TIM17EN       OFFSET(18) NUMBITS(1) [],
    ],
    pub C2APB3ENR [
        SUBGHZSPIEN       OFFSET(0) NUMBITS(1) [],
    ],
    pub C2AHB1SMENR [
        DMA1SMEN OFFSET(0) NUMBITS(1) [],
        DMA2SMEN OFFSET(1) NUMBITS(1) [],
        DMAMUX1SMEN OFFSET(2) NUMBITS(1) [],
        CRCSMEN OFFSET(12) NUMBITS(1) [],
    ],
    pub C2AHB2SMENR [
        GPIOASMEN OFFSET(0) NUMBITS(1) [],
        GPIOBSMEN OFFSET(1) NUMBITS(1) [],
        GPIOCSMEN OFFSET(2) NUMBITS(1) [],
        GPIOHSMEN OFFSET(7) NUMBITS(1) [],
    ],
    pub C2AHB3SMENR [
        PKASMEN OFFSET(16) NUMBITS(1) [],
        AESSMEN OFFSET(17) NUMBITS(1) [],
        RNGSMEN OFFSET(18) NUMBITS(1) [],
        SRAM1SMEN OFFSET(23) NUMBITS(1) [],
        SRAM2SMEN OFFSET(24) NUMBITS(1) [],
        FLASHSMEN OFFSET(25) NUMBITS(1) [],
    ],
    pub C2APB1SMENR1 [
        TIM2SMEN OFFSET(0) NUMBITS(1) [],
        RTCAPBSMEN OFFSET(10) NUMBITS(1) [],
        SPI2S2SMEN OFFSET(14) NUMBITS(1) [],
        USART2SMEN OFFSET(17) NUMBITS(1) [],
        I2C1SMEN OFFSET(21) NUMBITS(1) [],
        I2C2SMEN OFFSET(22) NUMBITS(1) [],
        I2C3SMEN OFFSET(23) NUMBITS(1) [],
        DACSMEN OFFSET(29) NUMBITS(1) [],
        LPTIM1SMEN OFFSET(31) NUMBITS(1) [],
    ],
    pub C2APB1SMENR2 [
        LPUART1SMEN OFFSET(0) NUMBITS(1) [],
        LPTIM2SMEN OFFSET(5) NUMBITS(1) [],
        LPTIM3SMEN OFFSET(6) NUMBITS(1) [],
    ],
    pub C2APB2SMENR [
        ADCSMEN OFFSET(9) NUMBITS(1) [],
        TIM1SMEN OFFSET(11) NUMBITS(1) [],
        SPI1SMEN OFFSET(12) NUMBITS(1) [],
        USART1SMEN OFFSET(14) NUMBITS(1) [],
        TIM16SMEN OFFSET(17) NUMBITS(1) [],
        TIM17SMEN OFFSET(18) NUMBITS(1) [],
    ],
    pub C2APB3SMENR [
        SUBGHZSPISMEN OFFSET(0) NUMBITS(1) [],
    ],

];

const RCC_BASE: StaticRef<RccRegisters> =
    unsafe { StaticRef::new(0x5800_0000 as *const RccRegisters) };

// Default values for PLL configuration
pub const DEFAULT_PLLM_VALUE: u32 = 1; // Default value for PLLM divider
pub const DEFAULT_PLLN_VALUE: u32 = 8; // Default value for PLLN divider
pub const DEFAULT_PLLP_VALUE: u32 = 2; // Default value for PLLP divider
pub const DEFAULT_PLLQ_VALUE: u32 = 2; // Default value for PLLQ divider
pub const DEFAULT_PLLR_VALUE: u32 = 2; // Default value for PLLR divider

pub struct Rcc {
    registers: StaticRef<RccRegisters>,
}

impl Rcc {
    pub(crate) fn registers(&self) -> &RccRegisters {
        &*self.registers
    }
}