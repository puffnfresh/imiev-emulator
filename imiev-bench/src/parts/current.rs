//! Traction battery current sensor.

use mh8106f::System;

use crate::adc::bmu_adc;
use crate::{CanBus, Part, HEATER_ID};

const CHARGER_ID: u16 = 0x389; // b2: DC amps into the pack, 0.1 A
const CHARGER_AMPS_PER_BIT: f32 = 0.1;
const HEATER_AMPS_PER_BIT: f32 = 0.1; // 0x384 b4
const AC_AMPS_PER_BIT: f32 = 0.001; // 0x384 b0:b1

const ADC_COUNTS_PER_VOLT: f32 = 1024.0 / 5.0;
const LOW_AMPS_PER_VOLT: f32 = 15.0; // 30 A over 2 V
const HIGH_AMPS_PER_VOLT: f32 = 175.0; // 350 A over 2 V
const ZERO_COUNTS: f32 = 512.0; // 2.5 V
const ADC_MAX: f32 = 1023.0;

#[derive(Default)]
pub struct CurrentSensor;

fn pack_amps(bus: &CanBus) -> f32 {
    let charging = bus.last(CHARGER_ID).map_or(0.0, |f| f.data[2] as f32 * CHARGER_AMPS_PER_BIT);
    let climate = bus.last(HEATER_ID).map_or(0.0, |f| {
        f.data[4] as f32 * HEATER_AMPS_PER_BIT + u16::from_be_bytes([f.data[0], f.data[1]]) as f32 * AC_AMPS_PER_BIT
    });
    charging - climate
}

fn counts(amps: f32, amps_per_volt: f32) -> u16 {
    (ZERO_COUNTS + amps / amps_per_volt * ADC_COUNTS_PER_VOLT).round().clamp(0.0, ADC_MAX) as u16
}

impl Part for CurrentSensor {
    fn update(&mut self, bmu: &mut System, bus: &CanBus) {
        let amps = pack_amps(bus);
        bmu.adc_mut().set_channel(bmu_adc::PACK_CURRENT_LO, counts(amps, LOW_AMPS_PER_VOLT));
        bmu.adc_mut().set_channel(bmu_adc::PACK_CURRENT_HI, counts(amps, HIGH_AMPS_PER_VOLT));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frame;

    #[test]
    fn charging_in_heating_out() {
        let mut bus = CanBus::default();
        assert_eq!(pack_amps(&bus), 0.0);
        bus.record(frame(CHARGER_ID, &[0xa1, 0xf4, 86, 0x4b, 0x4b, 0xca, 0x7f, 0x06]));
        assert!((pack_amps(&bus) - 8.6).abs() < 1e-4);
        bus.record(frame(HEATER_ID, &[0, 0, 0, 0x1f, 150, 0, 0, 0]));
        assert!((pack_amps(&bus) + 6.4).abs() < 1e-4);
    }

    #[test]
    fn spans_its_ranges() {
        assert_eq!(counts(0.0, LOW_AMPS_PER_VOLT), 0x200);
        assert_eq!(counts(2.34, LOW_AMPS_PER_VOLT), 0x220);
        assert_eq!(counts(30.0, LOW_AMPS_PER_VOLT), 0x200 + 410);
        assert_eq!(counts(-350.0, HIGH_AMPS_PER_VOLT), 0x200 - 410);
    }
}
