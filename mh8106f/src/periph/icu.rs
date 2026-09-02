//! Interrupt Control Unit.
//!
//! Present the source IVECT in the ICU vector register at `0x800000`.
//!
//! Currently only single-level (one pending request latched at a time).

use super::Peripheral;

pub const IVECT: u32 = 0x0080_0000; // ICU vector register (16-bit, read by dispatcher)
pub const ISLEVEL: u32 = 0x0080_0004; // in-service byte saved/restored by the prologue

pub const EI_VECTOR: u32 = 0x0000_0080;

pub struct Icu {
    ivect: u16,           // presented at 0x800000
    islevel: u8,          // 0x800004 scratch
    pending: Option<u16>, // latched request awaiting delivery
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
            islevel: 0,
            pending: None,
        }
    }

    pub fn raise(&mut self, ivect: u16) {
        if self.pending.is_none() {
            self.pending = Some(ivect);
        }
    }

    pub fn pending(&self) -> Option<u16> {
        self.pending
    }

    pub fn deliver(&mut self) -> Option<u16> {
        let iv = self.pending.take()?;
        self.ivect = iv;
        Some(iv)
    }
}

impl Peripheral for Icu {
    fn handles(&self, a: u32) -> bool {
        matches!(a, IVECT | ISLEVEL)
    }

    fn read(&mut self, a: u32, _size: u32) -> u32 {
        match a {
            IVECT => self.ivect as u32,
            ISLEVEL => self.islevel as u32,
            _ => 0,
        }
    }

    fn write(&mut self, a: u32, _size: u32, v: u32) {
        match a {
            IVECT => self.ivect = v as u16,
            ISLEVEL => self.islevel = v as u8,
            _ => {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn latches_and_delivers_one_request() {
        let mut icu = Icu::new();
        assert_eq!(icu.pending(), None);
        icu.raise(0x00bc);
        assert_eq!(icu.pending(), Some(0x00bc));
        // A second request while one is pending is dropped.
        icu.raise(0x00b0);
        assert_eq!(icu.pending(), Some(0x00bc));

        assert_eq!(icu.deliver(), Some(0x00bc));
        assert_eq!(icu.pending(), None);
        // The vector register now presents the delivered IVECT to the dispatcher.
        assert_eq!(icu.read(IVECT, 2), 0x00bc);
        assert_eq!(icu.deliver(), None);
    }
}
