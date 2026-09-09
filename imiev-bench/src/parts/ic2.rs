//! IC2: the coprocessor watchdog of the EV-ECU

use mh8106f::System;

use crate::{CanBus, Part};

const EV_ECU_DISPATCH_JL: u32 = 0x0000_39bc;
const EV_ECU_DISPATCH_JL_RET: u32 = 0x0000_39c0;
const EV_ECU_CAN0_RX_ISR: u32 = 0x0002_7d40;

const IC2_Q_MARKER: u32 = 0x0080_8249; // & 0xf0 = slot family (0x10 / 0x30)
const IC2_Q_ID: u32 = 0x0080_824a; // message id being asked
const IC2_Q_DATA: u32 = 0x0080_824b; // the data byte the question carries
const IC2_TX_SLOTS: u32 = 0x0080_81ac; // 30 entries * 4: [id, marker|state, byte2, byte3]
const IC2_STARTUP_STATE: u32 = 0x0080_825a; // 0->4->0xFFFF(POST done)
const IC2_TX_DONE_PC: u32 = 0x0001_31ac; // handler reached when a question frame is fully sent

#[derive(Default)]
pub struct Can0RxIsr {
    armed: bool,
}

impl Part for Can0RxIsr {
    fn update(&mut self, chip: &mut System, _bus: &CanBus) {
        if !self.armed && chip.peek(IC2_STARTUP_STATE, 2) == 0xffff {
            chip.configure_can0_rx_isr(EV_ECU_DISPATCH_JL, EV_ECU_DISPATCH_JL_RET, EV_ECU_CAN0_RX_ISR);
            self.armed = true;
        }
    }
}

#[derive(Default)]
pub struct Ic2Companion {
    armed_watch: bool,
    question_sent: bool,
    arm_ctr: u32,
}

impl Ic2Companion {
    fn reply(chip: &System) -> [u8; 5] {
        let marker = chip.peek(IC2_Q_MARKER, 1) as u8 & 0xf0;
        let id = chip.peek(IC2_Q_ID, 1) as u8;

        let mut slot_data = chip.peek(IC2_Q_DATA, 1) as u8;
        for s in 0..30u32 {
            let base = IC2_TX_SLOTS + s * 4;
            let state = chip.peek(base + 1, 1) as u8 & 0x0f;
            if chip.peek(base, 1) as u8 == id && (state == 3 || state == 4) {
                slot_data = chip.peek(base + 2, 1) as u8;
                break;
            }
        }

        let post_post = chip.peek(IC2_STARTUP_STATE, 1) as u8 == 0xff;
        let payload = if post_post && id == 0x01 {
            [0x00, 0x01, 0x21, 0x00] // id-1 status word (== flash constant)
        } else if id == 0x1f {
            [0x11, 0x00, 0x00, 0x00] // status ack (sets the group ack bits)
        } else if marker == 0x30 {
            [0x25, id, slot_data, 0xAA] // per-id ack for a 0x30-family slot
        } else {
            [0x35, id, slot_data, 0xAA] // per-id ack for a 0x10-family slot
        };

        Self::framed(payload)
    }

    fn framed(payload: [u8; 4]) -> [u8; 5] {
        let mut sum = 0u32;
        for &b in &payload {
            sum += b as u32;
            sum = (sum & 0xff) + (sum >> 8);
        }
        [payload[0], payload[1], payload[2], payload[3], !sum as u8]
    }
}

impl Part for Ic2Companion {
    fn update(&mut self, chip: &mut System, _bus: &CanBus) {
        if !self.armed_watch {
            chip.watch_pc(IC2_TX_DONE_PC);
            self.armed_watch = true;
        }
        if chip.take_pc_hit() {
            self.question_sent = true;
        }
        if chip.ic2_rx_pending() {
            return; // previous frame not yet consumed by DMA5
        }
        if !chip.ic2_rx_armed() {
            return;
        }
        if chip.peek(IC2_STARTUP_STATE, 2) == 0xffff {
            self.arm_ctr = self.arm_ctr.wrapping_add(1);
            if self.arm_ctr % 64 == 32 {
                chip.ic2_take_rx_armed();
                chip.ic2_answer(&Self::framed([0x11, 0x00, 0x00, 0x00]));
                return;
            }
            if self.arm_ctr.is_multiple_of(64) {
                chip.ic2_take_rx_armed();
                let ctr = chip.peek(0x0080_8293, 1) as u8;
                chip.ic2_answer(&Self::framed([0x33, 0x00, ctr, 0xAA]));
                return;
            }
        }
        if self.question_sent {
            self.question_sent = false;
            chip.ic2_take_rx_armed();
            let frame = Self::reply(chip);
            chip.ic2_answer(&frame);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::adc::EV_ECU_BOOT_ADC;
    use crate::EV_ECU_FW;

    #[test]
    fn ic2_handshake_completes_post() {
        let mut ecu = System::new(EV_ECU_FW);
        for &(ch, raw) in EV_ECU_BOOT_ADC {
            ecu.adc_mut().set_channel(ch, raw);
        }
        let mut ic2 = Ic2Companion::default();
        let bus = CanBus::default();
        for _ in 0..8_000_000u64 {
            ecu.step();
            ic2.update(&mut ecu, &bus);
            if ecu.peek(IC2_STARTUP_STATE, 2) == 0xffff {
                break;
            }
        }
        assert_eq!(
            ecu.peek(IC2_STARTUP_STATE, 2),
            0xffff,
            "IC2 handshake did not complete POST (startup_state != 0xFFFF)"
        );
    }
}
