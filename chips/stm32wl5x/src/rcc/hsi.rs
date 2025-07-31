use crate::rcc::registers::*;
use kernel::utilities::registers::interfaces::{ReadWriteable, Readable};

impl Rcc {
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
}