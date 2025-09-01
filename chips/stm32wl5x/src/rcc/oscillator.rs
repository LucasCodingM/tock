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

    pub (crate) fn is_hse_enabled(&self) -> bool {
        self.registers().cr.is_set(CR::HSEON)
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

    //*******************************************************************//
    //                      HSI Clock Methods                            //
    //******************************************************************//

    pub (crate) fn set_hsi_enable(&self, enable: bool) {
        if enable {
            self.registers().cr.modify(CR::HSION::SET);
        } else {
            self.registers().cr.modify(CR::HSION::CLEAR);
        }
    }

    pub (crate) fn is_hsi_enabled(&self) -> bool {
        self.registers().cr.is_set(CR::HSION)
    }

    pub (crate) fn is_hsi_is_ready(&self) -> bool {
        self.registers().cr.is_set(CR::HSIRDY)
    }

    // This bit is set and cleared by software to force HSI16 on even in Stop modes. HSI16 enabled
    // by HSIKERON can only feed USARTs, LPUARTs and I2Cs peripherals configured with
    // HSI16 as kernel clock. Keeping HSI16 on in Stop modes avoids slowing down the
    // communication speed because of the HSI16 startup time. This bit has no effect on HSION
    // value
    pub (crate) fn set_hsi_kernel_clock_enable(&self, enable: bool) {
        if enable {
            self.registers().cr.modify(CR::HSIKERON::SET);
        } else {
            self.registers().cr.modify(CR::HSIKERON::CLEAR);
        }
    }

    pub(crate) fn is_hsi_kernel_clock_enabled(&self) -> bool {
        self.registers().cr.is_set(CR::HSIKERON)
    }

    pub (crate) fn set_hsi_auto_start_from_stop_mode(&self, enable: bool) {
        if enable {
            self.registers().cr.modify(CR::HSIASFS::SET);
        } else {
            self.registers().cr.modify(CR::HSIASFS::CLEAR);
        }
    }

    //This bit is set and cleared by hardware to indicate that the HSI16 oscillator is stable or not,
    // when enabled by HSIKERON or a peripheral kernel clock request. This bit is not set when
    // HSI16 is enabled by software with HSION setting or by wake-up from Standby.
    // Note: Once HSIKERON is cleared, HSIKERDY goes low after six HSI16 clock cycles
    pub (crate) fn is_hsi_kernel_ready(&self) -> bool {
        self.registers().cr.is_set(CR::HSIKERDY)
    }

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

    //*******************************************************************//
    //                      LSI Clock Methods                            //
    //******************************************************************//

    pub (crate) fn is_lsi_clock_enabled(&self) -> bool {
        self.registers().csr.is_set(CSR::LSION)
    }

    pub (crate) fn is_lsi_clock_ready(&self) -> bool {
        self.registers().csr.is_set(CSR::LSIRDY)
    }

    pub (crate) fn set_lsi_enable(&self, enable: bool) {
        if enable {
            self.registers().csr.modify(CSR::LSION::SET);
        } else {
            self.registers().csr.modify(CSR::LSION::CLEAR);
        }
    }

    //*******************************************************************//
    //                      LSE Clock Methods                            //
    //******************************************************************//

    pub (crate) fn is_lse_clock_enabled(&self) -> bool {
        self.registers().bdcr.is_set(BDCR::LSEON)
    }

    pub (crate) fn is_lse_clock_ready(&self) -> bool {
        self.registers().bdcr.is_set(BDCR::LSERDY)
    }

    pub (crate) fn set_lse_enable(&self, enable: bool) {
        if enable {
            self.registers().bdcr.modify(BDCR::LSEON::SET);
        } else {
            self.registers().bdcr.modify(BDCR::LSEON::CLEAR);
        }
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

impl MsiRange {
    /// Retourne la fréquence en Hz
    pub fn frequency_hz(&self) -> u32 {
        match self {
            MsiRange::Range100kHz => 100_000,
            MsiRange::Range200kHz => 200_000,
            MsiRange::Range400kHz => 400_000,
            MsiRange::Range800kHz => 800_000,
            MsiRange::Range1MHz   => 1_000_000,
            MsiRange::Range2MHz   => 2_000_000,
            MsiRange::Range4MHz   => 4_000_000,
            MsiRange::Range8MHz   => 8_000_000,
            MsiRange::Range16MHz  => 16_000_000,
            MsiRange::Range24MHz  => 24_000_000,
            MsiRange::Range32MHz  => 32_000_000,
            MsiRange::Range48MHz  => 48_000_000,
        }
    }

    /// Retourne la fréquence en MHz (arrondie)
    pub fn frequency_mhz(&self) -> u32 {
        self.frequency_hz() / 1_000_000
    }
}

/// HSE Mode
#[derive(PartialEq)]
pub enum HseMode {
    BYPASS, // Bypass mode, external clock source 
    CRYSTAL, // External Crystal oscillator mode
}