use crate::rcc::registers::*;
use kernel::utilities::registers::interfaces::{ReadWriteable, Readable};

impl Rcc {
    //*******************************************************************//
    //                      HSE Clock Methods                            //
    //******************************************************************//

    pub (crate) fn set_hse_enable(&self, enable: bool) {
        if enable {
            self.registers().cr.modify(CR::HSEON::SET);
        } else {
            self.registers().cr.modify(CR::HSEON::CLEAR);
        }
    }

    pub (crate) fn is_hse_is_ready(&self) -> bool {
        self.registers().cr.is_set(CR::HSERDY)
    }

    pub (crate) fn set_hse_security_system_enable(&self, enable: bool) {
        if enable {
            self.registers().cr.modify(CR::CSSON::SET);
        } else {
            self.registers().cr.modify(CR::CSSON::CLEAR);
        }
    }

    // Control the division factor of SYSCLK when selecting HSE32 clock
    // 0: not divided
    // 1: divided by  2
    pub (crate) fn set_hse_sysclock_prescaler_enable(&self, enable: bool) {
        if enable {
            self.registers().cr.modify(CR::HSEPRE::SET);
        } else {
            self.registers().cr.modify(CR::HSEPRE::CLEAR);
        }
    }

    // It can only be written when HSE32 oscillator is disabled
    pub (crate) fn set_hse_vddtcxo_output_enable(&self, enable: bool) {
        if enable {
            // VDDTCXO output
            self.registers().cr.modify(CR::HSEBYPPWR::SET);
        } else {
            // PB0 selected (GPIO pin)
            self.registers().cr.modify(CR::HSEBYPPWR::CLEAR);
        }
    }
}