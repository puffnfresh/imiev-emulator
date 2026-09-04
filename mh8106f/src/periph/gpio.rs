//! General-purpose I/O ports.

use super::Peripheral;

pub const PORT_BASE: u32 = 0x0080_0700;
const NUM_PORTS: usize = 0x18;

pub struct Gpio {
    out: [u8; NUM_PORTS],     // firmware output latch, per port
    in_mask: [u8; NUM_PORTS], // bits an external part drives (1 = input)
    in_val: [u8; NUM_PORTS],  // the level those input bits present
}

impl Default for Gpio {
    fn default() -> Self {
        Self::new()
    }
}

impl Gpio {
    pub fn new() -> Gpio {
        Gpio {
            out: [0; NUM_PORTS],
            in_mask: [0; NUM_PORTS],
            in_val: [0; NUM_PORTS],
        }
    }

    #[inline]
    fn index(addr: u32) -> Option<usize> {
        let i = addr.checked_sub(PORT_BASE)? as usize;
        (i < NUM_PORTS).then_some(i)
    }

    pub fn set_input(&mut self, addr: u32, mask: u8, value: u8) {
        if let Some(i) = Self::index(addr) {
            self.in_mask[i] = mask;
            self.in_val[i] = value & mask;
        }
    }

    pub fn output(&self, addr: u32) -> u8 {
        Self::index(addr).map_or(0, |i| self.out[i])
    }

    pub fn pin_level(&self, addr: u32) -> u8 {
        Self::index(addr).map_or(0, |i| self.level(i))
    }

    fn level(&self, i: usize) -> u8 {
        (self.out[i] & !self.in_mask[i]) | (self.in_val[i] & self.in_mask[i])
    }
}

impl Peripheral for Gpio {
    fn handles(&self, a: u32) -> bool {
        Self::index(a).is_some()
    }

    fn read(&mut self, a: u32, size: u32) -> u32 {
        let mut v = 0u32;
        for k in 0..size {
            let byte = Self::index(a + k).map_or(0, |i| self.level(i));
            v = (v << 8) | byte as u32;
        }
        v
    }

    fn write(&mut self, a: u32, size: u32, val: u32) {
        for k in 0..size {
            if let Some(i) = Self::index(a + k) {
                let shift = 8 * (size - 1 - k);
                self.out[i] = (val >> shift) as u8;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const P7DATA: u32 = 0x0080_0707;
    const P9DATA: u32 = 0x0080_0709;

    #[test]
    fn output_latch_reads_back() {
        let mut g = Gpio::new();
        g.write(P7DATA, 1, 0x90);
        assert_eq!(g.read(P7DATA, 1), 0x90);
        assert_eq!(g.output(P7DATA), 0x90);
    }

    #[test]
    fn driven_input_bits_override_the_latch() {
        let mut g = Gpio::new();
        // Firmware sets bit4 as an output; the bench drives bit1 as an input.
        g.write(P9DATA, 1, 0x10);
        g.set_input(P9DATA, 0x02, 0x02);
        // bit1 reads the driven level, bit4 reads the firmware latch.
        assert_eq!(g.read(P9DATA, 1), 0x12);
        // A firmware write to an input bit is masked out on read.
        g.write(P9DATA, 1, 0x10);
        assert_eq!(g.read(P9DATA, 1) & 0x02, 0x02);
    }
}
