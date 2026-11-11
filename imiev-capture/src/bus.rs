//! KWP2000 requests to ECUs

use std::collections::BTreeMap;
use std::io::Write;

use embedded_can::Frame as _;

use crate::can;
use crate::kwp::{self, describe, Dtc, Ecu, IsoTpRx, Request, Rx, IDENTIFICATION};
use obd_adapter::{CanAdapter, Event};

pub const KWP_TIMEOUT_US: u64 = 500_000;

pub struct Bus<A> {
    pub port: A,
    pub adapter: String,
    out: Box<dyn Write>,
    iface: String,
    warn: Box<dyn FnMut(&str)>,
    pub per_id: BTreeMap<u32, u64>,
    pub other: u64,
    pub timeout_us: u64,
    pub lost: bool,
}

impl<A: CanAdapter> Bus<A> {
    pub fn new(port: A, adapter: String, out: Box<dyn Write>, iface: &str, warn: Box<dyn FnMut(&str)>) -> Bus<A> {
        Bus {
            port,
            adapter,
            out,
            iface: iface.into(),
            warn,
            per_id: BTreeMap::new(),
            other: 0,
            timeout_us: KWP_TIMEOUT_US,
            lost: false,
        }
    }

    pub fn reattach(&mut self, port: A, adapter: String) {
        (self.port, self.adapter) = (port, adapter);
        self.lost = false;
    }

    pub async fn close(&mut self) {
        let _ = self.out.flush();
        self.port.close().await;
    }

    pub async fn poll(&mut self, timeout_ms: u32) -> Result<Vec<can::Frame>, String> {
        let mut frames = Vec::new();
        let events = self.port.recv(timeout_ms).await.inspect_err(|_| self.lost = true)?;
        for event in events {
            match event {
                Event::Frame { time_us, frame } => {
                    let line = can::candump_line(&frame, time_us, &self.iface);
                    writeln!(self.out, "{line}").map_err(|e| e.to_string())?;
                    *self.per_id.entry(can::raw_id(frame.id())).or_default() += 1;
                    frames.push(frame);
                }
                Event::Warning(warning) => (self.warn)(&warning),
                Event::Other => self.other += 1,
            }
        }
        self.out.flush().map_err(|e| e.to_string())?;
        Ok(frames)
    }

    async fn send(&mut self, ecu: Ecu, req: Request) -> Result<(), String> {
        let frame = req.frame(ecu).ok_or(format!("{req:?} to the {} is not an allowed request", ecu.name()))?.can();
        self.port.send(frame).await.inspect_err(|_| self.lost = true)?;
        let line = can::candump_line(&frame, self.port.now_us(), &self.iface);
        writeln!(self.out, "{line}").map_err(|e| e.to_string())
    }

    pub async fn ask(&mut self, ecu: Ecu, req: Request) -> Result<Vec<u8>, String> {
        let pdu = self.request(ecu, req).await?;
        if !kwp::needs_session(&pdu) {
            return Ok(pdu);
        }
        self.request(ecu, Request::OpenSession).await?;
        self.request(ecu, req).await
    }

    pub async fn request(&mut self, ecu: Ecu, req: Request) -> Result<Vec<u8>, String> {
        self.port.filter(Some(ecu.response_id().into()));
        self.send(ecu, req).await?;
        let mut rx = IsoTpRx::default();
        let mut deadline = self.port.now_us() + self.timeout_us;
        while self.port.now_us() < deadline {
            for frame in self.poll(10).await? {
                if can::raw_id(frame.id()) != u32::from(ecu.response_id()) {
                    continue;
                }
                match rx.push(frame.data()) {
                    Rx::SendFlowControl => self.send(ecu, Request::FlowControl).await?,
                    Rx::Done(pdu) if kwp::is_response_pending(&pdu) => deadline = self.port.now_us() + self.timeout_us,
                    Rx::Lost => return Err(format!("lost a frame of the {}'s answer", ecu.name())),
                    Rx::Done(pdu) => return Ok(pdu),
                    Rx::Incomplete => {}
                }
            }
        }
        Err(format!("{} did not answer {req:?} (is the car in ON?)", ecu.name()))
    }

    pub async fn expect(&mut self, ecu: Ecu, req: Request) -> Result<(), String> {
        let pdu = self.request(ecu, req).await?;
        if !req.acknowledged(&pdu) {
            return Err(format!("{} refused {req:?}: {}", ecu.name(), describe(&pdu)));
        }
        Ok(())
    }

    pub async fn identification(&mut self, ecu: Ecu) -> Result<Vec<(&'static str, String)>, String> {
        let mut lines = Vec::new();
        for (option, label) in IDENTIFICATION {
            let req = Request::Identify(option);
            let pdu = self.ask(ecu, req).await?;
            lines.push((
                label,
                match req.answer(&pdu) {
                    Some([_, id @ ..]) => kwp::render(id),
                    _ => format!("refused ({})", describe(&pdu)),
                },
            ));
        }
        Ok(lines)
    }

    pub async fn dtcs(&mut self, ecu: Ecu) -> Result<Vec<Dtc>, String> {
        let pdu = self.ask(ecu, Request::ReadDtcs).await?;
        kwp::parse_dtcs(&pdu).ok_or(format!("{} refused ReadDtcs: {}", ecu.name(), describe(&pdu)))
    }

    pub async fn clear_dtcs(&mut self, ecu: Ecu) -> Result<(), String> {
        if !ecu.can_clear() {
            return Err(format!("clearing the {}'s DTCs has not been checked against its firmware", ecu.name()));
        }
        self.expect(ecu, Request::OpenSession).await?;
        self.expect(ecu, Request::ClearDtcs).await
    }
}

#[cfg(test)]
mod tests {
    use std::cell::{Cell, RefCell};
    use std::collections::VecDeque;
    use std::rc::Rc;

    use obd_adapter::openport::OpenPort;
    use obd_adapter::Transport;

    use super::*;

    type Sent = Rc<RefCell<Vec<Vec<u8>>>>;

    struct FakeAdapter {
        script: Vec<(Vec<u8>, Vec<Vec<u8>>)>,
        pending: VecDeque<u8>,
        sent: Sent,
        now: Cell<u64>,
    }

    impl FakeAdapter {
        fn received(&mut self, id: u16, data: &[u8]) {
            let mut v = vec![b'a', b'r', b'5', (9 + data.len()) as u8, 0];
            v.extend_from_slice(&(self.now.get() as u32).to_be_bytes());
            v.extend_from_slice(&u32::from(id).to_be_bytes());
            v.extend_from_slice(data);
            self.pending.extend(v);
        }
    }

    impl Transport for FakeAdapter {
        async fn write(&mut self, bytes: &[u8]) -> Result<(), String> {
            let text = String::from_utf8_lossy(bytes);
            let ack: &[u8] = match text.trim_start() {
                t if t.starts_with("ati") => b"ari fake\r\n",
                t if t.starts_with("atf") => b"arf\r\n",
                t if t.starts_with("att") => {
                    let data = bytes[bytes.len() - 8..].to_vec();
                    self.sent.borrow_mut().push(data.clone());
                    let answer = self.script.iter().find(|(req, _)| *req == data).map(|(_, a)| a.clone());
                    for frame in answer.unwrap_or_default() {
                        self.received(0x752, &frame);
                    }
                    b""
                }
                _ => b"aro\r\n",
            };
            self.pending.extend(ack);
            Ok(())
        }

        async fn read(&mut self, timeout_ms: u32) -> Result<Vec<u8>, String> {
            if self.pending.is_empty() {
                self.now.set(self.now.get() + u64::from(timeout_ms) * 1000);
            }
            Ok(self.pending.drain(..).collect())
        }

        fn now_us(&self) -> u64 {
            self.now.get()
        }
    }

    fn bus(script: Vec<(Vec<u8>, Vec<Vec<u8>>)>) -> (Bus<OpenPort<FakeAdapter>>, Sent) {
        let sent = Sent::default();
        let fake = FakeAdapter { script, pending: VecDeque::new(), sent: Rc::clone(&sent), now: Cell::new(0) };
        let (port, adapter) = pollster::block_on(OpenPort::open_can(fake)).unwrap();
        (Bus::new(port, adapter, Box::new(std::io::sink()), "can0", Box::new(|_| {})), sent)
    }

    #[test]
    fn opens_the_session_when_asked_to() {
        let (mut bus, sent) = bus(vec![
            (vec![0x04, 0x18, 0x00, 0xff, 0x00, 0, 0, 0], vec![vec![0x03, 0x7f, 0x18, 0x80, 0, 0, 0, 0]]),
            (vec![0x02, 0x10, 0x92, 0, 0, 0, 0, 0], vec![vec![0x02, 0x50, 0x92, 0, 0, 0, 0, 0]]),
        ]);
        assert_eq!(bus.adapter, "ari fake");
        let refused = pollster::block_on(bus.dtcs(Ecu::EvEcu));
        assert!(refused.unwrap_err().contains("ServiceNotSupportedInActiveSession"));
        assert_eq!(sent.borrow().len(), 3);
        assert_eq!(sent.borrow()[1], [0x02, 0x10, 0x92, 0, 0, 0, 0, 0]);
    }

    #[test]
    fn reassembles_with_flow_control() {
        let (mut bus, sent) = bus(vec![
            (vec![0x04, 0x18, 0x00, 0xff, 0x00, 0, 0, 0], vec![vec![0x10, 0x08, 0x58, 0x02, 0x1a, 0x15, 0x20, 0xd8]]),
            (vec![0x30, 0, 0, 0, 0, 0, 0, 0], vec![vec![0x21, 0x02, 0x60, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa]]),
        ]);
        let dtcs = pollster::block_on(bus.dtcs(Ecu::EvEcu)).unwrap();
        assert_eq!(dtcs.iter().map(|d| d.label()).collect::<Vec<_>>(), ["P1A15", "U1802"]);
        assert_eq!(sent.borrow().len(), 2);
    }

    #[test]
    fn times_out_without_an_answer() {
        let (mut bus, _) = bus(vec![]);
        let err = pollster::block_on(bus.dtcs(Ecu::Heater)).unwrap_err();
        assert!(err.contains("heater did not answer"));
    }

    #[test]
    fn never_sends_what_kwp_refuses() {
        let (mut bus, sent) = bus(vec![]);
        assert!(pollster::block_on(bus.clear_dtcs(Ecu::Charger)).is_err());
        assert!(sent.borrow().is_empty());
    }
}
