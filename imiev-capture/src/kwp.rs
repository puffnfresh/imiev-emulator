//! Keyword Protocol 2000 specific messages for the i-MiEV.

use automotive_diag::kwp2000::{KwpCommand, KwpError, KwpErrorByte, KwpSessionType};

use crate::can;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ecu {
    EvEcu,
    Bmu,
    Charger,
    Heater,
    Meter,
}

impl Ecu {
    pub const ALL: [Ecu; 5] = [Ecu::EvEcu, Ecu::Bmu, Ecu::Charger, Ecu::Heater, Ecu::Meter];

    pub fn name(self) -> &'static str {
        match self {
            Ecu::EvEcu => "ev-ecu",
            Ecu::Bmu => "bmu",
            Ecu::Charger => "charger",
            Ecu::Heater => "heater",
            Ecu::Meter => "meter",
        }
    }

    pub fn from_name(name: &str) -> Option<Ecu> {
        Ecu::ALL.into_iter().find(|e| e.name() == name)
    }

    pub fn request_id(self) -> u16 {
        match self {
            Ecu::EvEcu => 0x751,
            Ecu::Bmu => 0x761,
            Ecu::Charger => 0x765,
            Ecu::Heater => 0x771,
            Ecu::Meter => 0x782,
        }
    }

    pub fn response_id(self) -> u16 {
        self.request_id() + 1
    }

    pub fn can_clear(self) -> bool {
        matches!(self, Ecu::EvEcu | Ecu::Bmu)
    }

    pub fn can_dump(self) -> bool {
        matches!(self, Ecu::EvEcu | Ecu::Bmu)
    }
}

pub const MEMORY_SIZE: u32 = 0x10_0000;

pub const IDENTIFICATION: [(u8, &str); 7] = [
    (0x87, "ECU identification"),
    (0x88, "original VIN"),
    (0x89, "diagnostic variant code"),
    (0x90, "current VIN"),
    (0x9a, "ECU code fingerprint"),
    (0x9c, "ECU code identification"),
    (0x9e, "boot code identification"),
];

pub const EV_ECU_RECORDS: [u8; 12] = [0x01, 0x02, 0x03, 0x20, 0x21, 0x22, 0x23, 0x24, 0x40, 0xe0, 0xe1, 0xe4];

const NEGATIVE_RESPONSE: u8 = 0x7f;
const POSITIVE_RESPONSE: u8 = 0x40;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Request {
    OpenSession,
    OpenProgrammingSession,
    CloseSession,
    ReadDtcs,
    ClearDtcs,
    Identify(u8),
    ReadLocal(u8),
    ReadMemory { address: u32, size: u8 },
    FlowControl,
}

impl Request {
    fn service(self) -> Option<KwpCommand> {
        Some(match self {
            Request::OpenSession | Request::OpenProgrammingSession | Request::CloseSession => {
                KwpCommand::StartDiagnosticSession
            }
            Request::ReadDtcs => KwpCommand::ReadDiagnosticTroubleCodesByStatus,
            Request::ClearDtcs => KwpCommand::ClearDiagnosticInformation,
            Request::Identify(_) => KwpCommand::ReadECUIdentification,
            Request::ReadLocal(_) => KwpCommand::ReadDataByLocalIdentifier,
            Request::ReadMemory { .. } => KwpCommand::ReadMemoryByAddress,
            Request::FlowControl => return None,
        })
    }

    fn arguments(self) -> Vec<u8> {
        match self {
            Request::OpenSession => vec![KwpSessionType::ExtendedDiagnostics.into()],
            Request::OpenProgrammingSession => vec![KwpSessionType::Reprogramming.into()],
            Request::CloseSession => vec![KwpSessionType::Normal.into()],
            Request::ReadDtcs => vec![0x00, 0xff, 0x00],
            Request::ClearDtcs => vec![0xff, 0x00],
            Request::Identify(option) => vec![option],
            Request::ReadLocal(lid) => vec![lid],
            Request::ReadMemory { address, size } => {
                let [_, a2, a1, a0] = address.to_be_bytes();
                vec![a2, a1, a0, size]
            }
            Request::FlowControl => vec![],
        }
    }

    pub fn answer(self, pdu: &[u8]) -> Option<&[u8]> {
        let positive = u8::from(self.service()?) + POSITIVE_RESPONSE;
        match pdu.split_first() {
            Some((&sid, rest)) if sid == positive => Some(rest),
            _ => None,
        }
    }

    pub fn acknowledged(self, pdu: &[u8]) -> bool {
        self.answer(pdu) == Some(&self.arguments())
    }

    pub fn frame(self, ecu: Ecu) -> Option<RequestFrame> {
        match self {
            Request::ClearDtcs if !ecu.can_clear() => return None,
            Request::OpenProgrammingSession if !ecu.can_dump() => return None,
            Request::Identify(o) if !IDENTIFICATION.iter().any(|(i, _)| *i == o) => return None,
            Request::ReadMemory { address, size } if !ecu.can_dump() || size == 0 || address + size as u32 > MEMORY_SIZE => {
                return None;
            }
            _ => {}
        }
        let mut data = [0u8; 8];
        match self.service() {
            None => data[0] = 0x30,
            Some(service) => {
                let args = self.arguments();
                data[0] = 1 + args.len() as u8;
                data[1] = service.into();
                data[2..2 + args.len()].copy_from_slice(&args);
            }
        }
        can::Frame::standard(ecu.request_id(), &data).map(RequestFrame)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RequestFrame(can::Frame);

impl RequestFrame {
    pub fn can(self) -> can::Frame {
        self.0
    }
}

pub fn negative(pdu: &[u8]) -> Option<KwpErrorByte> {
    match pdu {
        [NEGATIVE_RESPONSE, _, code] => Some((*code).into()),
        _ => None,
    }
}

pub fn is_response_pending(pdu: &[u8]) -> bool {
    negative(pdu) == Some(KwpError::RequestCorrectlyReceivedResponsePending.into())
}

pub fn needs_session(pdu: &[u8]) -> bool {
    negative(pdu) == Some(KwpError::ServiceNotSupportedInActiveSession.into())
}

pub enum Rx {
    Incomplete,
    SendFlowControl,
    Done(Vec<u8>),
    Lost,
}

#[derive(Default)]
pub struct IsoTpRx {
    buf: Vec<u8>,
    expected: usize,
    next_sn: u8,
}

impl IsoTpRx {
    pub fn push(&mut self, frame: &[u8]) -> Rx {
        let Some(&pci) = frame.first() else { return Rx::Incomplete };
        match pci & 0xf0 {
            0x00 => {
                let len = ((pci & 0x0f) as usize).min(frame.len() - 1);
                Rx::Done(frame[1..1 + len].to_vec())
            }
            0x10 if frame.len() >= 2 => {
                self.expected = (((pci & 0x0f) as usize) << 8) | frame[1] as usize;
                self.buf = frame[2..].to_vec();
                self.next_sn = 1;
                Rx::SendFlowControl
            }
            0x20 if self.expected > 0 => {
                if pci & 0x0f != self.next_sn {
                    self.expected = 0;
                    return Rx::Lost;
                }
                self.next_sn = (self.next_sn + 1) & 0x0f;
                self.buf.extend_from_slice(&frame[1..]);
                if self.buf.len() >= self.expected {
                    self.buf.truncate(self.expected);
                    self.expected = 0;
                    Rx::Done(std::mem::take(&mut self.buf))
                } else {
                    Rx::Incomplete
                }
            }
            _ => Rx::Incomplete,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Dtc {
    pub code: u16,
    pub status: u8,
}

impl Dtc {
    pub fn label(&self) -> String {
        let cat = b"PCBU"[(self.code >> 14) as usize] as char;
        format!("{cat}{:04X}", self.code & 0x3fff)
    }
}

impl std::fmt::Display for Dtc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} (status {:02x})", self.label(), self.status)
    }
}

pub fn render(bytes: &[u8]) -> String {
    let printable = |b: &u8| (0x20..0x7f).contains(b);
    let mut parts = Vec::new();
    let mut i = 0;
    while i < bytes.len() {
        let run = bytes[i..].iter().take_while(|b| printable(b)).count();
        if run >= 4 {
            parts.push(format!("\"{}\"", String::from_utf8_lossy(&bytes[i..i + run])));
            i += run;
        } else {
            parts.push(format!("{:02x}", bytes[i]));
            i += 1;
        }
    }
    parts.join(" ")
}

pub fn parse_dtcs(pdu: &[u8]) -> Option<Vec<Dtc>> {
    let (&count, dtcs) = Request::ReadDtcs.answer(pdu)?.split_first()?;
    Some(
        dtcs.as_chunks::<3>()
            .0
            .iter()
            .take(count as usize)
            .map(|c| Dtc { code: u16::from_be_bytes([c[0], c[1]]), status: c[2] })
            .collect(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use embedded_can::Frame as _;

    #[test]
    fn request_frames() {
        let data = |r: Request| r.frame(Ecu::EvEcu).unwrap().can().data().to_vec();
        assert_eq!(data(Request::OpenSession), [0x02, 0x10, 0x92, 0, 0, 0, 0, 0]);
        assert_eq!(data(Request::OpenProgrammingSession), [0x02, 0x10, 0x85, 0, 0, 0, 0, 0]);
        assert_eq!(data(Request::CloseSession), [0x02, 0x10, 0x81, 0, 0, 0, 0, 0]);
        assert_eq!(Request::OpenProgrammingSession.frame(Ecu::Meter), None);
        assert_eq!(data(Request::ReadDtcs), [0x04, 0x18, 0x00, 0xff, 0x00, 0, 0, 0]);
        assert_eq!(data(Request::ClearDtcs), [0x03, 0x14, 0xff, 0x00, 0, 0, 0, 0]);
        assert_eq!(data(Request::Identify(0x9c)), [0x02, 0x1a, 0x9c, 0, 0, 0, 0, 0]);
        assert_eq!(data(Request::ReadLocal(0x20)), [0x02, 0x21, 0x20, 0, 0, 0, 0, 0]);
        assert_eq!(data(Request::FlowControl), [0x30, 0, 0, 0, 0, 0, 0, 0]);
        assert_eq!(can::raw_id(Request::ReadDtcs.frame(Ecu::EvEcu).unwrap().can().id()), 0x751);
        assert_eq!(can::raw_id(Request::ReadDtcs.frame(Ecu::Bmu).unwrap().can().id()), 0x761);
        assert_eq!(Ecu::Meter.response_id(), 0x783);
    }

    #[test]
    fn memory_reads_stay_inside_the_flash() {
        let read = |address, size| Request::ReadMemory { address, size };
        assert_eq!(read(0x0e4ae2, 0xff).frame(Ecu::EvEcu).unwrap().can().data(), &[0x05, 0x23, 0x0e, 0x4a, 0xe2, 0xff, 0, 0]);
        assert!(read(0x0fff02, 0xff).frame(Ecu::Bmu).is_none());
        assert!(read(0x0ffff0, 0x10).frame(Ecu::Bmu).is_some());
        assert!(read(0, 0).frame(Ecu::EvEcu).is_none());
        assert!(read(0, 0x10).frame(Ecu::Charger).is_none());
    }

    #[test]
    fn detects_a_lost_consecutive_frame() {
        let mut rx = IsoTpRx::default();
        assert!(matches!(rx.push(&[0x10, 0x14, 0x63, 1, 2, 3, 4, 5]), Rx::SendFlowControl));
        assert!(matches!(rx.push(&[0x21, 6, 7, 8, 9, 10, 11, 12]), Rx::Incomplete));
        assert!(matches!(rx.push(&[0x23, 0, 0, 0, 0, 0, 0, 0]), Rx::Lost));
    }

    #[test]
    fn refuses_what_was_not_verified() {
        assert!(Request::ClearDtcs.frame(Ecu::Charger).is_none());
        assert!(Request::ClearDtcs.frame(Ecu::Bmu).is_some());
        assert!(Request::Identify(0x91).frame(Ecu::EvEcu).is_none());
    }

    #[test]
    fn ecu_names_round_trip() {
        assert!(Ecu::ALL.iter().all(|&e| Ecu::from_name(e.name()) == Some(e)));
    }

    #[test]
    fn renders_identification() {
        let mut b = vec![0x01, 0x04, 0x85, 0x00, 0x04, 0xff, 0x80, 0x14, 0x07];
        b.extend_from_slice(b"9499A43902");
        assert_eq!(render(&b), "01 04 85 00 04 ff 80 14 07 \"9499A43902\"");
        assert_eq!(render(&[0xd7, 0x40, 0x00, 0x00]), "d7 40 00 00");
    }

    #[test]
    fn dtc_labels() {
        assert_eq!(Dtc { code: 0x1a15, status: 0 }.label(), "P1A15");
        assert_eq!(Dtc { code: 0xd802, status: 0 }.label(), "U1802");
        assert_eq!(Dtc { code: 0x600a, status: 0 }.label(), "C200A");
    }

    #[test]
    fn parses_dtc_answers() {
        let dtcs = parse_dtcs(&[0x58, 0x01, 0xd8, 0x02, 0x20]).unwrap();
        assert_eq!(dtcs.len(), 1);
        assert_eq!(dtcs[0].to_string(), "U1802 (status 20)");
        assert_eq!(parse_dtcs(&[0x58, 0x00]).unwrap().len(), 0);
        assert!(parse_dtcs(&[0x7f, 0x18, 0x80]).is_none());
    }

    #[test]
    fn reassembles_multi_frame_answer() {
        let mut rx = IsoTpRx::default();
        assert!(matches!(rx.push(&[0x10, 0x0a, 0x61, 0x20, 1, 2, 3, 4]), Rx::SendFlowControl));
        match rx.push(&[0x21, 5, 6, 7, 8, 0xaa, 0xaa, 0xaa]) {
            Rx::Done(pdu) => assert_eq!(pdu, [0x61, 0x20, 1, 2, 3, 4, 5, 6, 7, 8]),
            _ => panic!("second frame completes the answer"),
        }
    }

    #[test]
    fn negative_responses() {
        assert!(needs_session(&[0x7f, 0x14, 0x80]));
        assert!(is_response_pending(&[0x7f, 0x21, 0x78]));
        assert_eq!(negative(&[0x7f, 0x23, 0x11]), Some(KwpError::ServiceNotSupported.into()));
        assert_eq!(negative(&[0x54, 0xff, 0x00]), None);
    }

    #[test]
    fn positive_answers() {
        assert!(Request::OpenSession.acknowledged(&[0x50, 0x92]));
        assert!(!Request::OpenSession.acknowledged(&[0x50, 0x85]));
        assert!(Request::ClearDtcs.acknowledged(&[0x54, 0xff, 0x00]));
        assert_eq!(Request::ReadLocal(0x20).answer(&[0x61, 0x20, 1, 2]), Some(&[0x20, 1, 2][..]));
        assert_eq!(Request::ReadLocal(0x20).answer(&[0x7f, 0x21, 0x12]), None);
        assert_eq!(Request::FlowControl.answer(&[0x70]), None);
    }
}
