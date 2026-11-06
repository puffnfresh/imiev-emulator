//! On-board charger (OBC)

use mh8106f::System;

use crate::{CanBus, Part};

const WAKE_PORT: u32 = 0x0080_0706; // P6DATA
const WAKE: u8 = 0x20; // P6.5 input, active low: the OBC asks the EV-ECU to power it
const RELAY_PORT: u32 = 0x0080_070b; // P11DATA
const RELAY_COIL: u8 = 0x08; // P11.3 output: the OBC power-supply relay
const CONTACT_PORT: u32 = 0x0080_0700; // P0DATA
const CONTACT_MONITOR: u8 = 0x40; // P0.6 input: the relay's switched side

#[derive(Default)]
pub struct Charger;

impl Part for Charger {
    fn update(&mut self, chip: &mut System, _bus: &CanBus) {
        chip.set_gpio_input(WAKE_PORT, WAKE, WAKE);
        let on = chip.gpio_level(RELAY_PORT) & RELAY_COIL != 0;
        chip.set_gpio_input(CONTACT_PORT, CONTACT_MONITOR, if on { CONTACT_MONITOR } else { 0 });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EV_ECU_FW;

    #[test]
    fn unplugged_and_the_contact_follows_the_relay() {
        let mut ecu = System::new(EV_ECU_FW);
        let bus = CanBus::default();
        Charger.update(&mut ecu, &bus);
        assert_eq!(ecu.gpio_level(WAKE_PORT) & WAKE, WAKE);
        assert_eq!(ecu.gpio_level(CONTACT_PORT) & CONTACT_MONITOR, 0);
        ecu.set_gpio_input(RELAY_PORT, RELAY_COIL, RELAY_COIL); // the EV-ECU switches it on
        Charger.update(&mut ecu, &bus);
        assert_eq!(ecu.gpio_level(CONTACT_PORT) & CONTACT_MONITOR, CONTACT_MONITOR);
    }
}
