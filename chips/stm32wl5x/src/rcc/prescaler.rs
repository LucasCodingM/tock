use crate::rcc::registers::*;
use kernel::utilities::registers::interfaces::{ReadWriteable, Readable};

impl Rcc {
    // AHB prescaler
    pub(crate) fn set_ahb_prescaler(&self, ahb_prescaler: AHBPrescaler) {
        self.registers()
            .cfgr
            .modify(CFGR::HPRE.val(ahb_prescaler as u32));
    }

    pub(crate) fn get_ahb_prescaler(&self) -> AHBPrescaler {
        match self.registers().cfgr.read(CFGR::HPRE) {
            0b0000 => AHBPrescaler::Div1,
            0b0001 => AHBPrescaler::Div3,
            0b0010 => AHBPrescaler::Div5,
            0b0101 => AHBPrescaler::Div6,
            0b0110 => AHBPrescaler::Div10,
            0b0111 => AHBPrescaler::Div32,
            0b1000 => AHBPrescaler::Div2,
            0b1001 => AHBPrescaler::Div4,
            0b1010 => AHBPrescaler::Div8,
            0b1011 => AHBPrescaler::Div16,
            0b1100 => AHBPrescaler::Div64,
            0b1101 => AHBPrescaler::Div128,
            0b1110 => AHBPrescaler::Div256,
            0b1111 => AHBPrescaler::Div512,
            _ => panic!("Invalid AHB prescaler value"),
        }
    }

    // APB1 prescaler
     pub(crate) fn set_apb1_prescaler(&self, apb1_prescaler: APBPrescaler) {
        self.registers()
            .cfgr
            .modify(CFGR::PPRE1.val(apb1_prescaler as u32));
    }

    pub(crate) fn get_apb1_prescaler(&self) -> APBPrescaler {
        match self.registers().cfgr.read(CFGR::PPRE1) {
            0b000 => APBPrescaler::Div1, // No division
            0b100 => APBPrescaler::Div2,
            0b101 => APBPrescaler::Div4,
            0b110 => APBPrescaler::Div8,
            0b111 => APBPrescaler::Div16,
            _ => panic!("Invalid APB1 prescaler value"),
        }
    }

    // APB2 prescaler

    pub(crate) fn set_apb2_prescaler(&self, apb2_prescaler: APBPrescaler) {
        self.registers()
            .cfgr
            .modify(CFGR::PPRE2.val(apb2_prescaler as u32));
    }

    pub(crate) fn get_apb2_prescaler(&self) -> APBPrescaler {
        match self.registers().cfgr.read(CFGR::PPRE2) {
            0b000 => APBPrescaler::Div1, // No division
            0b100 => APBPrescaler::Div2,
            0b101 => APBPrescaler::Div4,
            0b110 => APBPrescaler::Div8,
            0b111 => APBPrescaler::Div16,
            _ => panic!("Invalid APB2 prescaler value"),
        }
    }

}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum AHBPrescaler {
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
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum APBPrescaler {
    Div1 = 0b000, // No division
    Div2 = 0b100,
    Div4 = 0b101,
    Div8 = 0b110,
    Div16 = 0b111,
}