//! IC2 companion-chip serial link
//!
//! The EV-ECU talks to its 20-pin "System LSI" safety companion (IC2) over this
//! link as a question/answer watchdog.

use super::Peripheral;

pub const SI23ST: u32 = 0x0080_0100;
pub const S2TXB: u32 = 0x0080_0132; // 16-bit; the data byte is the low half (0x800133)
pub const S2TXB_LO: u32 = 0x0080_0133;
pub const S2RCNT: u32 = 0x0080_0136;
pub const DM59ITST: u32 = 0x0080_0408;
pub const DM5CNT0: u32 = 0x0080_0418;
pub const DM5DA: u32 = 0x0080_041c;
const DM5DA_LO: u32 = 0x0080_041d;

pub const TX_COMPLETE: u8 = 0x08; // SI23ST bit3
pub const DMA5_COMPLETE: u8 = 0x01; // DM59ITST bit0
const DMA_ARM_CMD: u32 = 0x6d;
const RX_ERROR: u8 = 0x80; // S2RCNT bit7

const TX_LATENCY: u32 = 256; // cycles to clock one byte out of S2TXB
const RX_BYTE_LATENCY: u32 = 256; // cycles to clock one answer byte in over the serial link

#[derive(Default)]
pub struct Ic2 {
    si23st: u8,
    dm59itst: u8,
    s2rcnt: u8,
    tx_event: bool, // a byte has finished clocking out of S2TXB (TX-complete)
    tx_latency: u32, // cycles remaining before the in-flight TX byte completes
    tx_bytes: Vec<u8>, // bytes sent to the companion chip
    rx_armed: bool, // firmware armed DMA5 to receive the next answer
    dm5da: u16,     // where DMA5 writes the answer
    rx_buf: [u8; 8],
    rx_len: usize,
    rx_latency: u32,
    rx_ready: bool,
}

impl Ic2 {
    pub fn new() -> Ic2 {
        Ic2::default()
    }

    pub fn tick(&mut self, cycles: u64) {
        if self.tx_latency > 0 {
            self.tx_latency = self.tx_latency.saturating_sub(cycles as u32);
            if self.tx_latency == 0 {
                self.tx_event = true;
            }
        }
        if self.rx_latency > 0 {
            self.rx_latency = self.rx_latency.saturating_sub(cycles as u32);
            if self.rx_latency == 0 {
                self.rx_ready = true;
            }
        }
    }

    pub fn take_tx_bytes(&mut self) -> Vec<u8> {
        core::mem::take(&mut self.tx_bytes)
    }

    pub fn take_tx_event(&mut self) -> bool {
        core::mem::take(&mut self.tx_event)
    }

    pub fn rx_armed(&self) -> bool {
        self.rx_armed
    }

    pub fn take_rx_armed(&mut self) -> bool {
        core::mem::take(&mut self.rx_armed)
    }

    pub fn queue_rx(&mut self, bytes: &[u8]) {
        let n = bytes.len().min(self.rx_buf.len());
        self.rx_buf[..n].copy_from_slice(&bytes[..n]);
        self.rx_len = n;
        self.rx_latency = RX_BYTE_LATENCY * n.max(1) as u32;
        self.rx_ready = false;
    }

    pub fn take_ready_rx(&mut self) -> Option<([u8; 8], usize)> {
        if core::mem::take(&mut self.rx_ready) {
            Some((self.rx_buf, self.rx_len))
        } else {
            None
        }
    }

    pub fn set_tx_complete(&mut self) {
        self.si23st |= TX_COMPLETE;
    }

    pub fn set_dma5_complete(&mut self) {
        self.dm59itst |= DMA5_COMPLETE;
    }

    pub fn rx_destination(&self) -> u32 {
        0x0080_0000 | self.dm5da as u32
    }

    pub fn rx_pending(&self) -> bool {
        self.dm59itst & DMA5_COMPLETE != 0 || self.rx_latency > 0 || self.rx_ready
    }
}

impl Peripheral for Ic2 {
    fn handles(&self, a: u32) -> bool {
        matches!(a, SI23ST | S2TXB | S2TXB_LO | S2RCNT | DM59ITST | DM5CNT0 | DM5DA | DM5DA_LO)
    }

    fn read(&mut self, a: u32, size: u32) -> u32 {
        match a {
            SI23ST => self.si23st as u32,
            DM59ITST => self.dm59itst as u32,
            S2RCNT => self.s2rcnt as u32,
            DM5DA if size == 2 => self.dm5da as u32,
            DM5DA => (self.dm5da >> 8) as u32,
            DM5DA_LO => (self.dm5da & 0xff) as u32,
            _ => 0,
        }
    }

    fn write(&mut self, a: u32, size: u32, v: u32) {
        match a {
            // Status registers: write-0-to-clear (a 0 bit clears, a 1 bit is unchanged).
            SI23ST => self.si23st &= v as u8,
            DM59ITST => self.dm59itst &= v as u8,
            S2RCNT => self.s2rcnt = (v as u8) & !RX_ERROR, // never leave RX-error latched
            S2TXB | S2TXB_LO => {
                self.tx_bytes.push(v as u8);
                self.tx_latency = TX_LATENCY;
            }
            DM5CNT0 if v == DMA_ARM_CMD => self.rx_armed = true,
            DM5DA if size == 2 => self.dm5da = v as u16,
            DM5DA => self.dm5da = (self.dm5da & 0x00ff) | ((v as u16 & 0xff) << 8),
            DM5DA_LO => self.dm5da = (self.dm5da & 0xff00) | (v as u16 & 0xff),
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_registers_are_write_zero_to_clear() {
        let mut ic2 = Ic2::new();
        ic2.set_tx_complete();
        ic2.set_dma5_complete();
        assert_eq!(ic2.read(SI23ST, 1) as u8 & TX_COMPLETE, TX_COMPLETE);
        // Ack by writing ~serviced: the serviced bit clears, siblings must NOT set.
        ic2.write(SI23ST, 1, !TX_COMPLETE as u32);
        assert_eq!(ic2.read(SI23ST, 1), 0);
        ic2.write(DM59ITST, 1, !DMA5_COMPLETE as u32);
        assert_eq!(ic2.read(DM59ITST, 1), 0);
    }

    #[test]
    fn s2txb_write_flags_a_tx_event_after_latency() {
        let mut ic2 = Ic2::new();
        assert!(!ic2.take_tx_event());
        ic2.write(S2TXB_LO, 1, 0x35);
        // TX is not instantaneous: no completion until the byte has clocked out.
        assert!(!ic2.take_tx_event(), "TX completes only after its latency elapses");
        ic2.tick(TX_LATENCY as u64);
        assert!(ic2.take_tx_event());
        assert!(!ic2.take_tx_event(), "event is one-shot");
    }

    #[test]
    fn transmitted_bytes_are_kept_for_the_companion() {
        let mut ic2 = Ic2::new();
        ic2.write(S2TXB, 2, 0x0055);
        ic2.write(S2TXB_LO, 1, 0x10);
        assert_eq!(ic2.take_tx_bytes(), [0x55, 0x10]);
        assert!(ic2.take_tx_bytes().is_empty());
    }

    #[test]
    fn dm5cnt0_arm_command_arms_rx() {
        let mut ic2 = Ic2::new();
        ic2.write(DM5CNT0, 1, DMA_ARM_CMD);
        assert!(ic2.rx_armed());
        assert!(ic2.take_rx_armed());
        assert!(!ic2.rx_armed());
    }

    #[test]
    fn queued_answer_clocks_in_over_serial_latency() {
        let mut ic2 = Ic2::new();
        ic2.queue_rx(&[0x35, 0x01, 0x00, 0x00, 0xc9]);
        // The frame is in flight, not yet available, and blocks a second queue.
        assert!(ic2.rx_pending(), "an in-flight answer counts as pending");
        assert!(ic2.take_ready_rx().is_none(), "answer isn't ready until it clocks in");
        ic2.tick((RX_BYTE_LATENCY * 5) as u64);
        let (buf, len) = ic2.take_ready_rx().expect("answer ready after its serial latency");
        assert_eq!(len, 5);
        assert_eq!(&buf[..len], &[0x35, 0x01, 0x00, 0x00, 0xc9]);
        assert!(ic2.take_ready_rx().is_none(), "delivery is one-shot");
    }

    #[test]
    fn dm5da_sets_where_answers_land() {
        let mut ic2 = Ic2::new();
        ic2.write(DM5DA, 2, 0x824e);
        assert_eq!(ic2.rx_destination(), 0x0080_824e);
        ic2.write(DM5DA_LO, 1, 0xc6);
        ic2.write(DM5DA, 1, 0x82);
        assert_eq!(ic2.rx_destination(), 0x0080_82c6);
        assert_eq!(ic2.read(DM5DA, 2), 0x82c6);
    }
}
