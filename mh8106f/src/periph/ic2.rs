//! IC2 companion-chip serial link
//!
//! The EV-ECU talks to its 20-pin "System LSI" safety companion (IC2) over this
//! link as a question/answer watchdog — POST does not complete until IC2 answers.

use super::Peripheral;

pub const SI23ST: u32 = 0x0080_0100;
pub const S2TXB: u32 = 0x0080_0132; // 16-bit; the data byte is the low half (0x800133)
pub const S2TXB_LO: u32 = 0x0080_0133;
pub const S2RCNT: u32 = 0x0080_0136;
pub const DM59ITST: u32 = 0x0080_0408;
pub const DM5CNT0: u32 = 0x0080_0418;

pub const TX_COMPLETE: u8 = 0x08; // SI23ST bit3
pub const DMA5_COMPLETE: u8 = 0x01; // DM59ITST bit0
const DMA_ARM_CMD: u32 = 0x6d;
const RX_ERROR: u8 = 0x80; // S2RCNT bit7

#[derive(Default)]
pub struct Ic2 {
    si23st: u8,
    dm59itst: u8,
    s2rcnt: u8,
    tx_event: bool, // firmware clocked a byte out of S2TXB since last poll
    rx_armed: bool, // firmware armed DMA5 to receive the next answer
}

impl Ic2 {
    pub fn new() -> Ic2 {
        Ic2::default()
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

    pub fn set_tx_complete(&mut self) {
        self.si23st |= TX_COMPLETE;
    }

    pub fn set_dma5_complete(&mut self) {
        self.dm59itst |= DMA5_COMPLETE;
    }
}

impl Peripheral for Ic2 {
    fn handles(&self, a: u32) -> bool {
        matches!(a, SI23ST | S2TXB | S2TXB_LO | S2RCNT | DM59ITST | DM5CNT0)
    }

    fn read(&mut self, a: u32, _size: u32) -> u32 {
        match a {
            SI23ST => self.si23st as u32,
            DM59ITST => self.dm59itst as u32,
            S2RCNT => self.s2rcnt as u32,
            _ => 0,
        }
    }

    fn write(&mut self, a: u32, _size: u32, v: u32) {
        match a {
            SI23ST => self.si23st &= v as u8,
            DM59ITST => self.dm59itst &= v as u8,
            S2RCNT => self.s2rcnt = (v as u8) & !RX_ERROR, // never leave RX-error latched
            S2TXB | S2TXB_LO => self.tx_event = true, // a question byte was clocked out
            DM5CNT0 => {
                if v == DMA_ARM_CMD {
                    self.rx_armed = true;
                }
            }
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
    fn s2txb_write_flags_a_tx_event() {
        let mut ic2 = Ic2::new();
        assert!(!ic2.take_tx_event());
        ic2.write(S2TXB_LO, 1, 0x35);
        assert!(ic2.take_tx_event());
        assert!(!ic2.take_tx_event(), "event is one-shot");
    }

    #[test]
    fn dm5cnt0_arm_command_arms_rx() {
        let mut ic2 = Ic2::new();
        ic2.write(DM5CNT0, 1, DMA_ARM_CMD);
        assert!(ic2.rx_armed());
        assert!(ic2.take_rx_armed());
        assert!(!ic2.rx_armed());
    }
}
