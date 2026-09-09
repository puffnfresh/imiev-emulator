//! Memory-map and bus-routing checks plus firmware boot tests that exercise the
//! full [`System`] (CPU + [`Machine`](crate::machine::Machine) + interrupts)
//! against the real BMU/EV-ECU images.

use m32r_emulator::Bus;

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
    // The firmware armed the vehicle-bus frames it consumes (key ON, inverter, etc).
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
