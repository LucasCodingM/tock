use crate::rcc::registers::*;
use kernel::utilities::registers::interfaces::{ReadWriteable};

impl Rcc {
    pub(crate) fn reset_sub_radio(&self) {
            self.registers().csr.modify(CSR::RFRST::SET);
            self.registers().csr.modify(CSR::RFRST::CLEAR);
    }
}