//! Lamps, including the reverse lights

use mh8106f::System;

use crate::{CanBus, Part};

const COIL_PORT: u32 = 0x0080_070b; // P11DATA
const COIL: u8 = 0x10; // P11.4 output: the backlamp relay, on in R
const FB_PORT: u32 = 0x0080_070d; // P13DATA
const FB: u8 = 0x80; // P13.7 input: the relay's switched side

#[derive(Default)]
pub struct BacklampRelay;

impl Part for BacklampRelay {
    fn update(&mut self, chip: &mut System, _bus: &CanBus) {
        let on = chip.gpio_level(COIL_PORT) & COIL != 0;
        chip.set_gpio_input(FB_PORT, FB, if on { FB } else { 0 });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EV_ECU_FW;

    #[test]
    fn feedback_follows_the_coil() {
        let mut ecu = System::new(EV_ECU_FW);
        let bus = CanBus::default();
        BacklampRelay.update(&mut ecu, &bus);
        assert_eq!(ecu.gpio_level(FB_PORT) & FB, 0);
        ecu.set_gpio_input(COIL_PORT, COIL, COIL);
        BacklampRelay.update(&mut ecu, &bus);
        assert_eq!(ecu.gpio_level(FB_PORT) & FB, FB);
    }
}
