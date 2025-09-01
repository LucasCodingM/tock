/// PLL-related constants for specific for a specific chip
pub trait PllConstants {
    /// PLL minimum frequency in MHz
    const MIN_FREQ_MHZ: usize = 12;
    /// PLL maximum frequency in MHz
    // All boards support PLL frequencies up to 48MHz
    const MAX_FREQ_MHZ: usize = 48;

    const PLL_VCO_IN_MIN_MHZ: f32 = 2.66;
    const PLL_VCO_IN_MAX_MHZ: f32 = 16.0;

    const PLL_M_RANGE: core::ops::RangeInclusive<u8> = 1..=8;
    const PLL_N_RANGE: core::ops::RangeInclusive<u16> = 6..=127;

    const PLL_P_ALLOWED: core::ops::RangeInclusive<u8> = 2..=8;
    const PLL_Q_ALLOWED: core::ops::RangeInclusive<u8> = 2..=8;
    const PLL_R_ALLOWED: core::ops::RangeInclusive<u8> = 2..=32;

    const PLL_P_MAX_MHZ: f32  = 62.0;
    const PLL_QR_MAX_MHZ: f32 = 48.0;

}

/// Generic clock constants for a specific chip
pub trait SystemClockConstants {
    /// Maximum allowed APB1 frequency in MHz
    const APB1_FREQUENCY_LIMIT_MHZ: usize;
    /// Maximum allowed APB2 frequency in MHz
    const APB2_FREQUENCY_LIMIT_MHZ: usize;
    /// Bus radio sub-GHz
    /// Maximum allowed APB3 frequency in MHz
    const APB3_FREQUENCY_LIMIT_MHZ: usize;
    /// Maximum allowed system clock frequency in MHz
    const SYS_CLOCK_FREQUENCY_LIMIT_MHZ: usize;
}

/// Clock constants for a specific chip
pub trait ClockConstants: SystemClockConstants + PllConstants {}

impl<T: SystemClockConstants + PllConstants> ClockConstants for T {}
