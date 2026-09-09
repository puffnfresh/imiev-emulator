//! The main contactor and the condenser precharge circuit.

use mh8106f::System;

use crate::adc::ev_ecu_adc;
use crate::{CanBus, Part};

const COIL_PORT: u32 = 0x0080_0701; // P1DATA - contactor coil-command outputs
const AUX_SENSE_PORT: u32 = 0x0080_0703; // P3DATA - contactor auxiliary-contact sense inputs
const PRECHARGE_FB_PORT: u32 = 0x0080_0709; // P9DATA b1 = precharge-complete
const CONTACTOR_FB_BIT: u8 = 0x02; // bit1 on each port

const MAIN_AUX_MASK: u8 = 0x0c; // P3 bits 2,3

#[derive(Default)]
pub struct Contactor;

fn main_aux_sense(coils: u8) -> u8 {
    (((coils >> 6) & 1) << 2) | (((coils >> 7) & 1) << 3)
}

impl Part for Contactor {
    fn update(&mut self, chip: &mut System, _bus: &CanBus) {
        let sense = main_aux_sense(chip.gpio_output(COIL_PORT));
        chip.set_gpio_input(AUX_SENSE_PORT, MAIN_AUX_MASK, sense);
    }
}

const AC_RELAY_CMD: u32 = 0x0080_e893; // firmware's A/C-compressor relay command variable
const AC_RELAY_FB_PORT: u32 = 0x0080_070d; // P13DATA b6 = A/C relay feedback
const AC_RELAY_FB_BIT: u8 = 0x40;

#[derive(Default)]
pub struct AcRelay;

impl Part for AcRelay {
    fn update(&mut self, chip: &mut System, _bus: &CanBus) {
        let commanded = chip.peek(AC_RELAY_CMD, 1) != 0;
        let fb = if commanded { AC_RELAY_FB_BIT } else { 0 };
        chip.set_gpio_input(AC_RELAY_FB_PORT, AC_RELAY_FB_BIT, fb);
    }
}

const HV_UP_PORT: u32 = 0x0080_0707; // EV-ECU P7DATA
const HV_UP_BIT: u8 = 0x10; // b4 = HV-start command (firmware output)

/// Charged reading: ~360 V pack
const CONDENSER_FULL_RAW: u16 = 0x0333;
// Per-step fraction (num/den of the remaining distance) toward the target: a gentle
// RC charge through the current-limit resistor, a faster passive discharge.
const CHARGE_NUM: u32 = 1;
const CHARGE_DEN: u32 = 8;
const DISCHARGE_NUM: u32 = 1;
const DISCHARGE_DEN: u32 = 4;

#[derive(Default)]
struct CondenserModel {
    raw: u16,
}

impl CondenserModel {
    fn step(&mut self, charging: bool) {
        let (target, num, den) = if charging {
            (CONDENSER_FULL_RAW, CHARGE_NUM, CHARGE_DEN)
        } else {
            (0, DISCHARGE_NUM, DISCHARGE_DEN)
        };
        let diff = target as i32 - self.raw as i32;
        if diff == 0 {
            return;
        }
        // At least one count of progress, so it fully settles rather than creeping.
        let mag = (diff.unsigned_abs() * num / den).max(1) as i32;
        let delta = if diff < 0 { -mag } else { mag };
        self.raw = (self.raw as i32 + delta).clamp(0, CONDENSER_FULL_RAW as i32) as u16;
    }
}

#[derive(Default)] // discharged at power-on (CondenserModel::default is raw = 0)
pub struct Condenser {
    model: CondenserModel,
}

const PRECHARGE_MASTER_STATE: u32 = 0x0080_e5ac; // 0 REST / 5 precharge-request / 2 precharge / 6 HV-active

const CONDENSER_VOLTAGE: u32 = 0x0080_cf60;
const PRECHARGE_DONE_V: f32 = 9.0;

impl Part for Condenser {
    fn update(&mut self, chip: &mut System, _bus: &CanBus) {
        let commanded = chip.gpio_level(HV_UP_PORT) & HV_UP_BIT != 0
            || chip.peek(PRECHARGE_MASTER_STATE, 1) != 0;
        self.model.step(commanded);
        chip.adc_mut().set_channel(ev_ecu_adc::CONDENSER, self.model.raw);
        let bus_up = f32::from_bits(chip.peek(CONDENSER_VOLTAGE, 4)) >= PRECHARGE_DONE_V;
        let fb = if commanded && bus_up { 0 } else { CONTACTOR_FB_BIT };
        chip.set_gpio_input(PRECHARGE_FB_PORT, CONTACTOR_FB_BIT, fb);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EV_ECU_FW;

    #[test]
    fn contactor_aux_sense_follows_coil_commands() {
        // No coils: no main aux-sense bits. Both main coils (P1.6/P1.7): both senses follow.
        assert_eq!(main_aux_sense(0x00), 0x00);
        assert_eq!(main_aux_sense(0xc0), MAIN_AUX_MASK);
        assert_eq!(main_aux_sense(0x40), 0x04); // CNT- only -> P3.2
        assert_eq!(main_aux_sense(0x80), 0x08); // CNT+ only -> P3.3
        // A fresh ECU commands no main coils, so the part asserts no aux-sense.
        let mut ecu = System::new(EV_ECU_FW);
        Contactor.update(&mut ecu, &CanBus::default());
        assert_eq!(ecu.gpio_level(AUX_SENSE_PORT) & MAIN_AUX_MASK, 0);
    }

    #[test]
    fn ac_relay_feedback_tracks_the_command() {
        let mut ecu = System::new(EV_ECU_FW);
        AcRelay.update(&mut ecu, &CanBus::default());
        // Command off at reset: feedback low.
        assert_eq!(ecu.gpio_level(AC_RELAY_FB_PORT) & AC_RELAY_FB_BIT, 0);
    }

    #[test]
    fn condenser_charges_and_discharges() {
        let mut c = CondenserModel::default();
        // Charging settles fully at the pack voltage.
        for _ in 0..200 {
            c.step(true);
        }
        assert_eq!(c.raw, CONDENSER_FULL_RAW);
        // Deasserting bleeds it all the way back to zero.
        for _ in 0..200 {
            c.step(false);
        }
        assert_eq!(c.raw, 0);
    }

    #[test]
    fn condenser_precharge_sequence() {
        let mut ecu = System::new(EV_ECU_FW);
        let bus = CanBus::default();
        let mut cond = Condenser::default();

        // At REST (no HV-start) the condenser sits discharged.
        cond.update(&mut ecu, &bus);
        assert_eq!(ecu.adc().channel(ev_ecu_adc::CONDENSER), 0);
        assert_eq!(ecu.gpio_level(PRECHARGE_FB_PORT) & CONTACTOR_FB_BIT, CONTACTOR_FB_BIT);

        // Command HV-start (P7.4): the cap ramps up to the pack voltage on the ADC.
        ecu.set_gpio_input(HV_UP_PORT, HV_UP_BIT, HV_UP_BIT);
        for _ in 0..200 {
            cond.update(&mut ecu, &bus);
        }
        assert_eq!(ecu.adc().channel(ev_ecu_adc::CONDENSER), CONDENSER_FULL_RAW);
        assert_eq!(ecu.gpio_level(PRECHARGE_FB_PORT) & CONTACTOR_FB_BIT, CONTACTOR_FB_BIT);

        // Release HV-start: the cap bleeds back down to zero.
        ecu.set_gpio_input(HV_UP_PORT, HV_UP_BIT, 0);
        for _ in 0..200 {
            cond.update(&mut ecu, &bus);
        }
        assert_eq!(ecu.adc().channel(ev_ecu_adc::CONDENSER), 0);
    }
}
