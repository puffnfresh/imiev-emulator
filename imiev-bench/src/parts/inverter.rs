//! Various chassis frames and inverter's periodic reports.

use mh8106f::CanFrame;

use crate::{frame, BusSource, CanBus};

const CHASSIS_FRAMES: &[(u16, [u8; 8])] = &[
    (0x424, [0x43, 0x00, 0x0C, 0x00, 0xCF, 0x94, 0x03, 0xFF]), // ETACS lights/locks
    (0x231, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]), // brake switch (b4 = 0, released)
    (0x200, [0x00, 0x03, 0xC0, 0x00, 0xC0, 0x00, 0xFF, 0xFF]),
    (0x3a4, [0x0D, 0x90, 0x5E, 0x79, 0x58, 0x30, 0x00, 0x5E]),
    (0x285, [0, 0, 0, 0, 0, 0, 0, 0]), // EV-ECU acceleration command (idle)
    (0x286, [0, 0, 0, 0, 0, 0, 0, 0]), // EV-ECU secondary command
    (0x5a1, [0, 0, 0, 0, 0, 0, 0, 0]), // diagnostic-status mailboxes
    (0x565, [0, 0, 0, 0, 0, 0, 0, 0]),
    (0x564, [0, 0, 0, 0, 0, 0, 0, 0]),
];

#[derive(Default)]
pub struct Vehicle;

impl BusSource for Vehicle {
    fn frames(&mut self, _bus: &CanBus) -> Vec<CanFrame> {
        CHASSIS_FRAMES.iter().map(|&(id, data)| frame(id, &data)).collect()
    }
}

const MASS: f32 = 1100.0; // kg (i-MiEV ~1080 curb)
const GEAR: f32 = 7.065; // single-speed reduction
const WHEEL_R: f32 = 0.28; // m
const WHEEL_CIRC: f32 = 2.0 * std::f32::consts::PI * WHEEL_R;
const CDA: f32 = 0.62; // Cd*A
const RHO: f32 = 1.2; // air density
const CRR: f32 = 0.012; // rolling-resistance coefficient
const G: f32 = 9.81;
const TORQUE_SCALE: f32 = 180.0 / 3650.0; // d384 torque request (0..3650) -> Nm (~180 Nm peak)
const BRAKE_FORCE: f32 = 3500.0; // N at full brake
pub const VEHICLE_DT: f32 = 0.01;

const INV_RPM_ID: u16 = 0x288; // b0:b1 const 0x07D0, b2:b3 rpm+10000, b4 DC-link/2, b6:b7 status
const INV_TORQUE_ID: u16 = 0x298;
const WHEEL_SPEED_ID: u16 = 0x412; // ignition status + wheel speed (b1 = km/h)
const WHEEL_SPEED_BASE: [u8; 8] = [0xFE, 0x00, 0x01, 0x64, 0x20, 0x00, 0x21, 0x06];
const INV_STANDSTILL: u16 = 10_000; // rpm word offset (word = 10000 + rpm)
const INV_DCLINK_PRECHARGE_HALF: u8 = 39; // 78 V / 2, the inverter's reported DC-link during precharge
const INV_DCLINK_PACK_HALF: u8 = 162; // 324 V / 2, DC-link at pack once the main contactor is closed
const INV_GATE_IDS: [u16; 3] = [0x100, 0x110, 0x111]; // gate-driver identity frames
const INV_GATE_ID_WORD: [u8; 2] = [0x01, 0x01]; // matches the ECU's expected_id_a
const ECU_GEAR_ID: u16 = 0x418; // the ECU re-broadcasts the selected gear here
const INV_STATUS_PARK: (u8, u8) = (0x11, 0x10); // 0x288 b6:b7 - inverter idle/ready in Park
const INV_STATUS_DRIVE: (u8, u8) = (0x1f, 0x1c); // b6:b7 - gate drivers enabled, drive-engaged

const INV_STATUS_SETTLED_BIT: u8 = 0x10; // 0x288 b6 bit4: inverter status "settled" (steady park or drive)

#[derive(Default)]
pub struct Inverter {
    pub(crate) drive_dclink: bool,
    pub(crate) engaging: bool,
    speed: f32, // m/s
    rpm: f32,   // motor rpm (via the reduction gear)
}

impl Inverter {
    pub(crate) fn integrate(&mut self, torque_req: f32, brake: f32, coupled: bool, dt: f32) {
        let f_trac = if coupled {
            (torque_req.max(0.0) * TORQUE_SCALE) * GEAR / WHEEL_R // N at the wheels
        } else {
            0.0
        };
        let f_drag = 0.5 * RHO * CDA * self.speed * self.speed;
        let f_roll = if self.speed > 0.01 { CRR * MASS * G } else { 0.0 };
        let f_brake = brake.clamp(0.0, 1.0) * BRAKE_FORCE;
        let accel = (f_trac - f_drag - f_roll - f_brake) / MASS;
        self.speed = (self.speed + accel * dt).max(0.0);
        self.rpm = self.speed / WHEEL_CIRC * GEAR * 60.0;
    }

    pub(crate) fn kmh(&self) -> f32 {
        self.speed * 3.6
    }

    fn rpm_word(&self) -> [u8; 2] {
        let w = (INV_STANDSTILL as f32 + self.rpm).clamp(0.0, u16::MAX as f32) as u16;
        w.to_be_bytes()
    }

    pub(crate) fn frames(&self, bus: &CanBus) -> Vec<CanFrame> {
        let [wh, wl] = self.rpm_word();
        let gear418 = bus.last(ECU_GEAR_ID).map(|f| f.data[0]).unwrap_or(0);
        let drive_gear = matches!(gear418, 0x44 | 0x52 | 0x42 | 0x43); // D | R | B | C
        let (mut b6, b7) = if drive_gear { INV_STATUS_DRIVE } else { INV_STATUS_PARK };
        if self.engaging {
            b6 &= !INV_STATUS_SETTLED_BIT;
        }
        let dclink = if self.drive_dclink { INV_DCLINK_PACK_HALF } else { INV_DCLINK_PRECHARGE_HALF };
        let mut speed_frame = WHEEL_SPEED_BASE;
        speed_frame[1] = self.kmh().clamp(0.0, 254.0) as u8;
        let mut out = vec![
            frame(INV_RPM_ID, &[0x07, 0xD0, wh, wl, dclink, 0x00, b6, b7]),
            frame(INV_TORQUE_ID, &[0x2e, 0x2e, 0x2f, 0x2e, 0x00, 0x00, wh, wl]),
            frame(WHEEL_SPEED_ID, &speed_frame),
        ];
        for id in INV_GATE_IDS {
            out.push(frame(id, &[INV_GATE_ID_WORD[0], INV_GATE_ID_WORD[1], 0, 0, 0, 0, 0, 0]));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coupled_drive_torque_accelerates_the_car() {
        let mut inv = Inverter::default();
        for _ in 0..500 {
            inv.integrate(3650.0, 0.0, true, VEHICLE_DT);
        }
        assert!(inv.kmh() > 5.0, "car did not accelerate under drive torque");
        assert!(u16::from_be_bytes(inv.rpm_word()) > INV_STANDSTILL);
    }

    #[test]
    fn park_decouples_the_driveline() {
        let mut inv = Inverter::default();
        for _ in 0..500 {
            inv.integrate(3650.0, 0.0, false, VEHICLE_DT);
        }
        assert_eq!(inv.kmh(), 0.0);
        assert_eq!(u16::from_be_bytes(inv.rpm_word()), INV_STANDSTILL);
    }

    #[test]
    fn braking_brings_it_back_to_rest() {
        let mut inv = Inverter::default();
        for _ in 0..500 {
            inv.integrate(3650.0, 0.0, true, VEHICLE_DT);
        }
        assert!(inv.kmh() > 5.0);
        for _ in 0..2000 {
            inv.integrate(0.0, 1.0, true, VEHICLE_DT);
        }
        assert_eq!(inv.kmh(), 0.0);
    }
}
