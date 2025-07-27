use crate::chip_specific::clock_constants::ClockConstants;
use crate::chip_specific::flash::FlashChipSpecific;

pub trait ChipSpecs: ClockConstants + FlashChipSpecific {}

impl<T: ClockConstants + FlashChipSpecific> ChipSpecs for T {}
