//! Representation of CAN messages.

use embedded_can::{ExtendedId, Id, StandardId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Frame {
    id: Id,
    len: u8,
    data: [u8; 8],
}

impl Frame {
    pub fn standard(id: u16, data: &[u8]) -> Option<Frame> {
        embedded_can::Frame::new(StandardId::new(id)?, data)
    }

    pub fn from_raw_id(id: u32, data: &[u8]) -> Option<Frame> {
        match u16::try_from(id) {
            Ok(id) if id <= StandardId::MAX.as_raw() => Frame::standard(id, data),
            _ => embedded_can::Frame::new(ExtendedId::new(id)?, data),
        }
    }
}

pub fn raw_id(id: Id) -> u32 {
    match id {
        Id::Standard(id) => id.as_raw().into(),
        Id::Extended(id) => id.as_raw(),
    }
}

pub fn candump_line(frame: &impl embedded_can::Frame, time_us: u64, iface: &str) -> String {
    let id = match frame.id() {
        Id::Standard(id) => format!("{:03X}", id.as_raw()),
        Id::Extended(id) => format!("{:08X}", id.as_raw()),
    };
    let data: String = frame.data().iter().map(|b| format!("{b:02X}")).collect();
    format!("({}.{:06}) {iface} {id}#{data}", time_us / 1_000_000, time_us % 1_000_000)
}

impl embedded_can::Frame for Frame {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Frame> {
        let mut bytes = [0; 8];
        bytes.get_mut(..data.len())?.copy_from_slice(data);
        Some(Frame { id: id.into(), len: data.len() as u8, data: bytes })
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Frame> {
        None
    }

    fn is_extended(&self) -> bool {
        matches!(self.id, Id::Extended(_))
    }

    fn is_remote_frame(&self) -> bool {
        false
    }

    fn id(&self) -> Id {
        self.id
    }

    fn dlc(&self) -> usize {
        self.len as usize
    }

    fn data(&self) -> &[u8] {
        &self.data[..self.len as usize]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_can::Frame as _;

    #[test]
    fn candump_formatting() {
        let f = Frame::standard(0x373, &[0xde, 0xad]).unwrap();
        assert_eq!(candump_line(&f, 1_700_000_000_000_042, "can0"), "(1700000000.000042) can0 373#DEAD");
        let ext = Frame::from_raw_id(0x18da_f110, &[]).unwrap();
        assert_eq!(candump_line(&ext, 0, "can0"), "(0.000000) can0 18DAF110#");
        assert_eq!(raw_id(ext.id()), 0x18da_f110);
    }

    #[test]
    fn ids_and_lengths() {
        assert!(!Frame::from_raw_id(0x7ff, &[]).unwrap().is_extended());
        assert!(Frame::from_raw_id(0x800, &[]).unwrap().is_extended());
        assert_eq!(Frame::from_raw_id(0x2000_0000, &[]), None);
        assert_eq!(Frame::standard(0x800, &[]), None);
        assert_eq!(Frame::standard(0x100, &[0; 9]), None);
        assert_eq!(Frame::standard(0x100, &[1, 2, 3]).unwrap().data(), &[1, 2, 3]);
    }
}
