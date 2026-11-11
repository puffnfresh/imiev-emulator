//! WASM wrapper around [`imiev_bench::Simulation`].

use imiev_bench::{firmware, CanFrame, Ecu, Gear as BenchGear, RemoteBmu, Simulation};
use wasm_bindgen::prelude::*;

const EV_ECU: &firmware::EvEcu = &firmware::EV_ECU_9499B13103;
const BMU: &firmware::Bmu = &firmware::BMU_9499B11500;

#[wasm_bindgen]
#[derive(Clone, Copy)]
pub enum Gear {
    Park,
    Reverse,
    Neutral,
    Drive,
    Eco,
    Comfort,
}

impl From<Gear> for BenchGear {
    fn from(g: Gear) -> BenchGear {
        match g {
            Gear::Park => BenchGear::Park,
            Gear::Reverse => BenchGear::Reverse,
            Gear::Neutral => BenchGear::Neutral,
            Gear::Drive => BenchGear::Drive,
            Gear::Eco => BenchGear::Eco,
            Gear::Comfort => BenchGear::Comfort,
        }
    }
}

#[wasm_bindgen]
pub struct Sim {
    inner: Simulation,
}

#[wasm_bindgen]
impl Sim {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Sim {
        Sim { inner: Simulation::new(EV_ECU, BMU) }
    }

    pub fn run(&mut self, steps: u32) {
        self.inner.run(steps as u64);
    }

    pub fn warm_bmu(&mut self, steps: u32) {
        self.inner.warm_bmu(steps as u64);
    }

    pub fn set_gear(&mut self, gear: Gear) {
        self.inner.set_gear(gear.into());
    }

    pub fn set_pedal(&mut self, pct: f32) {
        self.inner.set_pedal(pct);
    }

    pub fn set_brake(&mut self, pct: f32) {
        self.inner.set_brake(pct);
    }

    pub fn set_adc_bmu(&mut self, ch: u32, raw12: u16) {
        self.inner.bmu_mut().set_adc(ch as usize, raw12);
    }
    pub fn set_adc_ecu(&mut self, ch: u32, raw12: u16) {
        self.inner.ev_ecu_mut().set_adc(ch as usize, raw12);
    }

    pub fn inject_can0_bmu(&mut self, id: u16, data: &[u8]) -> bool {
        self.inner.bmu_mut().system_mut().inject_can0(id, data)
    }
    pub fn inject_can0_ecu(&mut self, id: u16, data: &[u8]) -> bool {
        self.inner.ev_ecu_mut().system_mut().inject_can0(id, data)
    }
    pub fn inject_can1_bmu(&mut self, slot: u32, sid: u16, data: &[u8]) {
        self.inner.bmu_mut().system_mut().inject_can1(slot, sid, data);
    }

    pub fn bus_last(&self, id: u16) -> Option<Vec<u8>> {
        self.inner
            .bus()
            .last(id)
            .map(|f| f.data[..f.dlc as usize].to_vec())
    }

    pub fn bus_ids(&self) -> Vec<u16> {
        self.inner.bus().ids().collect()
    }

    pub fn ecu_op_mode(&self) -> u32 {
        self.inner.ecu_op_mode()
    }

    pub fn ecu_torque_command(&self) -> f32 {
        self.inner.ecu_torque_command()
    }

    pub fn ecu_dtcs(&mut self) -> Vec<String> {
        fmt_dtcs(&mut self.inner, Ecu::EvEcu)
    }

    pub fn bmu_dtcs(&mut self) -> Vec<String> {
        fmt_dtcs(&mut self.inner, Ecu::Bmu)
    }
}

fn fmt_dtcs(sim: &mut Simulation, ecu: Ecu) -> Vec<String> {
    sim.request_dtcs();
    sim.last_dtcs(ecu).unwrap_or_default().iter().map(|d| d.to_string()).collect()
}

impl Default for Sim {
    fn default() -> Self {
        Sim::new()
    }
}

const FRAME_BYTES: usize = 11;

fn encode(frames: &[CanFrame]) -> Vec<u8> {
    let mut out = Vec::with_capacity(frames.len() * FRAME_BYTES);
    for f in frames {
        out.extend_from_slice(&f.id.to_be_bytes());
        out.push(f.dlc);
        out.extend_from_slice(&f.data);
    }
    out
}

fn decode(bytes: &[u8]) -> Vec<CanFrame> {
    bytes
        .chunks_exact(FRAME_BYTES)
        .map(|b| CanFrame { id: u16::from_be_bytes([b[0], b[1]]), dlc: b[2], data: b[3..11].try_into().unwrap() })
        .collect()
}

#[wasm_bindgen]
pub struct SplitEcu {
    inner: Simulation,
}

#[wasm_bindgen]
impl SplitEcu {
    #[wasm_bindgen(constructor)]
    pub fn new() -> SplitEcu {
        SplitEcu { inner: Simulation::new(EV_ECU, BMU) }
    }

    pub fn run_to_pump(&mut self) {
        self.inner.run_ev_ecu_to_pump();
    }

    pub fn pump(&mut self, bmu_tx: &[u8]) -> Vec<u8> {
        encode(&self.inner.pump_with_remote_bmu(decode(bmu_tx)))
    }

    pub fn set_gear(&mut self, gear: Gear) {
        self.inner.set_gear(gear.into());
    }

    pub fn set_pedal(&mut self, pct: f32) {
        self.inner.set_pedal(pct);
    }

    pub fn bus_last(&self, id: u16) -> Option<Vec<u8>> {
        self.inner.bus().last(id).map(|f| f.data[..f.dlc as usize].to_vec())
    }

    pub fn ecu_op_mode(&self) -> u32 {
        self.inner.ecu_op_mode()
    }
    pub fn ecu_torque_command(&self) -> f32 {
        self.inner.ecu_torque_command()
    }
    pub fn ecu_dtcs(&mut self) -> Vec<String> {
        fmt_dtcs(&mut self.inner, Ecu::EvEcu)
    }
    pub fn bmu_dtcs(&mut self) -> Vec<String> {
        fmt_dtcs(&mut self.inner, Ecu::Bmu)
    }
}

impl Default for SplitEcu {
    fn default() -> Self {
        SplitEcu::new()
    }
}

#[wasm_bindgen]
pub struct SplitBmu {
    inner: RemoteBmu,
}

#[wasm_bindgen]
impl SplitBmu {
    #[wasm_bindgen(constructor)]
    pub fn new() -> SplitBmu {
        SplitBmu { inner: RemoteBmu::new(BMU) }
    }

    pub fn run_to_pump(&mut self) {
        self.inner.run_to_pump();
    }

    pub fn drain_tx(&mut self) -> Vec<u8> {
        encode(&self.inner.drain_tx())
    }

    pub fn receive(&mut self, frames: &[u8]) {
        self.inner.receive(&decode(frames));
    }
}

impl Default for SplitBmu {
    fn default() -> Self {
        SplitBmu::new()
    }
}
