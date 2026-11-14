//! On-board charger (OBC)

use mh8106f::System;

use crate::adc::ev_ecu_adc;
use crate::{CanBus, Part};

const WAKE_PORT: u32 = 0x0080_0706; // P6DATA
const WAKE: u8 = 0x20; // P6.5 input, active low: the OBC asks the EV-ECU to power it
const QUICK_PORT: u32 = 0x0080_0708; // P8DATA
const QUICK_POWER: u8 = 0x08; // P8.3 input, active low: the quick charger's power supply signal
const RELAY_PORT: u32 = 0x0080_070b; // P11DATA
const RELAY_COIL: u8 = 0x08; // P11.3 output: the OBC power-supply relay
const CONTACT_PORT: u32 = 0x0080_0700; // P0DATA
const CONTACT_MONITOR: u8 = 0x40; // P0.6 input: the relay's switched side

// The charging connector's sense (EV-ECU pin 103): 4.2-4.8 V with no
// connector, 1.2-2.0 V with one latched in, 2.4-3.2 V with its release
// button held.
const CONNECTOR_NONE_RAW: u16 = 0x380; // ~4.4 V
const CONNECTOR_LATCHED_RAW: u16 = 0x147; // ~1.6 V

#[derive(Default)]
pub struct Charger {
    pub plugged: bool,
}

impl Charger {
    pub fn waking(&self) -> bool {
        self.plugged
    }
}

impl Part for Charger {
    fn update(&mut self, chip: &mut System, _bus: &CanBus) {
        let connector = if self.plugged { CONNECTOR_LATCHED_RAW } else { CONNECTOR_NONE_RAW };
        chip.adc_mut().set_channel(ev_ecu_adc::CHARGE_PORT, connector);
        chip.set_gpio_input(WAKE_PORT, WAKE, if self.waking() { 0 } else { WAKE });
        chip.set_gpio_input(QUICK_PORT, QUICK_POWER, QUICK_POWER);
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
        let mut charger = Charger::default();
        charger.update(&mut ecu, &bus);
        assert_eq!(ecu.gpio_level(WAKE_PORT) & WAKE, WAKE);
        assert_eq!(ecu.adc().channel(ev_ecu_adc::CHARGE_PORT), CONNECTOR_NONE_RAW);
        assert_eq!(ecu.gpio_level(CONTACT_PORT) & CONTACT_MONITOR, 0);
        ecu.set_gpio_input(RELAY_PORT, RELAY_COIL, RELAY_COIL); // the EV-ECU switches it on
        charger.update(&mut ecu, &bus);
        assert_eq!(ecu.gpio_level(CONTACT_PORT) & CONTACT_MONITOR, CONTACT_MONITOR);
    }

    #[test]
    fn plugged_in_it_wakes_the_ev_ecu() {
        let mut ecu = System::new(EV_ECU_FW);
        let mut charger = Charger { plugged: true };
        charger.update(&mut ecu, &CanBus::default());
        assert_eq!(ecu.gpio_level(WAKE_PORT) & WAKE, 0);
        assert_eq!(ecu.adc().channel(ev_ecu_adc::CHARGE_PORT), CONNECTOR_LATCHED_RAW);
    }
}
