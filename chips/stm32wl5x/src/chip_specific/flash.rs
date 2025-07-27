//! Chip-specific flash code

use core::fmt::Debug;

pub trait FlashChipSpecific {
    type FlashLatency: RegisterToFlashLatency + Clone + Copy + PartialEq + Debug + Into<u32>;

    // The number of wait cycles depends on two factors: system clock frequency and the supply
    // voltage. Currently, this method assumes 2.7-3.6V voltage supply (default value).
    // TODO: Take into account the power supply
    //
    // The number of wait cycles varies from chip to chip
    fn get_number_wait_cycles_based_on_frequency(frequency_mhz: usize) -> Self::FlashLatency;
}

pub trait RegisterToFlashLatency {
    fn convert_register_to_enum(flash_latency_register: u32) -> Self;
}

// Refer to the table ‘Number of wait states according
// to flash clock (HCLK3) frequency’ in the reference manual (RM0453).
#[derive(Copy, Clone, PartialEq, Debug)]
pub enum FlashLatency3 {
    Latency0,
    Latency1,
    Latency2,
}

impl RegisterToFlashLatency for FlashLatency3 {
    fn convert_register_to_enum(flash_latency_register: u32) -> Self {
        match flash_latency_register {
            0 => Self::Latency0,
            1 => Self::Latency1,
            _ => Self::Latency2,
        }
    }
}

impl From<FlashLatency3> for u32 {
    fn from(val: FlashLatency3) -> Self {
        val as u32
    }
}
