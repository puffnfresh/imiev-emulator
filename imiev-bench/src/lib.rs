//! Simulated components of the i-MiEV.

use std::collections::BTreeMap;

use mh8106f::{CanFrame, System};

const BMU_FW: &[u8] = include_bytes!("../../firmware/bmu.bin");
const ECU_FW: &[u8] = include_bytes!("../../firmware/ev-ecu.bin");

const CAN_DLC_MAX: usize = 8;

const BUS_PUMP_INTERVAL: u64 = 10_000;

const ADC_MIDSCALE: u16 = 0x800; // an undriven analog input floats to mid-scale
const SHUNT_ZERO_A: u16 = 0x200; // BMU main-current shunt at 2.5 V -> 0 A
const SUPPLY_5V: u16 = 0x330; // a healthy ~5 V sensor supply rail
const SUPPLY_12V_OK: u16 = 0x300; // an in-range reading for a 12 V control rail

mod bmu_adc {
    pub const CONTROL_SUPPLY: usize = 0; // 12 V control rail (DTC P1A4C out of 8..16 V)
    pub const TEMP_SENSOR_1: usize = 1; // pack temperature sensor 1
    pub const PACK_CURRENT_HI: usize = 2; // main shunt, HIGH range (0x200 = 0 A)
    pub const PACK_CURRENT_LO: usize = 3; // main shunt, LOW range  (0x200 = 0 A)
    pub const RELAY_SUPPLY: usize = 4; // EV-control-relay 12 V rail (DTC P1A4D)
    pub const TEMP_SENSOR_2: usize = 9; // pack temperature sensor 2 (same sample task as 1)
    pub const TEMP_SENSOR_3: usize = 0xB; // pack temperature sensor 3
}

mod ecu_adc {
    pub const CONDENSER: usize = 0; // HV DC-link voltage sense
    pub const BRAKE_SUPPLY: usize = 1; // brake-stroke sensor 5 V supply
    pub const ACCEL_1_SIGNAL: usize = 2; // accelerator sensor 1 (main) signal
    pub const BRAKE_SIGNAL: usize = 3; // brake-stroke sensor signal
    pub const VACUUM_SUPPLY: usize = 4; // brake-booster vacuum sensor 5 V supply
    pub const ACCEL_2_SIGNAL: usize = 5; // accelerator sensor 2 (sub) signal
    pub const VACUUM_OUTPUT: usize = 6; // brake-booster vacuum sensor output
    pub const ACCEL_1_SUPPLY: usize = 10; // accelerator sensor 1 (main) 5 V supply
    pub const ACCEL_2_SUPPLY: usize = 11; // accelerator sensor 2 (sub) 5 V supply
    pub const CHARGE_PORT: usize = 12; // charge-port connection sense
}

const BMU_BOOT_ADC: &[(usize, u16)] = &[
    (bmu_adc::CONTROL_SUPPLY, SUPPLY_12V_OK),
    (bmu_adc::RELAY_SUPPLY, SUPPLY_12V_OK),
    (bmu_adc::TEMP_SENSOR_1, 0x330), // ~mid-range temperature
    (bmu_adc::TEMP_SENSOR_2, ADC_MIDSCALE),
    (bmu_adc::TEMP_SENSOR_3, ADC_MIDSCALE),
    (bmu_adc::PACK_CURRENT_HI, SHUNT_ZERO_A),
    (bmu_adc::PACK_CURRENT_LO, SHUNT_ZERO_A),
];

const ECU_BOOT_ADC: &[(usize, u16)] = &[
    (ecu_adc::CONDENSER, 0x000),       // HV DC-link 0 V at boot (REST)
    (ecu_adc::BRAKE_SUPPLY, SUPPLY_5V),
    (ecu_adc::ACCEL_1_SIGNAL, 0x0c0),  // released accelerator, main
    (ecu_adc::BRAKE_SIGNAL, 0x130),    // released brake (~1.5 V)
    (ecu_adc::VACUUM_SUPPLY, SUPPLY_5V),
    (ecu_adc::ACCEL_2_SIGNAL, 0x060),  // released accelerator, sub (~1/2 of main)
    (ecu_adc::VACUUM_OUTPUT, 0x200),   // mid-range vacuum reading (~2.5 V)
    (ecu_adc::ACCEL_1_SUPPLY, SUPPLY_5V),
    (ecu_adc::ACCEL_2_SUPPLY, SUPPLY_5V),
    (ecu_adc::CHARGE_PORT, 0x380),     // charge port disconnected
];

pub fn frame(id: u16, data: &[u8]) -> CanFrame {
    let mut buf = [0u8; CAN_DLC_MAX];
    let n = data.len().min(CAN_DLC_MAX);
    buf[..n].copy_from_slice(&data[..n]);
    CanFrame { id, dlc: n as u8, data: buf }
}

pub trait Part {
    fn update(&mut self, chip: &mut System, bus: &CanBus);
}

pub trait BusSource {
    fn frames(&mut self, bus: &CanBus) -> Vec<CanFrame>;
}

/// One chip on the vehicle CAN bus, together with the parts wired to it.
pub struct Node {
    pub name: &'static str,
    sys: System,
    parts: Vec<Box<dyn Part>>,
    local_parts: Vec<Box<dyn Part>>,
}

impl Node {
    pub fn new(name: &'static str, firmware: &[u8]) -> Node {
        Node {
            name,
            sys: System::new(firmware),
            parts: Vec::new(),
            local_parts: Vec::new(),
        }
    }

    pub fn with_adc_env(mut self, env: &[(usize, u16)]) -> Self {
        for &(ch, raw) in env {
            self.set_adc(ch, raw);
        }
        self
    }

    pub fn with_part(mut self, part: Box<dyn Part>) -> Self {
        self.parts.push(part);
        self
    }

    pub fn with_local_part(mut self, part: Box<dyn Part>) -> Self {
        self.local_parts.push(part);
        self
    }

    pub fn system(&self) -> &System {
        &self.sys
    }
    pub fn system_mut(&mut self) -> &mut System {
        &mut self.sys
    }

    pub fn set_adc(&mut self, ch: usize, raw12: u16) {
        self.sys.adc_mut().set_channel(ch, raw12);
    }

    fn step(&mut self) -> bool {
        self.sys.step()
    }

    fn drain_tx(&mut self) -> Vec<CanFrame> {
        self.sys.take_can0_tx()
    }

    fn deliver(&mut self, f: &CanFrame) {
        self.sys.inject_can0(f.id, &f.data[..f.dlc as usize]);
    }

    fn update_parts(&mut self, bus: &CanBus) {
        let Node { sys, parts, .. } = self;
        for p in parts.iter_mut() {
            p.update(sys, bus);
        }
    }

    fn update_local_parts(&mut self, bus: &CanBus) {
        let Node { sys, local_parts, .. } = self;
        for p in local_parts.iter_mut() {
            p.update(sys, bus);
        }
    }
}

#[derive(Default)]
pub struct CanBus {
    last: BTreeMap<u16, CanFrame>,
}

impl CanBus {
    pub fn last(&self, id: u16) -> Option<&CanFrame> {
        self.last.get(&id)
    }
    /// Identifiers observed on the bus so far.
    pub fn ids(&self) -> impl Iterator<Item = u16> + '_ {
        self.last.keys().copied()
    }
    fn record(&mut self, f: CanFrame) {
        self.last.insert(f.id, f);
    }
}

/// Several [`Node`]s on one [`CanBus`], plus the [`BusSource`]s that stand in for
/// the un-emulated rest of the car.
pub struct Simulation {
    nodes: Vec<Node>,
    sources: Vec<Box<dyn BusSource>>,
    bus: CanBus,
    pump_every: u64,
    cycle: u64,
}

impl Simulation {
    pub fn new(nodes: Vec<Node>, pump_every: u64) -> Simulation {
        Simulation {
            nodes,
            sources: Vec::new(),
            bus: CanBus::default(),
            pump_every: pump_every.max(1),
            cycle: 0,
        }
    }

    pub fn imiev() -> Simulation {
        let bmu = Node::new("BMU", BMU_FW)
            .with_adc_env(BMU_BOOT_ADC)
            .with_part(Box::new(Cmu::default()));
        let ecu = Node::new("EV-ECU", ECU_FW)
            .with_adc_env(ECU_BOOT_ADC)
            .with_part(Box::new(Condenser::default()))
            .with_part(Box::new(DriverControls::default()))
            .with_local_part(Box::new(Ic2Companion::default()));
        Simulation::new(vec![bmu, ecu], BUS_PUMP_INTERVAL)
            .with_source(Box::new(Vehicle))
            .with_source(Box::new(DcLink))
    }

    pub fn with_source(mut self, source: Box<dyn BusSource>) -> Self {
        self.sources.push(source);
        self
    }

    pub fn bus(&self) -> &CanBus {
        &self.bus
    }
    pub fn node(&self, i: usize) -> &Node {
        &self.nodes[i]
    }
    pub fn nodes(&self) -> &[Node] {
        &self.nodes
    }

    pub fn run(&mut self, steps: u64) {
        for _ in 0..steps {
            {
                let Simulation { nodes, bus, .. } = self;
                for n in nodes.iter_mut() {
                    n.step();
                    n.update_local_parts(bus);
                }
            }
            self.cycle += 1;
            if self.cycle.is_multiple_of(self.pump_every) {
                self.pump();
            }
        }
    }

    fn pump(&mut self) {
        let Simulation { nodes, sources, bus, .. } = self;
        let mut tx: Vec<CanFrame> = Vec::new();
        for n in nodes.iter_mut() {
            tx.append(&mut n.drain_tx());
        }
        for s in sources.iter_mut() {
            tx.extend(s.frames(bus));
        }
        for f in &tx {
            bus.record(*f);
            for n in nodes.iter_mut() {
                n.deliver(f);
            }
        }
        for n in nodes.iter_mut() {
            n.update_parts(bus);
        }
    }
}

const ETACS_STATUS_ID: u16 = 0x412;
const ETACS_IGNITION_ON: u8 = 0x04;

#[derive(Default)]
pub struct Vehicle;

impl BusSource for Vehicle {
    fn frames(&mut self, _bus: &CanBus) -> Vec<CanFrame> {
        vec![frame(ETACS_STATUS_ID, &[ETACS_IGNITION_ON, 0, 0, 0, 0, 0, 0, 0])]
    }
}

const DC_LINK_ID: u16 = 0x236;
const DC_LINK_CHARGED: [u8; 8] = [0x12, 0xf8, 0, 0, 0, 0, 0, 0];

pub struct DcLink;

impl BusSource for DcLink {
    fn frames(&mut self, _bus: &CanBus) -> Vec<CanFrame> {
        vec![frame(DC_LINK_ID, &DC_LINK_CHARGED)]
    }
}

const CMU_BOARDS: u16 = 12; // twelve CMU boards in the i-MiEV pack
const CMU_CELLS: [u16; 4] = [1, 3, 5, 7]; // odd cell index carried per report frame
const CMU_RX_SLOT: u32 = 30; // BMU pack-bus mailbox the CMU poll reprograms
const CELL_V_OFFSET_MV: i32 = 2100; // report raw = (cell mV − 2100) / 5  ⇔ (V−2.1)×200
const CELL_V_STEP_MV: i32 = 5;
const TEMP_C_BIAS: i16 = 50; // report byte = °C + 50

pub struct Cmu {
    pub cell_mv: u16,
    pub temp_c: i8,
    next: usize,
}

impl Cmu {
    /// A pack of uniform cells.
    pub fn new(cell_mv: u16, temp_c: i8) -> Cmu {
        Cmu { cell_mv, temp_c, next: 0 }
    }

    fn report(&self) -> (u16, [u8; 8]) {
        let idx = self.next % (CMU_BOARDS as usize * CMU_CELLS.len());
        let board = (idx / CMU_CELLS.len()) as u16 + 1;
        let cell = CMU_CELLS[idx % CMU_CELLS.len()];
        let sid = 0x600 | (board << 4) | cell;

        let raw = ((self.cell_mv as i32 - CELL_V_OFFSET_MV) / CELL_V_STEP_MV).clamp(0, 0xffff) as u16;
        let [vh, vl] = raw.to_be_bytes();
        let temp = (self.temp_c as i16 + TEMP_C_BIAS) as u8;
        (sid, [0, 0, temp, 0, vh, vl, vh, vl])
    }
}

impl Default for Cmu {
    /// A healthy pack.
    fn default() -> Self {
        Cmu::new(3700, 25)
    }
}

impl Part for Cmu {
    fn update(&mut self, chip: &mut System, _bus: &CanBus) {
        let (sid, data) = self.report();
        chip.inject_can1(CMU_RX_SLOT, sid, &data);
        self.next += 1;
    }
}

const CNTP_FB_PORT: u32 = 0x0080_0703; // P3DATA b1 = main-contactor "closed" (CNTP)
const PRECHARGE_FB_PORT: u32 = 0x0080_0709; // P9DATA b1 = precharge-complete
const CONTACTOR_FB_BIT: u8 = 0x02; // bit1 on each port

pub struct Contactor {
    pub closed: bool,
}

impl Default for Contactor {
    fn default() -> Self {
        Contactor { closed: true }
    }
}

impl Part for Contactor {
    fn update(&mut self, chip: &mut System, _bus: &CanBus) {
        let level = if self.closed { CONTACTOR_FB_BIT } else { 0 };
        chip.set_gpio_input(CNTP_FB_PORT, CONTACTOR_FB_BIT, level);
    }
}

const HV_UP_PORT: u32 = 0x0080_0707; // ECU P7DATA
const HV_UP_BIT: u8 = 0x10; // b4 = HV-start command (firmware output)

/// Charged reading: ~360 V pack
const CONDENSER_FULL_RAW: u16 = 0x0333;
/// Precharge is complete once the cap is near full (main contactor may then close).
const PRECHARGE_DONE_RAW: u16 = 0x0266; // ~3/4 of full
// Per-step fraction (num/den of the remaining distance) toward the target: a gentle
// RC charge through the current-limit resistor, a faster passive discharge.
const CHARGE_NUM: u32 = 1;
const CHARGE_DEN: u32 = 8;
const DISCHARGE_NUM: u32 = 1;
const DISCHARGE_DEN: u32 = 4;

#[derive(Default)]
struct CondenserModel {
    raw: u16,
}

impl CondenserModel {
    fn step(&mut self, charging: bool) {
        let (target, num, den) = if charging {
            (CONDENSER_FULL_RAW, CHARGE_NUM, CHARGE_DEN)
        } else {
            (0, DISCHARGE_NUM, DISCHARGE_DEN)
        };
        let diff = target as i32 - self.raw as i32;
        if diff == 0 {
            return;
        }
        // At least one count of progress, so it fully settles rather than creeping.
        let mag = (diff.unsigned_abs() * num / den).max(1) as i32;
        let delta = if diff < 0 { -mag } else { mag };
        self.raw = (self.raw as i32 + delta).clamp(0, CONDENSER_FULL_RAW as i32) as u16;
    }
}

#[derive(Default)]
pub struct Condenser {
    model: CondenserModel,
}

const PRECHARGE_MASTER_STATE: u32 = 0x0080_e5ac; // 0 REST / 5 precharge-request / 2 precharge / 6 HV-active

impl Part for Condenser {
    fn update(&mut self, chip: &mut System, _bus: &CanBus) {
        let hv_up = chip.gpio_level(HV_UP_PORT) & HV_UP_BIT != 0;
        let precharging = chip.peek(PRECHARGE_MASTER_STATE, 1) != 0;
        self.model.step(hv_up || precharging);
        chip.adc_mut().set_channel(ecu_adc::CONDENSER, self.model.raw);
        let precharged = self.model.raw >= PRECHARGE_DONE_RAW;
        let fb = if precharged { CONTACTOR_FB_BIT } else { 0 };
        chip.set_gpio_input(PRECHARGE_FB_PORT, CONTACTOR_FB_BIT, fb);
    }
}

const SHIFT_MAIN_PORT: u32 = 0x0080_0704; // P4DATA — shift switch matrix (main channel)
const SHIFT_SUB_PORT: u32 = 0x0080_0702; // P2DATA — shift switch matrix (sub channel)
const SHIFT_MATRIX_MASK: u8 = 0x3f; // six position switches, active-low
const IGNITION_PORT: u32 = 0x0080_0709; // P9DATA
const IGNITION_ON_BITS: u8 = 0x60; // IG1 + ST (start) asserted
const RELAY_SENSE_PORT: u32 = 0x0080_0700; // P0DATA
const RELAY_SENSE_BIT: u8 = 0x40; // P0.6 = EV-control-relay-commanded-on sense
const P1_KEY_PORT: u32 = 0x0080_0701; // P1DATA
const P1_KEY_BIT: u8 = 0x20; // P1.5, asserted with the key

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Gear {
    Park,
    Reverse,
    Neutral,
    Drive,
    Eco,
    Comfort,
}

impl Gear {
    fn matrix(self) -> u8 {
        let bit = match self {
            Gear::Park => 5,
            Gear::Reverse => 4,
            Gear::Neutral => 3,
            Gear::Drive => 2,
            Gear::Eco => 1,
            Gear::Comfort => 0,
        };
        SHIFT_MATRIX_MASK & !(1 << bit)
    }
}

pub struct DriverControls {
    pub gear: Gear,
    pub key_on: bool,
}

impl Default for DriverControls {
    fn default() -> Self {
        DriverControls { gear: Gear::Park, key_on: true }
    }
}

impl Part for DriverControls {
    fn update(&mut self, chip: &mut System, _bus: &CanBus) {
        let m = self.gear.matrix();
        chip.set_gpio_input(SHIFT_MAIN_PORT, SHIFT_MATRIX_MASK, m);
        chip.set_gpio_input(SHIFT_SUB_PORT, SHIFT_MATRIX_MASK, m);
        let key = |bits| if self.key_on { bits } else { 0 };
        chip.set_gpio_input(IGNITION_PORT, IGNITION_ON_BITS, key(IGNITION_ON_BITS));
        chip.set_gpio_input(RELAY_SENSE_PORT, RELAY_SENSE_BIT, key(RELAY_SENSE_BIT));
        chip.set_gpio_input(P1_KEY_PORT, P1_KEY_BIT, key(P1_KEY_BIT));
    }
}

const IC2_Q_MARKER: u32 = 0x0080_8249; // & 0xf0 = slot family (0x10 / 0x30)
const IC2_Q_ID: u32 = 0x0080_824a; // message id being asked
const IC2_Q_DATA: u32 = 0x0080_824b; // the data byte the question carries
const IC2_TX_SLOTS: u32 = 0x0080_81ac; // 30 entries × 4: [id, marker|state, byte2, byte3]
const IC2_STARTUP_STATE: u32 = 0x0080_825a; // 0->4->0xFFFF(POST done)
const IC2_TX_DONE_PC: u32 = 0x0001_31ac; // handler reached when a question frame is fully sent

#[derive(Default)]
pub struct Ic2Companion {
    armed_watch: bool,
    question_sent: bool,
}

impl Ic2Companion {
    fn reply(chip: &System) -> [u8; 5] {
        let marker = chip.peek(IC2_Q_MARKER, 1) as u8 & 0xf0;
        let id = chip.peek(IC2_Q_ID, 1) as u8;

        let mut slot_data = chip.peek(IC2_Q_DATA, 1) as u8;
        for s in 0..30u32 {
            let base = IC2_TX_SLOTS + s * 4;
            let state = chip.peek(base + 1, 1) as u8 & 0x0f;
            if chip.peek(base, 1) as u8 == id && (state == 3 || state == 4) {
                slot_data = chip.peek(base + 2, 1) as u8;
                break;
            }
        }

        let post_post = chip.peek(IC2_STARTUP_STATE, 1) as u8 == 0xff;
        let payload = if post_post && id == 0x01 {
            [0x00, 0x01, 0x21, 0x00] // id-1 status word (== flash constant)
        } else if id == 0x1f {
            [0x11, 0x00, 0x00, 0x00] // status ack (sets the group ack bits)
        } else if marker == 0x30 {
            [0x25, id, slot_data, 0xAA] // per-id ack for a 0x30-family slot
        } else {
            [0x35, id, slot_data, 0xAA] // per-id ack for a 0x10-family slot
        };

        let mut sum = 0u32;
        for &b in &payload {
            sum += b as u32;
            sum = (sum & 0xff) + (sum >> 8);
        }
        let chk = !sum as u8;
        [payload[0], payload[1], payload[2], payload[3], chk]
    }
}

impl Part for Ic2Companion {
    fn update(&mut self, chip: &mut System, _bus: &CanBus) {
        if !self.armed_watch {
            chip.watch_pc(IC2_TX_DONE_PC);
            self.armed_watch = true;
        }
        if chip.take_pc_hit() {
            self.question_sent = true;
        }
        if self.question_sent && !chip.ic2_rx_pending() && chip.ic2_take_rx_armed() {
            self.question_sent = false;
            let frame = Self::reply(chip);
            chip.ic2_answer(&frame);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ECU_OPERATING_MODE: u32 = 0x0080_dd4e; // 0 REST/2 PRECHARGE/3 READY/4 DRIVE/5 SHUTDOWN
    const OP_MODE_PRECHARGE: u32 = 2;

    #[test]
    fn frame_truncates_to_can_limit() {
        let f = frame(0x374, &[9; 12]);
        assert_eq!(f.dlc, 8);
        assert_eq!(f.data, [9; 8]);
    }

    #[test]
    fn imiev_ecu_completes_post() {
        let mut sim = Simulation::imiev();
        sim.run(16_000_000);
        let ss = sim.node(1).system().peek(IC2_STARTUP_STATE, 2);
        assert_eq!(ss, 0xffff, "ECU did not complete POST in the stock co-sim");
    }

    #[test]
    fn imiev_ecu_reaches_precharge() {
        let mut sim = Simulation::imiev();
        let mut reached_precharge = false;
        for _ in 0..160 {
            sim.run(100_000);
            if sim.node(1).system().peek(ECU_OPERATING_MODE, 1) == OP_MODE_PRECHARGE {
                reached_precharge = true;
                break;
            }
        }
        assert!(reached_precharge, "ECU never reached PRECHARGE (mode machine stalled in REST)");
    }

    #[test]
    fn ic2_handshake_completes_post() {
        let mut ecu = System::new(ECU_FW);
        for &(ch, raw) in ECU_BOOT_ADC {
            ecu.adc_mut().set_channel(ch, raw);
        }
        let mut ic2 = Ic2Companion::default();
        let bus = CanBus::default();
        for _ in 0..8_000_000u64 {
            ecu.step();
            ic2.update(&mut ecu, &bus);
            if ecu.peek(IC2_STARTUP_STATE, 2) == 0xffff {
                break;
            }
        }
        assert_eq!(
            ecu.peek(IC2_STARTUP_STATE, 2),
            0xffff,
            "IC2 handshake did not complete POST (startup_state != 0xFFFF)"
        );
    }

    #[test]
    fn imiev_bmu_broadcasts_on_the_bus() {
        let mut sim = Simulation::imiev();
        sim.run(16_000_000);
        let f = sim
            .bus()
            .last(0x373)
            .expect("BMU never broadcast 0x373 onto the bus");
        assert!(f.data.iter().any(|&b| b != 0), "0x373 payload all zero");
    }

    #[test]
    fn imiev_bmu_records_cmu_cell_voltage() {
        const CELL_V: u32 = 0x0080_7f37; // board array entry 0, cell voltage (BE, raw)
        let mut sim = Simulation::imiev();
        sim.run(16_000_000);
        let recorded = sim.node(0).system().peek(CELL_V, 2);
        assert_eq!(recorded, 0x0140, "BMU did not record the 3.7 V cells the CMUs reported");
    }

    #[test]
    fn contactor_presents_closed_feedback_to_the_ecu() {
        let ecu = Node::new("EV-ECU", ECU_FW).with_part(Box::new(Contactor::default()));
        let mut sim = Simulation::new(vec![ecu], 1_000);
        sim.run(2_000); // a couple of bus pumps
        let ecu = sim.node(0).system();
        assert_eq!(ecu.gpio_level(CNTP_FB_PORT) & CONTACTOR_FB_BIT, CONTACTOR_FB_BIT);
    }

    #[test]
    fn condenser_charges_and_discharges() {
        let mut c = CondenserModel::default();
        // Charging settles fully at the pack voltage.
        for _ in 0..200 {
            c.step(true);
        }
        assert_eq!(c.raw, CONDENSER_FULL_RAW);
        // Deasserting bleeds it all the way back to zero.
        for _ in 0..200 {
            c.step(false);
        }
        assert_eq!(c.raw, 0);
    }

    #[test]
    fn condenser_precharge_sequence() {
        let mut ecu = System::new(ECU_FW);
        let bus = CanBus::default();
        let mut cond = Condenser::default();

        // At REST (no HV-start) the condenser sits discharged.
        cond.update(&mut ecu, &bus);
        assert_eq!(ecu.adc().channel(ecu_adc::CONDENSER), 0);
        assert_eq!(ecu.gpio_level(PRECHARGE_FB_PORT) & CONTACTOR_FB_BIT, 0);

        // Command HV-start (P7.4): the cap ramps up and precharge completes.
        ecu.set_gpio_input(HV_UP_PORT, HV_UP_BIT, HV_UP_BIT);
        for _ in 0..200 {
            cond.update(&mut ecu, &bus);
        }
        assert_eq!(ecu.adc().channel(ecu_adc::CONDENSER), CONDENSER_FULL_RAW);
        assert_eq!(ecu.gpio_level(PRECHARGE_FB_PORT) & CONTACTOR_FB_BIT, CONTACTOR_FB_BIT);

        // Release HV-start: it bleeds back down and precharge-complete drops.
        ecu.set_gpio_input(HV_UP_PORT, HV_UP_BIT, 0);
        for _ in 0..200 {
            cond.update(&mut ecu, &bus);
        }
        assert_eq!(ecu.adc().channel(ecu_adc::CONDENSER), 0);
        assert_eq!(ecu.gpio_level(PRECHARGE_FB_PORT) & CONTACTOR_FB_BIT, 0);
    }
}
