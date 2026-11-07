//! Model of the Renesas MH8106F
//! The i-MiEV's M32R-family SoC used in both the BMU and EV-ECU.
//!
//! There is no public MH8106F datasheet; the peripheral register semantics
//! are taken from the closest documented part (M32192).
//!
//! # Memory map
//!
//! | Region       | Range                      | Access                     |
//! |--------------|----------------------------|----------------------------|
//! | Flash (code) | `0x0000_0000..0x0010_0000` | read-only (writes dropped) |
//! | SFR block    | `0x0080_0000..0x0080_4000` | peripheral registers       |
//! | Internal RAM | `0x0080_4000..0x0081_4000` | read/write                 |
//!
//! [`Machine`] is the memory/peripheral bus; [`System`] wraps it with an M32R
//! [`Cpu`] and models EIT interrupt delivery.

use m32r_emulator::Cpu;

pub mod periph;
pub use periph::{Adc, CanFrame, CanModule, Gpio, Ic2, Icu, Timer};

mod machine;
use machine::Machine;

#[cfg(test)]
mod tests;

const SLOW_TICK_IVECT: u16 = 0x00b0;

pub const FLASH_BASE: u32 = 0x0000_0000;
pub const FLASH_SIZE: u32 = 0x0010_0000; // 1 MB
pub const RAM_BASE: u32 = 0x0080_0000;
pub const RAM_END: u32 = 0x0081_4000; // exclusive
pub const RAM_LEN: usize = (RAM_END - RAM_BASE) as usize;
pub const SFR_END: u32 = 0x0080_4000;

#[inline]
pub(crate) fn be_read(buf: &[u8], off: usize, size: u32) -> u32 {
    let mut v = 0u32;
    for i in 0..size as usize {
        v = (v << 8) | *buf.get(off + i).unwrap_or(&0) as u32;
    }
    v
}

#[inline]
pub(crate) fn be_write(buf: &mut [u8], off: usize, size: u32, v: u32) {
    for i in 0..size as usize {
        let shift = 8 * (size as usize - 1 - i);
        if let Some(b) = buf.get_mut(off + i) {
            *b = (v >> shift) as u8;
        }
    }
}

/// An M32R core bound to its [`Machine`], with EIT interrupt delivery.
pub struct System {
    cpu: Cpu,
    mem: Machine,
    interrupts_taken: u64,
}

impl System {
    pub fn new(firmware: &[u8]) -> System {
        System {
            cpu: Cpu::new(),
            mem: Machine::new(firmware),
            interrupts_taken: 0,
        }
    }

    pub fn set_write_watch(&mut self, range: Option<(u32, u32)>) {
        self.mem.wwatch = range;
        self.mem.wwatch_hits.clear();
    }

    pub fn take_write_hits(&mut self) -> Vec<(u32, u32, u32)> {
        core::mem::take(&mut self.mem.wwatch_hits)
    }

    pub fn cpu(&self) -> &Cpu {
        &self.cpu
    }
    pub fn interrupts_taken(&self) -> u64 {
        self.interrupts_taken
    }
    pub fn peek(&self, addr: u32, size: u32) -> u32 {
        self.mem.peek(addr, size)
    }
    pub fn timer(&self) -> &Timer {
        &self.mem.timer
    }
    pub fn icu(&self) -> &Icu {
        &self.mem.icu
    }
    pub fn adc(&self) -> &Adc {
        &self.mem.adc
    }

    pub fn adc_mut(&mut self) -> &mut Adc {
        &mut self.mem.adc
    }

    pub fn set_gpio_input(&mut self, addr: u32, mask: u8, value: u8) {
        self.mem.gpio.set_input(addr, mask, value);
    }

    pub fn gpio_output(&self, addr: u32) -> u8 {
        self.mem.gpio.output(addr)
    }

    pub fn gpio_level(&self, addr: u32) -> u8 {
        self.mem.gpio.pin_level(addr)
    }

    pub fn inject_can0(&mut self, id: u16, data: &[u8]) -> bool {
        if let Some(iv) = self.mem.can0.deliver_rx(id, data) {
            self.mem.icu.raise(iv);
            true
        } else {
            false
        }
    }

    pub fn inject_can1(&mut self, slot: u32, sid: u16, data: &[u8]) {
        let iv = self.mem.can1.deliver_rx_into(slot, sid, data);
        self.mem.icu.raise(iv);
    }

    pub fn ic2_take_rx_armed(&mut self) -> bool {
        self.mem.ic2.take_rx_armed()
    }

    pub fn ic2_rx_armed(&self) -> bool {
        self.mem.ic2.rx_armed()
    }

    pub fn ic2_answer(&mut self, bytes: &[u8]) {
        self.mem.ic2_deliver_rx(bytes);
    }

    pub fn ic2_take_tx(&mut self) -> Vec<u8> {
        self.mem.ic2.take_tx_bytes()
    }

    pub fn activity(&self) -> u64 {
        self.mem.activity
    }

    pub fn ic2_rx_pending(&self) -> bool {
        self.mem.ic2.rx_pending()
    }


    pub fn take_can0_tx(&mut self) -> Vec<CanFrame> {
        self.mem.can0.take_tx()
    }

    pub fn take_can1_tx(&mut self) -> Vec<CanFrame> {
        self.mem.can1.take_tx()
    }

    /// Advance one CPU step, delivering a pending interrupt first if the core can
    /// take one. Returns the CPU's `step` result (false only on decode failure).
    pub fn step(&mut self) -> bool {
        self.mem.cur_pc = self.cpu.pc;
        // Deliver the hardware way: present the source IVECT at 0x800000, then
        // vector through the EIT entry to the firmware dispatcher.
        if self.cpu.in_eit == 0 && self.cpu.interrupts_enabled() {
            // The chained slow tick takes priority over the fast tick.
            if self.mem.take_slow_tick_request() {
                self.mem.icu.present(SLOW_TICK_IVECT);
                self.cpu.take_interrupt(periph::icu::EI_VECTOR);
                self.interrupts_taken += 1;
                self.mem.tick(1);
                return true;
            }
            if let Some(iv) = self.mem.icu.deliver() {
                if iv == periph::timer::TICK_IVECT {
                    self.mem.raise_fast_tick_subsource();
                }
                self.cpu.take_interrupt(periph::icu::EI_VECTOR);
                self.interrupts_taken += 1;
                self.mem.tick(1);
                return true;
            }
        }
        let ok = self.cpu.step(&mut self.mem);
        self.mem.tick(1);
        ok
    }
}
