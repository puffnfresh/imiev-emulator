//! Cell-monitoring units (CMUs)

use mh8106f::System;

use crate::{CanBus, Part};

const CMU_BOARDS: u16 = 12; // twelve CMU boards in the i-MiEV pack
const CELLS_PER_BOARD: [u16; CMU_BOARDS as usize] = [8, 8, 8, 8, 8, 4, 8, 8, 8, 8, 8, 4];
const CMU_RX_SLOT: u32 = 30; // BMU pack-bus mailbox the CMU poll reprograms
const CELL_V_OFFSET_MV: i32 = 2100; // report raw = (cell mV - 2100) / 5 <=> (V-2.1)*200
const CELL_V_STEP_MV: i32 = 5;
const TEMP_C_BIAS: i16 = 50; // report byte = tempC + 50

pub struct Cmu {
    pub cell_mv: u16,
    pub temp_c: i8,
    next: usize,
}

impl Cmu {
    /// A pack of uniform cells.
    pub fn new(cell_mv: u16, temp_c: i8) -> Cmu {
        Cmu { cell_mv, temp_c, next: 0 }
    }

    fn report(&self) -> (u16, [u8; 8]) {
        let total: usize = CELLS_PER_BOARD.iter().map(|&c| (c / 2) as usize).sum();
        let mut i = self.next % total;
        let mut board = 0u16;
        while i >= (CELLS_PER_BOARD[board as usize] / 2) as usize {
            i -= (CELLS_PER_BOARD[board as usize] / 2) as usize;
            board += 1;
        }
        let sid = 0x600 | ((board + 1) << 4) | (i as u16 + 1);
        let raw = ((self.cell_mv as i32 - CELL_V_OFFSET_MV) / CELL_V_STEP_MV).clamp(0, 0xffff) as u16;
        let [vh, vl] = raw.to_be_bytes();
        let temp = (self.temp_c as i16 + TEMP_C_BIAS) as u8;
        (sid, [0, 0, temp, 0, vh, vl, vh, vl])
    }
}

impl Default for Cmu {
    /// A healthy pack.
    fn default() -> Self {
        Cmu::new(3700, 25)
    }
}

impl Part for Cmu {
    fn update(&mut self, chip: &mut System, _bus: &CanBus) {
        let (sid, data) = self.report();
        chip.inject_can1(CMU_RX_SLOT, sid, &data);
        self.next += 1;
    }
}
