//! Brake booster's vacuum pump

use mh8106f::System;

use crate::{CanBus, Part};

const RELAY_PORT: u32 = 0x0080_070a; // P10DATA
const MAIN_RELAY_COIL: u8 = 0x80; // P10.7 output: the pump's main (fail-safe) relay
const CONTACT_PORT: u32 = 0x0080_0716; // P22DATA
const CONTACT_MONITOR: u8 = 0x08; // P22.3 input: the main relay's switched side
const COIL_PORT: u32 = 0x0080_070c; // P12DATA
const COIL_MONITOR: u8 = 0x01; // P12.0 input: the main relay coil's low side

#[derive(Default)]
pub struct VacuumPump;

impl Part for VacuumPump {
    fn update(&mut self, chip: &mut System, _bus: &CanBus) {
        let on = chip.gpio_level(RELAY_PORT) & MAIN_RELAY_COIL != 0;
        chip.set_gpio_input(CONTACT_PORT, CONTACT_MONITOR, if on { CONTACT_MONITOR } else { 0 });
        chip.set_gpio_input(COIL_PORT, COIL_MONITOR, if on { 0 } else { COIL_MONITOR });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EV_ECU_FW;

    #[test]
    fn monitors_follow_the_main_relay() {
        let mut ecu = System::new(EV_ECU_FW);
        let bus = CanBus::default();
        VacuumPump.update(&mut ecu, &bus);
        assert_eq!(ecu.gpio_level(CONTACT_PORT) & CONTACT_MONITOR, 0);
        assert_eq!(ecu.gpio_level(COIL_PORT) & COIL_MONITOR, COIL_MONITOR);
        ecu.set_gpio_input(RELAY_PORT, MAIN_RELAY_COIL, MAIN_RELAY_COIL); // the EV-ECU switches it on
        VacuumPump.update(&mut ecu, &bus);
        assert_eq!(ecu.gpio_level(CONTACT_PORT) & CONTACT_MONITOR, CONTACT_MONITOR);
        assert_eq!(ecu.gpio_level(COIL_PORT) & COIL_MONITOR, 0);
    }
}
