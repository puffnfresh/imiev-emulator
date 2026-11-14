//! On-board charger (OBC)

use mh8106f::System;

use crate::adc::ev_ecu_adc;
use crate::{frame, BusSource, CanBus, CanFrame, Part, BUS_PUMP_INTERVAL, CYCLES_PER_SECOND};

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

const PUMPS_PER_SECOND: u64 = CYCLES_PER_SECOND / BUS_PUMP_INTERVAL;
const STATUS_ID: u16 = 0x389;
const STATUS_PUMPS: u64 = PUMPS_PER_SECOND / 10;
const BOOT_PUMPS: u64 = PUMPS_PER_SECOND * 4; // first 0x389 ~4 s after its supply comes on
const READY_PUMPS: u64 = PUMPS_PER_SECOND * 5;
const COMMAND_ID: u16 = 0x286; // from the EV-ECU: b2 = DC current allowed, 0.1 A
const PACK_ID: u16 = 0x373; // from the BMU: b4:b5 = pack voltage, 0.1 V

const DC_VOLTS_PER_BIT: f32 = 2.01;
const MAINS_VOLTS: u8 = 244;
const MAX_DC_AMPS: f32 = 8.6; // what the car's charger gives from a 10 A allowance
const EFFICIENCY: f32 = 0.9;
const TEMPERATURE: u8 = 25 + 50;
const STATUS_STARTING: u8 = 0xc2;
const STATUS_READY: u8 = 0xca;
const STATUS_TAIL: u8 = 0x06;

#[derive(Default)]
pub struct Charger {
    pub plugged: bool,
    supplied: bool,
    pumps_on: u64,
}

impl Charger {
    pub fn waking(&self) -> bool {
        self.plugged
    }

    pub fn dc_amps(&self, bus: &CanBus) -> f32 {
        if !self.running() || self.pumps_on < READY_PUMPS {
            return 0.0;
        }
        let allowed = bus.last(COMMAND_ID).map_or(0.0, |f| f.data[2] as f32 / 10.0);
        allowed.min(MAX_DC_AMPS)
    }

    fn running(&self) -> bool {
        self.plugged && self.supplied
    }

    fn status(&self, bus: &CanBus) -> CanFrame {
        let pack_volts = bus.last(PACK_ID).map_or(0.0, |f| u16::from_be_bytes([f.data[4], f.data[5]]) as f32 / 10.0);
        let dc_amps = self.dc_amps(bus);
        let ac_amps = dc_amps * pack_volts / (MAINS_VOLTS as f32 * EFFICIENCY);
        let status = if self.pumps_on < READY_PUMPS { STATUS_STARTING } else { STATUS_READY };
        frame(
            STATUS_ID,
            &[
                (pack_volts / DC_VOLTS_PER_BIT) as u8,
                MAINS_VOLTS,
                (dc_amps * 10.0) as u8,
                TEMPERATURE,
                TEMPERATURE,
                status,
                (ac_amps * 10.0) as u8,
                STATUS_TAIL,
            ],
        )
    }
}

impl BusSource for Charger {
    fn frames(&mut self, bus: &CanBus) -> Vec<CanFrame> {
        if !self.running() {
            self.pumps_on = 0;
            return Vec::new();
        }
        self.pumps_on += 1;
        let booted = self.pumps_on >= BOOT_PUMPS;
        if booted && (self.pumps_on - BOOT_PUMPS).is_multiple_of(STATUS_PUMPS) {
            vec![self.status(bus)]
        } else {
            Vec::new()
        }
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
        self.supplied = on;
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
        let mut charger = Charger { plugged: true, ..Charger::default() };
        charger.update(&mut ecu, &CanBus::default());
        assert_eq!(ecu.gpio_level(WAKE_PORT) & WAKE, 0);
        assert_eq!(ecu.adc().channel(ev_ecu_adc::CHARGE_PORT), CONNECTOR_LATCHED_RAW);
    }

    #[test]
    fn reports_once_booted_and_charges_as_allowed() {
        let mut ecu = System::new(EV_ECU_FW);
        let mut bus = CanBus::default();
        bus.record(frame(PACK_ID, &[0, 0, 0, 0, 0x0d, 0xd1, 0, 0])); // 353.7 V
        let mut charger = Charger { plugged: true, ..Charger::default() };
        ecu.set_gpio_input(RELAY_PORT, RELAY_COIL, RELAY_COIL);
        charger.update(&mut ecu, &bus);
        let sent: Vec<CanFrame> = (0..READY_PUMPS + STATUS_PUMPS).flat_map(|_| charger.frames(&bus)).collect();
        assert_eq!(sent.len() as u64, (READY_PUMPS + STATUS_PUMPS - BOOT_PUMPS) / STATUS_PUMPS + 1, "every 100 ms once booted");
        assert_eq!(sent[0].data[0], (353.7 / DC_VOLTS_PER_BIT) as u8);
        assert_eq!(sent[0].data[5], STATUS_STARTING);
        assert_eq!(charger.dc_amps(&bus), 0.0, "nothing allowed on 0x286 yet");
        bus.record(frame(COMMAND_ID, &[0x0e, 0x74, 100, 0x35, 0x20, 0, 0, 0])); // 10 A allowed
        assert_eq!(charger.dc_amps(&bus), MAX_DC_AMPS);
        assert_eq!(charger.status(&bus).data[5], STATUS_READY);
    }
}
