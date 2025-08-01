use crate::rcc::registers::*;
use kernel::utilities::registers::interfaces::{ReadWriteable, Readable};

impl Rcc {
    //TIM1
    pub(crate) fn is_enabled_tim1_clock(&self) -> bool {
        self.registers().apb2enr.is_set(APB2ENR::TIM1EN)
    }
    pub(crate) fn set_tim1_enable(&self, enable: bool) {
        if enable {
            self.registers().apb2enr.modify(APB2ENR::TIM1EN::SET);
        } else {
            self.registers().apb2enr.modify(APB2ENR::TIM1EN::CLEAR);
        }
    }

    pub (crate) fn reset_tim1(&self) {
        self.registers().apb2rstr.modify(APB2RSTR::TIM1RST::SET);
    }

    //TIM2
    pub(crate) fn is_enabled_tim2_clock(&self) -> bool {
        self.registers().apb1enr1.is_set(APB1ENR1::TIM2EN)
    }
    pub(crate) fn set_tim2_enable(&self, enable: bool) {
        if enable {
            self.registers().apb1enr1.modify(APB1ENR1::TIM2EN::SET);
        } else {
            self.registers().apb1enr1.modify(APB1ENR1::TIM2EN::CLEAR);
        }
    }

    pub (crate) fn reset_tim2(&self) {
        self.registers().apb1rstr1.modify(APB1RSTR1::TIM2RST::SET);
    }

    //TIM16
    pub(crate) fn is_enabled_tim16_clock(&self) -> bool {
        self.registers().apb2enr.is_set(APB2ENR::TIM16EN)
    }
    pub(crate) fn set_tim16_enable(&self, enable: bool) {
        if enable {
            self.registers().apb2enr.modify(APB2ENR::TIM16EN::SET);
        } else {
            self.registers().apb2enr.modify(APB2ENR::TIM16EN::CLEAR);
        }
    }

    pub (crate) fn reset_tim16(&self) {
        self.registers().apb2rstr.modify(APB2RSTR::TIM16RST::SET);
    }

    //TIM17
    pub(crate) fn is_enabled_tim17_clock(&self) -> bool {
        self.registers().apb2enr.is_set(APB2ENR::TIM17EN)
    }
    pub(crate) fn set_tim17_enable(&self, enable: bool) {
        if enable {
            self.registers().apb2enr.modify(APB2ENR::TIM17EN::SET);
        } else {
            self.registers().apb2enr.modify(APB2ENR::TIM17EN::CLEAR);
        }
    }

    pub (crate) fn reset_tim17(&self) {
        self.registers().apb2rstr.modify(APB2RSTR::TIM17RST::SET);
    }


    //LPTIM1
    pub(crate) fn is_enabled_lptim1_clock(&self) -> bool {
        self.registers().apb1enr1.is_set(APB1ENR1::LPTIM1EN)
    }
    pub(crate) fn set_lptim1_enable(&self, enable: bool) {
        if enable {
            self.registers().apb1enr1.modify(APB1ENR1::LPTIM1EN::SET);
        } else {
            self.registers().apb1enr1.modify(APB1ENR1::LPTIM1EN::CLEAR);
        }   
    }
    pub (crate) fn reset_lptim1(&self) {
        self.registers().apb1rstr1.modify(APB1RSTR1::LPTIM1RST::SET);
    }

    //LPTIM2
    pub(crate) fn is_enabled_lptim2_clock(&self) -> bool {
        self.registers().apb1enr2.is_set(APB1ENR2::LPTIM2EN)
    }
    pub(crate) fn set_lptim2_enable(&self, enable: bool) {
        if enable {
            self.registers().apb1enr2.modify(APB1ENR2::LPTIM2EN::SET);
        } else {
            self.registers().apb1enr2.modify(APB1ENR2::LPTIM2EN::CLEAR);
        }   
    }
    pub (crate) fn reset_lptim2(&self) {
        self.registers().apb1rstr2.modify(APB1RSTR2::LPTIM2RST::SET);
    }

    //LPTIM3
    pub(crate) fn is_enabled_lptim3_clock(&self) -> bool {
        self.registers().apb1enr2.is_set(APB1ENR2::LPTIM3EN)
    }
    pub(crate) fn set_lptim3_enable(&self, enable: bool) {
        if enable {
            self.registers().apb1enr2.modify(APB1ENR2::LPTIM3EN::SET);
        } else {
            self.registers().apb1enr2.modify(APB1ENR2::LPTIM3EN::CLEAR);
        }   
    }
    pub (crate) fn reset_lptim3(&self) {
        self.registers().apb1rstr2.modify(APB1RSTR2::LPTIM3RST::SET);
    }

}