use crate::rcc::registers::*;
use kernel::utilities::registers::interfaces::{ReadWriteable, Readable};

impl Rcc {

    // I2C
    pub(crate) fn is_enabled_i2c1_clock(&self) -> bool {
        self.registers().apb1enr1.is_set(APB1ENR1::I2C1EN)
    }
    pub(crate) fn is_enabled_i2c2_clock(&self) -> bool {
        self.registers().apb1enr1.is_set(APB1ENR1::I2C2EN)
    }
    pub(crate) fn is_enabled_i2c3_clock(&self) -> bool {
        self.registers().apb1enr1.is_set(APB1ENR1::I2C3EN)
    }

    pub(crate) fn set_i2c1_enable(&self, enable: bool) {
        if enable {
            self.registers().apb1enr1.modify(APB1ENR1::I2C1EN::SET);
        } else {
            self.registers().apb1enr1.modify(APB1ENR1::I2C1EN::CLEAR);
        }
    }

    pub(crate) fn set_i2c2_enable(&self, enable: bool) {
        if enable {
            self.registers().apb1enr1.modify(APB1ENR1::I2C2EN::SET);
        } else {
            self.registers().apb1enr1.modify(APB1ENR1::I2C2EN::CLEAR);
        }
    }

    pub(crate) fn set_i2c3_enable(&self, enable: bool) {
        if enable {
            self.registers().apb1enr1.modify(APB1ENR1::I2C3EN::SET);
        } else {
            self.registers().apb1enr1.modify(APB1ENR1::I2C3EN::CLEAR);
        }
    }

    // SPI
    pub(crate) fn is_spi1_clock_enabled(&self) -> bool {
        self.registers().apb2enr.is_set(APB2ENR::SPI1EN)
    }

    pub(crate) fn is_spi2s2_clock_enabled(&self) -> bool {
        self.registers().apb1enr1.is_set(APB1ENR1::SPI2S2EN)
    }


    pub (crate) fn set_spi1_clock_enable(&self, enable: bool) {
        if enable {
            self.registers().apb2enr.modify(APB2ENR::SPI1EN::SET);
        } else {
            self.registers().apb2enr.modify(APB2ENR::SPI1EN::CLEAR);
        }
    }

    pub (crate) fn set_spi2s2_clock_enable(&self, enable: bool) {
        if enable {
            self.registers().apb1enr1.modify(APB1ENR1::SPI2S2EN::SET);
        } else {
            self.registers().apb1enr1.modify(APB1ENR1::SPI2S2EN::CLEAR);
        }
    }

    // USART
    pub (crate) fn is_usart1_clock_enabled(&self) -> bool {
        self.registers().apb2enr.is_set(APB2ENR::USART1EN)
    }

    pub (crate) fn is_usart2_clock_enabled(&self) -> bool {
        self.registers().apb1enr1.is_set(APB1ENR1::USART2EN)
    }

    pub (crate) fn set_usart1_clock_enable(&self, enable: bool) {
        if enable {
            self.registers().apb2enr.modify(APB2ENR::USART1EN::SET);
        } else {
            self.registers().apb2enr.modify(APB2ENR::USART1EN::CLEAR);
        }
    }

    pub (crate) fn set_usart2_clock_enable(&self, enable: bool) {
        if enable {
            self.registers().apb1enr1.modify(APB1ENR1::USART2EN::SET);
        } else {
            self.registers().apb1enr1.modify(APB1ENR1::USART2EN::CLEAR);
        }
    }

    // LPUART
    pub (crate) fn is_lpuart1_clock_enabled(&self) -> bool {
        self.registers().apb1enr2.is_set(APB1ENR2::LPUART1EN)
    }
    pub (crate) fn set_lpuart1_clock_enable(&self, enable: bool) {
        if enable {
            self.registers().apb1enr2.modify(APB1ENR2::LPUART1EN::SET);
        } else {
            self.registers().apb1enr2.modify(APB1ENR2::LPUART1EN::CLEAR);
        }
    }
    pub (crate) fn reset_lpuart(&self) {
        self.registers().apb1rstr2.modify(APB1RSTR2::LPUART1RST::SET);
    }

}