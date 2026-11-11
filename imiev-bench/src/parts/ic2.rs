//! IC2: the coprocessor watchdog of the EV-ECU

use mh8106f::System;

use crate::{CanBus, Part};

const SYNC: u8 = 0x55;
const TRAILER: u8 = 0xaa;
const FAMILY_10: u8 = 0x10;
const FAMILY_30: u8 = 0x30;
const STATUS_ID: u8 = 0x1f;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Question {
    family: u8,
    id: u8,
    data: u8,
}

#[derive(Default)]
struct QuestionReader {
    buf: Vec<u8>,
}

impl QuestionReader {
    fn push(&mut self, b: u8) -> Option<Question> {
        if self.buf.is_empty() && b != SYNC {
            return None;
        }
        self.buf.push(b);
        let (body, len) = match self.buf.get(1) {
            None => return None,
            Some(&FAMILY_10) => (3, 6),
            Some(&FAMILY_30) => (2, 5),
            Some(_) => {
                self.buf.clear();
                return None;
            }
        };
        if self.buf.len() < len {
            return None;
        }
        let frame = core::mem::take(&mut self.buf);
        let body_bytes = &frame[1..1 + body];
        if frame[1 + body] != TRAILER || frame[2 + body] != checksum(body_bytes) {
            return None;
        }
        Some(Question { family: frame[1], id: frame[2], data: if body == 3 { frame[3] } else { 0 } })
    }
}

fn checksum(bytes: &[u8]) -> u8 {
    let mut sum = 0u32;
    for &b in bytes {
        sum += b as u32;
        sum = (sum & 0xff) + (sum >> 8);
    }
    !sum as u8
}

#[derive(Default)]
pub struct Ic2Companion {
    reader: QuestionReader,
    question: Option<Question>,
    started: bool,
    arm_ctr: u32,
    counter: u8,
    registers: [u8; 0x20],
    counting: bool,
}

impl Ic2Companion {
    fn reply(&mut self, q: Question) -> [u8; 5] {
        let reg = &mut self.registers[q.id as usize % 0x20];
        if q.family == FAMILY_10 && q.data != q.id {
            *reg = q.data;
        }
        let value = *reg;
        let payload = if q.id == STATUS_ID {
            [0x11, 0x00, 0x00, 0x00]
        } else if q.family == FAMILY_30 {
            [0x25, q.id, TRAILER, TRAILER]
        } else {
            [0x35, q.id, value, TRAILER]
        };
        Self::framed(payload)
    }

    fn framed(payload: [u8; 4]) -> [u8; 5] {
        [payload[0], payload[1], payload[2], payload[3], checksum(&payload)]
    }
}

impl Part for Ic2Companion {
    fn update(&mut self, chip: &mut System, _bus: &CanBus) {
        self.counting = false;
        for b in chip.ic2_take_tx() {
            if let Some(q) = self.reader.push(b) {
                self.question = Some(q);
            }
        }
        if chip.ic2_rx_pending() {
            return;
        }
        if !chip.ic2_rx_armed() {
            return;
        }
        if self.started {
            self.counting = true;
            self.arm_ctr = self.arm_ctr.wrapping_add(1);
            if self.arm_ctr % 64 == 32 {
                chip.ic2_take_rx_armed();
                chip.ic2_answer(&Self::framed([0x11, 0x00, 0x00, 0x00]));
                return;
            }
            if self.arm_ctr.is_multiple_of(64) {
                chip.ic2_take_rx_armed();
                chip.ic2_answer(&Self::framed([0x33, 0x00, self.counter, TRAILER]));
                self.counter = self.counter.wrapping_add(3);
                return;
            }
        }
        if let Some(q) = self.question.take() {
            chip.ic2_take_rx_armed();
            let frame = self.reply(q);
            chip.ic2_answer(&frame);
            if q.family == FAMILY_30 {
                self.started = true;
            }
        }
    }
    fn polling(&self) -> bool {
        self.counting
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
        let mut status = false;
        for _ in 0..1_600 {
            for _ in 0..10_000 {
                ecu.step();
                ic2.update(&mut ecu, &bus);
            }
            status |= ecu.take_can0_tx().iter().any(|f| f.id == 0x285);
            if status {
                break;
            }
        }
        assert!(status, "the EV-ECU never finished POST: no status on 0x285");
    }

    fn read_all(bytes: &[u8]) -> Vec<Question> {
        let mut r = QuestionReader::default();
        bytes.iter().filter_map(|&b| r.push(b)).collect()
    }

    #[test]
    fn questions_decode_from_the_wire() {
        let wire = [0x55, 0x10, 0x01, 0x21, 0xaa, 0xcd, 0x55, 0x30, 0x0b, 0xaa, 0xc4];
        assert_eq!(
            read_all(&wire),
            [
                Question { family: FAMILY_10, id: 0x01, data: 0x21 },
                Question { family: FAMILY_30, id: 0x0b, data: 0 },
            ]
        );
        let wire = [0x55, 0x10, 0x01, 0x21, 0xaa, 0xce, 0x55, 0x30, 0x0b, 0xaa, 0xc4];
        assert_eq!(read_all(&wire), [Question { family: FAMILY_30, id: 0x0b, data: 0 }]);
    }

    #[test]
    fn a_question_with_its_own_id_reads_the_register_back() {
        let mut ic2 = Ic2Companion::default();
        let set = ic2.reply(Question { family: FAMILY_10, id: 0x1e, data: 0xc0 });
        assert_eq!(set[..4], [0x35, 0x1e, 0xc0, 0xaa]);
        let read = ic2.reply(Question { family: FAMILY_10, id: 0x1e, data: 0x1e });
        assert_eq!(read[..4], [0x35, 0x1e, 0xc0, 0xaa]);
    }
}
