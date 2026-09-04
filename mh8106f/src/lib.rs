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

use m32r_emulator::{Bus, Cpu};

pub mod periph;
pub use periph::{Adc, CanFrame, CanModule, Gpio, Ic2, Icu, Timer};
use periph::Peripheral;

const SIO23_IVECT: u16 = 0x00ec;
const DMA59_IVECT: u16 = 0x00e8;
const IC2_RX_BUFFER: u32 = 0x0080_824e;

const CAN0_BASE: u32 = 0x0080_1000;
const CAN1_BASE: u32 = 0x0080_1400;
const CAN1_RX_IVECT: u16 = 0x0110;

const ITOP10CR: u32 = 0x0080_0077;
const SLOW_TICK_REQ: u8 = 0x10;
const SLOW_TICK_IVECT: u16 = 0x00b0;

pub const FLASH_BASE: u32 = 0x0000_0000;
pub const FLASH_SIZE: u32 = 0x0010_0000; // 1 MB
pub const RAM_BASE: u32 = 0x0080_0000;
pub const RAM_END: u32 = 0x0081_4000; // exclusive
pub const RAM_LEN: usize = (RAM_END - RAM_BASE) as usize;
pub const SFR_END: u32 = 0x0080_4000;
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
            // CAN0 RX isn't modeled yet, so its RX IVECT is a placeholder.
            can0: CanModule::new(CAN0_BASE, 0),
            can1: CanModule::new(CAN1_BASE, CAN1_RX_IVECT),
            last_unclaimed_sfr_read: 0,
        }
    }

    /// Advance on-chip time by `cycles` and route any timer request into the ICU.
    pub fn tick(&mut self, cycles: u64) {
        self.adc.tick(cycles);
        self.ic2.tick(cycles);
        if let Some(iv) = self.timer.advance(cycles) {
            self.icu.raise(iv);
        }
    }

    #[inline]
    fn ram_off(&self, a: u32) -> Option<usize> {
        (RAM_BASE..RAM_END).contains(&a).then(|| (a - RAM_BASE) as usize)
    }

    fn take_slow_tick_request(&mut self) -> bool {
        if self.icu.icr_raw(ITOP10CR) & SLOW_TICK_REQ != 0 {
            self.icu.icr_clear(ITOP10CR, SLOW_TICK_REQ);
            true
        } else {
            false
        }
    }

    fn raise_fast_tick_subsource(&mut self) {
        self.timer.raise_topis(0);
    }

    fn ic2_service_tx(&mut self) {
        if self.ic2.take_tx_event() {
            self.ic2.set_tx_complete();
            self.icu.raise(SIO23_IVECT);
        }
    }

    fn ic2_deliver_rx(&mut self, bytes: &[u8]) {
        for (i, &b) in bytes.iter().enumerate() {
            if let Some(off) = self.ram_off(IC2_RX_BUFFER + i as u32) {
                self.ram[off] = b;
            }
        }
        self.ic2.set_dma5_complete();
        self.icu.raise(DMA59_IVECT);
    }

    fn peek(&self, a: u32, size: u32) -> u32 {
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

/// An M32R core bound to its [`Machine`], with EIT interrupt delivery.
pub struct System {
    cpu: Cpu,
    mem: Machine,
    interrupts_taken: u64,
    pc_watch: Option<u32>,
    pc_hit: bool,
}

impl System {
    pub fn new(firmware: &[u8]) -> System {
        System {
            cpu: Cpu::new(),
            mem: Machine::new(firmware),
            interrupts_taken: 0,
            pc_watch: None,
            pc_hit: false,
        }
    }

    pub fn watch_pc(&mut self, addr: u32) {
        self.pc_watch = Some(addr);
    }

    pub fn take_pc_hit(&mut self) -> bool {
        core::mem::take(&mut self.pc_hit)
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
        self.mem.can0.deliver_rx(id, data).is_some()
    }

    pub fn inject_can1(&mut self, slot: u32, sid: u16, data: &[u8]) {
        let iv = self.mem.can1.deliver_rx_into(slot, sid, data);
        self.mem.icu.raise(iv);
    }

    pub fn ic2_take_rx_armed(&mut self) -> bool {
        self.mem.ic2.take_rx_armed()
    }

    pub fn ic2_answer(&mut self, bytes: &[u8]) {
        self.mem.ic2_deliver_rx(bytes);
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
        if self.pc_watch == Some(self.cpu.pc) {
            self.pc_hit = true;
        }
        // Deliver the hardware way: present the source IVECT at 0x800000, then
        // vector through the EIT entry to the firmware dispatcher.
        if self.cpu.in_eit == 0 && self.cpu.interrupts_enabled() {
            self.mem.ic2_service_tx();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn flash_is_read_only_ram_is_writable() {
        let mut m = Machine::new(&[0x12, 0x34, 0x56, 0x78]);
        // Flash reads back the image, big-endian.
        assert_eq!(m.r32(0), 0x1234_5678);
        m.w32(0, 0xdead_beef); // dropped
        assert_eq!(m.r32(0), 0x1234_5678);

        // RAM proper is read/write.
        m.w32(0x804000, 0xcafe_babe);
        assert_eq!(m.r32(0x804000), 0xcafe_babe);
    }

    #[test]
    fn sfr_addresses_route_to_devices() {
        let mut m = Machine::new(&[]);
        // Timer TOPCEN (0x8002fe) is a device register, not backing RAM.
        m.w16(periph::timer::TOPCEN, 1);
        assert!(m.timer.is_enabled());
        // ICU vector register (0x800000) is a device register.
        m.w8(0x0080_0074, 0x04);
        m.icu.raise(0x00bc);
        m.icu.deliver();
        assert_eq!(m.r16(periph::icu::IVECT), 0x00bc);
    }

    #[test]
    fn unclaimed_sfr_addr_is_ram_scratch() {
        let mut m = Machine::new(&[]);
        // 0x800600 is inside the SFR block but claimed by no modeled device -> scratch.
        m.w32(0x800600, 0x0011_2233);
        assert_eq!(m.r32(0x800600), 0x0011_2233);
    }

    #[test]
    fn reset_vector_is_bra_to_handler() {
        // flash[0] = BRA; executing it from pc=0 should jump into the handler.
        let fw = include_bytes!("../../firmware/bmu.bin");
        let mut sys = System::new(fw);
        assert_eq!(sys.cpu.pc, 0);
        sys.step();
        assert_eq!(sys.cpu.pc, 0x3944, "BMU reset handler");
    }

    /// Make sure the BMU boots
    #[test]
    fn bmu_runs_under_timer_icu() {
        // Landmarks along the organic boot->tick->scheduler path.
        const DISPATCHER: u32 = 0x3994; // flash[0x80] BRA target
        const DEMUX: u32 = 0x8ce4; // sched_group_demux_fast

        let fw = include_bytes!("../../firmware/bmu.bin");
        let mut sys = System::new(fw);

        let mut armed = false;
        let mut reached_dispatcher = false;
        let mut reached_demux = false;
        const MAX_STEPS: u64 = 20_000_000;
        for i in 0..MAX_STEPS {
            armed |= sys.mem.timer.is_enabled();
            reached_dispatcher |= sys.cpu.pc == DISPATCHER;
            reached_demux |= sys.cpu.pc == DEMUX;
            if !sys.step() {
                panic!("decode failure at pc={:#010x} (step {i})", sys.cpu.pc);
            }
            // Success as soon as the first tick has been delivered and the
            // scheduler demux has been entered from it.
            if sys.interrupts_taken >= 1 && reached_demux {
                break;
            }
        }

        assert!(armed, "firmware never armed TOP0 (TOPCEN) organically");
        assert!(
            sys.interrupts_taken >= 1,
            "no TOP0 tick was delivered via the ICU/EIT"
        );
        assert!(reached_dispatcher, "interrupt did not vector into the dispatcher");
        assert!(
            reached_demux,
            "tick did not reach the fast-tick scheduler demux (0x8ce4)"
        );
        // Stack pointer was set into high RAM by the reset handler (R15/SPI fix).
        assert!(
            (0x808000..=RAM_END).contains(&sys.cpu.r[15]),
            "SP not initialized into RAM: {:#010x}",
            sys.cpu.r[15]
        );
    }

    #[test]
    fn bmu_adc_only_broadcasts_battery_frames() {
        let fw = include_bytes!("../../firmware/bmu.bin");
        let mut sys = System::new(fw);
        for (ch, v) in [(0usize, 0x300u16), (4, 0x300), (1, 0x330), (2, 0x200), (3, 0x200), (9, 0x800), (0xB, 0x800)] {
            sys.mem.adc.set_channel(ch, v);
        }
        let mut d373: Option<[u8; 8]> = None;
        for _ in 0..40_000_000u64 {
            sys.step();
            for f in sys.mem.can0.take_tx() {
                if f.id == 0x373 {
                    d373 = Some(f.data);
                }
            }
            if d373.is_some() {
                break;
            }
        }
        let d = d373.unwrap_or_else(|| panic!("BMU never broadcast 0x373 (taken={})", sys.interrupts_taken));
        assert!(d.iter().any(|&b| b != 0), "0x373 payload all zero: {d:02x?}");
        // Multiple ticks serviced (proves the ISR now returns via RTE, not hangs).
        assert!(sys.interrupts_taken > 1, "scheduler not running (taken={})", sys.interrupts_taken);
    }

    #[test]
    fn bmu_arms_its_own_rx_mailboxes() {
        let fw = include_bytes!("../../firmware/bmu.bin");
        let mut sys = System::new(fw);
        for (ch, v) in [(0usize, 0x300u16), (4, 0x300), (1, 0x330), (2, 0x200), (3, 0x200), (9, 0x800), (0xB, 0x800)] {
            sys.mem.adc.set_channel(ch, v);
        }
        for _ in 0..16_000_000u64 {
            sys.step();
        }
        let armed: Vec<u16> = sys.mem.can0.rx_slots().iter().map(|&(_, sid, _)| sid).collect();
        // The firmware armed the vehicle-bus frames it consumes (key ON, inverter…).
        for sid in [0x424u16, 0x412, 0x288, 0x286, 0x285, 0x01c] {
            assert!(armed.contains(&sid), "firmware did not arm RX for 0x{sid:x}");
        }
        // A frame now routes itself with no slot hint.
        assert!(sys.inject_can0(0x412, &[0x04, 0, 0, 0, 0, 0, 0, 0]));
        assert!(!sys.inject_can0(0x321, &[0; 8]), "unarmed SID must be dropped");
    }

    /// Make sure the EV-ECU boots
    #[test]
    fn ecu_boots_and_reaches_dispatcher() {
        const RESET_HANDLER: u32 = 0x393c; // flash[0] BRA target (ECU)
        const DISPATCHER: u32 = 0x398c; // flash[0x80] BRA target (ECU)

        let fw = include_bytes!("../../firmware/ev-ecu.bin");
        let mut sys = System::new(fw);

        // Reset vector branches to the ECU reset handler.
        assert_eq!(sys.cpu.pc, 0);
        sys.step();
        assert_eq!(sys.cpu.pc, RESET_HANDLER, "ECU reset handler");

        let mut reached_dispatcher = false;
        const MAX_STEPS: u64 = 20_000_000;
        for i in 0..MAX_STEPS {
            reached_dispatcher |= sys.cpu.pc == DISPATCHER;
            if !sys.step() {
                panic!("decode failure at pc={:#010x} (step {i})", sys.cpu.pc);
            }
            if sys.interrupts_taken >= 1 && reached_dispatcher {
                break;
            }
        }

        assert!(
            reached_dispatcher && sys.interrupts_taken >= 1,
            "ECU interrupt path did not reach the dispatcher (taken={})",
            sys.interrupts_taken
        );
        assert!(
            (0x808000..=RAM_END).contains(&sys.cpu.r[15]),
            "ECU SP not initialized into RAM: {:#010x}",
            sys.cpu.r[15]
        );
    }

    #[test]
    fn bmu_battery_model_unblocks_with_cmu_frames() {
        const BOARD_ARRAY: u32 = 0x0080_7f30; // can1_store_cell destination
        const CMU_RX_SLOT: u32 = 30;

        let fw = include_bytes!("../../firmware/bmu.bin");
        let mut sys = System::new(fw);

        // A CMU response frame:
        // data[2]=tempC+50
        // data[4:5]
        // data[6:7]=two cells each (V-2.1)*200
        //
        // 3.7V -> 320
        // 25C -> 75
        let [vh, vl] = 320u16.to_be_bytes();
        let frame = [0u8, 0, 75, 0, vh, vl, vh, vl];
        let mut sids = Vec::new();
        for board in 1..=12u16 {
            for cell in [1u16, 3, 5, 7] {
                sids.push(0x600 | (board << 4) | cell);
            }
        }

        let mut si = 0usize;
        let mut board_array_written = false;
        const MAX_STEPS: u64 = 12_000_000;
        for i in 0..MAX_STEPS {
            if sys.cpu.in_eit == 0 && sys.mem.icu.pending().is_none() {
                sys.inject_can1(CMU_RX_SLOT, sids[si % sids.len()], &frame);
                si += 1;
            }
            if !sys.step() {
                panic!("decode failure at pc={:#010x} (step {i})", sys.cpu.pc);
            }
            board_array_written |= sys.peek(BOARD_ARRAY, 4) != 0;
            if board_array_written && sys.interrupts_taken >= 100 {
                break;
            }
        }

        assert!(
            board_array_written,
            "CAN1 RX path not delivering"
        );
        assert!(
            sys.interrupts_taken >= 100,
            "battery model may still be hanging (interrupts_taken={})",
            sys.interrupts_taken
        );
    }

    #[test]
    fn bmu_records_injected_cell_voltage() {
        const CELL_V0: u32 = 0x0080_7f30 + 7; // board array entry 0, data[4:5]
        const CMU_RX_SLOT: u32 = 30;

        fn record(vraw: u16) -> u16 {
            let fw = include_bytes!("../../firmware/bmu.bin");
            let mut sys = System::new(fw);
            let [vh, vl] = vraw.to_be_bytes();
            let frame = [0u8, 0, 75, 0, vh, vl, vh, vl];
            let mut sids = Vec::new();
            for board in 1..=12u16 {
                for cell in [1u16, 3, 5, 7] {
                    sids.push(0x600 | (board << 4) | cell);
                }
            }
            let mut si = 0usize;
            for _ in 0..8_000_000u64 {
                if sys.cpu.in_eit == 0 && sys.mem.icu.pending().is_none() {
                    sys.inject_can1(CMU_RX_SLOT, sids[si % sids.len()], &frame);
                    si += 1;
                }
                sys.step();
                if sys.peek(CELL_V0, 2) as u16 == vraw {
                    break; // recorded the injected value
                }
            }
            sys.peek(CELL_V0, 2) as u16
        }

        // 3.7 V -> 320, 3.9 V -> 360.
        assert_eq!(record(320), 320, "board array should record 3.7 V");
        assert_eq!(record(360), 360, "board array should record 3.9 V (tracks, not constant)");
    }
}
