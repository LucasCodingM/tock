use crate::rcc::registers::*;
use kernel::utilities::registers::interfaces::{ReadWriteable, Readable};

impl Rcc {
    // ADC clock

    pub(crate) fn is_enabled_adc_clock(&self) -> bool {
        self.registers().apb2enr.is_set(APB2ENR::ADCEN)
    }

    pub(crate) fn set_adc_clock_enable(&self, enable: bool) {
        if enable {
            self.registers().apb2enr.modify(APB2ENR::ADCEN::SET);
        } else {
            self.registers().apb2enr.modify(APB2ENR::ADCEN::CLEAR);
        }
    }

    // DAC clock

    pub(crate) fn is_enabled_dac_clock(&self) -> bool {
        self.registers().apb1enr1.is_set(APB1ENR1::DACEN)
    }

    pub(crate) fn set_dac_clock_enable(&self, enable: bool) {
        if enable {
            self.registers().apb1enr1.modify(APB1ENR1::DACEN::SET);
        } else {
            self.registers().apb1enr1.modify(APB1ENR1::DACEN::CLEAR);
        }
    }
}