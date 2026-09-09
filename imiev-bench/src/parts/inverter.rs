//! Various chassis frames and inverter's periodic reports.

use mh8106f::CanFrame;

use crate::{frame, BusSource, CanBus};

const CHASSIS_FRAMES: &[(u16, [u8; 8])] = &[
    (0x412, [0xFE, 0x00, 0x01, 0x64, 0x20, 0x00, 0x21, 0x06]), // ignition status + speed (b1 = km/h)
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

const INV_RPM_ID: u16 = 0x288; // b0:b1 const 0x07D0, b2:b3 rpm+10000, b4 DC-link/2, b6:b7 status
const INV_TORQUE_ID: u16 = 0x298;
const INV_STANDSTILL: u16 = 10_000; // rpm word for 0 rpm
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
}

impl Inverter {
    pub(crate) fn frames(&self, bus: &CanBus) -> Vec<CanFrame> {
        let [wh, wl] = INV_STANDSTILL.to_be_bytes();
        let gear418 = bus.last(ECU_GEAR_ID).map(|f| f.data[0]).unwrap_or(0);
        let drive_gear = matches!(gear418, 0x44 | 0x52 | 0x42 | 0x43); // D | R | B | C
        let (mut b6, b7) = if drive_gear { INV_STATUS_DRIVE } else { INV_STATUS_PARK };
        if self.engaging {
            b6 &= !INV_STATUS_SETTLED_BIT;
        }
        let dclink = if self.drive_dclink { INV_DCLINK_PACK_HALF } else { INV_DCLINK_PRECHARGE_HALF };
        let mut out = vec![
            frame(INV_RPM_ID, &[0x07, 0xD0, wh, wl, dclink, 0x00, b6, b7]),
            frame(INV_TORQUE_ID, &[0x2e, 0x2e, 0x2f, 0x2e, 0x00, 0x00, wh, wl]),
        ];
        for id in INV_GATE_IDS {
            out.push(frame(id, &[INV_GATE_ID_WORD[0], INV_GATE_ID_WORD[1], 0, 0, 0, 0, 0, 0]));
        }
        out
    }
}
