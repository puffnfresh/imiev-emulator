//! Simulated components of the i-MiEV.

use std::collections::BTreeMap;

use mh8106f::{CanFrame, System};

mod adc;
mod parts;

#[cfg(test)]
mod tests;

use adc::{BMU_BOOT_ADC, EV_ECU_BOOT_ADC};
pub use parts::*;

const BMU_FW: &[u8] = include_bytes!("../../firmware/bmu.bin");
const EV_ECU_FW: &[u8] = include_bytes!("../../firmware/ev-ecu.bin");

const CAN_DLC_MAX: usize = 8;

const BUS_PUMP_INTERVAL: u64 = 10_000;

const ECU_MODE_STATUS_CODE: u32 = 0x0080_e595; // drive-engagement handshake stage (climbs to 4 then 5)
const EV_ECU_OPERATING_MODE: u32 = 0x0080_dd4e; // 0 REST/2 PRECHARGE/3 READY/4 DRIVE/5 SHUTDOWN

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

pub struct Simulation {
    bmu: Node,
    ev_ecu: Node,
    driver: DriverControls,
    sources: Vec<Box<dyn BusSource>>,
    inverter: Inverter,
    bus: CanBus,
    pump_every: u64,
    cycle: u64,
}

impl Simulation {
    pub fn imiev() -> Simulation {
        let bmu = Node::new("BMU", BMU_FW)
            .with_adc_env(BMU_BOOT_ADC)
            .with_part(Box::new(Cmu::default()));
        let ev_ecu = Node::new("EV-ECU", EV_ECU_FW)
            .with_adc_env(EV_ECU_BOOT_ADC)
            .with_part(Box::new(Condenser::default()))
            .with_local_part(Box::new(Ic2Companion::default()))
            .with_local_part(Box::new(Can0RxIsr::default()));
        Simulation {
            bmu,
            ev_ecu,
            driver: DriverControls::default(),
            sources: vec![Box::new(Vehicle)],
            inverter: Inverter::default(),
            bus: CanBus::default(),
            pump_every: BUS_PUMP_INTERVAL,
            cycle: 0,
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

    pub fn ecu_op_mode(&self) -> u32 {
        self.ev_ecu.system().peek(EV_ECU_OPERATING_MODE, 1)
    }

    pub fn ecu_mode_code(&self) -> u32 {
        self.ev_ecu.system().peek(ECU_MODE_STATUS_CODE, 1)
    }

    pub fn run(&mut self, steps: u64) {
        for _ in 0..steps {
            {
                let Simulation { bmu, ev_ecu, bus, .. } = &mut *self;
                for n in [bmu, ev_ecu] {
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

    pub fn warm_bmu(&mut self, steps: u64) {
        for _ in 0..steps {
            {
                let Simulation { bmu, bus, .. } = &mut *self;
                bmu.step();
                bmu.update_local_parts(bus);
            }
            self.cycle += 1;
            if self.cycle.is_multiple_of(self.pump_every) {
                self.pump();
            }
        }
    }

    fn pump(&mut self) {
        let Simulation { bmu, ev_ecu, driver, sources, inverter, bus, .. } = self;
        let mut tx: Vec<CanFrame> = Vec::new();
        tx.append(&mut bmu.drain_tx());
        tx.append(&mut ev_ecu.drain_tx());
        for s in sources.iter_mut() {
            tx.extend(s.frames(bus));
        }
        let ecu = ev_ecu.system();
        let code = ecu.peek(ECU_MODE_STATUS_CODE, 1);
        let op_mode = ecu.peek(EV_ECU_OPERATING_MODE, 1);
        inverter.drive_dclink = code >= 4 || op_mode >= 4;
        inverter.engaging = code >= 4 && op_mode < 4;
        tx.extend(inverter.frames(bus));
        for f in &tx {
            bus.record(*f);
            bmu.deliver(f);
            ev_ecu.deliver(f);
        }
        bmu.update_parts(bus);
        ev_ecu.update_parts(bus);
        driver.update(ev_ecu.system_mut(), bus);
    }
}
