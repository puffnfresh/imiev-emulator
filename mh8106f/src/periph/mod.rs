//! Each device claims a set of addresses in the SFR block.
//!
//! Devices advance their ticks and may raise an interrupt request.

pub mod adc;
pub mod icu;
pub mod timer;

pub use adc::Adc;
pub use icu::Icu;
pub use timer::Timer;

/// An MMIO device keyed by SFR address.
pub(crate) trait Peripheral {
    /// Does this device answer for `addr`? Must be disjoint across devices.
    fn handles(&self, addr: u32) -> bool;

    /// Read a `size`-byte (1/2/4) register at `addr`. Big-endian value.
    fn read(&mut self, addr: u32, size: u32) -> u32;

    /// Write a `size`-byte (1/2/4) register at `addr`.
    fn write(&mut self, addr: u32, size: u32, val: u32);

    /// Advance the device by `cycles` CPU steps. Default: no time behaviour.
    fn tick(&mut self, _cycles: u64) {}
}

#[inline]
pub(crate) fn width_mask(size: u32) -> u32 {
    if size >= 4 {
        u32::MAX
    } else {
        (1u32 << (size * 8)) - 1
    }
}
