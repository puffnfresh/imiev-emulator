//! The MH8106F has two CAN modules.

use super::Peripheral;
use crate::{be_read, be_write};

pub const BLOCK_SIZE: u32 = 0x400; // one CAN module's register block

const SLIST: u32 = 0x0c; // slot interrupt/status, 32-bit (offset from base)
const SLOT_CTRL_BASE: u32 = 0x50; // CnMSLnCNT = base + SLOT_CTRL_BASE + n
const SLOT_CTRL_END: u32 = SLOT_CTRL_BASE + NUM_SLOTS;
const SLOT_BASE: u32 = 0x100; // slot n at base + SLOT_BASE + n*SLOT_STRIDE
const SLOT_STRIDE: u32 = 0x10;
pub const NUM_SLOTS: u32 = 32;

// Slot-control byte bits (MSB-first labelling -> these byte values).
const TR: u8 = 0x80; // transmit request
const RR: u8 = 0x40; // receive request
const TRFIN: u8 = 0x01; // transmit/receive finished

// Message-slot field offsets, relative to the slot base.
const SLOT_SID0: u32 = 0x00;
const SLOT_SID1: u32 = 0x01;
const SLOT_DLC: u32 = 0x05;
const SLOT_DATA0: u32 = 0x06; // Data0..Data7 at +0x06..+0x0D

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CanFrame {
    pub id: u16,
    pub dlc: u8,
    pub data: [u8; 8],
}

fn encode_sid(id: u16) -> (u8, u8) {
    (((id >> 6) & 0x1f) as u8, (id & 0x3f) as u8)
}

fn decode_sid(sid0: u8, sid1: u8) -> u16 {
    ((sid0 as u16 & 0x1f) << 6) | (sid1 as u16 & 0x3f)
}

pub struct CanModule {
    base: u32,
    rx_ivect: u16,
    regs: [u8; BLOCK_SIZE as usize],
    /// Frames the firmware has transmitted (TR set), for the bench to observe.
    tx: Vec<CanFrame>,
}

impl CanModule {
    pub fn new(base: u32, rx_ivect: u16) -> CanModule {
        CanModule {
            base,
            rx_ivect,
            regs: [0; BLOCK_SIZE as usize],
            tx: Vec::new(),
        }
    }

    #[inline]
    fn off(&self, a: u32) -> usize {
        (a - self.base) as usize
    }

    #[inline]
    fn slot_field(&self, slot: u32, field: u32) -> usize {
        (SLOT_BASE + slot * SLOT_STRIDE + field) as usize
    }

    pub fn deliver_rx(&mut self, id: u16, data: &[u8]) -> Option<u16> {
        let (sid0, sid1) = encode_sid(id);
        let slot = (0..NUM_SLOTS).find(|&slot| {
            let ctrl = self.regs[(SLOT_CTRL_BASE + slot) as usize];
            let base = self.slot_field(slot, 0);
            ctrl & RR != 0
                && self.regs[base + SLOT_SID0 as usize] == sid0
                && self.regs[base + SLOT_SID1 as usize] == sid1
        })?;
        self.write_rx_slot(slot, id, data);
        Some(self.rx_ivect)
    }

    pub fn deliver_rx_into(&mut self, slot: u32, id: u16, data: &[u8]) -> u16 {
        self.write_rx_slot(slot, id, data);
        self.rx_ivect
    }

    fn write_rx_slot(&mut self, slot: u32, id: u16, data: &[u8]) {
        debug_assert!(slot < NUM_SLOTS, "slot out of range");
        let base = self.slot_field(slot, 0);
        let (sid0, sid1) = encode_sid(id);
        self.regs[base + SLOT_SID0 as usize] = sid0;
        self.regs[base + SLOT_SID1 as usize] = sid1;
        let len = data.len().min(8);
        self.regs[base + SLOT_DLC as usize] = len as u8;
        for i in 0..8 {
            self.regs[base + SLOT_DATA0 as usize + i] = 0;
        }
        self.regs[base + SLOT_DATA0 as usize..base + SLOT_DATA0 as usize + len]
            .copy_from_slice(&data[..len]);
        // Receive slot, finished receiving.
        self.regs[(SLOT_CTRL_BASE + slot) as usize] = RR | TRFIN;
        let slist = be_read(&self.regs, SLIST as usize, 4) | (0x8000_0000u32 >> slot);
        be_write(&mut self.regs, SLIST as usize, 4, slist);
    }

    pub fn take_tx(&mut self) -> Vec<CanFrame> {
        std::mem::take(&mut self.tx)
    }

    pub fn rx_slots(&self) -> Vec<(u32, u16, u8)> {
        (0..NUM_SLOTS)
            .filter_map(|slot| {
                let ctrl = self.regs[(SLOT_CTRL_BASE + slot) as usize];
                (ctrl & RR != 0).then(|| {
                    let base = self.slot_field(slot, 0);
                    let sid = decode_sid(
                        self.regs[base + SLOT_SID0 as usize],
                        self.regs[base + SLOT_SID1 as usize],
                    );
                    (slot, sid, ctrl)
                })
            })
            .collect()
    }

    fn transmit(&mut self, slot: u32) {
        let base = self.slot_field(slot, 0);
        let id = decode_sid(
            self.regs[base + SLOT_SID0 as usize],
            self.regs[base + SLOT_SID1 as usize],
        );
        let dlc = self.regs[base + SLOT_DLC as usize];
        let mut data = [0u8; 8];
        data.copy_from_slice(&self.regs[base + SLOT_DATA0 as usize..base + SLOT_DATA0 as usize + 8]);
        self.tx.push(CanFrame { id, dlc, data });
        // Completion: clear the request, mark finished, raise the slot's status bit.
        let ctrl = (SLOT_CTRL_BASE + slot) as usize;
        self.regs[ctrl] = (self.regs[ctrl] & !TR) | TRFIN;
        let slist = be_read(&self.regs, SLIST as usize, 4) | (0x8000_0000u32 >> slot);
        be_write(&mut self.regs, SLIST as usize, 4, slist);
    }
}

impl Peripheral for CanModule {
    fn handles(&self, a: u32) -> bool {
        (self.base..self.base + BLOCK_SIZE).contains(&a)
    }

    fn read(&mut self, a: u32, size: u32) -> u32 {
        be_read(&self.regs, self.off(a), size)
    }

    fn write(&mut self, a: u32, size: u32, v: u32) {
        let off = self.off(a);
        be_write(&mut self.regs, off, size, v);
        for a in a..a + size {
            let off = a - self.base;
            if (SLOT_CTRL_BASE..SLOT_CTRL_END).contains(&off) && self.regs[off as usize] & TR != 0 {
                self.transmit(off - SLOT_CTRL_BASE);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CAN1_BASE: u32 = 0x0080_1400;

    fn slot_ctrl(base: u32, n: u32) -> u32 {
        base + SLOT_CTRL_BASE + n
    }
    fn slot_field(base: u32, n: u32, field: u32) -> u32 {
        base + SLOT_BASE + n * SLOT_STRIDE + field
    }

    #[test]
    fn deliver_rx_routes_to_the_armed_mailbox() {
        // The firmware arms mailbox 15 to receive SID 0x412 (RR + acceptance ID).
        let mut can = CanModule::new(0x0080_1000, 0x0000);
        let base = 0x0080_1000;
        let slot = 15;
        let (s0, s1) = encode_sid(0x412);
        can.write(slot_field(base, slot, SLOT_SID0), 1, s0 as u32);
        can.write(slot_field(base, slot, SLOT_SID1), 1, s1 as u32);
        can.write(slot_ctrl(base, slot), 1, RR as u32);

        // A bus frame for a SID no mailbox wants is dropped.
        assert_eq!(can.deliver_rx(0x321, &[1, 2, 3]), None);
        // The 0x412 frame lands in mailbox 15 by acceptance matching — no slot given.
        assert_eq!(can.deliver_rx(0x412, &[0xaa, 0xbb]), Some(0x0000));
        let s0r = can.read(slot_field(base, slot, SLOT_SID0), 1) as u8;
        let s1r = can.read(slot_field(base, slot, SLOT_SID1), 1) as u8;
        assert_eq!(decode_sid(s0r, s1r), 0x412);
        assert_eq!(can.read(slot_field(base, slot, SLOT_DATA0), 1), 0xaa);
        assert_eq!(
            can.read(base + SLIST, 4) & (0x8000_0000u32 >> slot),
            0x8000_0000u32 >> slot
        );
    }

    #[test]
    fn deliver_rx_into_forces_a_slot() {
        let mut can = CanModule::new(CAN1_BASE, 0x0110);
        let slot = 30;
        let iv = can.deliver_rx_into(slot, 0x611, &[0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80]);
        assert_eq!(iv, 0x0110);

        let sid0 = can.read(slot_field(CAN1_BASE, slot, SLOT_SID0), 1) as u8;
        let sid1 = can.read(slot_field(CAN1_BASE, slot, SLOT_SID1), 1) as u8;
        assert_eq!(decode_sid(sid0, sid1), 0x611);
        assert_eq!(can.read(slot_field(CAN1_BASE, slot, SLOT_DLC), 1), 8);
        assert_eq!(can.read(slot_field(CAN1_BASE, slot, SLOT_DATA0 + 7), 1), 0x80);

        let cnt = can.read(slot_ctrl(CAN1_BASE, slot), 1) as u8;
        assert_eq!(cnt & (RR | TRFIN), RR | TRFIN); // receive slot, finished
        assert_eq!(cnt & TR, 0); // not a transmit request
        assert_eq!(
            can.read(CAN1_BASE + SLIST, 4) & (0x8000_0000u32 >> slot),
            0x8000_0000u32 >> slot
        );
    }

    #[test]
    fn sid_round_trips() {
        for id in [0x000, 0x373, 0x374, 0x375, 0x611, 0x6c4, 0x7ff] {
            let (s0, s1) = encode_sid(id);
            assert_eq!(decode_sid(s0, s1), id, "sid {id:#x}");
        }
    }

    #[test]
    fn firmware_transmit_is_captured() {
        let mut can = CanModule::new(0x0080_1000, 0x0000); // CAN0
        let base = 0x0080_1000;
        let slot = 15;
        // Firmware builds the frame: SID, DLC, data.
        let (s0, s1) = encode_sid(0x374);
        can.write(slot_field(base, slot, SLOT_SID0), 1, s0 as u32);
        can.write(slot_field(base, slot, SLOT_SID1), 1, s1 as u32);
        can.write(slot_field(base, slot, SLOT_DLC), 1, 8);
        for i in 0..8u32 {
            can.write(slot_field(base, slot, SLOT_DATA0 + i), 1, 0xa0 + i);
        }
        assert!(can.take_tx().is_empty(), "no frame until TR is set");

        // Then sets the transmit request (TR).
        can.write(slot_ctrl(base, slot), 1, TR as u32);
        let tx = can.take_tx();
        assert_eq!(tx.len(), 1);
        assert_eq!(tx[0].id, 0x374);
        assert_eq!(tx[0].dlc, 8);
        assert_eq!(tx[0].data[0], 0xa0);
        assert_eq!(tx[0].data[7], 0xa7);
        // Draining is one-shot.
        assert!(can.take_tx().is_empty());
        // TR cleared, TRFIN set, slot status bit raised.
        let cnt = can.read(slot_ctrl(base, slot), 1) as u8;
        assert_eq!(cnt & TR, 0);
        assert_eq!(cnt & TRFIN, TRFIN);
    }
}
