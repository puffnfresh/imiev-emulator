//! The main contactor and the A/C relay.

use mh8106f::System;

use crate::{CanBus, Part};

const COIL_PORT: u32 = 0x0080_0701; // P1DATA - relay coil outputs: P1.5 precharge, P1.6 CNT-, P1.7 CNT+
const AUX_SENSE_PORT: u32 = 0x0080_0703; // P3DATA - auxiliary-contact sense inputs

const AUX_MASK: u8 = 0x0e; // P3 bits 1..3
const PRECHARGE_COIL: u8 = 0x20; // P1.5
const CNT_MINUS_COIL: u8 = 0x40; // P1.6
const CNT_PLUS_COIL: u8 = 0x80; // P1.7

#[derive(Default)]
pub struct Contactor;

fn aux_sense(coils: u8) -> u8 {
    ((coils >> 4) & 0x0c) | (!(coils >> 4) & 0x02)
}

pub fn dclink_connected(chip: &System) -> bool {
    let coils = chip.gpio_level(COIL_PORT);
    coils & CNT_MINUS_COIL != 0 && coils & (CNT_PLUS_COIL | PRECHARGE_COIL) != 0
}

impl Part for Contactor {
    fn update(&mut self, chip: &mut System, _bus: &CanBus) {
        let sense = aux_sense(chip.gpio_output(COIL_PORT));
        chip.set_gpio_input(AUX_SENSE_PORT, AUX_MASK, sense);
    }
}

const AC_RELAY_COIL_PORT: u32 = 0x0080_070b; // P11DATA b5 = A/C-compressor relay coil
const AC_RELAY_COIL_BIT: u8 = 0x20;
const AC_RELAY_FB_PORT: u32 = 0x0080_070d; // P13DATA b6 = A/C relay feedback
const AC_RELAY_FB_BIT: u8 = 0x40;

#[derive(Default)]
pub struct AcRelay;

impl Part for AcRelay {
    fn update(&mut self, chip: &mut System, _bus: &CanBus) {
        let commanded = chip.gpio_level(AC_RELAY_COIL_PORT) & AC_RELAY_COIL_BIT != 0;
        let fb = if commanded { AC_RELAY_FB_BIT } else { 0 };
        chip.set_gpio_input(AC_RELAY_FB_PORT, AC_RELAY_FB_BIT, fb);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::EV_ECU_FW;

    #[test]
    fn contactor_aux_sense_follows_coil_commands() {
        assert_eq!(aux_sense(0x00), 0x02);
        assert_eq!(aux_sense(0xe0), 0x0c);
        assert_eq!(aux_sense(0x20), 0x00);
        assert_eq!(aux_sense(0x40), 0x06);
        assert_eq!(aux_sense(0x80), 0x0a);
        assert_eq!(aux_sense(0x1f), 0x02);
        let mut ecu = System::new(EV_ECU_FW);
        Contactor.update(&mut ecu, &CanBus::default());
        assert_eq!(ecu.gpio_level(AUX_SENSE_PORT) & AUX_MASK, 0x02);
    }

    #[test]
    fn ac_relay_feedback_tracks_the_command() {
        let mut ecu = System::new(EV_ECU_FW);
        AcRelay.update(&mut ecu, &CanBus::default());
        assert_eq!(ecu.gpio_level(AC_RELAY_FB_PORT) & AC_RELAY_FB_BIT, 0);
        ecu.set_gpio_input(AC_RELAY_COIL_PORT, AC_RELAY_COIL_BIT, AC_RELAY_COIL_BIT);
        AcRelay.update(&mut ecu, &CanBus::default());
        assert_eq!(ecu.gpio_level(AC_RELAY_FB_PORT) & AC_RELAY_FB_BIT, AC_RELAY_FB_BIT);
    }
}
