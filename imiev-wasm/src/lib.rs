//! WASM wrapper around [`imiev_bench::Simulation`].

use imiev_bench::{Gear as BenchGear, Simulation};
use wasm_bindgen::prelude::*;

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
        Sim { inner: Simulation::imiev() }
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

    pub fn bmu_peek(&self, addr: u32, size: u32) -> u32 {
        self.inner.bmu().system().peek(addr, size)
    }
    pub fn ecu_peek(&self, addr: u32, size: u32) -> u32 {
        self.inner.ev_ecu().system().peek(addr, size)
    }

    pub fn ecu_op_mode(&self) -> u32 {
        self.inner.ecu_op_mode()
    }
    pub fn ecu_mode_code(&self) -> u32 {
        self.inner.ecu_mode_code()
    }
}

impl Default for Sim {
    fn default() -> Self {
        Sim::new()
    }
}
