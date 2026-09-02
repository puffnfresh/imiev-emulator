//! Multi-Junction-Timer (MJT) which drives the RTOS tick.
//!
//! `timer_top0_start` (BMU `0x15f5c`) writes the period to `TOP0CT`/`TOP0RL`,
//! then sets `TOPCEN` bit0. Only then does the down-counter run; when it
//! underflows it reloads from `TOP0RL` and raises the tick interrupt
//! (IVECT `0x00BC`), whose ISR advances the scheduler.
//!
//! Register addresses were decoded from the firmware bytes of `timer_top0_start`:
//!
//! ```text
//!   e2 80 02 40   LD24 R2,#0x800240   ; R2 = TOP0CT
//!   20 22         STH  R0,@R2         ; TOP0CT = period
//!   42 02         ADDI R2,#2          ; R2 = 0x800242
//!   20 22         STH  R0,@R2         ; TOP0RL = period
//!   e4 80 02 fe   LD24 R4,#0x8002fe   ; R4 = TOPCEN
//!   ... TOPCEN = (TOPCEN & 0xfffe) | 1 ; enable bit0
//! ```
//!
//! Prescaler: `PRS0`/`PRS1` (8-bit) with select bit `TOPPRO` bit0. The counter
//! decrements once every `(PRS + 1)` CPU cycles, so the tick period in cycles is
//! `TOP0RL * (PRS + 1)`.

use super::Peripheral;

pub const TOP0CT: u32 = 0x0080_0240; // live down-counter (read gives current count)
pub const TOP0RL: u32 = 0x0080_0242; // reload value
pub const PRS0: u32 = 0x0080_0202; // prescaler 0
pub const PRS1: u32 = 0x0080_0203; // prescaler 1
pub const TOPPRO: u32 = 0x0080_02fc; // prescaler select (bit0: 0=PRS0, 1=PRS1)
pub const TOPCEN: u32 = 0x0080_02fe; // count enable (bit0)

/// A group of MJT counter registers of one sub-unit: channel `ch`'s counter is at
/// `base + ch*stride` (a single-instance counter is `channels: 1`, with `stride`
/// then unused beyond the base match).
struct CounterBank {
    base: u32,
    stride: u32,
    channels: u32,
}

impl CounterBank {
    fn holds(&self, a: u32) -> bool {
        a >= self.base
            && (a - self.base).is_multiple_of(self.stride)
            && (a - self.base) / self.stride < self.channels
    }
}

/// The MJT counter registers, taken from the M32192 hardware manual.. The MJT is
/// a 55-channel timer whose sub-units (TOP/TIO/TMS/TML/TID/TOU) each expose
/// counter registers at `base + ch*stride`. We approximate every channel as the
/// shared counter which is indistinguishable to code that only measures elapsed
/// differences. TOP0 (bank 0, channel 0) is the periodic down-counter modeled
/// explicitly above and is handled before this.
const MJT_COUNTER_BANKS: [CounterBank; 8] = [
    CounterBank { base: 0x0080_0240, stride: 0x10, channels: 11 }, // TOP0..TOP10
    CounterBank { base: 0x0080_0300, stride: 0x10, channels: 10 }, // TIO0..TIO9
    CounterBank { base: 0x0080_03c0, stride: 0x10, channels: 2 },  // TMS0..TMS1
    CounterBank { base: 0x0080_03e0, stride: 0x10, channels: 1 },  // TML0 (32-bit, upper half)
    CounterBank { base: 0x0080_078c, stride: 0x400, channels: 2 }, // TID0, TID1
    CounterBank { base: 0x0080_0790, stride: 0x08, channels: 8 },  // TOU0_0..TOU0_7 (upper)
    CounterBank { base: 0x0080_0b90, stride: 0x08, channels: 8 },  // TOU1_0..TOU1_7 (upper)
    CounterBank { base: 0x0080_0fe0, stride: 0x10, channels: 1 },  // TML1 (32-bit, upper half)
];

fn is_freerun_counter(a: u32) -> bool {
    a != TOP0CT && MJT_COUNTER_BANKS.iter().any(|bank| bank.holds(a))
}

pub const TICK_IVECT: u16 = 0x00bc;

const BIT0: u32 = 1 << 0;

pub struct Timer {
    count: u16,  // TOP0CT: current down-count
    reload: u16, // TOP0RL
    prs0: u8,
    prs1: u8,
    prescale_sel: bool, // TOPPRO bit0
    enabled: bool,      // TOPCEN bit0
    prescale_accum: u32,
    /// Elapsed CPU cycles.
    cycles: u64,
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

impl Timer {
    pub fn new() -> Timer {
        Timer {
            count: 0,
            reload: 0,
            prs0: 0,
            prs1: 0,
            prescale_sel: false,
            enabled: false,
            prescale_accum: 0,
            cycles: 0,
        }
    }

    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    fn prescale_div(&self) -> u32 {
        let prs = if self.prescale_sel { self.prs1 } else { self.prs0 };
        prs as u32 + 1
    }

    pub fn advance(&mut self, cycles: u64) -> Option<u16> {
        self.cycles = self.cycles.wrapping_add(cycles);
        if !self.enabled {
            return None;
        }
        let div = self.prescale_div();
        self.prescale_accum += cycles as u32;
        let mut fired = None;
        while self.prescale_accum >= div {
            self.prescale_accum -= div;
            if self.count == 0 {
                self.count = self.reload;
                fired = Some(TICK_IVECT);
            } else {
                self.count -= 1;
            }
        }
        fired
    }
}

impl Peripheral for Timer {
    fn handles(&self, a: u32) -> bool {
        matches!(a, TOP0CT | TOP0RL | PRS0 | PRS1 | TOPPRO | TOPCEN) || is_freerun_counter(a)
    }

    fn read(&mut self, a: u32, size: u32) -> u32 {
        match a {
            TOP0CT => self.count as u32,
            TOP0RL => self.reload as u32,
            PRS0 => self.prs0 as u32,
            PRS1 => self.prs1 as u32,
            TOPPRO => self.prescale_sel as u32,
            TOPCEN => self.enabled as u32,
            _ if is_freerun_counter(a) => {
                // Monotonic free-running count, masked to the access width.
                self.cycles as u32 & super::width_mask(size)
            }
            _ => 0,
        }
    }

    fn write(&mut self, a: u32, _size: u32, v: u32) {
        match a {
            TOP0CT => self.count = v as u16,
            TOP0RL => self.reload = v as u16,
            PRS0 => self.prs0 = v as u8,
            PRS1 => self.prs1 = v as u8,
            TOPPRO => self.prescale_sel = (v & BIT0) != 0,
            TOPCEN => {
                let en = (v & BIT0) != 0;
                if en && !self.enabled {
                    self.prescale_accum = 0; // fresh arm
                }
                self.enabled = en;
            }
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn disabled_timer_never_fires() {
        let mut t = Timer::new();
        t.write(TOP0RL, 2, 10);
        t.write(TOP0CT, 2, 10);
        // TOPCEN not set -> no ticks regardless of elapsed cycles.
        for _ in 0..1000 {
            assert_eq!(t.advance(1), None);
        }
    }

    #[test]
    fn fires_at_programmed_period() {
        let mut t = Timer::new();
        // Program like timer_top0_start: PRS0=39 (div 40), period 5.
        t.write(PRS0, 1, 39);
        t.write(TOPPRO, 1, 0);
        t.write(TOP0RL, 2, 5);
        t.write(TOP0CT, 2, 5);
        t.write(TOPCEN, 2, 1); // arm

        let mut fires = 0u32;
        let mut first_fire_at = None;
        for c in 1..=(6 * 40) {
            if t.advance(1) == Some(TICK_IVECT) {
                fires += 1;
                first_fire_at.get_or_insert(c);
            }
        }
        assert_eq!(fires, 1, "exactly one tick in the first period");
        assert_eq!(first_fire_at, Some(6 * 40));
    }

    #[test]
    fn freerun_counter_advances_with_time() {
        let mut t = Timer::new();
        // A sibling channel reads monotonic elapsed cycles regardless of TOPCEN.
        assert_eq!(t.read(0x800250, 4), 0);
        t.advance(1234);
        assert_eq!(t.read(0x800250, 4), 1234);
    }

    #[test]
    fn top0ct_reads_back_live_count() {
        let mut t = Timer::new();
        t.write(PRS0, 1, 0); // div 1: one decrement per cycle
        t.write(TOP0RL, 2, 100);
        t.write(TOP0CT, 2, 100);
        t.write(TOPCEN, 2, 1);
        t.advance(10);
        assert_eq!(t.read(TOP0CT, 2), 90);
    }
}
