//! Diagnostic tester on the OBD port

use std::collections::VecDeque;

use embedded_can::Frame as _;
use imiev_capture::kwp::{self, Dtc, Ecu, IsoTpRx, Request, Rx};

use crate::{frame, CanFrame, BUS_PUMP_INTERVAL, CYCLES_PER_SECOND};

const PUMPS_PER_MS: u32 = (CYCLES_PER_SECOND / BUS_PUMP_INTERVAL / 1000) as u32;
const TIMEOUT_PUMPS: u32 = 1000 * PUMPS_PER_MS;
const FLOW_CONTROL_DELAY_PUMPS: u32 = PUMPS_PER_MS;

struct Exchange {
    ecu: Ecu,
    request: Request,
    rx: IsoTpRx,
    waited: u32,
    flow_control_in: Option<u32>,
}

#[derive(Default)]
pub(crate) struct Tester {
    queue: VecDeque<(Ecu, Request)>,
    current: Option<Exchange>,
    answers: VecDeque<(Ecu, Request, Option<Vec<u8>>)>,
    last_dtcs: Vec<(Ecu, Vec<Dtc>)>,
}

fn to_bench(f: imiev_capture::can::Frame) -> CanFrame {
    let id = match f.id() {
        embedded_can::Id::Standard(id) => id.as_raw(),
        embedded_can::Id::Extended(_) => unreachable!("KWP requests use standard ids"),
    };
    frame(id, f.data())
}

impl Tester {
    pub(crate) fn ask(&mut self, ecu: Ecu, request: Request) {
        self.queue.push_back((ecu, request));
    }

    pub(crate) fn busy(&self) -> bool {
        self.current.is_some() || !self.queue.is_empty()
    }

    pub(crate) fn take_answer(&mut self, ecu: Ecu, request: Request) -> Option<Option<Vec<u8>>> {
        let i = self.answers.iter().position(|(e, r, _)| *e == ecu && *r == request)?;
        self.answers.remove(i).map(|(_, _, a)| a)
    }

    pub(crate) fn last_dtcs(&self, ecu: Ecu) -> Option<&[Dtc]> {
        self.last_dtcs.iter().find(|(e, _)| *e == ecu).map(|(_, d)| d.as_slice())
    }

    fn finish(&mut self, answer: Option<Vec<u8>>) {
        let Some(Exchange { ecu, request, .. }) = self.current.take() else { return };
        if request == Request::ReadDtcs
            && let Some(dtcs) = answer.as_deref().and_then(kwp::parse_dtcs)
        {
            self.last_dtcs.retain(|(e, _)| *e != ecu);
            self.last_dtcs.push((ecu, dtcs));
        }
        self.answers.push_back((ecu, request, answer));
    }

    pub(crate) fn exchange(&mut self, seen: &[CanFrame]) -> Vec<CanFrame> {
        let mut out = Vec::new();
        if let Some(x) = self.current.as_mut() {
            let ecu = x.ecu;
            let mut done = None;
            for f in seen.iter().filter(|f| f.id == ecu.response_id()) {
                match x.rx.push(&f.data[..f.dlc as usize]) {
                    Rx::Done(pdu) if kwp::is_response_pending(&pdu) => x.waited = 0,
                    Rx::Done(pdu) => done = Some(Some(pdu)),
                    Rx::SendFlowControl => x.flow_control_in = Some(FLOW_CONTROL_DELAY_PUMPS),
                    Rx::Lost => done = Some(None),
                    Rx::Incomplete => {}
                }
                if done.is_some() {
                    break;
                }
            }
            match x.flow_control_in {
                Some(0) => {
                    out.extend(Request::FlowControl.frame(ecu).map(|f| to_bench(f.can())));
                    x.flow_control_in = None;
                }
                Some(n) => x.flow_control_in = Some(n - 1),
                None => {}
            }
            x.waited += 1;
            if done.is_none() && x.waited > TIMEOUT_PUMPS {
                done = Some(None);
            }
            if let Some(answer) = done {
                self.finish(answer);
            }
        }
        while self.current.is_none()
            && let Some((ecu, request)) = self.queue.pop_front()
        {
            match request.frame(ecu) {
                Some(f) => {
                    out.push(to_bench(f.can()));
                    self.current =
                        Some(Exchange { ecu, request, rx: IsoTpRx::default(), waited: 0, flow_control_in: None });
                }
                None => self.answers.push_back((ecu, request, None)),
            }
        }
        out
    }
}
