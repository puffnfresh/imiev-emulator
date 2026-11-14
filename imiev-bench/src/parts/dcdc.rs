//! DC-DC converter that charges 12V battery from the traction battery.

use mh8106f::System;

use crate::adc::ev_ecu_adc;
use crate::{dclink_connected, CanBus, Part};

const DRIVE_PORT: u32 = 0x0080_0701; // P1DATA
const DRIVE: u8 = 0x08; // P1.3 output: run the DC-DC converter
const FEEDBACK_PORT: u32 = 0x0080_0702; // P2DATA
const RUNNING: u8 = 0x80; // P2.7 input, active low: the DC-DC converter is running

const BATTERY_RAW: u16 = 0x2b5; // 12.7 V
const CHARGING_RAW: u16 = 0x30c; // 14.3 V

const RISE_NUM: u32 = 1;
const RISE_DEN: u32 = 8;
const FALL_NUM: u32 = 1;
const FALL_DEN: u32 = 4;

struct SystemVoltage {
    raw: u16,
}

impl Default for SystemVoltage {
    fn default() -> Self {
        SystemVoltage { raw: BATTERY_RAW }
    }
}

impl SystemVoltage {
    fn step(&mut self, charging: bool) {
        let (target, num, den) = if charging { (CHARGING_RAW, RISE_NUM, RISE_DEN) } else { (BATTERY_RAW, FALL_NUM, FALL_DEN) };
        let diff = target as i32 - self.raw as i32;
        if diff != 0 {
            let mag = (diff.unsigned_abs() * num / den).max(1) as i32;
            self.raw = (self.raw as i32 + diff.signum() * mag) as u16;
        }
    }
}

#[derive(Default)]
pub struct DcDcConverter {
    system: SystemVoltage,
}

impl Part for DcDcConverter {
    fn update(&mut self, chip: &mut System, _bus: &CanBus) {
        let running = chip.gpio_level(DRIVE_PORT) & DRIVE != 0 && dclink_connected(chip);
        self.system.step(running);
        chip.adc_mut().set_channel(ev_ecu_adc::SYSTEM_VOLTAGE, self.system.raw);
        chip.set_gpio_input(FEEDBACK_PORT, RUNNING, if running { 0 } else { RUNNING });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EV_ECU_FW;

    const CONTACTORS: (u32, u8) = (0x0080_0701, 0xc0); // P1.6 CNT-, P1.7 CNT+

    #[test]
    fn charges_the_12v_system_only_when_driven_with_the_contactors_closed() {
        let mut ecu = System::new(EV_ECU_FW);
        let bus = CanBus::default();
        let mut dcdc = DcDcConverter::default();
        let settle = |dcdc: &mut DcDcConverter, ecu: &mut System| {
            for _ in 0..200 {
                dcdc.update(ecu, &bus);
            }
            (ecu.adc().channel(ev_ecu_adc::SYSTEM_VOLTAGE), ecu.gpio_level(FEEDBACK_PORT) & RUNNING)
        };
        assert_eq!(settle(&mut dcdc, &mut ecu), (BATTERY_RAW, RUNNING));
        ecu.set_gpio_input(DRIVE_PORT, DRIVE, DRIVE);
        assert_eq!(settle(&mut dcdc, &mut ecu), (BATTERY_RAW, RUNNING), "no traction battery to run from");
        ecu.set_gpio_input(CONTACTORS.0, CONTACTORS.1, CONTACTORS.1);
        assert_eq!(settle(&mut dcdc, &mut ecu), (CHARGING_RAW, 0));
        ecu.set_gpio_input(DRIVE_PORT, DRIVE, 0);
        assert_eq!(settle(&mut dcdc, &mut ecu), (BATTERY_RAW, RUNNING));
    }
}
