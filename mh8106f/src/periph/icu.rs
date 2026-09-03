//! Interrupt Control Unit.

use super::Peripheral;

pub const IVECT: u32 = 0x0080_0000; // ICU vector register (16-bit, read by dispatcher)
pub const IMASK: u32 = 0x0080_0004; // interrupt request mask (accept threshold)

const ICR_BASE: u32 = 0x0080_0056;
const ICR_END: u32 = 0x0080_0080; // exclusive
const ICR_RESET: u8 = 0x07; // ILEVEL = 7 = disabled
const IMASK_RESET: u8 = 0x07; // accept priority levels 0..6

const ILEVEL_MASK: u8 = 0x07;
const LEVEL_DISABLED: u8 = 0x07;

pub const EI_VECTOR: u32 = 0x0000_0080;

pub struct Icu {
    ivect: u16,           // presented at 0x800000
    imask: u8,            // 0x800004 accept threshold
    icr: [u8; (ICR_END - ICR_BASE) as usize], // per-source control (ILEVEL + IREQ)
    latched: Vec<u16>,    // edge-latched pending requests (by source IVECT), unique
}

impl Default for Icu {
    fn default() -> Self {
        Self::new()
    }
}

impl Icu {
    pub fn new() -> Icu {
        Icu {
            ivect: 0,
            imask: IMASK_RESET,
            icr: [ICR_RESET; (ICR_END - ICR_BASE) as usize],
            latched: Vec::new(),
        }
    }

    fn source_icr(ivect: u16) -> Option<u32> {
        match ivect {
            0x00bc => Some(0x0080_0074), // MJT Output Interrupt 2 (TOP0 fast tick)
            0x00b0 => Some(0x0080_0077), // MJT Output Interrupt 5 (chained slow tick)
            _ => None,
        }
    }

    fn source_enabled(&self, ivect: u16) -> bool {
        match Self::source_icr(ivect) {
            Some(a) => {
                let level = self.icr[(a - ICR_BASE) as usize] & ILEVEL_MASK;
                level != LEVEL_DISABLED && level < (self.imask & ILEVEL_MASK)
            }
            None => true,
        }
    }

    pub fn raise(&mut self, ivect: u16) {
        if !self.latched.contains(&ivect) {
            self.latched.push(ivect);
        }
    }

    pub fn pending(&self) -> Option<u16> {
        self.latched
            .iter()
            .copied()
            .filter(|&iv| self.source_enabled(iv))
            .min_by_key(|&iv| {
                Self::source_icr(iv)
                    .map(|a| self.icr[(a - ICR_BASE) as usize] & ILEVEL_MASK)
                    .unwrap_or(0)
            })
    }

    pub fn deliver(&mut self) -> Option<u16> {
        let iv = self.pending()?;
        self.latched.retain(|&x| x != iv);
        self.ivect = iv;
        Some(iv)
    }

    pub fn present(&mut self, ivect: u16) {
        self.ivect = ivect;
    }

    pub fn icr_raw(&self, addr: u32) -> u8 {
        self.icr[(addr - ICR_BASE) as usize]
    }

    pub fn icr_clear(&mut self, addr: u32, mask: u8) {
        self.icr[(addr - ICR_BASE) as usize] &= !mask;
    }
}

impl Peripheral for Icu {
    fn handles(&self, a: u32) -> bool {
        a == IVECT || a == IMASK || (ICR_BASE..ICR_END).contains(&a)
    }

    fn read(&mut self, a: u32, _size: u32) -> u32 {
        match a {
            IVECT => self.ivect as u32,
            IMASK => self.imask as u32,
            _ => self.icr[(a - ICR_BASE) as usize] as u32,
        }
    }

    fn write(&mut self, a: u32, _size: u32, v: u32) {
        match a {
            IVECT => self.ivect = v as u16,
            IMASK => self.imask = v as u8,
            _ => self.icr[(a - ICR_BASE) as usize] = v as u8,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn masked_source_stays_pending_until_enabled() {
        let mut icu = Icu::new();
        // Fast-tick source (0xBC / IMJTOCR2 0x800074) starts disabled (reset 0x07).
        icu.raise(0x00bc);
        assert_eq!(icu.pending(), None, "disabled source must not deliver");
        // Firmware enables it at level 4.
        icu.write(0x0080_0074, 1, 0x04);
        assert_eq!(icu.pending(), Some(0x00bc), "enabled source now deliverable");
        assert_eq!(icu.deliver(), Some(0x00bc));
        assert_eq!(icu.pending(), None, "latch cleared on delivery");
        assert_eq!(icu.read(IVECT, 2), 0x00bc);
    }

    #[test]
    fn unmapped_source_is_not_gated() {
        let mut icu = Icu::new();
        icu.raise(0x0055); // not in the source map
        assert_eq!(icu.pending(), Some(0x0055));
    }

    #[test]
    fn higher_priority_source_wins() {
        let mut icu = Icu::new();
        icu.write(0x0080_0074, 1, 0x04); // 0xBC at level 4
        icu.write(0x0080_007f, 1, 0x02); // 0x110 at level 2 (higher priority)
        icu.raise(0x00bc);
        icu.raise(0x0110);
        assert_eq!(icu.pending(), Some(0x0110), "lower ILEVEL = higher priority");
    }
}
