use crate::rcc::registers::*;
use kernel::utilities::registers::interfaces::{ReadWriteable};

impl Rcc {
    // GPIO
    pub(crate) fn reset_gpioa(&self) {
        self.registers().ahb2rstr.modify(AHB2RSTR::GPIOARST::SET);
        self.registers().ahb2rstr.modify(AHB2RSTR::GPIOARST::CLEAR);
    }

    pub(crate) fn reset_gpiob(&self) {
        self.registers().ahb2rstr.modify(AHB2RSTR::GPIOBRST::SET);
        self.registers().ahb2rstr.modify(AHB2RSTR::GPIOBRST::CLEAR);
    }
    

    pub(crate) fn reset_gpioc(&self) {
        self.registers().ahb2rstr.modify(AHB2RSTR::GPIOCRST::SET);
        self.registers().ahb2rstr.modify(AHB2RSTR::GPIOCRST::CLEAR);
    }

    pub(crate) fn reset_gpioh(&self) {
        self.registers().ahb2rstr.modify(AHB2RSTR::GPIOHRST::SET);
        self.registers().ahb2rstr.modify(AHB2RSTR::GPIOHRST::CLEAR);
    }

    pub(crate) fn set_gpioa_enable(&self, enable: bool) {
        if enable {
            self.registers().ahb2enr.modify(AHB2ENR::GPIOAEN::SET);
        } else {
            self.registers().ahb2enr.modify(AHB2ENR::GPIOAEN::CLEAR);
        }
    }

    pub(crate) fn set_gpiob_enable(&self, enable: bool) {
        if enable {
            self.registers().ahb2enr.modify(AHB2ENR::GPIOBEN::SET);
        } else {
            self.registers().ahb2enr.modify(AHB2ENR::GPIOBEN::CLEAR);
        }
    }

    pub(crate) fn set_gpioc_enable(&self, enable: bool) {
        if enable {
            self.registers().ahb2enr.modify(AHB2ENR::GPIOCEN::SET);
        } else {
            self.registers().ahb2enr.modify(AHB2ENR::GPIOCEN::CLEAR);
        }
    }
    pub(crate) fn set_gpioh_enable(&self, enable: bool) {
        if enable {
            self.registers().ahb2enr.modify(AHB2ENR::GPIOHEN::SET);
        } else {
            self.registers().ahb2enr.modify(AHB2ENR::GPIOHEN::CLEAR);
        }
    }
    
}