//! Program to capture and dump i-MiEV information available using Tactrix
//! OpenPort 2.0 clones or ELM327 adapters.

use std::fs::File;
use std::io::{self, BufWriter, Write};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant};

use clap::builder::{PossibleValue, PossibleValuesParser, TypedValueParser};
use clap::{value_parser, Args, Parser, Subcommand, ValueEnum};
use imiev_capture::bus::Bus;
use imiev_capture::kwp::{self, describe, hex, Ecu, Request, EV_ECU_RECORDS};
use imiev_capture::report;
use obd_adapter::elm327::{self, Elm327};
use obd_adapter::openport::{OpenPort, CAN_BAUD};
use obd_adapter::serial::{self, Serial};
use obd_adapter::trace::{Show, Trace};
use obd_adapter::usb::Usb;
use obd_adapter::{can, CanAdapter, Event};

const DUMP_TIMEOUT_US: u64 = 1_000_000;
const DUMP_ATTEMPTS: u32 = 3;

const ELM_BAUDS: [u32; 5] = [38400, 9600, 115200, 230400, 500000];

type UsbBus = Bus<Adapter>;

#[derive(Parser)]
struct Cli {
    #[command(flatten)]
    elm: ElmArgs,
    #[command(subcommand)]
    command: Cmd,
}

#[derive(Args, Clone)]
struct ElmArgs {
    /// Use an ELM327 adapter instead of the OpenPort: a serial device
    /// (/dev/ttyUSB0, /dev/rfcomm0), a paired Bluetooth address, or a TCP
    /// host:port (WiFi adapters are usually 192.168.0.10:35000).
    #[arg(long, global = true, value_name = "DEVICE")]
    elm: Option<String>,
    /// The ELM327's serial speed [default: tries 38400, 9600, 115200, 230400, 500000].
    #[arg(long, global = true, requires = "elm")]
    baud: Option<u32>,
}

static ELM: OnceLock<ElmArgs> = OnceLock::new();

/// Whichever adapter `--elm` picked.
enum Adapter {
    OpenPort(OpenPort<Usb>),
    Elm(Elm327<Trace<Serial>>),
}

impl CanAdapter for Adapter {
    async fn send(&mut self, frame: can::Frame) -> Result<(), String> {
        match self {
            Adapter::OpenPort(a) => a.send(frame).await,
            Adapter::Elm(a) => a.send(frame).await,
        }
    }

    async fn recv(&mut self, timeout_ms: u32) -> Result<Vec<Event>, String> {
        match self {
            Adapter::OpenPort(a) => a.recv(timeout_ms).await,
            Adapter::Elm(a) => a.recv(timeout_ms).await,
        }
    }

    fn filter(&mut self, id: Option<u32>) {
        match self {
            Adapter::OpenPort(a) => a.filter(id),
            Adapter::Elm(a) => a.filter(id),
        }
    }

    fn now_us(&self) -> u64 {
        match self {
            Adapter::OpenPort(a) => a.now_us(),
            Adapter::Elm(a) => a.now_us(),
        }
    }

    fn skipped_bytes(&self) -> u64 {
        match self {
            Adapter::OpenPort(a) => a.skipped_bytes(),
            Adapter::Elm(a) => a.skipped_bytes(),
        }
    }

    async fn close(&mut self) {
        match self {
            Adapter::OpenPort(a) => a.close().await,
            Adapter::Elm(a) => a.close().await,
        }
    }
}

async fn open_adapter() -> Result<(Adapter, String), String> {
    let ElmArgs { elm, baud } = ELM.get().expect("set in main");
    let Some(device) = elm else {
        let (port, id) = OpenPort::open_can(Usb::open()?).await?;
        return Ok((Adapter::OpenPort(port), id));
    };
    let serial = if serial::bluetooth_address(device).is_some() {
        Serial::bluetooth(device)?
    } else if !device.starts_with('/') && device.contains(':') {
        Serial::tcp(device)?
    } else if let Some(baud) = baud {
        Serial::open(device, *baud)?
    } else {
        find_speed(device).await?
    };
    let (elm, id) = Elm327::open_can(trace(serial)).await?;
    Ok((Adapter::Elm(elm), id))
}

fn trace(serial: Serial) -> Trace<Serial> {
    let show: Option<Show> = match std::env::var_os("IMIEV_TRACE") {
        Some(_) => Some(Box::new(|line| eprintln!("{line}"))),
        None => None,
    };
    Trace::new(serial, show)
}

async fn find_speed(device: &str) -> Result<Serial, String> {
    for baud in ELM_BAUDS {
        let mut serial = trace(Serial::open(device, baud)?);
        if elm327::probe(&mut serial).await? {
            return Serial::open(device, baud);
        }
    }
    Err(format!("no ELM327 answered on {device} at {ELM_BAUDS:?} baud; try --baud"))
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
    /// Read and decode known data records [default: the EV-ECU].
    Data {
        #[command(flatten)]
        ecus: EcuArgs,
        /// Ask for every potential record, 00 to FF - not just the known values.
        #[arg(long)]
        all: bool,
    },
    /// List the actuator tests; never runs one [default: EV-ECU and BMU].
    Actuators(EcuArgs),
    /// Read an ECU's 1 MB of memory into DIR (resumable).
    Dump(DumpArgs),
}

fn main() {
    let cli = Cli::parse();
    ELM.set(cli.elm).ok();
    let result = pollster::block_on(async {
        match cli.command {
            Cmd::Identify => identify().await,
            Cmd::Sniff(args) => log(args).await,
            Cmd::Log { args, lids } => log(LogArgs { lids: lids.unwrap_or(EV_ECU_RECORDS.to_vec()), ..args }).await,
            Cmd::Info(e) => info(e.or(&Ecu::ALL)).await,
            Cmd::Dtc(e) => dtc(&e.or(&Ecu::ALL), false).await,
            Cmd::Clear { ecu } => dtc(&[ecu], true).await,
            Cmd::Data { ecus, all } => data(ecus.or(&[Ecu::EvEcu]), all).await,
            Cmd::Actuators(e) => actuators(e.or(&[Ecu::EvEcu, Ecu::Bmu])).await,
            Cmd::Dump(args) => dump_memory(args).await,
        }
    });
    if let Err(e) = result {
        eprintln!("imiev-capture: {e}");
        std::process::exit(1);
    }
}

async fn identify() -> Result<(), String> {
    if ELM.get().is_some_and(|e| e.elm.is_some()) {
        let (mut adapter, id) = open_adapter().await?;
        adapter.close().await;
        println!("{id}");
    } else {
        println!("{}", OpenPort::identify(Usb::open()?).await?);
    }
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

async fn open_bus(out: Box<dyn Write>, iface: &str) -> Result<UsbBus, String> {
    let (port, adapter) = open_adapter().await?;
    eprintln!("{adapter}");
    Ok(Bus::new(port, adapter, out, iface, Box::new(|w| eprintln!("{w}"))))
}

async fn log(args: LogArgs) -> Result<(), String> {
    let out: Box<dyn Write> = match &args.out {
        Some(path) => Box::new(BufWriter::new(File::create(path).map_err(|e| format!("{path}: {e}"))?)),
        None => Box::new(BufWriter::new(io::stdout())),
    };

    let stop = Arc::new(AtomicBool::new(false));
    let flag = stop.clone();
    ctrlc::set_handler(move || flag.store(true, Ordering::SeqCst)).map_err(|e| e.to_string())?;

    let mut bus = open_bus(out, &args.iface).await?;
    eprintln!("listening at {} kbit/s; Ctrl-C to stop", CAN_BAUD / 1000);
    let started = Instant::now();
    let deadline = args.seconds.map(|s| started + Duration::from_secs_f64(s));
    let running = || !stop.load(Ordering::SeqCst) && deadline.is_none_or(|d| Instant::now() < d);

    let result = if args.lids.is_empty() {
        async {
            while running() {
                if let Err(e) = bus.poll(100).await {
                    if !bus.lost {
                        return Err(e);
                    }
                    reconnect(&mut bus, &running).await;
                }
            }
            Ok(())
        }
        .await
    } else {
        poll_records(&mut bus, &args.lids, running).await
    };

    bus.close().await;
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

async fn reconnect(bus: &mut UsbBus, running: &impl Fn() -> bool) {
    let lost_at = Instant::now();
    let mut last_error = String::new();
    eprintln!("adapter lost; reconnecting");
    while running() {
        std::thread::sleep(Duration::from_millis(250));
        match open_adapter().await {
            Ok((port, adapter)) => {
                bus.reattach(port, adapter);
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

async fn poll_records(bus: &mut UsbBus, lids: &[u8], running: impl Fn() -> bool) -> Result<(), String> {
    let mut answering = None;
    for &lid in lids.iter().cycle() {
        if !running() {
            break;
        }
        let answer = bus.ask(Ecu::EvEcu, Request::ReadLocal(lid)).await.map(|_| ());
        if bus.lost {
            reconnect(bus, &running).await;
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

async fn with_bus(f: impl AsyncFnOnce(&mut UsbBus) -> Result<(), String>) -> Result<(), String> {
    with_logged_bus(Box::new(io::sink()), f).await
}

async fn with_logged_bus(out: Box<dyn Write>, f: impl AsyncFnOnce(&mut UsbBus) -> Result<(), String>) -> Result<(), String> {
    let mut bus = open_bus(out, "can0").await?;
    let result = f(&mut bus).await;
    bus.close().await;
    result
}

async fn info(ecus: Vec<Ecu>) -> Result<(), String> {
    with_bus(async |bus| {
        for ecu in ecus {
            println!("{}:", ecu.name());
            match bus.identification(ecu).await {
                Ok(lines) => lines.iter().for_each(|(label, id)| println!("  {label}: {id}")),
                Err(_) => println!("  no answer"),
            }
        }
        Ok(())
    })
    .await
}

async fn dtc(ecus: &[Ecu], clear: bool) -> Result<(), String> {
    with_bus(async |bus| {
        for &ecu in ecus {
            let result = async {
                print_dtcs(bus, ecu, if clear { "before" } else { "stored" }).await?;
                if clear {
                    bus.clear_dtcs(ecu).await?;
                    eprintln!("{}: cleared", ecu.name());
                    print_dtcs(bus, ecu, "after").await?;
                }
                Ok::<_, String>(())
            }
            .await;
            if let Err(e) = result {
                if ecus.len() == 1 {
                    return Err(e);
                }
                println!("{}: {e}", ecu.name());
            }
        }
        Ok(())
    })
    .await
}

async fn print_dtcs(bus: &mut UsbBus, ecu: Ecu, label: &str) -> Result<(), String> {
    print!("{}", report::dtc_lines(bus, ecu, label).await?);
    Ok(())
}

async fn data(ecus: Vec<Ecu>, all: bool) -> Result<(), String> {
    with_bus(async |bus| {
        for ecu in ecus {
            print!("{}", report::records(bus, ecu, all).await?);
        }
        Ok(())
    })
    .await
}

async fn actuators(ecus: Vec<Ecu>) -> Result<(), String> {
    with_bus(async |bus| {
        print!("{}", report::actuators(bus, &ecus).await);
        Ok(())
    })
    .await
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

async fn dump_memory(args: DumpArgs) -> Result<(), String> {
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

    with_logged_bus(log_out, async |bus| {
        bus.expect(ecu, Request::OpenSession).await?;

        let mut id = format!("{} identification\n", ecu.name());
        for (label, line) in bus.identification(ecu).await? {
            id += &format!("{label}: {line}\n");
        }
        eprint!("{id}");
        std::fs::write(&id_path, id).map_err(|e| format!("{id_path}: {e}"))?;

        if session == 0x85 {
            bus.expect(ecu, Request::OpenProgrammingSession).await?;
            eprintln!("in programming session 85");
        }
        bus.timeout_us = DUMP_TIMEOUT_US;
        let read = read_memory(bus, &args, &mut out, &stop, &mut address).await;
        if session == 0x85 {
            match bus.request(ecu, Request::CloseSession).await {
                Ok(pdu) => eprintln!("back to the default session: {}", hex(&pdu)),
                Err(e) => eprintln!("leaving session 85: {e}; turn the key off and on to reset the ECU"),
            }
        }
        read
    })
    .await?;

    if address < kwp::MEMORY_SIZE {
        eprintln!("read up to 0x{address:05X}; run dump again to carry on");
        return Ok(());
    }
    std::fs::rename(&part_path, &bin_path).map_err(|e| format!("{part_path} -> {bin_path}: {e}"))?;
    eprintln!("wrote {bin_path} ({address} bytes)");
    Ok(())
}

async fn read_memory(
    bus: &mut UsbBus,
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
        let data = read_block(bus, args, *address, size, &mut retries).await?;
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

async fn read_block(bus: &mut UsbBus, args: &DumpArgs, address: u32, size: u8, retries: &mut u32) -> Result<Vec<u8>, String> {
    let &DumpArgs { ecu, session, .. } = args;
    let mut last = String::new();
    for attempt in 0..DUMP_ATTEMPTS {
        *retries += (attempt > 0) as u32;
        let req = Request::ReadMemory { address, size };
        let answer = if session == 0x92 { bus.ask(ecu, req).await } else { bus.request(ecu, req).await };
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
