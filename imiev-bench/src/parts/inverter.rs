//! Various chassis frames and inverter's periodic reports.

use mh8106f::CanFrame;

use crate::{frame, BusSource, CanBus};

const CHASSIS_FRAMES: &[(u16, [u8; 8])] = &[
    (0x424, [0x43, 0x00, 0x0C, 0x00, 0xCF, 0x94, 0x03, 0xFF]), // ETACS lights/locks
    (0x231, [0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00]), // brake switch (b4 = 0, released)
    (0x3a4, [0x0D, 0x90, 0x5E, 0x79, 0x58, 0x30, 0x00, 0x5E]),
    (0x286, [0, 0, 0, 0, 0, 0, 0, 0]), // EV-ECU secondary command
    (0x5a1, [0, 0, 0, 0, 0, 0, 0, 0]), // diagnostic-status mailboxes
    (0x565, [0, 0, 0, 0, 0, 0, 0, 0]),
    (0x564, [0, 0, 0, 0, 0, 0, 0, 0]),
    (0x384, [0x00, 0x00, 0x00, 0x1F, 0x00, 0x41, 0x41, 0x00]), // heater/A-C compressor
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
const ECU_TORQUE_ID: u16 = 0x285; // b0:b1 = commanded motor torque * 10 + 2000 (Nm)
const TORQUE_OFFSET: f32 = 2000.0;
const TORQUE_PER_BIT: f32 = 0.1; // Nm
const BRAKE_FORCE: f32 = 3500.0; // N at full brake
pub const VEHICLE_DT: f32 = crate::BUS_PUMP_INTERVAL as f32 / crate::CYCLES_PER_SECOND as f32;

const INV_RPM_ID: u16 = 0x288; // b0:b1 torque*10+2000, b2:b3 rpm+10000, b4 DC-link/2, b6:b7 status
const INV_TORQUE_ID: u16 = 0x298;
const WHEEL_SPEED_ID: u16 = 0x412; // ignition status + wheel speed (b1 = km/h)
const ABS_FRONT_ID: u16 = 0x200; // b2:b3 / b4:b5 = front left / right wheel rpm word
const ABS_REAR_ID: u16 = 0x208; // b2:b3 brake pedal, b4:b5 / b6:b7 = rear right / left wheel rpm word
const ABS_SPEED_ID: u16 = 0x215; // b0:b1 = vehicle speed * 128 (km/h)
const WHEEL_RPM_ZERO: u16 = 0xc000; // wheel rpm word = 0xc000 + rpm * 2 (the EV-ECU reads the low 14 bits)
const BRAKE_PEDAL_RELEASED: [u8; 2] = [0x60, 0x00];
const WHEEL_SPEED_BASE: [u8; 8] = [0xFE, 0x00, 0x01, 0x64, 0x20, 0x00, 0x21, 0x06];
const INV_STANDSTILL: u16 = 10_000; // rpm word offset (word = 10000 + rpm)
const INV_DCLINK_IDLE_HALF: f32 = 1.0; // 2 V: the real inverter's reading until the link is charged
const INV_DCLINK_PACK_HALF: u8 = 162; // 324 V / 2, DC-link at pack once the main contactor is closed
const PRECHARGE_TAU_S: f32 = 0.05;
const ECU_MODE_DRIVE: u8 = 0x14; // 0x285 b2 once the EV-ECU is in DRIVE
const INV_GATE_IDS: [u16; 3] = [0x100, 0x110, 0x111]; // gate-driver identity frames
const INV_GATE_ID_WORD: [u8; 2] = [0x01, 0x01]; // matches the ECU's expected_id_a
const ECU_GEAR_ID: u16 = 0x418; // the ECU re-broadcasts the selected gear here
const INV_STATUS_PARK: (u8, u8) = (0x11, 0x10); // 0x288 b6:b7 - inverter idle/ready in Park
const INV_STATUS_DRIVE: (u8, u8) = (0x1f, 0x1c); // b6:b7 - gate drivers enabled, drive-engaged

const INV_STATUS_SETTLED_BIT: u8 = 0x10; // 0x288 b6 bit4: inverter status "settled" (steady park or drive)

pub struct Inverter {
    pub(crate) hv_connected: bool,
    dclink_half: f32, // DC-link voltage / 2
    torque_nm: f32,
    speed: f32, // m/s
    rpm: f32, // motor rpm (via the reduction gear)
}

impl Default for Inverter {
    fn default() -> Self {
        Inverter { hv_connected: false, dclink_half: INV_DCLINK_IDLE_HALF, torque_nm: 0.0, speed: 0.0, rpm: 0.0 }
    }
}

impl Inverter {
    fn engaging(&self, bus: &CanBus) -> bool {
        let driving = bus.last(ECU_TORQUE_ID).is_some_and(|f| f.data[2] == ECU_MODE_DRIVE);
        self.hv_connected && !driving
    }

    fn commanded_torque(bus: &CanBus) -> f32 {
        bus.last(ECU_TORQUE_ID).map_or(0.0, |f| {
            (u16::from_be_bytes([f.data[0], f.data[1]]) as f32 - TORQUE_OFFSET) * TORQUE_PER_BIT
        })
    }

    pub(crate) fn integrate(&mut self, bus: &CanBus, brake: f32, dt: f32) {
        let target = if self.hv_connected { INV_DCLINK_PACK_HALF as f32 } else { INV_DCLINK_IDLE_HALF };
        self.dclink_half += (target - self.dclink_half) * (dt / PRECHARGE_TAU_S).min(1.0);
        self.drive(Self::commanded_torque(bus), brake, dt);
    }

    fn drive(&mut self, torque_nm: f32, brake: f32, dt: f32) {
        self.torque_nm = torque_nm;
        let f_trac = torque_nm * GEAR / WHEEL_R;
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

    fn torque_word(&self) -> [u8; 2] {
        let w = (self.torque_nm / TORQUE_PER_BIT + TORQUE_OFFSET).round().clamp(0.0, u16::MAX as f32) as u16;
        w.to_be_bytes()
    }

    fn wheel_word(&self) -> [u8; 2] {
        let rpm = self.speed / WHEEL_CIRC * 60.0;
        (WHEEL_RPM_ZERO + (rpm * 2.0).clamp(0.0, 0x3ffe as f32) as u16).to_be_bytes()
    }

    fn rpm_word(&self) -> [u8; 2] {
        let w = (INV_STANDSTILL as f32 + self.rpm).clamp(0.0, u16::MAX as f32) as u16;
        w.to_be_bytes()
    }

    pub(crate) fn frames(&self, bus: &CanBus) -> Vec<CanFrame> {
        let [wh, wl] = self.rpm_word();
        let [th, tl] = self.torque_word();
        let gear418 = bus.last(ECU_GEAR_ID).map(|f| f.data[0]).unwrap_or(0);
        let drive_gear = matches!(gear418, 0x44 | 0x52 | 0x42 | 0x43); // D | R | B | C
        let (mut b6, b7) = if drive_gear { INV_STATUS_DRIVE } else { INV_STATUS_PARK };
        if self.engaging(bus) {
            b6 &= !INV_STATUS_SETTLED_BIT;
        }
        let dclink = self.dclink_half as u8;
        let mut speed_frame = WHEEL_SPEED_BASE;
        speed_frame[1] = self.kmh().clamp(0.0, 254.0) as u8;
        let mut out = vec![
            frame(INV_RPM_ID, &[th, tl, wh, wl, dclink, 0x00, b6, b7]),
            frame(INV_TORQUE_ID, &[0x2e, 0x2e, 0x2f, 0x2e, 0x00, 0x00, wh, wl]),
            frame(WHEEL_SPEED_ID, &speed_frame),
        ];
        let [vh, vl] = ((self.kmh() * 128.0).clamp(0.0, u16::MAX as f32) as u16).to_be_bytes();
        let [rh, rl] = self.wheel_word();
        let [bh, bl] = BRAKE_PEDAL_RELEASED;
        out.push(frame(ABS_FRONT_ID, &[0x00, 0x03, rh, rl, rh, rl, 0xff, 0xff]));
        out.push(frame(ABS_REAR_ID, &[0x00, 0x20, bh, bl, rh, rl, rh, rl]));
        out.push(frame(ABS_SPEED_ID, &[vh, vl, 0, 0, 0, 0, 0, 0]));
        for id in INV_GATE_IDS {
            out.push(frame(id, &[INV_GATE_ID_WORD[0], INV_GATE_ID_WORD[1], 0, 0, 0, 0, 0, 0]));
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn steps(seconds: f32) -> u32 {
        (seconds / VEHICLE_DT) as u32
    }

    fn commanding(nm: f32) -> CanBus {
        let mut bus = CanBus::default();
        let word = (nm / TORQUE_PER_BIT + TORQUE_OFFSET) as u16;
        let [hi, lo] = word.to_be_bytes();
        bus.record(frame(ECU_TORQUE_ID, &[hi, lo, 0x14, 0, 0, 0, 0, 0]));
        bus
    }

    #[test]
    fn commanded_torque_accelerates_the_car() {
        let mut inv = Inverter::default();
        let bus = commanding(60.0);
        assert_eq!(Inverter::commanded_torque(&bus), 60.0);
        for _ in 0..steps(3.0) {
            inv.integrate(&bus, 0.0, VEHICLE_DT);
        }
        assert!(inv.kmh() > 5.0, "car did not accelerate under drive torque");
        assert!(u16::from_be_bytes(inv.rpm_word()) > INV_STANDSTILL);
        assert_eq!(u16::from_be_bytes(inv.torque_word()), 2600);
    }

    #[test]
    fn abs_reports_the_car_s_speed() {
        let mut inv = Inverter::default();
        let bus = commanding(60.0);
        let word = |inv: &Inverter, id, at: usize| {
            let f = inv.frames(&bus).into_iter().find(|f| f.id == id).unwrap();
            u16::from_be_bytes([f.data[at], f.data[at + 1]])
        };
        assert_eq!(word(&inv, ABS_SPEED_ID, 0), 0);
        assert_eq!(word(&inv, ABS_REAR_ID, 4), WHEEL_RPM_ZERO);
        for _ in 0..steps(3.0) {
            inv.integrate(&bus, 0.0, VEHICLE_DT);
        }
        assert_eq!(word(&inv, ABS_SPEED_ID, 0), (inv.kmh() * 128.0) as u16);
        let rpm = (word(&inv, ABS_FRONT_ID, 2) & 0x3fff) as f32 * 0.5;
        assert!((rpm * WHEEL_CIRC * 60.0 / 1000.0 - inv.kmh()).abs() < 0.1, "wheel rpm {rpm} vs {} km/h", inv.kmh());
    }

    #[test]
    fn no_torque_command_leaves_it_at_rest() {
        let mut inv = Inverter::default();
        for bus in [CanBus::default(), commanding(0.0)] {
            for _ in 0..steps(3.0) {
                inv.integrate(&bus, 0.0, VEHICLE_DT);
            }
        }
        assert_eq!(inv.kmh(), 0.0);
        assert_eq!(u16::from_be_bytes(inv.rpm_word()), INV_STANDSTILL);
    }

    #[test]
    fn braking_brings_it_back_to_rest() {
        let mut inv = Inverter::default();
        let bus = commanding(60.0);
        for _ in 0..steps(3.0) {
            inv.integrate(&bus, 0.0, VEHICLE_DT);
        }
        assert!(inv.kmh() > 5.0);
        let coasting = commanding(0.0);
        for _ in 0..steps(10.0) {
            inv.integrate(&coasting, 1.0, VEHICLE_DT);
        }
        assert_eq!(inv.kmh(), 0.0);
    }

    fn dclink_and_status(inv: &Inverter, bus: &CanBus) -> (u8, u8) {
        let f = inv.frames(bus).into_iter().find(|f| f.id == INV_RPM_ID).unwrap();
        (f.data[4], f.data[6])
    }

    #[test]
    fn dc_link_charges_once_connected() {
        let mut inv = Inverter::default();
        let mut bus = CanBus::default();
        bus.record(frame(ECU_TORQUE_ID, &[0x07, 0xd0, 0x03, 0, 0, 0, 0, 0]));
        // Contactors open: the link idles near 0 V and the inverter is settled.
        assert_eq!(dclink_and_status(&inv, &bus), (1, INV_STATUS_PARK.0));
        inv.hv_connected = true;
        for _ in 0..steps(0.25) {
            inv.integrate(&bus, 0.0, VEHICLE_DT);
        }
        let (dclink, status) = dclink_and_status(&inv, &bus);
        assert!(dclink >= INV_DCLINK_PACK_HALF - 2, "DC link only reached {dclink}");
        assert_eq!(status & INV_STATUS_SETTLED_BIT, 0);
        // Once the EV-ECU is in DRIVE the inverter reports settled again.
        assert_ne!(dclink_and_status(&inv, &commanding(0.0)).1 & INV_STATUS_SETTLED_BIT, 0);
    }
}
