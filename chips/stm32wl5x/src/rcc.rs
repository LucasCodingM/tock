// Licensed under the Apache License, Version 2.0 or the MIT License.
// SPDX-License-Identifier: Apache-2.0 OR MIT
// Copyright Tock Contributors 2022.

use kernel::utilities::registers::interfaces::{ReadWriteable, Readable};
use kernel::utilities::registers::{register_bitfields, ReadWrite};
use kernel::utilities::StaticRef;

/// Reset and clock control
#[repr(C)]
struct RccRegisters {
    cr: ReadWrite<u32, CR::Register>,
    icscr: ReadWrite<u32, ICSCR::Register>,
    cfgr: ReadWrite<u32, CFGR::Register>,
    pllcfgr: ReadWrite<u32, PLLCFGR::Register>,
    cier: ReadWrite<u32, CIER::Register>,
    cifr: ReadWrite<u32, CIFR::Register>,
    cicr: ReadWrite<u32, CICR::Register>,
    ahb1rstr: ReadWrite<u32, AHB1RSTR::Register>,
    ahb2rstr: ReadWrite<u32, AHB2RSTR::Register>,
    ahb3rstr: ReadWrite<u32, AHB3RSTR::Register>,
    apb1rstr1: ReadWrite<u32, APB1RSTR1::Register>,
    apb1rstr2: ReadWrite<u32, APB1RSTR2::Register>,
    apb2rstr: ReadWrite<u32, APB2RSTR::Register>,
    apb3rstr: ReadWrite<u32, APB3RSTR::Register>,
    ahb1enr: ReadWrite<u32, AHB1ENR::Register>,
    ahb2enr: ReadWrite<u32, AHB2ENR::Register>,
    ahb3enr: ReadWrite<u32, AHB3ENR::Register>,
    apb1enr1: ReadWrite<u32, APB1ENR1::Register>,
    apb1enr2: ReadWrite<u32, APB1ENR2::Register>,
    apb2enr: ReadWrite<u32, APB2ENR::Register>,
    apb3enr: ReadWrite<u32, APB3ENR::Register>,
    ahb1smenr: ReadWrite<u32, AHB1SMENR::Register>,
    ahb2smenr: ReadWrite<u32, AHB2SMENR::Register>,
    ahb3smenr: ReadWrite<u32, AHB3SMENR::Register>,
    apb1smenr1: ReadWrite<u32, APB1SMENR1::Register>,
    apb1smenr2: ReadWrite<u32, APB1SMENR2::Register>,
    apb2smenr: ReadWrite<u32, APB2SMENR::Register>,
    apb3smenr: ReadWrite<u32, APB3SMENR::Register>,
    ccipr: ReadWrite<u32, CCIPR::Register>,
    bdcr: ReadWrite<u32, BDCR::Register>,
    csr: ReadWrite<u32, CSR::Register>,
    extcfgr: ReadWrite<u32, EXTCFGR::Register>,
    c2ahb1enr: ReadWrite<u32, C2AHB1ENR::Register>,
    c2ahb2enr: ReadWrite<u32, C2AHB2ENR::Register>,
    c2ahb3enr: ReadWrite<u32, C2AHB3ENR::Register>,
    c2apb1enr1: ReadWrite<u32, C2APB1ENR1::Register>,
    c2apb1enr2: ReadWrite<u32, C2APB1ENR2::Register>,
    capb2enr: ReadWrite<u32, C2APB2ENR::Register>,
    c2apb3enr: ReadWrite<u32, C2APB3ENR::Register>,
    c2ahb1smenr: ReadWrite<u32, C2AHB1SMENR::Register>,
    c2ahb2smenr: ReadWrite<u32, C2AHB2SMENR::Register>,
    c2ahb3smenr: ReadWrite<u32, C2AHB3SMENR::Register>,
    c2apb1smenr1: ReadWrite<u32, C2APB1SMENR1::Register>,
    c2apb1smenr2: ReadWrite<u32, C2APB1SMENR2::Register>,
    c2apb2smenr: ReadWrite<u32, C2APB2SMENR::Register>,
    c2apb3smenr: ReadWrite<u32, C2APB3SMENR::Register>,    
}

register_bitfields![u32,
    CR [
        MSION OFFSET(0) NUMBITS(1) [],
        MSIRDY OFFSET(1) NUMBITS(1) [],
        MSIPLLEN OFFSET(2) NUMBITS(1) [],
        MSIRGSEL OFFSET(3) NUMBITS(1) [],
        MSIRANGE OFFSET(4) NUMBITS(4) [],
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
    ICSCR [
        MSICAL OFFSET(0) NUMBITS(8) [],
        MSITRIM OFFSET(8) NUMBITS(8) [],
        HSICAL OFFSET(16) NUMBITS(8) [],
        HSITRIM OFFSET(24) NUMBITS(8) [],
    ],
    CFGR [
    MCOPRE OFFSET(28) NUMBITS(3) [],

    MCOSEL OFFSET(24) NUMBITS(4) [],

    PPRE2F OFFSET(18) NUMBITS(1) [],
    PPRE1F OFFSET(17) NUMBITS(1) [],
    
    HPREF OFFSET(16) NUMBITS(1) [],

    STOPWUCK OFFSET(15) NUMBITS(1) [],
    /// APB high-speed prescaler (APB2)
    PPRE2 OFFSET(11) NUMBITS(3) [],
    /// APB Low speed prescaler (APB1)
    PPRE1 OFFSET(8) NUMBITS(3) [],
    /// AHB prescaler
    HIPRE OFFSET(4) NUMBITS(4) [],
    /// System clock switch status
    SWS OFFSET(2) NUMBITS(2) [
        MSI = 0b00,
        HSI16 = 0b01,
        HSE32 = 0b10,
        PLLRCLK = 0b11,        
    ],
    /// System clock switch
    SW OFFSET(0) NUMBITS(2) [
        MSI = 0b00,
        HSI16 = 0b01,
        HSE32 = 0b10,
        PLLRCLK = 0b11,        
    ]
    ],
    PLLCFGR [
        PLLSRC OFFSET(0) NUMBITS(1) [
            HSI = 0,
            HSE = 1,
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
        PLLQ OFFSET(25) NUMBITS(3) [],
        PLLREN OFFSET(28) NUMBITS(1) [],
        PLLR OFFSET(29) NUMBITS(3) [],           
    ],
    CIER [
        LSIRDYIE OFFSET(0) NUMBITS(1) [],
        LSERDYIE OFFSET(1) NUMBITS(1) [],
        MSIRDYIE OFFSET(2) NUMBITS(1) [],
        HSIRDYIE OFFSET(3) NUMBITS(1) [],
        HSERDYIE OFFSET(4) NUMBITS(1) [],
        PLLRDYIE OFFSET(5) NUMBITS(1) [],
        LSECSSIE OFFSET(9) NUMBITS(1) [],
    ],
    CIFR [
        LSIRDYF OFFSET(0) NUMBITS(1) [],
        LSERDYF OFFSET(1) NUMBITS(1) [],
        MSIRDYF OFFSET(2) NUMBITS(1) [],
        HSIRDYF OFFSET(3) NUMBITS(1) [],
        HSERDYF OFFSET(4) NUMBITS(1) [],
        PLLRDYF OFFSET(5) NUMBITS(1) [],
        CSSF    OFFSET(8) NUMBITS(1) [],
        LSECSSF OFFSET(9) NUMBITS(1) []
    ],
    CICR [
        LSIRDYC OFFSET(0) NUMBITS(1) [],
        LSERDYC OFFSET(1) NUMBITS(1) [],
        MSIDYC OFFSET(2) NUMBITS(1) [],
        HSIRDYC OFFSET(3) NUMBITS(1) [],
        HSERDYC OFFSET(4) NUMBITS(1) [],
        PLLRDYC OFFSET(5) NUMBITS(1) [],
        CSSC    OFFSET(8) NUMBITS(1) [],
        LSECSSC OFFSET(9) NUMBITS(1) []
    ],
    AHB1RSTR [
         DMA1RST OFFSET(0) NUMBITS(1) [],
         DMA2RST OFFSET(1) NUMBITS(1) [],
         DMAMUX1RST OFFSET(2) NUMBITS(1) [],
         CRCRST OFFSET(12) NUMBITS(1) [],
    ],
    AHB2RSTR [
        GPIOARST OFFSET(0) NUMBITS(1) [],
        GPIOBRST OFFSET(1) NUMBITS(1) [],
        GPIOCRST OFFSET(2) NUMBITS(1) [],
        GPIOHRST OFFSET(7) NUMBITS(1) [],
    ],
    AHB3RSTR [
        PKARST OFFSET(16) NUMBITS(1) [],
        AESRST OFFSET(17) NUMBITS(1) [],
        RNGRST OFFSET(18) NUMBITS(1) [],
        HSEMRST OFFSET(19) NUMBITS(1) [],
        IPCCRST OFFSET(20) NUMBITS(1) [],
        FLASHRST OFFSET(25) NUMBITS(1) [],
    ],
    APB1RSTR1 [
        TIM2RST OFFSET(0) NUMBITS(1) [],
        SPI2S2RST OFFSET(14) NUMBITS(1) [],
        USART2RST OFFSET(17) NUMBITS(1) [],
        I2C1RST OFFSET(21) NUMBITS(1) [],
        I2C2RST OFFSET(22) NUMBITS(1) [],
        I2C3RST OFFSET(23) NUMBITS(1) [],
        DACRST OFFSET(29) NUMBITS(1) [],
        LPTIM1RST OFFSET(31) NUMBITS(1) [],
    ],
    APB1RSTR2 [
        LPUART1RST OFFSET(0) NUMBITS(1) [],
        LPTIM2RST OFFSET(5) NUMBITS(1) [],
        LPTIM3RST OFFSET(6) NUMBITS(1) [],
    ],
    APB2RSTR [
        ADCRST OFFSET(9) NUMBITS(1) [],
        TIM1RST OFFSET(11) NUMBITS(1) [],
        SPI1RST OFFSET(12) NUMBITS(1) [],
        USART1RST OFFSET(14) NUMBITS(1) [],
        TIM16RST OFFSET(17) NUMBITS(1) [],
        TIM17RST OFFSET(18) NUMBITS(1) [],
    ],
    APB3RSTR [
        SUBGHZSPIRST OFFSET(0) NUMBITS(1) [],
    ],
    AHB1ENR [
        DMA1EN OFFSET(0) NUMBITS(1) [],
        DMA2EN OFFSET(1) NUMBITS(1) [],
        DMAMUX1EN OFFSET(2) NUMBITS(1) [],
        CRCEN OFFSET(12) NUMBITS(1) [],
    ],
    AHB2ENR [
        GPIOAEN OFFSET(0) NUMBITS(1) [],
        GPIOBEN OFFSET(1) NUMBITS(1) [],
        GPIOCEN OFFSET(2) NUMBITS(1) [],
        GPIOHEN OFFSET(7) NUMBITS(1) [],
    ],
    AHB3ENR [
        PKAEN OFFSET(16) NUMBITS(1) [],
        AESEN OFFSET(17) NUMBITS(1) [],
        RNGEN OFFSET(18) NUMBITS(1) [],
        HSEMEN OFFSET(19) NUMBITS(1) [],
        IPCC OFFSET(20) NUMBITS(1) [],
        FLASHEN OFFSET(25) NUMBITS(1) [],
    ],
    APB1ENR1 [
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
    APB1ENR2 [
        LPUART1EN     OFFSET(0) NUMBITS(1) [],
        LPTIM2EN      OFFSET(5) NUMBITS(1) [],
        LPTIM3EN      OFFSET(6) NUMBITS(1) [],
    ],
    APB2ENR [
        ADCEN       OFFSET(9) NUMBITS(1) [],
        TIM1EN       OFFSET(11) NUMBITS(1) [],
        SPI1EN       OFFSET(12) NUMBITS(1) [],
        USART1EN       OFFSET(14) NUMBITS(1) [],
        TIM16EN       OFFSET(17) NUMBITS(1) [],
        TIM17EN       OFFSET(18) NUMBITS(1) [],
    ],
    APB3ENR [
        SUBGHZSPIEN       OFFSET(0) NUMBITS(1) [],
    ],
    AHB1SMENR [
        DMA1SMEN OFFSET(0) NUMBITS(1) [],
        DMA2SMEN OFFSET(1) NUMBITS(1) [],
        DMAMUX1SMEN OFFSET(2) NUMBITS(1) [],
        CRCSMEN OFFSET(12) NUMBITS(1) [],
    ],
    AHB2SMENR [
        GPIOASMEN OFFSET(0) NUMBITS(1) [],
        GPIOBSMEN OFFSET(1) NUMBITS(1) [],
        GPIOCSMEN OFFSET(2) NUMBITS(1) [],
        GPIOHSMEN OFFSET(7) NUMBITS(1) [],
    ],
    AHB3SMENR [
        PKASMEN OFFSET(16) NUMBITS(1) [],
        AESSMEN OFFSET(17) NUMBITS(1) [],
        RNGSMEN OFFSET(18) NUMBITS(1) [],
        SRAM1SMEN OFFSET(23) NUMBITS(1) [],
        SRAM2SMEN OFFSET(24) NUMBITS(1) [],
        FLASHSMEN OFFSET(25) NUMBITS(1) [],
    ],
    APB1SMENR1 [
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
    APB1SMENR2 [
        LPUART1SMEN OFFSET(0) NUMBITS(1) [],
        LPTIM2SMEN OFFSET(5) NUMBITS(1) [],
        LPTIM3SMEN OFFSET(6) NUMBITS(1) [],
    ],
    APB2SMENR [
        ADCSMEN OFFSET(9) NUMBITS(1) [],
        TIM1SMEN OFFSET(11) NUMBITS(1) [],
        SPI1SMEN OFFSET(12) NUMBITS(1) [],
        USART1SMEN OFFSET(14) NUMBITS(1) [],
        TIM16SMEN OFFSET(17) NUMBITS(1) [],
        TIM17SMEN OFFSET(18) NUMBITS(1) [],
    ],
    APB3SMENR [
        SUBGHZSPISMEN OFFSET(0) NUMBITS(1) [],
    ],
    CCIPR [
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
    BDCR [
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
    CSR [
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
    EXTCFGR [
        SHDHPRE OFFSET(0) NUMBITS(4) [
            DIVIDEDBY2 = 0b1000,
            DIVIDEDBY3 = 0b0001,
            DIVIDEDBY4 = 0b1001,
            DIVIDEDBY5 = 0b0010,
            DIVIDEDBY6 = 0b0101,
            DIVIDEDBY8 = 0b1010,
            DIVIDEDBY10 = 0b0110,
            DIVIDEDBY16 = 0b1011,           
            DIVIDEDBY32 = 0b0111,
            DIVIDEDBY64 = 0b1100,
            DIVIDEDBY128 = 0b1101,
            DIVIDEDBY256 = 0b1110,
            DIVIDEDBY512 = 0b1111,
        ],
        C2HPRE OFFSET(4) NUMBITS(4) [
            DIVIDEDBY2 = 0b1000,
            DIVIDEDBY3 = 0b0001,
            DIVIDEDBY4 = 0b1001,
            DIVIDEDBY5 = 0b0010,
            DIVIDEDBY6 = 0b0101,
            DIVIDEDBY8 = 0b1010,
            DIVIDEDBY10 = 0b0110,
            DIVIDEDBY16 = 0b1011,           
            DIVIDEDBY32 = 0b0111,
            DIVIDEDBY64 = 0b1100,
            DIVIDEDBY128 = 0b1101,
            DIVIDEDBY256 = 0b1110,
            DIVIDEDBY512 = 0b1111,
        ],
        SHDHPREF OFFSET(16) NUMBITS(1) [],
        C2HPREF OFFSET(17) NUMBITS(1) [],
    ],
    C2AHB1ENR [
        DMA1EN OFFSET(0) NUMBITS(1) [],
        DMA2EN OFFSET(1) NUMBITS(1) [],
        DMAMUX1EN OFFSET(2) NUMBITS(1) [],
        CRCEN OFFSET(12) NUMBITS(1) [],
    ],
    C2AHB2ENR [
        GPIOAEN OFFSET(0) NUMBITS(1) [],
        GPIOBEN OFFSET(1) NUMBITS(1) [],
        GPIOCEN OFFSET(2) NUMBITS(1) [],
        GPIOHEN OFFSET(7) NUMBITS(1) [],
    ],
    C2AHB3ENR [
        PKAEN OFFSET(16) NUMBITS(1) [],
        AESEN OFFSET(17) NUMBITS(1) [],
        RNGEN OFFSET(18) NUMBITS(1) [],
        HSEMEN OFFSET(19) NUMBITS(1) [],
        IPCC OFFSET(20) NUMBITS(1) [],
        FLASHEN OFFSET(25) NUMBITS(1) [],
    ],
    C2APB1ENR1 [
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
    C2APB1ENR2 [
        LPUART1EN     OFFSET(0) NUMBITS(1) [],
        LPTIM2EN      OFFSET(5) NUMBITS(1) [],
        LPTIM3EN      OFFSET(6) NUMBITS(1) [],
    ],
    C2APB2ENR [
        ADCEN       OFFSET(9) NUMBITS(1) [],
        TIM1EN       OFFSET(11) NUMBITS(1) [],
        SPI1EN       OFFSET(12) NUMBITS(1) [],
        USART1EN       OFFSET(14) NUMBITS(1) [],
        TIM16EN       OFFSET(17) NUMBITS(1) [],
        TIM17EN       OFFSET(18) NUMBITS(1) [],
    ],
    C2APB3ENR [
        SUBGHZSPIEN       OFFSET(0) NUMBITS(1) [],
    ],
    C2AHB1SMENR [
        DMA1SMEN OFFSET(0) NUMBITS(1) [],
        DMA2SMEN OFFSET(1) NUMBITS(1) [],
        DMAMUX1SMEN OFFSET(2) NUMBITS(1) [],
        CRCSMEN OFFSET(12) NUMBITS(1) [],
    ],
    C2AHB2SMENR [
        GPIOASMEN OFFSET(0) NUMBITS(1) [],
        GPIOBSMEN OFFSET(1) NUMBITS(1) [],
        GPIOCSMEN OFFSET(2) NUMBITS(1) [],
        GPIOHSMEN OFFSET(7) NUMBITS(1) [],
    ],
    C2AHB3SMENR [
        PKASMEN OFFSET(16) NUMBITS(1) [],
        AESSMEN OFFSET(17) NUMBITS(1) [],
        RNGSMEN OFFSET(18) NUMBITS(1) [],
        SRAM1SMEN OFFSET(23) NUMBITS(1) [],
        SRAM2SMEN OFFSET(24) NUMBITS(1) [],
        FLASHSMEN OFFSET(25) NUMBITS(1) [],
    ],
    C2APB1SMENR1 [
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
    C2APB1SMENR2 [
        LPUART1SMEN OFFSET(0) NUMBITS(1) [],
        LPTIM2SMEN OFFSET(5) NUMBITS(1) [],
        LPTIM3SMEN OFFSET(6) NUMBITS(1) [],
    ],
    C2APB2SMENR [
        ADCSMEN OFFSET(9) NUMBITS(1) [],
        TIM1SMEN OFFSET(11) NUMBITS(1) [],
        SPI1SMEN OFFSET(12) NUMBITS(1) [],
        USART1SMEN OFFSET(14) NUMBITS(1) [],
        TIM16SMEN OFFSET(17) NUMBITS(1) [],
        TIM17SMEN OFFSET(18) NUMBITS(1) [],
    ],
    C2APB3SMENR [
        SUBGHZSPISMEN OFFSET(0) NUMBITS(1) [],
    ],

];

const RCC_BASE: StaticRef<RccRegisters> =
    unsafe { StaticRef::new(0x5800_0000 as *const RccRegisters) };

// Default values when the hardware is reset. Uncomment if you need them.
//pub(crate) const RESET_PLLM_VALUE: usize = PLLM::DivideBy16; // M = 16
//pub(crate) const RESET_PLLP_VALUE: PLLP = PLLP::DivideBy2; // P = 2
//pub(crate) const RESET_PLLQ_VALUE: PLLQ = PLLQ::DivideBy4; // Q = 4
pub(crate) const RESET_PLLN_VALUE: usize = 0b011_000_000; // N = 192

// Default PLL configuration. See Rcc::init_pll_clock() for more details.
//
// Choose PLLM::DivideBy8 for reduced PLL jitter or PLLM::DivideBy16 for default hardware
// configuration
pub(crate) const DEFAULT_PLLM_VALUE: PLLM = PLLM::DivideBy8;
// DON'T CHANGE THIS VALUE
pub(crate) const DEFAULT_PLLN_VALUE: usize = RESET_PLLN_VALUE;
// Dynamically computing the default PLLP value based on the PLLM value
pub(crate) const DEFAULT_PLLP_VALUE: PLLP = match DEFAULT_PLLM_VALUE {
    PLLM::DivideBy1 => PLLP::DivideBy8,  // VCO = 192 MHz → /8 = 24 MHz
    PLLM::DivideBy2 => PLLP::DivideBy4,  // VCO = 96 MHz → /4 = 24 MHz
    PLLM::DivideBy3 => PLLP::DivideBy4,  // VCO = ~64 MHz → /4 = 16 MHz
    PLLM::DivideBy4 => PLLP::DivideBy2,  // VCO = 48 MHz → /2 = 24 MHz
    PLLM::DivideBy5 => PLLP::DivideBy2,
    PLLM::DivideBy6 => PLLP::DivideBy2,
    PLLM::DivideBy7 => PLLP::DivideBy2,
    PLLM::DivideBy8 => PLLP::DivideBy2,
};
// Dynamically computing the default PLLQ value based on the PLLM value
pub(crate) const DEFAULT_PLLQ_VALUE: PLLQ = match DEFAULT_PLLM_VALUE {
    PLLM::DivideBy1 => PLLQ::DivideBy8, // VCO = 192 MHz → /8 = 24 MHz
    PLLM::DivideBy2 => PLLQ::DivideBy4, // VCO = 96 MHz → /4 = 24 MHz
    PLLM::DivideBy3 => PLLQ::DivideBy3, // VCO = 64 MHz → /3 ≈ 21.3 MHz
    PLLM::DivideBy4 => PLLQ::DivideBy2, // VCO = 48 MHz → /2 = 24 MHz
    PLLM::DivideBy5 => PLLQ::DivideBy2, // VCO = 38.4 MHz
    PLLM::DivideBy6 => PLLQ::DivideBy2, // VCO = 32 MHz
    PLLM::DivideBy7 => PLLQ::DivideBy2, // VCO = ~27.4 MHz
    PLLM::DivideBy8 => PLLQ::DivideBy2, // VCO = 24 MHz
};

pub struct Rcc {
    registers: StaticRef<RccRegisters>,
}

pub enum RtcClockSource {
    LSI,
    LSE,
    HSE32,
}

impl Rcc {
    pub fn new() -> Self {
        let rcc = Self {
            registers: RCC_BASE,
        };
        rcc.init();
        rcc
    }

    // Some clocks need to be initialized before use
    fn init(&self) {
        self.init_pll_clock();
    }

    // Init the PLL clock. The default configuration:
    // + if DEFAULT_PLLM_VALUE == PLLM::DivideBy8:
    //   + 2MHz VCO input frequency for reduced PLL jitter: freq_VCO_input = freq_source / PLLM
    //   + 384MHz VCO output frequency: freq_VCO_output = freq_VCO_input * PLLN
    //   + 96MHz main output frequency: freq_PLL = freq_VCO_output / PLLP
    //   + 48MHz PLL48CLK output frequency: freq_PLL48CLK = freq_VCO_output / PLLQ
    // + if DEFAULT_PLLM_VALUE == PLLM::DivideBy16: (default hardware configuration)
    //   + 1MHz VCO input frequency for reduced PLL jitter: freq_VCO_input = freq_source / PLLM
    //   + 384MHz VCO output frequency: freq_VCO_output = freq_VCO_input * PLLN
    //   + 96MHz main output frequency: freq_PLL = freq_VCO_output / PLLP
    //   + 48MHz PLL48CLK output frequency: freq_PLL48CLK = freq_VCO_output / PLLQ
    fn init_pll_clock(&self) {
        self.set_pll_clocks_source(PllSource::HSI);
        self.set_pll_clocks_m_divider(DEFAULT_PLLM_VALUE);
        self.set_pll_clock_n_multiplier(DEFAULT_PLLN_VALUE);
        self.set_pll_clock_p_divider(DEFAULT_PLLP_VALUE);
        self.set_pll_clock_q_divider(DEFAULT_PLLQ_VALUE);
    }

    // Get the current system clock source
    pub(crate) fn get_sys_clock_source(&self) -> SysClockSource {
        match self.registers.cfgr.read(CFGR::SWS) {
            0b00 => SysClockSource::MSI,
            0b01 => SysClockSource::HSI16,
            0b10 => SysClockSource::HSE32,
            _ => SysClockSource::PLLRCLK,
        }
    }

    // Set the system clock source
    // The source must be enabled
    // NOTE: The flash latency also needs to be configured when changing the system clock frequency
    pub(crate) fn set_sys_clock_source(&self, source: SysClockSource) {
        self.registers.cfgr.modify(CFGR::SW.val(source as u32));
    }   

    /* HSI clock */
    // The HSI clock must not be configured as the system clock, either directly or indirectly.
    pub(crate) fn disable_hsi_clock(&self) {
        self.registers.cr.modify(CR::HSION::CLEAR);
    }

    pub(crate) fn enable_hsi_clock(&self) {
        self.registers.cr.modify(CR::HSION::SET);
    }

    pub(crate) fn is_enabled_hsi_clock(&self) -> bool {
        self.registers.cr.is_set(CR::HSION)
    }

    // Indicates whether the HSI oscillator is stable
    pub(crate) fn is_ready_hsi_clock(&self) -> bool {
        self.registers.cr.is_set(CR::HSIRDY)
    }

    pub(crate) fn enable_hse_clock(&self) {
        self.registers.cr.modify(CR::HSEON::SET);
    }

    pub(crate) fn is_enabled_hse_clock(&self) -> bool {
        self.registers.cr.is_set(CR::HSEON)
    }

    // Indicates whether the HSE oscillator is stable
    pub(crate) fn is_ready_hse_clock(&self) -> bool {
        self.registers.cr.is_set(CR::HSERDY)
    }

    /* Main PLL clock*/

    // The main PLL clock must not be configured as the system clock.
    pub(crate) fn disable_pll_clock(&self) {
        self.registers.cr.modify(CR::PLLON::CLEAR);
    }

    pub(crate) fn enable_pll_clock(&self) {
        self.registers.cr.modify(CR::PLLON::SET);
    }

    pub(crate) fn is_enabled_pll_clock(&self) -> bool {
        self.registers.cr.is_set(CR::PLLON)
    }

    // The PLL clock is locked when its signal is stable
    pub(crate) fn is_locked_pll_clock(&self) -> bool {
        self.registers.cr.is_set(CR::PLLRDY)
    }

    pub(crate) fn get_pll_clocks_source(&self) -> PllSource {
        match self.registers.pllcfgr.read(PLLCFGR::PLLSRC) {
            0b0 => PllSource::HSI,
            _ => PllSource::HSE,
        }
    }

    // This method must be called only when all PLL clocks are disabled
    pub(crate) fn set_pll_clocks_source(&self, source: PllSource) {
        self.registers
            .pllcfgr
            .modify(PLLCFGR::PLLSRC.val(source as u32));
    }

    pub(crate) fn get_pll_clocks_m_divider(&self) -> PLLM {
        match self.registers.pllcfgr.read(PLLCFGR::PLLM) {
            1 => PLLM::DivideBy1,
            2 => PLLM::DivideBy2,
            3 => PLLM::DivideBy3,
            4 => PLLM::DivideBy4,
            5 => PLLM::DivideBy5,
            6 => PLLM::DivideBy6,
            7 => PLLM::DivideBy7,
            8 => PLLM::DivideBy8,
            _ => panic!("Unexpected PLLM divider"),
        }
    }

    // This method must be called only when all PLL clocks are disabled
    pub(crate) fn set_pll_clocks_m_divider(&self, m: PLLM) {
        self.registers.pllcfgr.modify(PLLCFGR::PLLM.val(m as u32));
    }

    pub(crate) fn get_pll_clock_n_multiplier(&self) -> usize {
        self.registers.pllcfgr.read(PLLCFGR::PLLN) as usize
    }

    // This method must be called only if the main PLL clock is disabled
    pub(crate) fn set_pll_clock_n_multiplier(&self, n: usize) {
        self.registers.pllcfgr.modify(PLLCFGR::PLLN.val(n as u32));
    }

    pub(crate) fn get_pll_clock_p_divider(&self) -> PLLP {
        match self.registers.pllcfgr.read(PLLCFGR::PLLP) {
            0b00 => PLLP::DivideBy2,
            0b01 => PLLP::DivideBy4,
            0b10 => PLLP::DivideBy6,
            _ => PLLP::DivideBy8,
        }
    }

    // This method must be called only if the main PLL clock is disabled
    pub(crate) fn set_pll_clock_p_divider(&self, p: PLLP) {
        self.registers.pllcfgr.modify(PLLCFGR::PLLP.val(p as u32));
    }

    pub(crate) fn _get_pll_clock_q_divider(&self) -> PLLQ {
        match self.registers.pllcfgr.read(PLLCFGR::PLLQ) {
            2 => PLLQ::DivideBy2,
            3 => PLLQ::DivideBy3,
            4 => PLLQ::DivideBy4,
            5 => PLLQ::DivideBy5,
            6 => PLLQ::DivideBy6,
            7 => PLLQ::DivideBy7,
            8 => PLLQ::DivideBy8,
            _ => panic!("Unexpected PLLQ divider"),
        }
    }

    // This method must be called only if the main PLL clock is disabled
    pub(crate) fn set_pll_clock_q_divider(&self, q: PLLQ) {
        self.registers.pllcfgr.modify(PLLCFGR::PLLQ.val(q as u32));
    }   

    /* APB1 prescaler */

    pub(crate) fn set_apb1_prescaler(&self, apb1_prescaler: APBPrescaler) {
        self.registers
            .cfgr
            .modify(CFGR::PPRE1.val(apb1_prescaler as u32));
    }

    pub(crate) fn get_apb1_prescaler(&self) -> APBPrescaler {
        match self.registers.cfgr.read(CFGR::PPRE1) {
            0b100 => APBPrescaler::DivideBy2,
            0b101 => APBPrescaler::DivideBy4,
            0b110 => APBPrescaler::DivideBy8,
            0b111 => APBPrescaler::DivideBy16,
            _ => APBPrescaler::DivideBy1, // 0b0xx means no division
        }
    }

    /* APB2 prescaler */

    pub(crate) fn set_apb2_prescaler(&self, apb2_prescaler: APBPrescaler) {
        self.registers
            .cfgr
            .modify(CFGR::PPRE2.val(apb2_prescaler as u32));
    }

    pub(crate) fn get_apb2_prescaler(&self) -> APBPrescaler {
        match self.registers.cfgr.read(CFGR::PPRE2) {
            0b100 => APBPrescaler::DivideBy2,
            0b101 => APBPrescaler::DivideBy4,
            0b110 => APBPrescaler::DivideBy8,
            0b111 => APBPrescaler::DivideBy16,
            _ => APBPrescaler::DivideBy1, // 0b0xx means no division
        }
    }
    

    pub(crate) fn configure_rng_clock(&self) {
        self.registers.pllcfgr.modify(PLLCFGR::PLLQ.val(2));
        self.registers.cr.modify(CR::PLLON::SET);
    }   

    // DMA1 clock

    pub(crate) fn is_enabled_dma1_clock(&self) -> bool {
        self.registers.ahb1enr.is_set(AHB1ENR::DMA1EN)
    }

    pub(crate) fn enable_dma1_clock(&self) {
        self.registers.ahb1enr.modify(AHB1ENR::DMA1EN::SET)
    }

    pub(crate) fn disable_dma1_clock(&self) {
        self.registers.ahb1enr.modify(AHB1ENR::DMA1EN::CLEAR)
    }

    // DMA2 clock
    pub(crate) fn is_enabled_dma2_clock(&self) -> bool {
        self.registers.ahb1enr.is_set(AHB1ENR::DMA2EN)
    }

    pub(crate) fn enable_dma2_clock(&self) {
        self.registers.ahb1enr.modify(AHB1ENR::DMA2EN::SET)
    }

    pub(crate) fn disable_dma2_clock(&self) {
        self.registers.ahb1enr.modify(AHB1ENR::DMA2EN::CLEAR)
    }
   

    // USART1 clock
    pub(crate) fn is_enabled_usart1_clock(&self) -> bool {
        self.registers.apb2enr.is_set(APB2ENR::USART1EN)
    }

    pub(crate) fn enable_usart1_clock(&self) {
        self.registers.apb2enr.modify(APB2ENR::USART1EN::SET)
    }

    pub(crate) fn disable_usart1_clock(&self) {
        self.registers.apb2enr.modify(APB2ENR::USART1EN::CLEAR)
    }
    

    // RTC clock
    pub(crate) fn source_into_u32(source: RtcClockSource) -> u32 {
        match source {
            RtcClockSource::LSE => 1,
            RtcClockSource::LSI => 2,
            RtcClockSource::HSE32 => 3,
        }
    }

    pub(crate) fn enable_lsi_clock(&self) {
        self.registers.csr.modify(CSR::LSION::SET);
    }
   

    pub(crate) fn enable_rtc_clock(&self, source: RtcClockSource) {
        // Enable LSI
        self.enable_lsi_clock();
        let mut counter = 1_000;
        while counter > 0 && !self.registers.csr.is_set(CSR::LSION) {
            counter -= 1;
        }
        if counter == 0 {
            panic!("Unable to activate lsi clock");
        }

        // Select RTC clock source
        let source_num = Rcc::source_into_u32(source);
        self.registers.bdcr.modify(BDCR::RTCSEL.val(source_num));

        // Enable RTC clock
        self.registers.bdcr.modify(BDCR::RTCEN::SET);
    }

    pub(crate) fn disable_rtc_clock(&self) {
        self.registers.bdcr.modify(BDCR::RTCEN.val(1));
        self.registers.bdcr.modify(BDCR::RTCSEL.val(0));
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub(crate) enum PLLP {
    DivideBy2 = 0b00001,
    DivideBy4 = 0b00011,
    DivideBy6 = 0b00101,
    DivideBy8 = 0b01000,
}

impl From<PLLP> for usize {
    // (variant_value + 1) * 2 = X for X in DivideByX
    fn from(item: PLLP) -> Self {
        (item as usize + 1) << 1
    }
}

// Theoretically, the PLLM value can range from 2 to 63. However, the current implementation was
// designed to support 1MHz frequency precision. In a future update, PLLM will become a usize.
#[allow(dead_code)]
pub(crate) enum PLLM {
    DivideBy1 = 1,
    DivideBy2 = 2,
    DivideBy3,
    DivideBy4,
    DivideBy5,
    DivideBy6,
    DivideBy7,
    DivideBy8,
}

#[derive(Copy, Clone, Debug, PartialEq)]
// Due to the restricted values for PLLM, PLLQ 2/10-15 values are meaningless.
pub(crate) enum PLLQ {
    DivideBy2 = 2,
    DivideBy3,
    DivideBy4,
    DivideBy5,
    DivideBy6,
    DivideBy7,
    DivideBy8,
}

/// Clock sources for the CPU
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SysClockSource {
    MSI = 0b00,
    HSI16 = 0b01,
    HSE32 = 0b10,
    PLLRCLK = 0b11
}

pub enum PllSource {
    HSI = 0b0,
    HSE = 0b1,
}

pub enum MCO1Source {
    HSI = 0b00,
    //LSE = 0b01, // When support for LSE is added, uncomment this
    HSE = 0b10,
    PLL = 0b11,
}

/// HSE Mode
#[derive(PartialEq)]
pub enum HseMode {
    BYPASS,
    CRYSTAL,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AHBPrescaler {
    DivideBy1 = 0b0000,
    DivideBy2 = 0b1000,
    DivideBy4 = 0b1001,
    DivideBy8 = 0b1010,
    DivideBy16 = 0b1011,
    DivideBy64 = 0b1100,
    DivideBy128 = 0b1101,
    DivideBy256 = 0b1110,
    DivideBy512 = 0b1111,
}

impl From<AHBPrescaler> for usize {
    fn from(item: AHBPrescaler) -> usize {
        match item {
            AHBPrescaler::DivideBy1 => 1,
            AHBPrescaler::DivideBy2 => 2,
            AHBPrescaler::DivideBy4 => 4,
            AHBPrescaler::DivideBy8 => 8,
            AHBPrescaler::DivideBy16 => 16,
            AHBPrescaler::DivideBy64 => 64,
            AHBPrescaler::DivideBy128 => 128,
            AHBPrescaler::DivideBy256 => 256,
            AHBPrescaler::DivideBy512 => 512,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum APBPrescaler {
    DivideBy1 = 0b000, // No division
    DivideBy2 = 0b100,
    DivideBy4 = 0b101,
    DivideBy8 = 0b110,
    DivideBy16 = 0b111,
}

impl From<APBPrescaler> for usize {
    fn from(item: APBPrescaler) -> Self {
        match item {
            APBPrescaler::DivideBy1 => 1,
            APBPrescaler::DivideBy2 => 2,
            APBPrescaler::DivideBy4 => 4,
            APBPrescaler::DivideBy8 => 8,
            APBPrescaler::DivideBy16 => 16,
        }
    }
}
