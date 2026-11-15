//! High-voltage water heater.

use mh8106f::System;

use crate::{dclink_connected, frame, BusSource, CanBus, CanFrame, ClimateMode, REMOTE_CLIMATE_ID, VEHICLE_DT};

const RELAY_PORT: u32 = 0x0080_070b; // EV-ECU P11DATA
const RELAY: u8 = 0x20; // P11.5: the A/C relay

pub const HEATER_ID: u16 = 0x384;
const REPORT_PUMPS: u64 = (0.1 / VEHICLE_DT) as u64; // every 100 ms
const RUN: u8 = 0x01;

const AMPS: f32 = 15.0; // ~5 kW from the traction battery
const AMPS_PER_BIT: f32 = 0.1;
const TWELVE_VOLT_CURRENT: u8 = 0x1f;
const TEMP_PER_BIT: f32 = 0.6;
const TEMP_OFFSET: f32 = 40.0;

const AMBIENT_C: f32 = 10.0;
const WATER_ON_C: f32 = 60.0;
const WATER_OFF_C: f32 = 70.0;
const HEAT_CAPACITY_J_PER_K: f32 = 12_000.0; // ~3 kg of coolant
const LOSS_W_PER_K: f32 = 60.0; // through the heater core to the cabin
const HEATER_WATTS: f32 = 5000.0;
const RETURN_DROP_C: f32 = 5.0;

pub struct Heater {
    water_c: f32,
    heating: bool,
    pumps: u64,
}

impl Default for Heater {
    fn default() -> Self {
        Heater { water_c: AMBIENT_C, heating: false, pumps: 0 }
    }
}

fn requested(bus: &CanBus) -> Option<ClimateMode> {
    let b0 = bus.last(REMOTE_CLIMATE_ID)?.data[0];
    if b0 & RUN == 0 {
        return None;
    }
    match b0 >> 1 & 3 {
        1 => Some(ClimateMode::Cool),
        2 => Some(ClimateMode::Heat),
        3 => Some(ClimateMode::Defrost),
        _ => None,
    }
}

fn temp_byte(c: f32) -> u8 {
    ((c + TEMP_OFFSET) / TEMP_PER_BIT).round().clamp(0.0, 255.0) as u8
}

impl Heater {
    pub fn heating(&self) -> bool {
        self.heating
    }

    pub fn water_c(&self) -> f32 {
        self.water_c
    }

    pub fn amps(&self) -> f32 {
        if self.heating { AMPS } else { 0.0 }
    }

    pub fn update(&mut self, ev_ecu: &System, ev_ecu_powered: bool, bus: &CanBus) {
        let supplied = ev_ecu_powered && ev_ecu.gpio_level(RELAY_PORT) & RELAY != 0 && dclink_connected(ev_ecu);
        let wanted = matches!(requested(bus), Some(ClimateMode::Heat | ClimateMode::Defrost));
        self.step(supplied && wanted);
    }

    fn step(&mut self, on: bool) {
        if !on || self.water_c >= WATER_OFF_C {
            self.heating = false;
        } else if self.water_c <= WATER_ON_C {
            self.heating = true;
        }
        let watts = if self.heating { HEATER_WATTS } else { 0.0 };
        self.water_c += (watts - LOSS_W_PER_K * (self.water_c - AMBIENT_C)) / HEAT_CAPACITY_J_PER_K * VEHICLE_DT;
    }

    fn report(&self) -> [u8; 8] {
        let amps = (self.amps() / AMPS_PER_BIT).round() as u8;
        [0, 0, 0, TWELVE_VOLT_CURRENT, amps, temp_byte(self.water_c - RETURN_DROP_C), temp_byte(self.water_c), 0]
    }
}

impl BusSource for Heater {
    fn frames(&mut self, _bus: &CanBus) -> Vec<CanFrame> {
        let due = self.pumps.is_multiple_of(REPORT_PUMPS);
        self.pumps += 1;
        if due { vec![frame(HEATER_ID, &self.report())] } else { Vec::new() }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn run(heater: &mut Heater, on: bool, seconds: f32) {
        for _ in 0..(seconds / VEHICLE_DT) as u32 {
            heater.step(on);
        }
    }

    #[test]
    fn heats_its_water_and_holds_it() {
        let mut heater = Heater::default();
        run(&mut heater, false, 10.0);
        assert!(!heater.heating());
        assert_eq!(heater.report()[4], 0);
        run(&mut heater, true, 1.0);
        assert!(heater.heating());
        assert_eq!(heater.report()[4], 150, "15 A in 0.1 A");
        run(&mut heater, true, 600.0);
        assert!((WATER_ON_C - 1.0..=WATER_OFF_C + 1.0).contains(&heater.water_c()), "{} °C", heater.water_c());
        run(&mut heater, false, 1.0);
        assert!(!heater.heating());
    }

    #[test]
    fn reports_temperatures_as_ovms_reads_them() {
        let heater = Heater { water_c: 50.0, heating: false, pumps: 0 };
        let r = heater.report();
        assert!((r[6] as f32 * TEMP_PER_BIT - TEMP_OFFSET - 50.0).abs() < 0.6);
        assert!((r[5] as f32 * TEMP_PER_BIT - TEMP_OFFSET - 45.0).abs() < 0.6);
    }

    #[test]
    fn follows_a_running_heat_or_defrost_request() {
        let mut bus = CanBus::default();
        assert_eq!(requested(&bus), None);
        bus.record(frame(REMOTE_CLIMATE_ID, &[0xc5, 0]));
        assert_eq!(requested(&bus), Some(ClimateMode::Heat));
        bus.record(frame(REMOTE_CLIMATE_ID, &[0xc4, 0]));
        assert_eq!(requested(&bus), None);
        bus.record(frame(REMOTE_CLIMATE_ID, &[0xc7, 0]));
        assert_eq!(requested(&bus), Some(ClimateMode::Defrost));
    }
}
