//! Simulated components of the i-MiEV.

use std::collections::BTreeMap;

use mh8106f::System;

mod adc;
pub mod dtc;
pub mod firmware;
pub mod layout;
mod parts;

#[cfg(test)]
mod tests;

use adc::{BMU_BOOT_ADC, EV_ECU_BOOT_ADC};
pub use dtc::DtcTable;
pub use firmware::{Bmu, EvEcu};
pub use layout::EvEcuLayout;
pub use mh8106f::CanFrame;
pub use parts::*;

#[cfg(test)]
const BMU_FW: &[u8] = firmware::BMU_9499B11500.image;
#[cfg(test)]
const EV_ECU_FW: &[u8] = firmware::EV_ECU_9499A18206.image;

const CAN_DLC_MAX: usize = 8;

const BUS_PUMP_INTERVAL: u64 = 10_000;

pub const CYCLES_PER_SECOND: u64 = 160_000_000;


pub fn frame(id: u16, data: &[u8]) -> CanFrame {
    let mut buf = [0u8; CAN_DLC_MAX];
    let n = data.len().min(CAN_DLC_MAX);
    buf[..n].copy_from_slice(&data[..n]);
    CanFrame { id, dlc: n as u8, data: buf }
}

pub trait Part: Send {
    fn update(&mut self, chip: &mut System, bus: &CanBus);

    fn polling(&self) -> bool {
        false
    }
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
    local_seen: Option<u64>,
    local_polling: bool,
}

impl Node {
    pub fn new(name: &'static str, firmware: &[u8]) -> Node {
        Node {
            name,
            sys: System::new(firmware),
            parts: Vec::new(),
            local_parts: Vec::new(),
            local_seen: None,
            local_polling: false,
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

    fn run_steps(&mut self, n: u64, bus: &CanBus) {
        for _ in 0..n {
            self.step();
            self.update_local_parts(bus);
        }
    }

    fn update_parts(&mut self, bus: &CanBus) {
        let Node { sys, parts, .. } = self;
        for p in parts.iter_mut() {
            p.update(sys, bus);
        }
    }

    fn update_local_parts(&mut self, bus: &CanBus) {
        let Node { sys, local_parts, local_seen, local_polling, .. } = self;
        if !*local_polling && *local_seen == Some(sys.activity()) {
            return;
        }
        *local_polling = false;
        for p in local_parts.iter_mut() {
            p.update(sys, bus);
            *local_polling |= p.polling();
        }
        *local_seen = Some(sys.activity());
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

pub struct Simulation {
    bmu: Node,
    ev_ecu: Node,
    driver: DriverControls,
    sources: Vec<Box<dyn BusSource>>,
    inverter: Inverter,
    bus: CanBus,
    pump_every: u64,
    cycle: u64,
    layout: EvEcuLayout,
    bmu_dtc: DtcTable,
}

impl Simulation {
    pub fn imiev() -> Simulation {
        Self::new(&firmware::EV_ECU_9499A18206, &firmware::BMU_9499B11500)
    }

    pub fn new(ev_ecu: &EvEcu, bmu: &Bmu) -> Simulation {
        let layout = ev_ecu.layout;
        let ev_ecu = Node::new("EV-ECU", ev_ecu.image)
            .with_adc_env(EV_ECU_BOOT_ADC)
            .with_part(Box::new(Condenser::default()))
            .with_part(Box::new(Contactor))
            .with_part(Box::new(AcRelay))
            .with_part(Box::new(VacuumPump::default()))
            .with_part(Box::new(AirbagEcu::default()))
            .with_part(Box::new(Charger))
            .with_part(Box::new(BacklampRelay))
            .with_local_part(Box::new(Ic2Companion::default()));
        Simulation {
            bmu: bmu_node(bmu),
            ev_ecu,
            driver: DriverControls::default(),
            sources: vec![Box::new(Vehicle)],
            inverter: Inverter::default(),
            bus: CanBus::default(),
            pump_every: BUS_PUMP_INTERVAL,
            cycle: 0,
            layout,
            bmu_dtc: bmu.dtc,
        }
    }

    pub fn bus(&self) -> &CanBus {
        &self.bus
    }
    pub fn bmu(&self) -> &Node {
        &self.bmu
    }
    pub fn ev_ecu(&self) -> &Node {
        &self.ev_ecu
    }
    pub fn bmu_mut(&mut self) -> &mut Node {
        &mut self.bmu
    }
    pub fn ev_ecu_mut(&mut self) -> &mut Node {
        &mut self.ev_ecu
    }

    pub fn set_gear(&mut self, gear: Gear) {
        self.driver.gear = gear;
    }

    pub fn set_pedal(&mut self, pct: f32) {
        self.driver.pedal_pct = pct.clamp(0.0, 100.0);
    }

    pub fn set_brake(&mut self, pct: f32) {
        self.driver.brake_pct = pct.clamp(0.0, 100.0);
    }

    pub fn ecu_op_mode(&self) -> u32 {
        self.ev_ecu.system().peek(self.layout.operating_mode, 1)
    }

    pub fn ecu_mode_code(&self) -> u32 {
        self.ev_ecu.system().peek(self.layout.mode_status_code, 1)
    }

    pub fn ecu_torque_request(&self) -> i32 {
        self.layout.torque_request.map_or(0, |a| self.ev_ecu.system().peek(a, 2) as i16 as i32)
    }

    pub fn ecu_dtcs(&self) -> Vec<(String, bool)> {
        dtc::scan(self.ev_ecu.system(), &self.layout.dtc)
    }

    pub fn bmu_dtcs(&self) -> Vec<(String, bool)> {
        dtc::scan(self.bmu.system(), &self.bmu_dtc)
    }

    pub fn layout(&self) -> &EvEcuLayout {
        &self.layout
    }

    pub fn run(&mut self, steps: u64) {
        self.run_chunks(steps, |sim, n| {
            let Simulation { bmu, ev_ecu, bus, .. } = sim;
            run_both(bmu, ev_ecu, n, bus);
        });
    }

    pub fn warm_bmu(&mut self, steps: u64) {
        self.run_chunks(steps, |sim, n| sim.bmu.run_steps(n, &sim.bus));
    }

    fn run_chunks(&mut self, steps: u64, mut step_nodes: impl FnMut(&mut Simulation, u64)) {
        let mut left = steps;
        while left > 0 {
            let n = left.min(self.pump_every - self.cycle % self.pump_every);
            step_nodes(self, n);
            self.cycle += n;
            left -= n;
            if self.cycle.is_multiple_of(self.pump_every) {
                self.pump();
            }
        }
    }

    fn pump(&mut self) {
        let bmu_tx = self.bmu.drain_tx();
        let tx = self.pump_ev_ecu(bmu_tx);
        RemoteBmu::receive_into(&mut self.bmu, &tx, &self.bus);
    }

    fn pump_ev_ecu(&mut self, bmu_tx: Vec<CanFrame>) -> Vec<CanFrame> {
        let Simulation { ev_ecu, driver, sources, inverter, bus, .. } = self;
        let mut tx = bmu_tx;
        tx.append(&mut ev_ecu.drain_tx());
        for s in sources.iter_mut() {
            tx.extend(s.frames(bus));
        }
        inverter.hv_connected = dclink_connected(ev_ecu.system());
        inverter.integrate(bus, driver.brake_pct / 100.0, VEHICLE_DT);
        tx.extend(inverter.frames(bus));
        for f in &tx {
            bus.record(*f);
            ev_ecu.deliver(f);
        }
        ev_ecu.update_parts(bus);
        driver.update(ev_ecu.system_mut(), bus);
        tx
    }

    pub fn run_ev_ecu_to_pump(&mut self) {
        let n = self.pump_every - self.cycle % self.pump_every;
        self.ev_ecu.run_steps(n, &self.bus);
        self.cycle += n;
    }

    pub fn pump_with_remote_bmu(&mut self, bmu_tx: Vec<CanFrame>) -> Vec<CanFrame> {
        self.pump_ev_ecu(bmu_tx)
    }
}

fn bmu_node(bmu: &Bmu) -> Node {
    Node::new("BMU", bmu.image)
        .with_adc_env(BMU_BOOT_ADC)
        .with_part(Box::new(Cmu::default()))
        .with_part(Box::new(LeakDetector))
        .with_local_part(Box::new(Eeprom::default()))
}

pub struct RemoteBmu {
    node: Node,
    bus: CanBus,
    cycle: u64,
    dtc: DtcTable,
}

impl RemoteBmu {
    pub fn imiev() -> RemoteBmu {
        Self::new(&firmware::BMU_9499B11500)
    }

    pub fn new(bmu: &Bmu) -> RemoteBmu {
        RemoteBmu { node: bmu_node(bmu), bus: CanBus::default(), cycle: 0, dtc: bmu.dtc }
    }

    pub fn run_to_pump(&mut self) {
        let n = BUS_PUMP_INTERVAL - self.cycle % BUS_PUMP_INTERVAL;
        self.node.run_steps(n, &self.bus);
        self.cycle += n;
    }

    pub fn drain_tx(&mut self) -> Vec<CanFrame> {
        self.node.drain_tx()
    }

    pub fn receive(&mut self, tx: &[CanFrame]) {
        for f in tx {
            self.bus.record(*f);
        }
        Self::receive_into(&mut self.node, tx, &self.bus);
    }

    fn receive_into(bmu: &mut Node, tx: &[CanFrame], bus: &CanBus) {
        for f in tx {
            bmu.deliver(f);
        }
        bmu.update_parts(bus);
    }

    pub fn node(&self) -> &Node {
        &self.node
    }

    pub fn dtcs(&self) -> Vec<(String, bool)> {
        dtc::scan(self.node.system(), &self.dtc)
    }
}

#[cfg(not(target_arch = "wasm32"))]
const PARALLEL_MIN_STEPS: u64 = 1_000;

#[cfg(not(target_arch = "wasm32"))]
fn run_both(bmu: &mut Node, ev_ecu: &mut Node, n: u64, bus: &CanBus) {
    if n < PARALLEL_MIN_STEPS {
        bmu.run_steps(n, bus);
        ev_ecu.run_steps(n, bus);
        return;
    }
    std::thread::scope(|s| {
        s.spawn(|| bmu.run_steps(n, bus));
        ev_ecu.run_steps(n, bus);
    });
}

#[cfg(target_arch = "wasm32")]
fn run_both(bmu: &mut Node, ev_ecu: &mut Node, n: u64, bus: &CanBus) {
    bmu.run_steps(n, bus);
    ev_ecu.run_steps(n, bus);
}
