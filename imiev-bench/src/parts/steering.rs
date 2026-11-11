//! Steering angle sensor

use crate::{frame, BusSource, CanBus, CanFrame, BUS_PUMP_INTERVAL, CYCLES_PER_SECOND};

const SAS_ID: u16 = 0x236;
const SAS_PUMPS: u64 = CYCLES_PER_SECOND / BUS_PUMP_INTERVAL / 100;
const STRAIGHT_AHEAD: [u8; 2] = [0x10, 0x00];
const AT_REST: [u8; 2] = [0x10, 0x00];

fn crc8(bytes: &[u8]) -> u8 {
    let crc = bytes.iter().fold(0u8, |mut crc, &b| {
        crc ^= b;
        for _ in 0..8 {
            crc = if crc & 0x80 != 0 { (crc << 1) ^ 0x1d } else { crc << 1 };
        }
        crc
    });
    crc ^ 0x0a
}

#[derive(Default)]
pub struct SteeringAngleSensor {
    pumps: u64,
    counter: u8,
}

impl BusSource for SteeringAngleSensor {
    fn frames(&mut self, _bus: &CanBus) -> Vec<CanFrame> {
        let due = self.pumps.is_multiple_of(SAS_PUMPS);
        self.pumps += 1;
        if !due {
            return Vec::new();
        }
        let mut data = [STRAIGHT_AHEAD[0], STRAIGHT_AHEAD[1], AT_REST[0], AT_REST[1], self.counter << 4, 0, 0, 0];
        data[7] = crc8(&data[..7]);
        self.counter = (self.counter + 1) & 0x0f;
        vec![frame(SAS_ID, &data)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn crc_matches_logged_frames() {
        for f in [
            [0x0f, 0xdc, 0x10, 0x00, 0x00, 0x00, 0x00, 0x53],
            [0x0f, 0xdc, 0x10, 0x00, 0xb0, 0x00, 0x00, 0xbb],
            [0x0f, 0xdc, 0x0f, 0xeb, 0x20, 0x00, 0x00, 0x3c],
            [0x0f, 0xdd, 0x0f, 0xf3, 0x40, 0x00, 0x00, 0x93],
        ] {
            assert_eq!(crc8(&f[..7]), f[7], "{f:02x?}");
        }
    }
}
