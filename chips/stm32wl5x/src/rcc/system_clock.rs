use crate::rcc::registers::*;
use crate::rcc::pll::PllSource;
use kernel::utilities::registers::interfaces::{ReadWriteable, Readable};

impl Rcc {
//*******************************************************************//
    //                          System Clock Methods                     //
    //*******************************************************************//

    // NOTE: The flash latency also needs to be configured when changing the system clock frequency
    pub (crate) fn set_sys_clock_source(&self, sys_clock_source: SysClockSource) {
        match sys_clock_source {
            SysClockSource::MSI => {
                self.set_msi_enable(true);
                self.registers().cfgr.modify(CFGR::SW::MSI);                
            },
            SysClockSource::HSI16 => {
                self.set_hsi_enable(true);
                self.registers().cfgr.modify(CFGR::SW::HSI16);
            },
            SysClockSource::HSE32 => {
                self.set_hse_enable(true);
                self.registers().cfgr.modify(CFGR::SW::HSE32);
            },
            SysClockSource::PLLRCLK => {
                self.set_pll_enable(true);
                self.registers().cfgr.modify(CFGR::SW::PLLRCLK);
            },
        }
        
    }
    // Get the current system clock source
    pub(crate) fn get_sys_clock_source(&self) -> SysClockSource {
        match self.registers().cfgr.read(CFGR::SWS) {
            0b00 => SysClockSource::MSI,
            0b01 => SysClockSource::HSI16,
            0b10 => SysClockSource::HSE32,
            0b11 => SysClockSource::PLLRCLK,
            _ => panic!("Invalid system clock source"),
        }
    }

    pub(crate) fn is_msi_clock_system_clock(&self) -> bool {
        let system_clock_source = self.get_sys_clock_source();
        system_clock_source == SysClockSource::MSI
            || system_clock_source == SysClockSource::PLLRCLK
                && self.registers().pllcfgr.read(PLLCFGR::PLLSRC) == PllSource::MSI as u32
    }

    pub(crate) fn is_hsi_clock_system_clock(&self) -> bool {
        let system_clock_source = self.get_sys_clock_source();
        system_clock_source == SysClockSource::HSI16
            || system_clock_source == SysClockSource::PLLRCLK
                && self.registers().pllcfgr.read(PLLCFGR::PLLSRC) == PllSource::HSI16 as u32
    }

    pub(crate) fn is_hse_clock_system_clock(&self) -> bool {
        let system_clock_source = self.get_sys_clock_source();
        system_clock_source == SysClockSource::HSE32
            || system_clock_source == SysClockSource::PLLRCLK
                && self.registers().pllcfgr.read(PLLCFGR::PLLSRC) == PllSource::HSE32 as u32
    }
}

/// Clock sources for the CPU
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum SysClockSource {
    MSI = 0b00,
    HSI16 = 0b01,
    HSE32 = 0b10,
    PLLRCLK = 0b11,
}
