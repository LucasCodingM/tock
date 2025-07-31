use crate::rcc::registers::*;
use kernel::utilities::registers::interfaces::{ReadWriteable, Readable};

impl Rcc {
    //*******************************************************************//
    //                      MSI Clock Methods                            //
    //******************************************************************//

    pub (crate) fn set_msi_enable(&self, enable: bool) {
        if enable {
            self.registers().cr.modify(CR::MSION::SET);
        } else {
            self.registers().cr.modify(CR::MSION::CLEAR);
        }
    }

    pub (crate) fn is_msi_is_ready(&self) -> bool {
        self.registers().cr.is_set(CR::MSIRDY)
    }

    pub (crate) fn is_msi_enabled(&self) -> bool {
        self.registers().cr.is_set(CR::MSION)
    }
    pub (crate) fn set_msi_frequency(&self, msir_range: MsiRange) {
        self.registers().cr.modify(CR::MSIRANGE.val(msir_range as u32));        
    }

    pub (crate) fn get_msi_frequency(&self) -> MsiRange {
        match self.registers().cr.read(CR::MSIRANGE) {
            0b0000 => MsiRange::Range100kHz,
            0b0001 => MsiRange::Range200kHz,
            0b0010 => MsiRange::Range400kHz,
            0b0011 => MsiRange::Range800kHz,
            0b0100 => MsiRange::Range1MHz,
            0b0101 => MsiRange::Range2MHz,
            0b0110 => MsiRange::Range4MHz, // reset value
            0b0111 => MsiRange::Range8MHz,
            0b1000 => MsiRange::Range16MHz,
            0b1001 => MsiRange::Range24MHz,
            0b1010 => MsiRange::Range32MHz,
            0b1011 => MsiRange::Range48MHz,
            _ => panic!("Invalid MSI frequency range"),
        }
    }

    // It must be enabled after LSE is enabled (LSEON = 1) and ready (LSERDY set by
    // hardware).There is a hardware protection to avoid enabling this bit if LSE is not ready.
    // This bit is cleared by hardware when LSE is disabled (LSEON = 0) or when the CSS on LSE
    // detects an LSE failure (refer to RCC_CSR register).
    pub (crate) fn set_msi_pll_enable(&self, enable: bool) {
        if enable {
            self.registers().cr.modify(CR::MSIPLLEN::SET);
        } else {
            self.registers().cr.modify(CR::MSIPLLEN::CLEAR);
        }
    }

    pub (crate) fn is_msi_pll_enabled(&self) -> bool {
        self.registers().cr.is_set(CR::MSIPLLEN)
    }

    pub (crate) fn set_msi_range_control_selection(&self) {
        self.registers().cr.modify(CR::MSIRGSEL::SET);
    }

    // This field can be modified only when MSI is off (MSION = 0) or when MSI is ready
    // (MSIRDY = 1). This filed must not be modified when MSI is on and when MSI is not
    // ready (MSION = 1 and MSIRDY = 0). 
    pub (crate) fn set_msi_clock_range(&self, range: MsiRange) {
        self.registers().cr.modify(CR::MSIRANGE.val(range as u32));
    }
}

pub enum MsiRange {
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
}