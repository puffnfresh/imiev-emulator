//! Remote's timer charging and remote climate control

use crate::{frame, BusSource, CanBus, CanFrame, BUS_PUMP_INTERVAL, CYCLES_PER_SECOND};

pub const REMOTE_CLIMATE_ID: u16 = 0x38d;
const REMOTE_CLIMATE_PUMPS: u64 = CYCLES_PER_SECOND / BUS_PUMP_INTERVAL / 20;

const RUN: u8 = 0x01;
const MODE_SHIFT: u8 = 1;
const ALWAYS_SET: u8 = 0xc0;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ClimateMode {
    Cool = 1,
    Heat = 2,
    Defrost = 3,
}

pub struct EvRemote {
    mode: ClimateMode,
    running: bool,
    pumps: u64,
}

impl Default for EvRemote {
    fn default() -> Self {
        EvRemote { mode: ClimateMode::Heat, running: false, pumps: 0 }
    }
}

impl EvRemote {
    pub fn set_climate(&mut self, mode: Option<ClimateMode>) {
        if let Some(mode) = mode {
            self.mode = mode;
        }
        self.running = mode.is_some();
    }

    pub fn request(&self) -> [u8; 2] {
        let run = if self.running { RUN } else { 0 };
        [ALWAYS_SET | (self.mode as u8) << MODE_SHIFT | run, 0]
    }
}

impl BusSource for EvRemote {
    fn frames(&mut self, _bus: &CanBus) -> Vec<CanFrame> {
        let due = self.pumps.is_multiple_of(REMOTE_CLIMATE_PUMPS);
        self.pumps += 1;
        if !due {
            return Vec::new();
        }
        let mut data = [0; 8];
        data[..2].copy_from_slice(&self.request());
        vec![frame(REMOTE_CLIMATE_ID, &data)]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn byte0(mode: Option<ClimateMode>, then: Option<ClimateMode>) -> [u8; 2] {
        let mut remote = EvRemote::default();
        remote.set_climate(mode);
        let first = remote.request()[0];
        remote.set_climate(then);
        [first, remote.request()[0]]
    }

    #[test]
    fn matches_the_frames_that_ran_a_9499b22901_ion() {
        assert_eq!(byte0(Some(ClimateMode::Heat), None), [0xc5, 0xc4]);
        assert_eq!(byte0(Some(ClimateMode::Defrost), None), [0xc7, 0xc6]);
        assert_eq!(byte0(Some(ClimateMode::Cool), None), [0xc3, 0xc2]);
    }

    #[test]
    fn decodes_as_9499b22901_reads_it() {
        let mut remote = EvRemote::default();
        remote.set_climate(Some(ClimateMode::Defrost));
        let b = remote.request();
        assert_eq!(b[0] & 1, 1, "run");
        assert_eq!((b[0] & 0xf) >> 1 & 3, ClimateMode::Defrost as u8, "mode");
        assert_eq!(b[1] & 7, 0);
    }

    #[test]
    fn sends_an_8_byte_frame_every_50_ms() {
        let mut remote = EvRemote::default();
        let bus = CanBus::default();
        let sent: Vec<(u64, CanFrame)> =
            (0..2 * REMOTE_CLIMATE_PUMPS).flat_map(|p| remote.frames(&bus).into_iter().map(move |f| (p, f))).collect();
        assert_eq!(sent.len(), 2);
        assert_eq!(sent[1].0 - sent[0].0, CYCLES_PER_SECOND / BUS_PUMP_INTERVAL / 20);
        assert!(sent.iter().all(|(_, f)| f.id == REMOTE_CLIMATE_ID && f.dlc == 8));
    }
}
