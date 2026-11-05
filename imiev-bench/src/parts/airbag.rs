//! Airbag signal

use mh8106f::System;

use crate::{CanBus, Part};

const LINE_PORT: u32 = 0x0080_070c; // P12DATA
const LINE_BIT: u8 = 0x02; // P12.1
const PUMPS_PER_BIT: u32 = 160;

const NO_CRASH: [u8; 12] = [1, 1, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0];

#[derive(Default)]
pub struct AirbagEcu {
    pumps: u32,
}

impl AirbagEcu {
    fn level(&self) -> bool {
        let bit = (self.pumps / PUMPS_PER_BIT) as usize % NO_CRASH.len();
        NO_CRASH[bit] != 0
    }
}

impl Part for AirbagEcu {
    fn update(&mut self, chip: &mut System, _bus: &CanBus) {
        chip.set_gpio_input(LINE_PORT, LINE_BIT, if self.level() { LINE_BIT } else { 0 });
        self.pumps = self.pumps.wrapping_add(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_is_a_start_bit_then_no_crash_then_idle() {
        let mut airbag = AirbagEcu::default();
        let mut bits = Vec::new();
        for _ in 0..NO_CRASH.len() {
            bits.push(airbag.level() as u8);
            airbag.pumps += PUMPS_PER_BIT;
        }
        assert_eq!(bits, NO_CRASH);
        let data = (1..=4).fold(0, |v, t| (v << 1) | (1 - NO_CRASH[t]));
        assert_eq!(data, 0b0101);
        let ones: u8 = (1..=5).map(|t| 1 - NO_CRASH[t]).sum();
        assert_eq!(ones % 2, 1, "odd parity");
    }
}
