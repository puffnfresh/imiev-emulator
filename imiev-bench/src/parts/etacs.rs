//! Electric Time Alarm Control System

use crate::{frame, BusSource, CanBus, CanFrame, BUS_PUMP_INTERVAL, CYCLES_PER_SECOND};

const ECU_SEED_ID: u16 = 0x100; // b0:b1 seed A, b2:b3 seed B = seed A * 5 + 0x77
const ECU_IMMO_STATE_ID: u16 = 0x101; // b0: 04 once the key is accepted
const SEED_REQUEST_ID: u16 = 0x110;
const KEY_ID: u16 = 0x111;
const KEY_CODE_ID: u16 = 0x591;
const VIN_ID: u16 = 0x29a;
const VIN_PART_PUMPS: u64 = CYCLES_PER_SECOND / BUS_PUMP_INTERVAL / 10;

const IMMO_UNLOCKED: u8 = 0x04;
const KEY_SLOT: u8 = 0;
const BENCH_KEY_CODE: [u8; 3] = [0x5a, 0xa5, 0x01];
const RETRY_PUMPS: u64 = CYCLES_PER_SECOND / BUS_PUMP_INTERVAL / 2;

pub fn immobiliser_key(seed_a: u16, seed_b: u16, vin: &[u8; 17]) -> [u8; 3] {
    let (mut a, mut b, mut c) = (seed_b & 0xff, seed_b >> 8, seed_a & 0xff);
    for v in [vin[16], vin[15], vin[13], vin[8]] {
        let v = v as u16;
        a = v ^ a.wrapping_add(v);
        b = b.wrapping_add(a);
        c = v ^ c.wrapping_add(b);
    }
    [c as u8, b as u8, a as u8]
}

pub struct Etacs {
    vin: [u8; 17],
    registered: bool,
    since_request: Option<u64>,
    answered: Option<[u8; 4]>,
    pumps: u64,
}

impl Etacs {
    pub fn new(vin: &[u8; 17]) -> Self {
        Etacs { vin: *vin, registered: false, since_request: None, answered: None, pumps: 0 }
    }

    fn vin_part(&self, part: usize) -> CanFrame {
        let mut data = [0xff; 8];
        data[0] = part as u8;
        let chars = &self.vin[7 * part..(7 * part + 7).min(17)];
        data[1..1 + chars.len()].copy_from_slice(chars);
        frame(VIN_ID, &data)
    }
}

impl BusSource for Etacs {
    fn frames(&mut self, bus: &CanBus) -> Vec<CanFrame> {
        let mut out = Vec::new();
        if self.pumps.is_multiple_of(VIN_PART_PUMPS) {
            out.push(self.vin_part((self.pumps / VIN_PART_PUMPS % 3) as usize));
        }
        self.pumps += 1;
        let Some(state) = bus.last(ECU_IMMO_STATE_ID).map(|f| f.data[0]) else {
            return out;
        };
        if !self.registered {
            let [c0, c1, c2] = BENCH_KEY_CODE;
            out.push(frame(KEY_CODE_ID, &[KEY_SLOT, c0, c1, c2]));
            self.registered = true;
        }
        if state != IMMO_UNLOCKED {
            let due = self.since_request.is_none_or(|n| n >= RETRY_PUMPS);
            if due {
                out.push(frame(SEED_REQUEST_ID, &[0x00]));
                self.since_request = Some(0);
            }
        }
        if let Some(n) = self.since_request.as_mut() {
            *n += 1;
        }
        if let Some(seed) = bus.last(ECU_SEED_ID) {
            let s = [seed.data[0], seed.data[1], seed.data[2], seed.data[3]];
            if self.answered != Some(s) {
                let key = immobiliser_key(u16::from_be_bytes([s[0], s[1]]), u16::from_be_bytes([s[2], s[3]]), &self.vin);
                out.push(frame(KEY_ID, &key));
                self.answered = Some(s);
            }
        }
        out
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_b_follows_from_seed_a() {
        assert_eq!(0xc1feu16.wrapping_mul(5).wrapping_add(0x77), 0xca6d);
    }

    #[test]
    fn key_mixes_the_9th_14th_16th_and_17th_characters() {
        let vin = *b"ABCDEFGHJKLMNPRST";
        let mut other = vin;
        for i in [0, 1, 2, 3, 4, 5, 6, 7, 9, 10, 11, 12, 14] {
            other[i] = b'0';
        }
        assert_eq!(immobiliser_key(0x1234, 0x5b13, &vin), immobiliser_key(0x1234, 0x5b13, &other));
        other[8] = b'0';
        assert_ne!(immobiliser_key(0x1234, 0x5b13, &vin), immobiliser_key(0x1234, 0x5b13, &other));
    }
}
