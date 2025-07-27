/// PLL-related constants for specific for a specific chip
pub trait PllConstants {
    /// PLL minimum frequency in MHz
    const MIN_FREQ_MHZ: usize;
    /// PLL maximum frequency in MHz
    // All boards support PLL frequencies up to 48MHz
    const MAX_FREQ_MHZ: usize = 48;
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
