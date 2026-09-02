//! The MH8106F has two CAN modules. At the moment this only handles receive
//! for CAN1.
//!
//! Raises CAN1-RX interrupt.

use super::Peripheral;
use crate::{be_read, be_write};

pub const CAN1_BASE: u32 = 0x0080_1400;
pub const CAN1_END: u32 = 0x0080_1800; // exclusive; block is 0x400 wide
pub const SLIST: u32 = 0x0080_140C; // slot interrupt/RX status (32-bit)
pub const SLOT_CTRL_BASE: u32 = 0x0080_1450; // C1MSLnCNT = SLOT_CTRL_BASE + n
pub const SLOT_BASE: u32 = 0x0080_1500; // slot n at SLOT_BASE + n*SLOT_STRIDE
pub const SLOT_STRIDE: u32 = 0x10;
pub const NUM_SLOTS: u32 = 32;

pub const CAN1_RX_IVECT: u16 = 0x0110;

// Per-slot control byte (`C1MSLnCNT`): bits 7:6 = mode (0b01 = receive), bit0 = valid.
const CNT_MODE_RX: u8 = 0x40;
const CNT_VALID: u8 = 0x01;

// Message-slot field offsets, relative to the slot base.
const SLOT_SID0: u32 = 0x00;
const SLOT_SID1: u32 = 0x01;
const SLOT_DLC: u32 = 0x05;
const SLOT_DATA0: u32 = 0x06; // Data0..Data7 at +0x06..+0x0D

pub struct Can1 {
    regs: [u8; (CAN1_END - CAN1_BASE) as usize],
}

impl Default for Can1 {
    fn default() -> Self {
        Self::new()
    }
}

impl Can1 {
    pub fn new() -> Can1 {
        Can1 {
            regs: [0; (CAN1_END - CAN1_BASE) as usize],
        }
    }

    #[inline]
    fn off(a: u32) -> usize {
        (a - CAN1_BASE) as usize
    }

    /// Write SID/DLC/data registers, flag it valid, and set bit in the slot
    /// interrupt status. Returns the CAN1-RX interrupt vector so the caller can
    /// raise it on the ICU.
    pub fn deliver_rx(&mut self, slot: u32, sid: u16, data: &[u8]) -> u16 {
        debug_assert!(slot < NUM_SLOTS, "slot out of range");
        let base = Self::off(SLOT_BASE + slot * SLOT_STRIDE);
        self.regs[base + SLOT_SID0 as usize] = ((sid >> 6) & 0x1f) as u8;
        self.regs[base + SLOT_SID1 as usize] = (sid & 0x3f) as u8;
        let len = data.len().min(8);
        self.regs[base + SLOT_DLC as usize] = len as u8;
        for i in 0..8 {
            self.regs[base + SLOT_DATA0 as usize + i] = 0;
        }
        self.regs[base + SLOT_DATA0 as usize..base + SLOT_DATA0 as usize + len]
            .copy_from_slice(&data[..len]);
        // Slot control: receive mode + message valid.
        self.regs[Self::off(SLOT_CTRL_BASE + slot)] = CNT_MODE_RX | CNT_VALID;
        // Slot interrupt status: set this slot's receive-complete bit (slot 0 is the MSB).
        let slist = be_read(&self.regs, Self::off(SLIST), 4) | (0x8000_0000u32 >> slot);
        be_write(&mut self.regs, Self::off(SLIST), 4, slist);
        CAN1_RX_IVECT
    }
}

impl Peripheral for Can1 {
    fn handles(&self, a: u32) -> bool {
        (CAN1_BASE..CAN1_END).contains(&a)
    }

    fn read(&mut self, a: u32, size: u32) -> u32 {
        be_read(&self.regs, Self::off(a), size)
    }

    fn write(&mut self, a: u32, size: u32, v: u32) {
        be_write(&mut self.regs, Self::off(a), size, v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn slot_addr(n: u32, field: u32) -> u32 {
        SLOT_BASE + n * SLOT_STRIDE + field
    }

    #[test]
    fn deliver_rx_populates_slot_and_status() {
        let mut can = Can1::new();
        let slot = 30; // any slot; the chip is agnostic about what it carries
        let iv = can.deliver_rx(slot, 0x611, &[0x10, 0x20, 0x30, 0x40, 0x50, 0x60, 0x70, 0x80]);
        assert_eq!(iv, CAN1_RX_IVECT);

        // SID encoded so the firmware's decode `(SID0&0x1f)*64 + SID1` == 0x611.
        let sid0 = can.read(slot_addr(slot, SLOT_SID0), 1) as u16;
        let sid1 = can.read(slot_addr(slot, SLOT_SID1), 1) as u16;
        assert_eq!((sid0 & 0x1f) * 64 + sid1, 0x611);

        // DLC + data bytes readable at their register offsets.
        assert_eq!(can.read(slot_addr(slot, SLOT_DLC), 1), 8);
        assert_eq!(can.read(slot_addr(slot, SLOT_DATA0), 1), 0x10);
        assert_eq!(can.read(slot_addr(slot, SLOT_DATA0 + 7), 1), 0x80);

        // Slot control reads RX + valid; the firmware's checks are (CNT&0xc0)==0x40
        // and (CNT&0x41)==0x41.
        let cnt = can.read(SLOT_CTRL_BASE + slot, 1) as u8;
        assert_eq!(cnt & 0xc0, CNT_MODE_RX); // bits 7:6 = receive mode
        assert_eq!(cnt & 0x41, 0x41);

        // Slot interrupt status bit for slot N is 0x8000_0000 >> N.
        assert_eq!(can.read(SLIST, 4) & (0x8000_0000u32 >> slot), 0x8000_0000u32 >> slot);
    }

    #[test]
    fn config_registers_are_readback_storage() {
        let mut can = Can1::new();
        can.write(CAN1_BASE + 0x10, 2, 0x4980); // e.g. configuration register
        assert_eq!(can.read(CAN1_BASE + 0x10, 2), 0x4980);
    }

    #[test]
    fn firmware_can_clear_slist_bit() {
        let mut can = Can1::new();
        let slot = 30;
        can.deliver_rx(slot, 0x611, &[0; 8]);
        // Firmware clears a serviced slot by writing ~(0x8000_0000 >> slot).
        can.write(SLIST, 4, !(0x8000_0000u32 >> slot));
        assert_eq!(can.read(SLIST, 4) & (0x8000_0000u32 >> slot), 0);
    }
}
