//! Driver inputs: the shift-lever switch matrix, ignition/key state, and the
//! accelerator-pedal position sensors, all presented to the EV-ECU as GPIO/ADC.

use mh8106f::System;

use crate::adc::ev_ecu_adc;
use crate::{CanBus, Part};

const SHIFT_MAIN_PORT: u32 = 0x0080_0704; // P4DATA - shift switch matrix (main channel)
const SHIFT_SUB_PORT: u32 = 0x0080_0702; // P2DATA - shift switch matrix (sub channel)
const SHIFT_MATRIX_MASK: u8 = 0x3f; // six position switches, active-low
const IGNITION_PORT: u32 = 0x0080_0709; // P9DATA
const IGNITION_ON_BITS: u8 = 0x60; // IG1 + ST (start) asserted
const APS_TEST_PORT: u32 = 0x0080_0701; // P1DATA
const APS_TEST_BIT: u8 = 0x01; // P1.0 output: a self-test pulse for accelerator sensor 2

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Gear {
    Park,
    Reverse,
    Neutral,
    Drive,
    Eco,
    Comfort,
}

impl Gear {
    fn matrix(self) -> u8 {
        let bit = match self {
            Gear::Park => 5,
            Gear::Reverse => 4,
            Gear::Neutral => 3,
            Gear::Drive => 2,
            Gear::Eco => 1,
            Gear::Comfort => 0,
        };
        SHIFT_MATRIX_MASK & !(1 << bit)
    }

    pub fn is_forward_drive(self) -> bool {
        matches!(self, Gear::Drive | Gear::Eco | Gear::Comfort)
    }
}

const APS_RELEASED_RAW: u16 = 0x0c0; // released pedal, APS1 main (~0.9 V)
const APS_FULL_RAW: u16 = 0x320;
const BPS_RELEASED_RAW: u16 = 0x130; // brake-stroke sensor, pedal up (~1.5 V)
const BPS_FULL_RAW: u16 = 0x600; // brake-stroke sensor, pedal fully pressed

pub struct DriverControls {
    pub gear: Gear,
    pub key_on: bool,
    pub pedal_pct: f32, // accelerator 0..100 %
    pub brake_pct: f32, // brake 0..100 %
}

impl Default for DriverControls {
    fn default() -> Self {
        DriverControls { gear: Gear::Park, key_on: true, pedal_pct: 0.0, brake_pct: 0.0 }
    }
}

fn pedal_raw(pct: f32, released: u16, full: u16) -> u16 {
    let span = (full - released) as f32;
    released + (pct.clamp(0.0, 100.0) / 100.0 * span) as u16
}

impl Part for DriverControls {
    fn update(&mut self, chip: &mut System, _bus: &CanBus) {
        let m = self.gear.matrix();
        chip.set_gpio_input(SHIFT_MAIN_PORT, SHIFT_MATRIX_MASK, m);
        chip.set_gpio_input(SHIFT_SUB_PORT, SHIFT_MATRIX_MASK, m);
        let key = |bits| if self.key_on { bits } else { 0 };
        chip.set_gpio_input(IGNITION_PORT, IGNITION_ON_BITS, key(IGNITION_ON_BITS));
        // Accelerator: both redundant APS channels (main ch2, sub ch5 at half) from pedal %.
        let main = pedal_raw(self.pedal_pct, APS_RELEASED_RAW, APS_FULL_RAW);
        chip.adc_mut().set_channel(ev_ecu_adc::ACCEL_1_SIGNAL, main);
        let test_pulse = chip.gpio_output(APS_TEST_PORT) & APS_TEST_BIT != 0;
        let sub = if test_pulse { 0 } else { main / 2 };
        chip.adc_mut().set_channel(ev_ecu_adc::ACCEL_2_SIGNAL, sub);
        let brake = pedal_raw(self.brake_pct, BPS_RELEASED_RAW, BPS_FULL_RAW);
        chip.adc_mut().set_channel(ev_ecu_adc::BRAKE_SIGNAL, brake);
        chip.adc_mut().set_channel(ev_ecu_adc::BRAKE_SIGNAL_CZERO, brake);
    }
}
