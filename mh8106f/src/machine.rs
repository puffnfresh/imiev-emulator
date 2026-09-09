//! The chip's memory system and peripheral bus: flash, internal RAM, the SFR
//! address decode that routes accesses to each modeled [`Peripheral`], and the
//! on-chip time tick that services the ADC/IC2/CAN and raises timer interrupts.
//!
//! [`Machine`] implements [`Bus`] for the CPU; the interrupt-delivery wrapper
//! around it is [`System`](crate::System).

use m32r_emulator::Bus;

use crate::periph::Peripheral;
use crate::{be_read, be_write, Adc, CanModule, Gpio, Ic2, Icu, Timer};
use crate::{FLASH_SIZE, RAM_BASE, RAM_END, RAM_LEN, SFR_END};

const SIO23_IVECT: u16 = 0x00ec;
const DMA59_IVECT: u16 = 0x00e8;
const IC2_RX_BUFFER: u32 = 0x0080_824e;

const CAN0_BASE: u32 = 0x0080_1000;
const CAN1_BASE: u32 = 0x0080_1400;
const CAN0_TR_IVECT: u16 = 0x010c; // CAN0 Transmit/Receive interrupt (flash[0x807c])
const CAN1_RX_IVECT: u16 = 0x0110;

const ITOP10CR: u32 = 0x0080_0077;
const SLOW_TICK_REQ: u8 = 0x10;

const FLASH_ERASED: u8 = 0xff;

pub(crate) struct Machine {
    flash: Vec<u8>,
    /// Backs the `0x800000..0x814000` span (SFR scratch + internal RAM).
    ram: Vec<u8>,
    pub timer: Timer,
    pub icu: Icu,
    pub adc: Adc,
    pub gpio: Gpio,
    pub ic2: Ic2,
    pub can0: CanModule,
    pub can1: CanModule,
    pub last_unclaimed_sfr_read: u32,
    pub(crate) wwatch: Option<(u32, u32)>,
    pub(crate) cur_pc: u32,
    pub(crate) wwatch_hits: Vec<(u32, u32, u32)>,
}

impl Machine {
    /// Build a machine from a firmware image (padded/truncated to 1 MB of flash).
    pub fn new(firmware: &[u8]) -> Machine {
        let mut flash = vec![FLASH_ERASED; FLASH_SIZE as usize];
        let n = firmware.len().min(FLASH_SIZE as usize);
        flash[..n].copy_from_slice(&firmware[..n]);
        Machine {
            flash,
            ram: vec![0u8; RAM_LEN],
            timer: Timer::new(),
            icu: Icu::new(),
            adc: Adc::new(),
            gpio: Gpio::new(),
            ic2: Ic2::new(),
            can0: CanModule::new(CAN0_BASE, CAN0_TR_IVECT),
            can1: CanModule::new(CAN1_BASE, CAN1_RX_IVECT),
            last_unclaimed_sfr_read: 0,
            wwatch: None,
            cur_pc: 0,
            wwatch_hits: Vec::new(),
        }
    }

    /// Advance on-chip time by `cycles` and route any timer request into the ICU.
    pub fn tick(&mut self, cycles: u64) {
        self.adc.tick(cycles);
        self.ic2.tick(cycles);
        self.ic2_service_tx();
        self.ic2_service_rx();
        if let Some(iv) = self.can0.take_tx_irq() {
            self.icu.raise(iv);
        }
        if let Some(iv) = self.timer.advance(cycles) {
            self.icu.raise(iv);
        }
    }

    #[inline]
    fn ram_off(&self, a: u32) -> Option<usize> {
        (RAM_BASE..RAM_END).contains(&a).then(|| (a - RAM_BASE) as usize)
    }

    pub(crate) fn take_slow_tick_request(&mut self) -> bool {
        if self.icu.icr_raw(ITOP10CR) & SLOW_TICK_REQ != 0 {
            self.icu.icr_clear(ITOP10CR, SLOW_TICK_REQ);
            true
        } else {
            false
        }
    }

    pub(crate) fn raise_fast_tick_subsource(&mut self) {
        self.timer.raise_topis(0);
    }

    fn ic2_service_tx(&mut self) {
        if self.ic2.take_tx_event() {
            self.ic2.set_tx_complete();
            self.icu.raise(SIO23_IVECT);
        }
    }

    pub(crate) fn ic2_deliver_rx(&mut self, bytes: &[u8]) {
        self.ic2.queue_rx(bytes);
    }

    fn ic2_service_rx(&mut self) {
        if let Some((buf, len)) = self.ic2.take_ready_rx() {
            for (i, &b) in buf.iter().take(len).enumerate() {
                if let Some(off) = self.ram_off(IC2_RX_BUFFER + i as u32) {
                    self.ram[off] = b;
                }
            }
            self.ic2.set_dma5_complete();
            self.icu.raise(DMA59_IVECT);
        }
    }

    pub(crate) fn peek(&self, a: u32, size: u32) -> u32 {
        if let Some(off) = self.ram_off(a) {
            return be_read(&self.ram, off, size);
        }
        if (a as usize) < self.flash.len() {
            return be_read(&self.flash, a as usize, size);
        }
        0
    }

    fn devices(&mut self) -> [&mut dyn Peripheral; 7] {
        [
            &mut self.timer,
            &mut self.icu,
            &mut self.adc,
            &mut self.gpio,
            &mut self.ic2,
            &mut self.can0,
            &mut self.can1,
        ]
    }

    fn read(&mut self, a: u32, size: u32) -> u32 {
        for dev in self.devices() {
            if dev.handles(a) {
                return dev.read(a, size);
            }
        }
        if (RAM_BASE..SFR_END).contains(&a) {
            self.last_unclaimed_sfr_read = a;
        }
        if let Some(off) = self.ram_off(a) {
            return be_read(&self.ram, off, size);
        }
        if (a as usize) < self.flash.len() {
            return be_read(&self.flash, a as usize, size);
        }
        0 // open bus
    }

    fn write(&mut self, a: u32, size: u32, v: u32) {
        if let Some((lo, hi)) = self.wwatch {
            let overlaps = a <= hi && a + size.saturating_sub(1) >= lo;
            if overlaps {
                self.wwatch_hits.push((self.cur_pc, a, v));
            }
        }
        for dev in self.devices() {
            if dev.handles(a) {
                return dev.write(a, size, v);
            }
        }
        if let Some(off) = self.ram_off(a) {
            be_write(&mut self.ram, off, size, v)
        }
        // Flash is read-only; out-of-range writes are dropped.
    }
}

impl Bus for Machine {
    fn r8(&mut self, a: u32) -> u8 {
        self.read(a, 1) as u8
    }
    fn w8(&mut self, a: u32, v: u8) {
        self.write(a, 1, v as u32);
    }
    fn r16(&mut self, a: u32) -> u16 {
        self.read(a, 2) as u16
    }
    fn w16(&mut self, a: u32, v: u16) {
        self.write(a, 2, v as u32);
    }
    fn r32(&mut self, a: u32) -> u32 {
        self.read(a, 4)
    }
    fn w32(&mut self, a: u32, v: u32) {
        self.write(a, 4, v);
    }
}
