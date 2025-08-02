//! High-level API for configuring the system clock (SYSCLK).
//! This abstracts the Rcc registers to safely switch between clocks,
//! configure PLL, and select SYSCLK source.

use crate::rcc::registers::Rcc;
use crate::clocks::{msi,hsi, hse, pll};
use crate::chip_specific::ChipSpecs as ChipSpecsTrait;
use kernel::utilities::registers::interfaces::{Readable, Writeable};
use kernel::utilities::cells::OptionalCell;

use crate::rcc::prescaler::AHBPrescaler;
use crate::rcc::system_clock::SysClockSource;

/// Main struct for configuring on-board clocks.
pub struct Clocks<'a/* , ChipSpecs */> {
    rcc: &'a Rcc,
    //flash: OptionalCell<&'a Flash<ChipSpecs>>,
    // High speed internal clock
    pub hsi: hsi::Hsi<'a>,
    // High speed external clock
    pub hse: hse::Hse<'a>,
    // Multi-Speed Internal clock
    pub msi: msi::Msi<'a>,
    // Main phase loop-lock clock
    //pub pll: pll::Pll<'a, ChipSpecs>,

}

impl<'a/* , ChipSpecs: ChipSpecsTrait */> Clocks<'a/* , ChipSpecs */> {
    // The constructor must be called when the default peripherals are created
    pub fn new(rcc: &'a Rcc) -> Self {
        Self {
            rcc,
            //flash: OptionalCell::empty(),
            hsi: hsi::Hsi::new(rcc),
            hse: hse::Hse::new(rcc),
            msi: msi::Msi::new(rcc),
            //pll: pll::Pll::new(rcc),
        }
    }

    /// Get the current configured AHB prescaler
    pub fn get_ahb_prescaler(&self) -> AHBPrescaler {
        self.rcc.get_ahb_prescaler()
    }

    /// Get the current system clock source
    pub fn get_sys_clock_source(&self) -> SysClockSource {
        self.rcc.get_sys_clock_source()
    }

    /// Get the current system clock frequency in MHz
    pub fn get_sys_clock_frequency_mhz(&self) -> usize {
        match self.get_sys_clock_source() {
            // These unwraps can't panic because set_sys_clock_frequency ensures that the source is
            // enabled. Also, Hsi and Pll structs ensure that the clocks can't be disabled when
            // they are configured as the system clock
            SysClockSource::HSI16 => self.hsi.get_frequency_mhz().unwrap(),
            SysClockSource::HSE32 => self.hse.get_frequency_mhz().unwrap(),
            //SysClockSource::PLLRCLK => self.pll.get_frequency_mhz().unwrap(),
            SysClockSource::MSI => self.msi.get_frequency_mhz().unwrap(),
            _ => {
                panic!("Unsupported system clock source");
            }   
        }
    }

    /// Get the frequency of the AHB
    pub fn get_ahb_frequency_mhz(&self) -> usize {
        let ahb_divider: usize = self.get_ahb_prescaler() as usize;
        self.get_sys_clock_frequency_mhz() / ahb_divider
    }
}

pub trait Stm32wl5xClocks {
    /// Get RCC instance
    fn get_rcc(&self) -> &Rcc;

    /// Get current AHB clock (HCLK) frequency in Hz
    fn get_ahb_frequency(&self) -> usize;
}

impl<'a/* , ChipSpecs: ChipSpecsTrait */> Stm32wl5xClocks for Clocks<'a/* , ChipSpecs */> {
    fn get_rcc(&self) -> &'a Rcc {
        self.rcc
    }

    fn get_ahb_frequency(&self) -> usize {
        self.get_ahb_frequency_mhz() * 1_000_000 // Convert mHz in Hz
    }
}