//! A/D converter

use super::Peripheral;

pub const AD0SIM0: u32 = 0x0080_0080;
const COMPLETE: u8 = 1 << 2;

pub const AD0DT_BASE: u32 = 0x0080_0090;
pub const AD0_CHANNELS: u32 = 16;
const AD0DT_STRIDE: u32 = 2;
const AD0DT_END: u32 = AD0DT_BASE + AD0_CHANNELS * AD0DT_STRIDE;
const RESULT_MASK: u16 = 0x0fff;

const CONVERT_TICKS: u32 = 2;

pub struct Adc {
    results: [u16; AD0_CHANNELS as usize],
    sim0: u8,        // AD0SIM0 image (start command; complete bit added on read)
    converting: u32, // ticks remaining until the pending conversion completes
}

impl Default for Adc {
    fn default() -> Self {
        Self::new()
    }
}

impl Adc {
    pub fn new() -> Adc {
        Adc {
            results: [0; AD0_CHANNELS as usize],
            sim0: 0,
            converting: 0,
        }
    }

    pub fn set_channel(&mut self, ch: usize, value: u16) {
        if let Some(slot) = self.results.get_mut(ch) {
            *slot = value & RESULT_MASK;
        }
    }
}

impl Peripheral for Adc {
    fn handles(&self, a: u32) -> bool {
        a == AD0SIM0
            || ((AD0DT_BASE..AD0DT_END).contains(&a) && (a - AD0DT_BASE).is_multiple_of(AD0DT_STRIDE))
    }

    fn read(&mut self, a: u32, _size: u32) -> u32 {
        if a == AD0SIM0 {
            // Complete bit set once the modeled conversion has elapsed.
            let done = if self.converting == 0 { COMPLETE } else { 0 };
            return (self.sim0 & !COMPLETE | done) as u32;
        }
        let ch = ((a - AD0DT_BASE) / AD0DT_STRIDE) as usize;
        self.results.get(ch).copied().unwrap_or(0) as u32
    }

    fn write(&mut self, a: u32, _size: u32, v: u32) {
        if a == AD0SIM0 {
            // Start-of-conversion: latch the command, clear complete, begin timing.
            self.sim0 = (v as u8) & !COMPLETE;
            self.converting = CONVERT_TICKS;
        }
        // AD0DTx are read-only results.
    }

    fn tick(&mut self, cycles: u64) {
        self.converting = self.converting.saturating_sub(cycles as u32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conversion_completes_after_start() {
        let mut adc = Adc::new();
        adc.write(AD0SIM0, 1, 0x09); // start
        let complete = COMPLETE as u32;
        assert_eq!(adc.read(AD0SIM0, 1) & complete, 0, "not complete immediately");
        adc.tick(CONVERT_TICKS as u64);
        assert_eq!(adc.read(AD0SIM0, 1) & complete, complete, "complete after latency");
    }

    #[test]
    fn data_registers_read_channel_results() {
        let mut adc = Adc::new();
        adc.set_channel(5, 0x02c0);
        assert_eq!(adc.read(AD0DT_BASE + 5 * 2, 2), 0x02c0);
        assert_eq!(adc.read(AD0DT_BASE, 2), 0); // default
    }
}
