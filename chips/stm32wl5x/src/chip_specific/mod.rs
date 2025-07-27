//! This module contains all chip-specific code.
//!
//! This module provides all the
//! chip-specific types and traits to be used by others modules in this crate or by other crates.

pub mod chip_specs;
pub mod clock_constants;
pub mod flash;

pub use chip_specs::ChipSpecs;
