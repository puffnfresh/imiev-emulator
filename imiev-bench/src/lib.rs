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

/// One chip on the vehicle CAN bus.
pub struct Node {
    pub name: &'static str,
    sys: System,
}

impl Node {
    pub fn new(name: &'static str, firmware: &[u8]) -> Node {
        Node {
            name,
            sys: System::new(firmware),
        }
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

pub trait Component {
    fn frames(&mut self, _bus: &CanBus) -> Vec<CanFrame> {
        Vec::new()
    }
    fn drive(&mut self, _nodes: &mut [Node], _bus: &CanBus) {}
}

/// Simulation of several [`Node`]s on one [`CanBus`], with [`Component`]s.
pub struct Simulation {
    nodes: Vec<Node>,
    components: Vec<Box<dyn Component>>,
    bus: CanBus,
    pump_every: u64,
    cycle: u64,
}

impl Simulation {
    pub fn new(nodes: Vec<Node>, pump_every: u64) -> Simulation {
        Simulation {
            nodes,
            components: Vec::new(),
            bus: CanBus::default(),
            pump_every: pump_every.max(1),
            cycle: 0,
        }
    }

    pub fn imiev() -> Simulation {
        let mut bmu = Node::new("BMU", BMU_FW);
        for &(ch, raw) in BMU_BOOT_ADC {
            bmu.set_adc(ch, raw);
        }

        let mut ecu = Node::new("EV-ECU", ECU_FW);
        for &(ch, raw) in ECU_BOOT_ADC {
            ecu.set_adc(ch, raw);
        }

        let mut sim = Simulation::new(vec![bmu, ecu], BUS_PUMP_INTERVAL);
        sim.components.push(Box::new(Vehicle::default()));
        sim
    }

    pub fn with_component(mut self, component: Box<dyn Component>) -> Self {
        self.components.push(component);
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
            for n in &mut self.nodes {
                n.step();
            }
            self.cycle += 1;
            if self.cycle % self.pump_every == 0 {
                self.pump();
            }
        }
    }

    fn pump(&mut self) {
        let Simulation { nodes, components, bus, .. } = self;
        let mut tx: Vec<CanFrame> = Vec::new();
        for n in nodes.iter_mut() {
            tx.append(&mut n.drain_tx());
        }
        for c in components.iter_mut() {
            tx.extend(c.frames(bus));
        }
        for f in &tx {
            bus.record(*f);
            for n in nodes.iter_mut() {
                n.deliver(f);
            }
        }
        for c in components.iter_mut() {
            c.drive(nodes, bus);
        }
    }
}

const ETACS_STATUS_ID: u16 = 0x412;
const ETACS_IGNITION_ON: u8 = 0x04;

#[derive(Default)]
pub struct Vehicle;

impl Component for Vehicle {
    fn frames(&mut self, _bus: &CanBus) -> Vec<CanFrame> {
        vec![frame(ETACS_STATUS_ID, &[ETACS_IGNITION_ON, 0, 0, 0, 0, 0, 0, 0])]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_truncates_to_can_limit() {
        let f = frame(0x374, &[9; 12]);
        assert_eq!(f.dlc, 8);
        assert_eq!(f.data, [9; 8]);
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
}
