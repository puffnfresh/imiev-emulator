//! ELM327 and clone adapters

use embedded_can::{Frame as _, Id};

use crate::{can, CanAdapter, Event, Transport};

const RESET_MS: u32 = 3000;
const COMMAND_MS: u32 = 1000;
const SEND_MS: u32 = 2000;
const RELISTEN_US: u64 = 500_000;
const SETTLE_US: u64 = 50_000;

const SETUP: &[(&str, bool)] = &[
    ("ATE0", true),
    ("ATL0", false),
    ("ATS0", false),
    ("ATH1", true),
    ("ATSP6", true),
    ("ATCFC1", false),
];

pub struct Elm327<T> {
    transport: T,
    line: Vec<u8>,
    queued: Vec<Event>,
    monitoring: bool,
    prompt_at: u64,
    relisten_at: u64,
    header: Option<u16>,
    formatting: Option<bool>,
    filter: Option<u32>,
    filter_set: Option<Option<u32>>,
    skipped: u64,
}

pub async fn probe<T: Transport>(transport: &mut T) -> Result<bool, String> {
    wake(transport).await?;
    transport.write(b"ATI\r").await?;
    let text = read_for(transport, 1000, |text| text.contains(&b'>')).await?;
    let text = String::from_utf8_lossy(&text).to_ascii_uppercase();
    Ok(["ELM", "STN", "OBD"].iter().any(|name| text.contains(name)))
}

async fn wake<T: Transport>(transport: &mut T) -> Result<(), String> {
    transport.write(b"ATI\r").await?;
    read_for(transport, 1000, |text| text.contains(&b'>')).await?;
    let deadline = transport.now_us() + 2_000_000;
    while transport.now_us() < deadline && !transport.read(200).await?.is_empty() {}
    Ok(())
}

async fn read_for<T: Transport>(transport: &mut T, timeout_ms: u32, done: impl Fn(&[u8]) -> bool) -> Result<Vec<u8>, String> {
    let deadline = transport.now_us() + u64::from(timeout_ms) * 1000;
    let mut text = Vec::new();
    while transport.now_us() < deadline && !done(&text) {
        text.extend(transport.read(50).await?);
    }
    Ok(text)
}

impl<T: Transport> Elm327<T> {
    pub async fn open_can(transport: T) -> Result<(Elm327<T>, String), String> {
        let mut elm = Elm327 {
            transport,
            line: Vec::new(),
            queued: Vec::new(),
            monitoring: false,
            prompt_at: 0,
            relisten_at: 0,
            header: None,
            formatting: None,
            filter: None,
            filter_set: None,
            skipped: 0,
        };
        wake(&mut elm.transport).await?;
        elm.command("ATZ", RESET_MS).await?;
        for &(cmd, required) in SETUP {
            elm.setting(cmd, required).await?;
        }
        let mut id = elm.command("ATI", COMMAND_MS).await?.join(" ");
        if let Ok(volts) = elm.command("ATRV", COMMAND_MS).await
            && let [v] = &volts[..]
            && v.ends_with('V')
        {
            id += &format!(", {v} at the OBD port");
        }
        Ok((elm, id))
    }

    async fn command(&mut self, cmd: &str, timeout_ms: u32) -> Result<Vec<String>, String> {
        self.type_line(cmd).await.map_err(|e| format!("{cmd}: write failed: {e}"))?;
        match self.read_until_prompt(timeout_ms).await? {
            (lines, true) => Ok(lines.into_iter().filter(|l| l != cmd).collect()),
            _ => Err(format!("{cmd}: the adapter did not answer within {timeout_ms} ms")),
        }
    }

    async fn setting(&mut self, cmd: &str, required: bool) -> Result<(), String> {
        let lines = self.command(cmd, COMMAND_MS).await?;
        if required && !lines.iter().any(|l| l == "OK") {
            return Err(format!("the adapter answered {cmd} with `{}`", lines.join(" ")));
        }
        Ok(())
    }

    async fn type_line(&mut self, line: &str) -> Result<(), String> {
        let ready = self.prompt_at + SETTLE_US;
        while self.transport.now_us() < ready {
            let wait_ms = (ready - self.transport.now_us()).div_ceil(1000) as u32;
            let bytes = self.transport.read(wait_ms).await?;
            let (lines, _) = self.feed(&bytes);
            let events: Vec<Event> = lines.iter().map(|l| self.event(l)).collect();
            self.queued.extend(events);
        }
        self.transport.write(format!("{line}\r").as_bytes()).await
    }

    async fn read_until_prompt(&mut self, timeout_ms: u32) -> Result<(Vec<String>, bool), String> {
        let deadline = self.transport.now_us() + u64::from(timeout_ms) * 1000;
        let mut lines = Vec::new();
        while self.transport.now_us() < deadline {
            let bytes = self.transport.read(50).await?;
            let (more, prompt) = self.feed(&bytes);
            lines.extend(more);
            if prompt {
                return Ok((lines, true));
            }
        }
        Ok((lines, false))
    }

    fn feed(&mut self, bytes: &[u8]) -> (Vec<String>, bool) {
        let (mut lines, mut prompt) = (Vec::new(), false);
        for &b in bytes {
            match b {
                b'\r' | b'\n' | b'>' => {
                    let line = String::from_utf8_lossy(&std::mem::take(&mut self.line)).trim().to_string();
                    if !line.is_empty() {
                        lines.push(line);
                    }
                    if b == b'>' {
                        prompt = true;
                        self.prompt_at = self.transport.now_us();
                    }
                }
                b' '..=b'~' => self.line.push(b),
                _ => self.skipped += 1,
            }
        }
        (lines, prompt)
    }

    fn event(&self, line: &str) -> Event {
        match parse_frame(line) {
            Some(frame) => Event::Frame { time_us: self.transport.now_us(), frame },
            None if ["OK", "STOPPED", "NO DATA"].contains(&line) => Event::Other,
            None => Event::Warning(format!("adapter: {line}")),
        }
    }

    async fn stop_monitoring(&mut self) -> Result<(), String> {
        if !self.monitoring {
            return Ok(());
        }
        self.transport.write(b"\r").await?;
        let (lines, prompt) = self.read_until_prompt(COMMAND_MS).await?;
        let events: Vec<Event> = lines.iter().map(|l| self.event(l)).collect();
        self.queued.extend(events);
        if !prompt {
            return Err("the adapter did not stop listening".into());
        }
        self.monitoring = false;
        Ok(())
    }

    async fn set_formatting(&mut self, on: bool) -> Result<(), String> {
        if self.formatting != Some(on) {
            self.setting(if on { "ATCAF1" } else { "ATCAF0" }, true).await?;
            self.formatting = Some(on);
        }
        Ok(())
    }

    async fn apply_filter(&mut self) -> Result<(), String> {
        if self.filter_set == Some(self.filter) {
            return Ok(());
        }
        let cmd = match self.filter {
            Some(id) => format!("ATCRA{id:03X}"),
            None => "ATAR".into(),
        };
        self.setting(&cmd, true).await?;
        self.filter_set = Some(self.filter);
        Ok(())
    }
}

impl<T: Transport> CanAdapter for Elm327<T> {
    async fn send(&mut self, frame: can::Frame) -> Result<(), String> {
        let Id::Standard(id) = frame.id() else {
            return Err("ELM327 adapters are only set up for 11-bit ids".into());
        };
        let bytes = frame.data();
        let payload = match bytes.first().map(|pci| pci >> 4) {
            Some(0) => &bytes[1..1 + usize::from(bytes[0] & 0x0f).min(bytes.len() - 1)],
            Some(3) => return Ok(()),
            _ => return Err(format!("ELM327 adapters only send single-frame requests, not {bytes:02X?}")),
        };
        self.stop_monitoring().await?;
        self.set_formatting(true).await?;
        self.apply_filter().await?;
        if self.header != Some(id.as_raw()) {
            self.setting(&format!("ATSH{:03X}", id.as_raw()), true).await?;
            self.header = Some(id.as_raw());
        }
        let data: String = payload.iter().map(|b| format!("{b:02X}")).collect();
        self.type_line(&data).await.map_err(|e| format!("write failed: {e}"))?;
        let (lines, prompt) = self.read_until_prompt(SEND_MS).await?;
        if lines.iter().any(|l| l == "?") {
            return Err(format!("the adapter refused to send {data}"));
        }
        let events: Vec<Event> = lines.iter().map(|l| self.event(l)).collect();
        self.queued.extend(events);
        if !prompt {
            return Err(format!("the adapter is still sending {data} (no CAN bus? it needs the car in ON)"));
        }
        Ok(())
    }

    async fn recv(&mut self, timeout_ms: u32) -> Result<Vec<Event>, String> {
        if !self.queued.is_empty() {
            return Ok(std::mem::take(&mut self.queued));
        }
        if !self.monitoring {
            if self.transport.now_us() < self.relisten_at {
                self.transport.read(timeout_ms).await?;
                return Ok(Vec::new());
            }
            if self.filter.is_none() {
                self.set_formatting(false).await?;
            }
            self.apply_filter().await?;
            self.type_line("ATMA").await?;
            self.monitoring = true;
        }
        let bytes = self.transport.read(timeout_ms).await?;
        let (lines, prompt) = self.feed(&bytes);
        if prompt {
            self.monitoring = false;
            self.relisten_at = self.transport.now_us() + RELISTEN_US;
        }
        Ok(lines.iter().map(|l| self.event(l)).collect())
    }

    fn filter(&mut self, id: Option<u32>) {
        self.filter = id;
    }

    fn now_us(&self) -> u64 {
        self.transport.now_us()
    }

    fn skipped_bytes(&self) -> u64 {
        self.skipped
    }

    async fn close(&mut self) {
        let _ = self.stop_monitoring().await;
        let _ = self.command("ATPC", COMMAND_MS).await;
    }
}

fn parse_frame(line: &str) -> Option<can::Frame> {
    let hex: String = line.chars().filter(|&c| c != ' ').collect();
    if hex.len() < 3 || hex.len().is_multiple_of(2) || !hex.bytes().all(|b| b.is_ascii_hexdigit()) {
        return None;
    }
    let id = u16::from_str_radix(&hex[..3], 16).ok()?;
    let data: Vec<u8> = (3..hex.len()).step_by(2).map(|i| u8::from_str_radix(&hex[i..i + 2], 16).ok()).collect::<Option<_>>()?;
    can::Frame::standard(id, &data)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn frame(id: u16, data: &[u8]) -> can::Frame {
        can::Frame::standard(id, data).unwrap()
    }

    #[test]
    fn frames_as_printed() {
        assert_eq!(parse_frame("7520314FF00"), Some(frame(0x752, &[0x03, 0x14, 0xff, 0x00])));
        assert_eq!(parse_frame("752 03 7F 18 80 00 00 00 00"), Some(frame(0x752, &[3, 0x7f, 0x18, 0x80, 0, 0, 0, 0])));
        assert_eq!(parse_frame("373"), Some(frame(0x373, &[])));
        for line in ["OK", "NO DATA", "BUFFER FULL", "7E", "7520314FF00112233445566", "ELM327 v1.5", "?"] {
            assert_eq!(parse_frame(line), None, "{line}");
        }
    }

    struct Fake {
        script: Vec<(&'static str, Vec<&'static str>)>,
        heard: Vec<&'static str>,
        typed: Vec<u8>,
        commands: Vec<String>,
        pending: Vec<u8>,
        monitoring: bool,
        now_us: u64,
    }

    impl Fake {
        fn new(script: Vec<(&'static str, Vec<&'static str>)>) -> Fake {
            Fake {
                script,
                heard: Vec::new(),
                typed: Vec::new(),
                commands: Vec::new(),
                pending: Vec::new(),
                monitoring: false,
                now_us: 1_000_000,
            }
        }

        fn answer(&mut self, cmd: &str) -> String {
            match cmd {
                "" => "\r>".into(),
                "ATZ" => "\r\rELM327 v1.5\r\r>".into(),
                "ATI" => "ELM327 v1.5\r\r>".into(),
                "ATRV" => "12.6V\r\r>".into(),
                "ATMA" => {
                    self.monitoring = !self.heard.iter().any(|l| l.contains('>'));
                    self.heard.iter().map(|l| format!("{l}\r")).collect()
                }
                c if c.starts_with("AT") => "OK\r\r>".into(),
                c if !c.bytes().all(|b| b.is_ascii_hexdigit()) => "?\r\r>".into(),
                data => match self.script.iter().find(|(d, _)| *d == data) {
                    Some((_, lines)) => lines.iter().map(|l| format!("{l}\r")).collect::<String>() + "\r>",
                    None => "NO DATA\r\r>".into(),
                },
            }
        }
    }

    impl Transport for Fake {
        async fn write(&mut self, bytes: &[u8]) -> Result<(), String> {
            for &b in bytes {
                if self.monitoring {
                    self.monitoring = false;
                    self.pending.extend(b"STOPPED\r\r>");
                    self.commands.push(if b == b'\r' { String::new() } else { char::from(b).to_string() });
                    continue;
                }
                if b != b'\r' {
                    self.typed.push(b);
                    continue;
                }
                let cmd = String::from_utf8(std::mem::take(&mut self.typed)).unwrap();
                let answer = self.answer(&cmd);
                self.pending.extend(answer.bytes());
                self.commands.push(cmd);
            }
            Ok(())
        }

        async fn read(&mut self, timeout_ms: u32) -> Result<Vec<u8>, String> {
            if self.pending.is_empty() {
                self.now_us += u64::from(timeout_ms) * 1000;
            }
            Ok(std::mem::take(&mut self.pending))
        }

        fn now_us(&self) -> u64 {
            self.now_us
        }
    }

    fn open(fake: Fake) -> Elm327<Fake> {
        let (elm, id) = pollster::block_on(Elm327::open_can(fake)).unwrap();
        assert_eq!(id, "ELM327 v1.5, 12.6V at the OBD port");
        elm
    }

    #[test]
    fn finds_the_adapter_without_repeating_a_command() {
        let mut fake = Fake::new(vec![]);
        assert!(pollster::block_on(probe(&mut fake)).unwrap());
        assert_eq!(fake.commands, ["ATI", "ATI"]);

        let mut fake = Fake::new(vec![]);
        fake.monitoring = true;
        open(fake);
    }

    #[test]
    fn sets_up_can() {
        let elm = open(Fake::new(vec![]));
        let commands = &elm.transport.commands;
        for cmd in ["ATZ", "ATE0", "ATH1", "ATSP6", "ATCFC1"] {
            assert!(commands.iter().any(|c| c == cmd), "{cmd} in {commands:?}");
        }
    }

    #[test]
    fn answers_heard_while_sending() {
        let mut elm = open(Fake::new(vec![("1800FF00", vec!["7520358025A15", "7620100"])]));
        elm.filter(Some(0x752));
        let request = frame(0x751, &[0x04, 0x18, 0x00, 0xff, 0x00, 0, 0, 0]);
        pollster::block_on(elm.send(request)).unwrap();
        let events = pollster::block_on(elm.recv(10)).unwrap();
        let [Event::Frame { frame: f, .. }, Event::Frame { frame: other, .. }] = &events[..] else { panic!("{events:?}") };
        assert_eq!(*f, frame(0x752, &[0x03, 0x58, 0x02, 0x5a, 0x15]));
        assert_eq!(*other, frame(0x762, &[0x01, 0x00]));

        pollster::block_on(elm.send(request)).unwrap();
        let commands = &elm.transport.commands;
        let set: Vec<&String> =
            commands.iter().filter(|c| ["ATCAF", "ATCRA", "ATSH"].iter().any(|p| c.starts_with(p))).collect();
        assert_eq!(set, ["ATCAF1", "ATCRA752", "ATSH751"]);
        assert_eq!(commands.iter().filter(|c| *c == "1800FF00").count(), 2);
    }

    #[test]
    fn leaves_flow_control_to_the_adapter() {
        let first = "76210165A8704850002";
        let mut elm = open(Fake::new(vec![("1A87", vec![first, "762210000000000000", "76222000000"])]));
        pollster::block_on(elm.send(frame(0x761, &[0x02, 0x1a, 0x87, 0, 0, 0, 0, 0]))).unwrap();
        let sent = elm.transport.commands.len();
        pollster::block_on(elm.send(frame(0x761, &[0x30, 0, 0, 0, 0, 0, 0, 0]))).unwrap();
        assert_eq!(elm.transport.commands.len(), sent);
        let events = pollster::block_on(elm.recv(10)).unwrap();
        assert_eq!(events.len(), 3);
        assert!(matches!(&events[2], Event::Frame { frame, .. } if frame.data() == [0x22, 0, 0, 0]));

        let first_frame = frame(0x761, &[0x10, 0x09, 1, 2, 3, 4, 5, 6]);
        assert!(pollster::block_on(elm.send(first_frame)).unwrap_err().contains("single-frame"));
    }

    #[test]
    fn listens_until_the_next_send() {
        let mut fake = Fake::new(vec![]);
        fake.heard = vec!["3730102", "BUFFER FULL"];
        let mut elm = open(fake);
        let events = pollster::block_on(elm.recv(10)).unwrap();
        assert_eq!(events.len(), 2);
        assert!(matches!(events[0], Event::Frame { time_us: 1_000_000.., .. }));
        assert_eq!(events[1], Event::Warning("adapter: BUFFER FULL".into()));
        assert!(elm.monitoring);

        pollster::block_on(elm.send(frame(0x751, &[0x02, 0x10, 0x92, 0, 0, 0, 0, 0]))).unwrap();
        assert!(!elm.monitoring);
        let commands = &elm.transport.commands;
        let tail: Vec<&str> = commands[commands.len() - 7..].iter().map(String::as_str).collect();
        assert_eq!(tail, ["ATCAF0", "ATAR", "ATMA", "", "ATCAF1", "ATSH751", "1092"]);
        assert_eq!(pollster::block_on(elm.recv(10)).unwrap(), [Event::Other, Event::Other]);
    }

    #[test]
    fn listens_again_after_stopping_by_itself() {
        let mut fake = Fake::new(vec![]);
        fake.heard = vec!["BUFFER FULL\r\r>"];
        let mut elm = open(fake);
        pollster::block_on(elm.recv(10)).unwrap();
        assert!(!elm.monitoring);
        assert_eq!(pollster::block_on(elm.recv(100)).unwrap(), []);
        assert!(!elm.monitoring);
        for _ in 0..5 {
            pollster::block_on(elm.recv(100)).unwrap();
        }
        assert_eq!(elm.transport.commands.iter().filter(|c| *c == "ATMA").count(), 2);
    }
}
