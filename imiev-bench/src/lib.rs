//! Models of the i-MiEV components:
//! * CMUs
//! * Contactors
//! * Inverter
//! * Condenser
//! * Vehicle dynamics
//!
//! Communicated to a chip using various protocols.

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CanFrame {
    pub id: u16,
    pub len: u8,
    pub data: [u8; 8],
}

impl CanFrame {
    pub fn new(id: u16, data: &[u8]) -> CanFrame {
        let mut buf = [0u8; 8];
        let len = data.len().min(8);
        buf[..len].copy_from_slice(&data[..len]);
        CanFrame { id, len: len as u8, data: buf }
    }

    pub fn payload(&self) -> &[u8] {
        &self.data[..self.len as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_frame_truncates_and_reports_payload() {
        let f = CanFrame::new(0x373, &[1, 2, 3]);
        assert_eq!(f.id, 0x373);
        assert_eq!(f.len, 3);
        assert_eq!(f.payload(), &[1, 2, 3]);

        // Over-long payloads are truncated to the 8-byte CAN limit.
        let big = CanFrame::new(0x374, &[9; 12]);
        assert_eq!(big.len, 8);
        assert_eq!(big.payload(), &[9; 8]);
    }
}
