//! Communication to a Tactrix OpenPort 2.0 clone.

use embedded_can::Frame as _;

use crate::{can, CanAdapter, Event, Transport};

pub const VENDOR_ID: u16 = 0x0403;
pub const PRODUCT_ID: u16 = 0xcc4d;

pub const CAN_BAUD: u32 = 500_000;

const PROTOCOL_CAN: u32 = 5;
const PASS_FILTER: u32 = 1;
const CHANNEL_CAN: u8 = b'5';
const KIND_RECEIVED: u8 = 0x00;
const MAX_TEXT: usize = 256;

#[derive(Debug, Clone, Copy)]
pub enum Command {
    Identify,
    Attach,
    OpenCan,
    PassAll,
    Transmit(can::Frame),
    CloseCan,
    Detach,
}

impl Command {
    pub fn bytes(self) -> Vec<u8> {
        match self {
            Command::Identify => b"\r\n\r\nati\r\n".to_vec(),
            Command::Attach => b"ata\r\n".to_vec(),
            Command::OpenCan => format!("ato{PROTOCOL_CAN} 0 {CAN_BAUD} 0\r\n").into_bytes(),
            Command::PassAll => {
                let mut v = format!("atf{PROTOCOL_CAN} {PASS_FILTER} 0 4\r\n").into_bytes();
                v.extend_from_slice(&[0; 8]);
                v
            }
            Command::Transmit(frame) => {
                let mut v = format!("att{PROTOCOL_CAN} {} 0\r\n", 4 + frame.data().len()).into_bytes();
                v.extend_from_slice(&can::raw_id(frame.id()).to_be_bytes());
                v.extend_from_slice(frame.data());
                v
            }
            Command::CloseCan => format!("atc{PROTOCOL_CAN}\r\n").into_bytes(),
            Command::Detach => b"atz\r\n".to_vec(),
        }
    }

    pub fn ack(self) -> &'static str {
        match self {
            Command::Identify => "ari",
            Command::PassAll => "arf",
            _ => "aro",
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Reply {
    Text(String),
    Received { timestamp_us: u32, frame: can::Frame },
    Data { channel: u8, kind: u8, timestamp_us: u32, payload: Vec<u8> },
}

#[derive(Default)]
pub struct Parser {
    buf: Vec<u8>,
    pub skipped: u64,
}

impl Parser {
    pub fn push(&mut self, bytes: &[u8]) {
        self.buf.extend_from_slice(bytes);
    }

    pub fn next_reply(&mut self) -> Option<Reply> {
        loop {
            match self.buf.windows(2).position(|w| w == b"ar") {
                Some(0) => {}
                Some(n) => self.skip(n),
                None => {
                    let keep = usize::from(self.buf.last() == Some(&b'a'));
                    self.skip(self.buf.len() - keep);
                    return None;
                }
            }
            let tag = *self.buf.get(2)?;
            if tag.is_ascii_lowercase() {
                match self.buf.windows(2).position(|w| w == b"\r\n") {
                    Some(end) => {
                        let line = String::from_utf8_lossy(&self.buf[..end]).into_owned();
                        self.buf.drain(..end + 2);
                        return Some(Reply::Text(line));
                    }
                    None if self.buf.len() > MAX_TEXT => self.skip(1),
                    None => return None,
                }
            } else {
                let len = *self.buf.get(3)? as usize;
                if len < 5 {
                    self.skip(1);
                    continue;
                }
                if self.buf.len() < 4 + len {
                    return None;
                }
                let bytes: Vec<u8> = self.buf.drain(..4 + len).collect();
                let (kind, timestamp_us) = (bytes[4], u32::from_be_bytes([bytes[5], bytes[6], bytes[7], bytes[8]]));
                let payload = &bytes[9..];
                if (tag, kind) == (CHANNEL_CAN, KIND_RECEIVED)
                    && let Some(frame) = can_frame(payload)
                {
                    return Some(Reply::Received { timestamp_us, frame });
                }
                return Some(Reply::Data { channel: tag, kind, timestamp_us, payload: payload.to_vec() });
            }
        }
    }

    fn skip(&mut self, n: usize) {
        self.buf.drain(..n);
        self.skipped += n as u64;
    }
}

fn can_frame(payload: &[u8]) -> Option<can::Frame> {
    let (id, data) = payload.split_first_chunk::<4>()?;
    can::Frame::from_raw_id(u32::from_be_bytes(*id), data)
}

const REPLY_TIMEOUT_US: u64 = 2_000_000;

pub struct OpenPort<T> {
    transport: T,
    parser: Parser,
    clock: Option<Clock>,
}

impl<T: Transport> OpenPort<T> {
    pub async fn open_can(transport: T) -> Result<(OpenPort<T>, String), String> {
        let mut port = OpenPort { transport, parser: Parser::default(), clock: None };
        let id = port.command(Command::Identify).await?;
        port.command(Command::Attach).await?;
        port.command(Command::OpenCan).await?;
        port.command(Command::PassAll).await?;
        Ok((port, id))
    }

    pub async fn identify(transport: T) -> Result<String, String> {
        let mut port = OpenPort { transport, parser: Parser::default(), clock: None };
        let id = port.command(Command::Identify).await?;
        port.command(Command::Attach).await?;
        port.command(Command::Detach).await?;
        Ok(id)
    }

    async fn command(&mut self, cmd: Command) -> Result<String, String> {
        self.transport.write(&cmd.bytes()).await.map_err(|e| format!("{cmd:?}: write failed: {e}"))?;
        let deadline = self.now_us() + REPLY_TIMEOUT_US;
        while self.now_us() < deadline {
            for reply in self.read(100).await? {
                if let Reply::Text(line) = reply {
                    if line.starts_with("are") {
                        return Err(format!("{cmd:?}: adapter error `{line}`"));
                    }
                    if line.starts_with(cmd.ack()) {
                        return Ok(line);
                    }
                }
            }
        }
        Err(format!("{cmd:?}: no `{}` reply within {}s", cmd.ack(), REPLY_TIMEOUT_US / 1_000_000))
    }

    pub async fn read(&mut self, timeout_ms: u32) -> Result<Vec<Reply>, String> {
        let bytes = self.transport.read(timeout_ms).await?;
        self.parser.push(&bytes);
        Ok(std::iter::from_fn(|| self.parser.next_reply()).collect())
    }

    pub fn now_us(&self) -> u64 {
        self.transport.now_us()
    }
}

impl<T: Transport> CanAdapter for OpenPort<T> {
    async fn send(&mut self, frame: can::Frame) -> Result<(), String> {
        let cmd = Command::Transmit(frame);
        self.transport.write(&cmd.bytes()).await.map_err(|e| format!("{cmd:?}: write failed: {e}"))
    }

    async fn recv(&mut self, timeout_ms: u32) -> Result<Vec<Event>, String> {
        let replies = self.read(timeout_ms).await?;
        let now = self.now_us();
        Ok(replies
            .into_iter()
            .map(|reply| match reply {
                Reply::Received { timestamp_us, frame } => {
                    let clock = self.clock.get_or_insert_with(|| Clock::new(now));
                    Event::Frame { time_us: clock.wall_us(timestamp_us), frame }
                }
                Reply::Text(line) if line.starts_with("are") => Event::Warning(format!("adapter error: {line}")),
                _ => Event::Other,
            })
            .collect())
    }

    fn now_us(&self) -> u64 {
        self.transport.now_us()
    }

    fn skipped_bytes(&self) -> u64 {
        self.parser.skipped
    }

    async fn close(&mut self) {
        let _ = self.command(Command::CloseCan).await;
        let _ = self.command(Command::Detach).await;
    }
}

struct Clock {
    anchor_us: u64,
    last: Option<u32>,
    elapsed_us: u64,
}

impl Clock {
    fn new(anchor_us: u64) -> Clock {
        Clock { anchor_us, last: None, elapsed_us: 0 }
    }

    fn wall_us(&mut self, adapter_ts: u32) -> u64 {
        if let Some(last) = self.last {
            self.elapsed_us += u64::from(adapter_ts.wrapping_sub(last));
        }
        self.last = Some(adapter_ts);
        self.anchor_us + self.elapsed_us
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn can_reply(ts: u32, id: u32, data: &[u8]) -> Vec<u8> {
        let mut v = vec![b'a', b'r', CHANNEL_CAN, (5 + 4 + data.len()) as u8, KIND_RECEIVED];
        v.extend_from_slice(&ts.to_be_bytes());
        v.extend_from_slice(&id.to_be_bytes());
        v.extend_from_slice(data);
        v
    }

    #[test]
    fn command_bytes_match_reference_driver() {
        assert_eq!(Command::Identify.bytes(), b"\r\n\r\nati\r\n");
        assert_eq!(Command::Attach.bytes(), b"ata\r\n");
        assert_eq!(Command::OpenCan.bytes(), b"ato5 0 500000 0\r\n");
        assert_eq!(Command::PassAll.bytes(), b"atf5 1 0 4\r\n\0\0\0\0\0\0\0\0");
        assert_eq!(Command::CloseCan.bytes(), b"atc5\r\n");
        let clear = can::Frame::standard(0x751, &[0x03, 0x14, 0xff, 0x00, 0, 0, 0, 0]).unwrap();
        assert_eq!(
            Command::Transmit(clear).bytes(),
            b"att5 12 0\r\n\x00\x00\x07\x51\x03\x14\xff\x00\x00\x00\x00\x00"
        );
        assert_eq!(Command::Detach.bytes(), b"atz\r\n");
    }

    #[test]
    fn text_replies() {
        let mut p = Parser::default();
        p.push(b"aro\r\nari OpenPort 2.0 fw:1.17\r\narf1\r\n");
        assert_eq!(p.next_reply(), Some(Reply::Text("aro".into())));
        assert_eq!(p.next_reply(), Some(Reply::Text("ari OpenPort 2.0 fw:1.17".into())));
        assert_eq!(p.next_reply(), Some(Reply::Text("arf1".into())));
        assert_eq!(p.next_reply(), None);
    }

    #[test]
    fn can_frame_split_across_reads() {
        let bytes = can_reply(0x0102_0304, 0x373, &[0x11, 0x22, 0x33]);
        let mut p = Parser::default();
        p.push(&bytes[..7]);
        assert_eq!(p.next_reply(), None);
        p.push(&bytes[7..]);
        let frame = can::Frame::standard(0x373, &[0x11, 0x22, 0x33]).unwrap();
        assert_eq!(p.next_reply(), Some(Reply::Received { timestamp_us: 0x0102_0304, frame }));
    }

    #[test]
    fn data_and_text_interleaved() {
        let mut bytes = can_reply(1, 0x101, &[]);
        bytes.extend_from_slice(b"aro\r\n");
        bytes.extend(can_reply(2, 0x102, &[0xff; 8]));
        let mut p = Parser::default();
        p.push(&bytes);
        assert!(matches!(p.next_reply(), Some(Reply::Received { timestamp_us: 1, .. })));
        assert_eq!(p.next_reply(), Some(Reply::Text("aro".into())));
        assert!(matches!(p.next_reply(), Some(Reply::Received { timestamp_us: 2, .. })));
        assert_eq!(p.skipped, 0);
    }

    #[test]
    fn malformed_can_payload_is_other_data() {
        let mut bytes = vec![b'a', b'r', CHANNEL_CAN, 5 + 3, KIND_RECEIVED, 0, 0, 0, 1];
        bytes.extend_from_slice(&[0, 0, 1]);
        let mut p = Parser::default();
        p.push(&bytes);
        assert!(matches!(p.next_reply(), Some(Reply::Data { .. })));
        assert_eq!(can_frame(&[0; 13]), None);
    }

    #[test]
    fn resyncs_after_garbage() {
        let mut p = Parser::default();
        p.push(b"\x00\x13xyza");
        assert_eq!(p.next_reply(), None);
        assert_eq!(p.skipped, 5);
        p.push(b"ro\r\n");
        assert_eq!(p.next_reply(), Some(Reply::Text("aro".into())));
    }

    #[test]
    fn short_length_byte_is_not_a_frame() {
        let mut p = Parser::default();
        p.push(b"ar5\x02aro\r\n");
        assert_eq!(p.next_reply(), Some(Reply::Text("aro".into())));
        assert_eq!(p.skipped, 4);
    }

    struct Fake {
        heard: Vec<u8>,
        pending: Vec<u8>,
        now_us: u64,
    }

    impl Transport for Fake {
        async fn write(&mut self, bytes: &[u8]) -> Result<(), String> {
            let ack: &[u8] = match bytes.trim_ascii_start() {
                b if b.starts_with(b"ati") => b"ari fake\r\n",
                b if b.starts_with(b"atf") => b"arf\r\n",
                _ => b"aro\r\n",
            };
            self.pending.extend_from_slice(ack);
            Ok(())
        }

        async fn read(&mut self, _timeout_ms: u32) -> Result<Vec<u8>, String> {
            self.pending.append(&mut self.heard);
            Ok(std::mem::take(&mut self.pending))
        }

        fn now_us(&self) -> u64 {
            self.now_us
        }
    }

    #[test]
    fn heard_as_events_on_the_program_clock() {
        let mut heard = can_reply(1_000, 0x373, &[1]);
        heard.extend_from_slice(b"are 7\r\narx\r\n");
        heard.extend(can_reply(1_250, 0x374, &[2]));
        let fake = Fake { heard: Vec::new(), pending: Vec::new(), now_us: 5_000_000 };
        let (mut port, id) = pollster::block_on(OpenPort::open_can(fake)).unwrap();
        assert_eq!(id, "ari fake");
        port.transport.heard = heard;
        let frame = |id, data| can::Frame::standard(id, data).unwrap();
        assert_eq!(
            pollster::block_on(port.recv(10)).unwrap(),
            [
                Event::Frame { time_us: 5_000_000, frame: frame(0x373, &[1]) },
                Event::Warning("adapter error: are 7".into()),
                Event::Other,
                Event::Frame { time_us: 5_000_250, frame: frame(0x374, &[2]) },
            ]
        );
    }

    #[test]
    fn clock_survives_adapter_wrap() {
        let mut c = Clock::new(1_000_000);
        assert_eq!(c.wall_us(u32::MAX - 9), 1_000_000);
        assert_eq!(c.wall_us(5), 1_000_015);
    }
}
