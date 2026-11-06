//! Brake booster's vacuum pump

use mh8106f::System;

use crate::adc::ev_ecu_adc;
use crate::{CanBus, Part, VEHICLE_DT};

const RELAY_PORT: u32 = 0x0080_070a; // P10DATA
const MAIN_RELAY_COIL: u8 = 0x80; // P10.7 output: the pump's main (fail-safe) relay
const CONTACT_PORT: u32 = 0x0080_0716; // P22DATA
const CONTACT_MONITOR: u8 = 0x08; // P22.3 input: the main relay's switched side
const COIL_PORT: u32 = 0x0080_070c; // P12DATA
const COIL_MONITOR: u8 = 0x01; // P12.0 input: the main relay coil's low side
const CONTROL_RELAY_COILS: u8 = 0x60; // P10.6 / P10.5 outputs: the pump motor's control relays

const PRESSURE_PER_COUNT: f32 = 0.1237;
const PRESSURE_AT_POWER_ON: f32 = 63.3;
const PRESSURE_ATMOSPHERE: f32 = 100.0;
const PRESSURE_PUMPED: f32 = 5.0;
const PUMP_TAU_S: f32 = 2.0;
const LEAK_TAU_S: f32 = 120.0;
const PUMP_VOLTS_PER_COUNT: f32 = 0.0183;
const SUPPLY_V: f32 = 13.5;
const MOTOR_SPIN_UP_TAU_S: f32 = 0.05;
const MOTOR_COAST_TAU_S: f32 = 0.15;
const BACK_EMF_AT_FULL_SPEED_V: f32 = 11.0;

pub struct VacuumPump {
    pressure: f32,
    motor_speed: f32, // 0..1 of full speed
}

impl Default for VacuumPump {
    fn default() -> Self {
        VacuumPump { pressure: PRESSURE_AT_POWER_ON, motor_speed: 0.0 }
    }
}

impl VacuumPump {
    fn step(&mut self, powered: bool, dt: f32) {
        let (speed, tau) = if powered { (1.0, MOTOR_SPIN_UP_TAU_S) } else { (0.0, MOTOR_COAST_TAU_S) };
        self.motor_speed += (speed - self.motor_speed) * (dt / tau).min(1.0);
        let (target, tau) = if powered { (PRESSURE_PUMPED, PUMP_TAU_S) } else { (PRESSURE_ATMOSPHERE, LEAK_TAU_S) };
        self.pressure += (target - self.pressure) * (dt / tau).min(1.0);
    }

    fn motor_volts(&self, powered: bool) -> f32 {
        if powered { SUPPLY_V } else { self.motor_speed * BACK_EMF_AT_FULL_SPEED_V }
    }
}

impl Part for VacuumPump {
    fn update(&mut self, chip: &mut System, _bus: &CanBus) {
        let relays = chip.gpio_level(RELAY_PORT);
        let on = relays & MAIN_RELAY_COIL != 0;
        chip.set_gpio_input(CONTACT_PORT, CONTACT_MONITOR, if on { CONTACT_MONITOR } else { 0 });
        chip.set_gpio_input(COIL_PORT, COIL_MONITOR, if on { 0 } else { COIL_MONITOR });
        let powered = on && relays & CONTROL_RELAY_COILS != 0;
        self.step(powered, VEHICLE_DT);
        let volts = self.motor_volts(powered);
        chip.adc_mut().set_channel(ev_ecu_adc::VACUUM_PUMP_VOLTAGE, (volts / PUMP_VOLTS_PER_COUNT) as u16);
        chip.adc_mut().set_channel(ev_ecu_adc::VACUUM_OUTPUT, (self.pressure / PRESSURE_PER_COUNT) as u16);
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
        let mut pump = VacuumPump::default();
        pump.update(&mut ecu, &bus);
        assert_eq!(ecu.gpio_level(CONTACT_PORT) & CONTACT_MONITOR, 0);
        assert_eq!(ecu.gpio_level(COIL_PORT) & COIL_MONITOR, COIL_MONITOR);
        ecu.set_gpio_input(RELAY_PORT, MAIN_RELAY_COIL, MAIN_RELAY_COIL);
        pump.update(&mut ecu, &bus);
        assert_eq!(ecu.gpio_level(CONTACT_PORT) & CONTACT_MONITOR, CONTACT_MONITOR);
        assert_eq!(ecu.gpio_level(COIL_PORT) & COIL_MONITOR, 0);
    }

    #[test]
    fn pumping_lowers_the_pressure_and_it_leaks_back() {
        let mut pump = VacuumPump::default();
        let steps = |s: f32| (s / VEHICLE_DT) as u32;
        for _ in 0..steps(3.0) {
            pump.step(true, VEHICLE_DT);
        }
        assert!(pump.pressure < 21.3, "pumped to {}", pump.pressure);
        let pumped = pump.pressure;
        for _ in 0..steps(10.0) {
            pump.step(false, VEHICLE_DT);
        }
        assert!(pump.pressure > pumped + 1.0 && pump.pressure < 41.3, "leaked to {}", pump.pressure);
    }

    #[test]
    fn the_motor_coasts_down_once_switched_off() {
        let mut pump = VacuumPump::default();
        for _ in 0..(1.0 / VEHICLE_DT) as u32 {
            pump.step(true, VEHICLE_DT);
        }
        assert_eq!(pump.motor_volts(true), SUPPLY_V);
        pump.step(false, VEHICLE_DT);
        assert!(pump.motor_volts(false) > 10.0, "no back-EMF just after switch-off");
        for _ in 0..(0.5 / VEHICLE_DT) as u32 {
            pump.step(false, VEHICLE_DT);
        }
        assert!(pump.motor_volts(false) < 1.0, "still {} V half a second later", pump.motor_volts(false));
    }
}
