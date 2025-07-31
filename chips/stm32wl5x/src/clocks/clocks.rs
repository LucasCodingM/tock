//! High-level API for configuring the system clock (SYSCLK).
//! This abstracts the Rcc registers to safely switch between clocks,
//! configure PLL, and select SYSCLK source.

use crate::rcc::registers::Rcc;
use crate::chip_specific::ChipSpecs as ChipSpecsTrait;
use kernel::utilities::registers::interfaces::{Readable, Writeable};

/// Main struct for configuring on-board clocks.
pub struct Clocks<'a, ChipSpecs> {
    rcc: &'a Rcc,
    //flash: OptionalCell<&'a Flash<ChipSpecs>>,
    /// High speed internal clock
    //pub hsi: Hsi<'a>,
    /// High speed external clock
    //pub hse: Hse<'a>,
    /// Main phase loop-lock clock
    //pub pll: Pll<'a, ChipSpecs>,
}

impl<'a, ChipSpecs: ChipSpecsTrait> Clocks<'a, ChipSpecs> {
    // The constructor must be called when the default peripherals are created
    pub fn new(rcc: &'a Rcc) -> Self {
        Self {
            rcc,
/*             flash: OptionalCell::empty(),
            hsi: Hsi::new(rcc),
            hse: Hse::new(rcc),
            pll: Pll::new(rcc), */
        }
    }
}

pub trait Stm32wl5xClocks {
    /// Get RCC instance
    fn get_rcc(&self) -> &Rcc;

    /// Get current AHB clock (HCLK) frequency in Hz
    fn get_ahb_frequency(&self) -> usize;
}

impl<'a, ChipSpecs: ChipSpecsTrait> Stm32wl5xClocks for Clocks<'a, ChipSpecs> {
    fn get_rcc(&self) -> &'a Rcc {
        self.rcc
    }

    fn get_ahb_frequency(&self) -> usize {
        self.get_ahb_frequency_mhz() * 1_000_000 // Convert mHz in Hz
    }
}