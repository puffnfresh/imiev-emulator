//! Traction battery cooling fan.

use mh8106f::System;

use crate::adc::bmu_adc;
use crate::{CanBus, Part, VEHICLE_DT};

const PWM_DRIVE_PHASE: u32 = 0x0080_0ba6;
const PWM_REST_PHASE: u32 = 0x0080_0ba4; // TOU12RLW

const FULL_RPM: f32 = 3000.0; // d = 1; a guess, comfortably over the 1645 r/min P1AC1 wants
const SPIN_TAU_S: f32 = 1.0; // spin-up and coast-down time constant
const SENSE_ZERO_V: f32 = 0.1;
const SENSE_FULL_V: f32 = 4.8;
const SENSE_FULL_RPM: f32 = 3948.0;
const SENSE_DIVIDER: f32 = 2.5;
const ADC_FULL_V: f32 = 5.0;
const ADC_COUNTS: f32 = 1024.0;

pub struct BatteryFan {
    pub connected: bool,
    rpm: f32,
}

impl Default for BatteryFan {
    fn default() -> Self {
        BatteryFan { connected: true, rpm: 0.0 }
    }
}

fn drive(on_phase: u16, rest_phase: u16) -> f32 {
    let counts = |phase: u16| (phase as i16 + 1) as f32;
    let period = counts(on_phase) + counts(rest_phase);
    if period > 0.0 { counts(on_phase) / period } else { 0.0 }
}

fn sense_raw(rpm: f32) -> u16 {
    let volts = SENSE_ZERO_V + rpm / SENSE_FULL_RPM * (SENSE_FULL_V - SENSE_ZERO_V);
    (volts / SENSE_DIVIDER / ADC_FULL_V * ADC_COUNTS) as u16
}

impl BatteryFan {
    fn spin(&mut self, drive: f32) -> u16 {
        let target = if self.connected { drive * FULL_RPM } else { 0.0 };
        self.rpm += (target - self.rpm) * (VEHICLE_DT / SPIN_TAU_S).min(1.0);
        sense_raw(self.rpm)
    }
}

impl Part for BatteryFan {
    fn update(&mut self, bmu: &mut System, _bus: &CanBus) {
        let asked = drive(bmu.peek(PWM_DRIVE_PHASE, 2) as u16, bmu.peek(PWM_REST_PHASE, 2) as u16);
        let raw = self.spin(asked);
        bmu.adc_mut().set_channel(bmu_adc::COOLING_FAN_MONITOR, raw);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const DRIVEN: (u16, u16) = (89, 9); // as pwm_tou1_duty_set leaves it for d = 0.9
    const STOPPED: (u16, u16) = (0xffff, 0x62);

    fn settle(fan: &mut BatteryFan, (on, rest): (u16, u16)) -> f32 {
        let mut raw = 0;
        for _ in 0..(5.0 * SPIN_TAU_S / VEHICLE_DT) as u32 {
            raw = fan.spin(drive(on, rest));
        }
        raw as f32 / ADC_COUNTS * ADC_FULL_V * SENSE_DIVIDER
    }

    #[test]
    fn reads_the_bmus_pwm() {
        assert!((drive(DRIVEN.0, DRIVEN.1) - 0.9).abs() < 1e-6);
        assert_eq!(drive(STOPPED.0, STOPPED.1), 0.0);
        assert_eq!(drive(0x62, 0xffff), 1.0);
    }

    #[test]
    fn spins_as_the_bmu_asks() {
        let mut fan = BatteryFan::default();
        let driven = settle(&mut fan, DRIVEN);
        assert!(driven > 2.0, "{driven} V is P1AC1's drive fail");
        assert!(driven < 5.0, "{driven} V is P1AC2's short to battery");
        assert!(settle(&mut fan, STOPPED) < 0.7, "still turning: P1AC3's rest fail");
    }

    #[test]
    fn unplugged_it_does_not_turn() {
        let mut fan = BatteryFan { connected: false, ..Default::default() };
        assert!(settle(&mut fan, DRIVEN) < 0.7);
    }
}
