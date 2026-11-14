//! The EV-ECU's control power supply relay.

use mh8106f::System;

const HOLD_PORT: u32 = 0x0080_0701; // P1DATA
const HOLD: u8 = 0x02; // P1.1 output

pub fn holds_power(chip: &System) -> bool {
    chip.gpio_driven(HOLD_PORT) & HOLD != 0
}
