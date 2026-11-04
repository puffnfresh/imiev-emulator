//! Program to capture and dump i-MiEV information available using Tactrix
//! OpenPort 2.0 clones.

use std::collections::BTreeMap;
use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};

use clap::builder::{PossibleValue, PossibleValuesParser, TypedValueParser};
use clap::{value_parser, Args, Parser, Subcommand, ValueEnum};
use embedded_can::Frame as _;
use imiev_capture::kwp::{self, Dtc, Ecu, IsoTpRx, Request, Rx, EV_ECU_RECORDS, IDENTIFICATION};
use imiev_capture::openport::{Clock, Command, Reply, CAN_BAUD};
use imiev_capture::usb::OpenPort;
use imiev_capture::can;

const KWP_TIMEOUT: Duration = Duration::from_millis(500);
const DUMP_TIMEOUT: Duration = Duration::from_millis(1000);
const DUMP_ATTEMPTS: u32 = 3;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Print the adapter's firmware version.
    Identify,
    /// Record the bus as a candump -L log; never transmits.
    Sniff(LogArgs),
    /// Record the bus while polling the EV-ECU's data records.
    Log {
        #[command(flatten)]
        args: LogArgs,
        /// Record ids to poll, in hex [default: every record the EV-ECU serves].
        #[arg(long, value_delimiter = ',', value_parser = parse_lid)]
        lids: Option<Vec<u8>>,
    },
    /// Read identification: part numbers, software, VIN [default: every ECU].
    Info(EcuArgs),
    /// Read DTCs [default: every ECU].
    Dtc(EcuArgs),
    /// Read, clear and read again the DTCs.
    Clear {
        #[arg(long, default_value = "ev-ecu", value_parser = ecu_where(Ecu::can_clear))]
        ecu: Ecu,
    },
    /// Read every data record, raw [default: the EV-ECU].
    Data(EcuArgs),
    /// Read an ECU's 1 MB of memory into DIR (resumable).
    Dump(DumpArgs),
}

fn main() {
    let result = match Cli::parse().command {
        Cmd::Identify => identify(),
        Cmd::Sniff(args) => log(args),
        Cmd::Log { args, lids } => log(LogArgs { lids: lids.unwrap_or(EV_ECU_RECORDS.to_vec()), ..args }),
        Cmd::Info(e) => info(e.or(&Ecu::ALL)),
        Cmd::Dtc(e) => dtc(&e.or(&Ecu::ALL), false),
        Cmd::Clear { ecu } => dtc(&[ecu], true),
        Cmd::Data(e) => data(e.or(&[Ecu::EvEcu])),
        Cmd::Dump(args) => dump_memory(args),
    };
    if let Err(e) = result {
        eprintln!("imiev-capture: {e}");
        std::process::exit(1);
    }
}

fn identify() -> Result<(), String> {
    let mut port = OpenPort::open()?;
    let id = port.command(Command::Identify)?;
    port.command(Command::Attach)?;
    println!("{id}");
    port.command(Command::Detach)?;
    Ok(())
}

#[derive(Args)]
struct LogArgs {
    /// Write the log here instead of stdout.
    #[arg(long)]
    out: Option<String>,
    /// Interface name written into each log line.
    #[arg(long, default_value = "can0")]
    iface: String,
    /// Stop after this many seconds instead of at Ctrl-C.
    #[arg(long)]
    seconds: Option<f64>,
    #[arg(skip)]
    lids: Vec<u8>,
}

fn parse_lid(s: &str) -> Result<u8, String> {
    u8::from_str_radix(s.trim().trim_start_matches("0x"), 16).map_err(|_| format!("bad record id `{s}`"))
}

#[derive(Args)]
struct EcuArgs {
    /// Which ECU, or `all`.
    #[arg(long)]
    ecu: Option<EcuChoice>,
}

impl EcuArgs {
    fn or(self, default: &[Ecu]) -> Vec<Ecu> {
        match self.ecu {
            None => default.to_vec(),
            Some(EcuChoice::All) => Ecu::ALL.to_vec(),
            Some(EcuChoice::One(e)) => vec![e],
        }
    }
}

#[derive(Clone, Copy)]
enum EcuChoice {
    One(Ecu),
    All,
}

impl ValueEnum for EcuChoice {
    fn value_variants<'a>() -> &'a [Self] {
        use EcuChoice::*;
        &[One(Ecu::EvEcu), One(Ecu::Bmu), One(Ecu::Charger), One(Ecu::Heater), One(Ecu::Meter), All]
    }

    fn to_possible_value(&self) -> Option<PossibleValue> {
        Some(PossibleValue::new(match self {
            EcuChoice::One(e) => e.name(),
            EcuChoice::All => "all",
        }))
    }
}

fn log(args: LogArgs) -> Result<(), String> {
    let out: Box<dyn Write> = match &args.out {
        Some(path) => Box::new(BufWriter::new(File::create(path).map_err(|e| format!("{path}: {e}"))?)),
        None => Box::new(BufWriter::new(io::stdout())),
    };

    let stop = Arc::new(AtomicBool::new(false));
    let flag = stop.clone();
    ctrlc::set_handler(move || flag.store(true, Ordering::SeqCst)).map_err(|e| e.to_string())?;

    let mut bus = Bus::open(out, args.iface)?;
    eprintln!("listening at {} kbit/s; Ctrl-C to stop", CAN_BAUD / 1000);
    let started = Instant::now();
    let deadline = args.seconds.map(|s| started + Duration::from_secs_f64(s));
    let running = || !stop.load(Ordering::SeqCst) && deadline.is_none_or(|d| Instant::now() < d);

    let result = if args.lids.is_empty() {
        (|| {
            while running() {
                if let Err(e) = bus.poll(Duration::from_millis(100)) {
                    if !bus.lost {
                        return Err(e);
                    }
                    bus.reconnect(&running);
                }
            }
            Ok(())
        })()
    } else {
        poll_records(&mut bus, &args.lids, running)
    };

    bus.close();
    let total: u64 = bus.per_id.values().sum();
    eprintln!(
        "{total} frames, {} ids in {:.1}s; {} other replies, {} bytes skipped",
        bus.per_id.len(),
        started.elapsed().as_secs_f64(),
        bus.other,
        bus.port.skipped_bytes()
    );
    for (id, n) in &bus.per_id {
        eprintln!("  {id:03X}  {n}");
    }
    result
}

fn poll_records(bus: &mut Bus, lids: &[u8], running: impl Fn() -> bool) -> Result<(), String> {
    let mut answering = None;
    for &lid in lids.iter().cycle() {
        if !running() {
            break;
        }
        let answer = bus.ask(Ecu::EvEcu, Request::ReadLocal(lid)).map(|_| ());
        if bus.lost {
            bus.reconnect(&running);
            answering = None;
            continue;
        }
        let ok = answer.is_ok();
        if answering != Some(ok) {
            match answer {
                Ok(()) => eprintln!("EV-ECU is answering"),
                Err(e) => eprintln!("{e}; still logging, will keep asking"),
            }
            answering = Some(ok);
        }
    }
    Ok(())
}

fn with_bus(f: impl FnOnce(&mut Bus) -> Result<(), String>) -> Result<(), String> {
    with_logged_bus(Box::new(io::sink()), f)
}

fn with_logged_bus(out: Box<dyn Write>, f: impl FnOnce(&mut Bus) -> Result<(), String>) -> Result<(), String> {
    let mut bus = Bus::open(out, "can0".into())?;
    let result = f(&mut bus);
    bus.close();
    result
}

fn info(ecus: Vec<Ecu>) -> Result<(), String> {
    with_bus(|bus| {
        for ecu in ecus {
            println!("{}:", ecu.name());
            for (option, label) in IDENTIFICATION {
                let req = Request::Identify(option);
                match bus.ask(ecu, req) {
                    Ok(pdu) => match req.answer(&pdu) {
                        Some([_, id @ ..]) => println!("  {label}: {}", kwp::render(id)),
                        _ => println!("  {label}: refused ({})", describe(&pdu)),
                    },
                    Err(_) => {
                        println!("  no answer");
                        break;
                    }
                }
            }
        }
        Ok(())
    })
}

fn dtc(ecus: &[Ecu], clear: bool) -> Result<(), String> {
    if let Some(e) = ecus.iter().find(|e| clear && !e.can_clear()) {
        return Err(format!("clearing the {}'s DTCs has not been checked against its firmware", e.name()));
    }
    with_bus(|bus| {
        for &ecu in ecus {
            let result = (|| {
                print_dtcs(bus, ecu, if clear { "before" } else { "stored" })?;
                if clear {
                    expect(bus, ecu, Request::OpenSession)?;
                    expect(bus, ecu, Request::ClearDtcs)?;
                    eprintln!("{}: cleared", ecu.name());
                    print_dtcs(bus, ecu, "after")?;
                }
                Ok(())
            })();
            if let Err(e) = result {
                if ecus.len() == 1 {
                    return Err(e);
                }
                println!("{}: {e}", ecu.name());
            }
        }
        Ok(())
    })
}

fn data(ecus: Vec<Ecu>) -> Result<(), String> {
    with_bus(|bus| {
        for ecu in ecus {
            println!("{}:", ecu.name());
            for lid in 0..=0xff {
                let req = Request::ReadLocal(lid);
                if let Some([_, record @ ..]) = req.answer(&bus.ask(ecu, req)?) {
                    println!("  {lid:02x}: {}", hex(record));
                }
            }
        }
        Ok(())
    })
}

#[derive(Args)]
struct DumpArgs {
    #[arg(long, default_value = "ev-ecu", value_parser = ecu_where(Ecu::can_dump))]
    ecu: Ecu,
    /// Where the identification and .bin go.
    #[arg(long, default_value = ".")]
    dir: String,
    /// Bytes per read.
    #[arg(long, default_value_t = u8::MAX, value_parser = value_parser!(u8).range(1..))]
    block: u8,
    /// Also write the bus traffic to this candump log.
    #[arg(long)]
    log: Option<String>,
    /// 85 is the programming session; nothing is unlocked, erased or written.
    #[arg(long, default_value = "92", value_parser = PossibleValuesParser::new(["92", "85"]).map(|s| u8::from_str_radix(&s, 16).unwrap()))]
    session: u8,
    /// Stop after this many bytes.
    #[arg(long, value_parser = value_parser!(u32).range(1..))]
    bytes: Option<u32>,
}

fn ecu_where(allowed: fn(Ecu) -> bool) -> impl TypedValueParser<Value = Ecu> {
    let names: Vec<&str> = Ecu::ALL.into_iter().filter(|&e| allowed(e)).map(Ecu::name).collect();
    PossibleValuesParser::new(names).map(|s| Ecu::from_name(&s).unwrap())
}

struct DumpFile {
    file: File,
    path: String,
}

impl DumpFile {
    fn append(&mut self, data: &[u8]) -> Result<(), String> {
        self.file.write_all(data).map_err(|e| format!("{}: {e}", self.path))
    }
}

fn dump_memory(args: DumpArgs) -> Result<(), String> {
    let DumpArgs { ecu, ref dir, ref log, session, .. } = args;
    let prefix = format!("{dir}/{}", ecu.name());
    let (id_path, part_path, bin_path) =
        (format!("{prefix}-identification.txt"), format!("{prefix}.bin.part"), format!("{prefix}.bin"));
    if std::fs::exists(&bin_path).unwrap_or(false) {
        return Err(format!("{bin_path} already exists"));
    }
    let log_out: Box<dyn Write> = match log {
        Some(path) => Box::new(BufWriter::new(File::create(path).map_err(|e| format!("{path}: {e}"))?)),
        None => Box::new(io::sink()),
    };

    let stop = Arc::new(AtomicBool::new(false));
    let flag = stop.clone();
    ctrlc::set_handler(move || flag.store(true, Ordering::SeqCst)).map_err(|e| e.to_string())?;

    let file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&part_path)
        .map_err(|e| format!("{part_path}: {e}"))?;
    let address = file.metadata().map_err(|e| format!("{part_path}: {e}"))?.len();
    if address > u64::from(kwp::MEMORY_SIZE) {
        return Err(format!("{part_path} is longer than the 0x{:05X}-byte memory", kwp::MEMORY_SIZE));
    }
    let mut address = address as u32;
    if address > 0 {
        eprintln!("resuming {part_path} from 0x{address:05X}");
    }
    let mut out = DumpFile { file, path: part_path.clone() };

    with_logged_bus(log_out, |bus| {
        expect(bus, ecu, Request::OpenSession)?;

        let mut id = format!("{} identification\n", ecu.name());
        for (option, label) in IDENTIFICATION {
            let req = Request::Identify(option);
            let line = match bus.ask(ecu, req) {
                Ok(pdu) => match req.answer(&pdu) {
                    Some([_, id @ ..]) => kwp::render(id),
                    _ => format!("refused ({})", describe(&pdu)),
                },
                Err(e) => e,
            };
            id += &format!("{label}: {line}\n");
        }
        eprint!("{id}");
        std::fs::write(&id_path, id).map_err(|e| format!("{id_path}: {e}"))?;

        if session == 0x85 {
            expect(bus, ecu, Request::OpenProgrammingSession)?;
            eprintln!("in programming session 85");
        }
        bus.timeout = DUMP_TIMEOUT;
        let read = read_memory(bus, &args, &mut out, &stop, &mut address);
        if session == 0x85 {
            match bus.request(ecu, Request::CloseSession) {
                Ok(pdu) => eprintln!("back to the default session: {}", hex(&pdu)),
                Err(e) => eprintln!("leaving session 85: {e}; turn the key off and on to reset the ECU"),
            }
        }
        read
    })?;

    if address < kwp::MEMORY_SIZE {
        eprintln!("read up to 0x{address:05X}; run dump again to carry on");
        return Ok(());
    }
    std::fs::rename(&part_path, &bin_path).map_err(|e| format!("{part_path} -> {bin_path}: {e}"))?;
    eprintln!("wrote {bin_path} ({address} bytes)");
    Ok(())
}

fn read_memory(
    bus: &mut Bus,
    args: &DumpArgs,
    out: &mut DumpFile,
    stop: &AtomicBool,
    address: &mut u32,
) -> Result<(), String> {
    let &DumpArgs { block, bytes, .. } = args;
    let started = (Instant::now(), *address);
    let end = bytes.map_or(kwp::MEMORY_SIZE, |b| (*address + b).min(kwp::MEMORY_SIZE));
    let mut retries = 0;
    while *address < end {
        if stop.load(Ordering::SeqCst) {
            return Err(format!("stopped at 0x{:05X}; run dump again to carry on", *address));
        }
        let size = (end - *address).min(block.into()) as u8;
        let data = read_block(bus, args, *address, size, &mut retries)?;
        out.append(&data)?;
        *address += size as u32;

        let done = *address - started.1;
        if done.is_multiple_of(64 * block as u32) || *address == end {
            let elapsed = started.0.elapsed().as_secs_f64();
            let rate = done as f64 / elapsed;
            let left = (kwp::MEMORY_SIZE - *address) as f64 / rate;
            let blocks = done.div_ceil(block as u32) as f64;
            eprintln!(
                "0x{:05X}  {:5.1}%  {:.1} kB/s  {:.0} ms/read  {retries} re-reads  {:.0} min left",
                *address,
                100.0 * *address as f64 / kwp::MEMORY_SIZE as f64,
                rate / 1000.0,
                1000.0 * elapsed / blocks,
                left / 60.0
            );
        }
    }
    Ok(())
}

fn read_block(bus: &mut Bus, args: &DumpArgs, address: u32, size: u8, retries: &mut u32) -> Result<Vec<u8>, String> {
    let &DumpArgs { ecu, session, .. } = args;
    let mut last = String::new();
    for attempt in 0..DUMP_ATTEMPTS {
        *retries += (attempt > 0) as u32;
        let req = Request::ReadMemory { address, size };
        let answer = if session == 0x92 { bus.ask(ecu, req) } else { bus.request(ecu, req) };
        match answer {
            Ok(pdu) => match req.answer(&pdu) {
                Some(data) if data.len() == size as usize => return Ok(data.to_vec()),
                _ if kwp::negative(&pdu).is_some() => {
                    let hint =
                        if session == 0x92 { " It may only allow memory reads in session 85 (--session 85)." } else { "" };
                    return Err(format!(
                        "{} refused the memory read at 0x{address:05X} in session {session:x}: {}.{hint}",
                        ecu.name(),
                        describe(&pdu)
                    ));
                }
                _ => last = format!("unexpected answer {}", hex(&pdu)),
            },
            Err(e) => last = e,
        }
    }
    Err(format!("reading 0x{address:05X}: {last}; run dump again to carry on"))
}

fn expect(bus: &mut Bus, ecu: Ecu, req: Request) -> Result<(), String> {
    let pdu = bus.request(ecu, req)?;
    if !req.acknowledged(&pdu) {
        return Err(format!("{} refused {req:?}: {}", ecu.name(), describe(&pdu)));
    }
    Ok(())
}

fn print_dtcs(bus: &mut Bus, ecu: Ecu, label: &str) -> Result<(), String> {
    let pdu = bus.ask(ecu, Request::ReadDtcs)?;
    let dtcs: Vec<Dtc> = kwp::parse_dtcs(&pdu).ok_or(format!("{} refused ReadDtcs: {}", ecu.name(), describe(&pdu)))?;
    if dtcs.is_empty() {
        println!("{} {label}: no DTCs", ecu.name());
    }
    for d in dtcs {
        println!("{} {label}: {d}", ecu.name());
        if ecu.can_read_dtc_record() {
            print_freeze_frame(bus, ecu, d)?;
        }
    }
    Ok(())
}

fn print_freeze_frame(bus: &mut Bus, ecu: Ecu, dtc: Dtc) -> Result<(), String> {
    let pdu = bus.ask(ecu, Request::ReadDtcRecord(dtc.code))?;
    match kwp::parse_dtc_record(dtc.code, &pdu) {
        None => println!("  freeze frame: {} refused ReadDtcRecord: {}", ecu.name(), describe(&pdu)),
        Some(None) => println!("  no freeze frame"),
        Some(Some(record)) => {
            println!("  freeze frame: {record}");
            for (i, row) in record.snapshot.chunks(16).enumerate() {
                println!("    {:02x}: {}", i * 16, hex(row));
            }
        }
    }
    Ok(())
}

struct Bus {
    port: OpenPort,
    out: Box<dyn Write>,
    iface: String,
    clock: Option<Clock>,
    per_id: BTreeMap<u32, u64>,
    other: u64,
    timeout: Duration,
    lost: bool,
}

fn open_can() -> Result<(OpenPort, String), String> {
    let mut port = OpenPort::open()?;
    let id = port.command(Command::Identify)?;
    port.command(Command::Attach)?;
    port.command(Command::OpenCan)?;
    port.command(Command::PassAll)?;
    Ok((port, id))
}

impl Bus {
    fn open(out: Box<dyn Write>, iface: String) -> Result<Bus, String> {
        let (port, id) = open_can()?;
        eprintln!("{id}");
        Ok(Bus { port, out, iface, clock: None, per_id: BTreeMap::new(), other: 0, timeout: KWP_TIMEOUT, lost: false })
    }

    fn reconnect(&mut self, running: &impl Fn() -> bool) {
        let lost_at = Instant::now();
        let mut last_error = String::new();
        eprintln!("adapter lost; reconnecting");
        while running() {
            std::thread::sleep(Duration::from_millis(250));
            match open_can() {
                Ok((port, _)) => {
                    self.port = port;
                    self.clock = None;
                    self.lost = false;
                    eprintln!("adapter back after {:.1}s", lost_at.elapsed().as_secs_f64());
                    return;
                }
                Err(e) if e != last_error => {
                    eprintln!("{e}; still trying");
                    last_error = e;
                }
                Err(_) => {}
            }
        }
    }

    fn close(&mut self) {
        let _ = self.out.flush();
        let _ = self.port.command(Command::CloseCan);
        let _ = self.port.command(Command::Detach);
    }

    fn poll(&mut self, timeout: Duration) -> Result<Vec<can::Frame>, String> {
        let mut frames = Vec::new();
        let replies = self.port.read(timeout).inspect_err(|_| self.lost = true)?;
        for reply in replies {
            match reply {
                Reply::Received { timestamp_us, frame } => {
                    let clock = self.clock.get_or_insert_with(|| Clock::new(now_us()));
                    let line = can::candump_line(&frame, clock.wall_us(timestamp_us), &self.iface);
                    writeln!(self.out, "{line}").map_err(|e| e.to_string())?;
                    *self.per_id.entry(can::raw_id(frame.id())).or_default() += 1;
                    frames.push(frame);
                }
                Reply::Text(line) if line.starts_with("are") => eprintln!("adapter error: {line}"),
                _ => self.other += 1,
            }
        }
        self.out.flush().map_err(|e| e.to_string())?;
        Ok(frames)
    }

    fn send(&mut self, ecu: Ecu, req: Request) -> Result<(), String> {
        let frame = req.frame(ecu).ok_or(format!("{req:?} to the {} is not an allowed request", ecu.name()))?.can();
        self.port.transmit(frame).inspect_err(|_| self.lost = true)?;
        let line = can::candump_line(&frame, now_us(), &self.iface);
        writeln!(self.out, "{line}").map_err(|e| e.to_string())
    }

    fn ask(&mut self, ecu: Ecu, req: Request) -> Result<Vec<u8>, String> {
        let pdu = self.request(ecu, req)?;
        if !kwp::needs_session(&pdu) {
            return Ok(pdu);
        }
        self.request(ecu, Request::OpenSession)?;
        self.request(ecu, req)
    }

    fn request(&mut self, ecu: Ecu, req: Request) -> Result<Vec<u8>, String> {
        self.send(ecu, req)?;
        let mut rx = IsoTpRx::default();
        let mut deadline = Instant::now() + self.timeout;
        while Instant::now() < deadline {
            for frame in self.poll(Duration::from_millis(10))? {
                if can::raw_id(frame.id()) != u32::from(ecu.response_id()) {
                    continue;
                }
                match rx.push(frame.data()) {
                    Rx::SendFlowControl => self.send(ecu, Request::FlowControl)?,
                    Rx::Done(pdu) if kwp::is_response_pending(&pdu) => deadline = Instant::now() + self.timeout,
                    Rx::Lost => return Err(format!("lost a frame of the {}'s answer", ecu.name())),
                    Rx::Done(pdu) => return Ok(pdu),
                    Rx::Incomplete => {}
                }
            }
        }
        Err(format!("{} did not answer {req:?} (is the car in ON?)", ecu.name()))
    }
}

fn describe(pdu: &[u8]) -> String {
    match kwp::negative(pdu) {
        Some(nrc) => format!("{} ({nrc:?})", hex(pdu)),
        None => hex(pdu),
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect::<Vec<_>>().join(" ")
}

fn now_us() -> u64 {
    SystemTime::now().duration_since(UNIX_EPOCH).map_or(0, |d| d.as_micros() as u64)
}
