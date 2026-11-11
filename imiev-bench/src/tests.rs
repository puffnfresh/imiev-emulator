//! End-to-end tests: drive a whole [`Simulation`].

use super::*;
use crate::dtc::Code;

const OP_MODE_PRECHARGE: u32 = 2;
const OP_MODE_READY: u32 = 3;
const OP_MODE_DRIVE: u32 = 4;
const OP_MODE_SHUTDOWN: u32 = 5;

#[test]
fn frame_truncates_to_can_limit() {
    let f = frame(0x374, &[9; 12]);
    assert_eq!(f.dlc, 8);
    assert_eq!(f.data, [9; 8]);
}

#[test]
fn imiev_ev_ecu_reaches_precharge() {
    let mut sim = Simulation::imiev();
    let mut reached_precharge = false;
    for _ in 0..160 {
        sim.run(100_000);
        if sim.ecu_op_mode() == OP_MODE_PRECHARGE {
            reached_precharge = true;
            break;
        }
    }
    assert!(reached_precharge, "ECU never reached PRECHARGE (mode machine stalled in REST)");
}

#[test]
fn imiev_ev_ecu_charges_the_dc_link_in_precharge() {
    const DC_LINK_HALF_VOLTS: usize = 4;
    let mut sim = Simulation::imiev();
    let mut charging = false;
    for _ in 0..400 {
        sim.run(200_000);
        let dc_link = sim.bus().last(0x285).map_or(0, |f| f.data[DC_LINK_HALF_VOLTS]);
        if sim.ecu_op_mode() == OP_MODE_PRECHARGE && dc_link > 20 {
            charging = true;
            break;
        }
    }
    assert!(charging, "the EV-ECU never reported the DC link charging during PRECHARGE");
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
fn imiev_bmu_broadcasts_full_battery_frame_set() {
    let mut sim = Simulation::imiev();
    sim.run(45_000_000);
    for id in [0x373u16, 0x374, 0x375] {
        let f = sim
            .bus()
            .last(id)
            .unwrap_or_else(|| panic!("BMU never broadcast 0x{id:03x}"));
        assert!(f.data.iter().any(|&b| b != 0), "0x{id:03x} payload all zero");
    }
}

#[test]
fn imiev_bmu_nvm_clears_p1a51() {
    let mut sim = Simulation::imiev();
    sim.run(45_000_000);
    assert!(!sim.bmu_dtcs().iter().any(|d| d == "P1A51"), "P1A51 confirmed: the BMU EEPROM NVM check failed");
}

#[test]
fn imiev_bmu_reports_the_cmus_cell_voltages() {
    const CELL_3V7: [u8; 2] = [0x01, 0x40];
    let mut sim = Simulation::imiev();
    sim.run(16_000_000);
    let cells = sim.bus().last(0x6e1).map(|f| [f.data[4], f.data[5], f.data[6], f.data[7]]);
    assert_eq!(cells, Some([CELL_3V7[0], CELL_3V7[1], CELL_3V7[0], CELL_3V7[1]]), "BMU did not pass on the CMUs' 3.7 V cells");
}

#[test]
fn imiev_ev_ecu_holds_precharge_without_false_undervoltage() {
    let mut sim = Simulation::imiev();
    sim.run(16_000_000);
    assert!(!sim.ecu_dtcs().iter().any(|d| d == "P0562"), "P0562 false-latched while the condenser was still precharging");
    assert_ne!(sim.ecu_op_mode(), OP_MODE_SHUTDOWN, "ECU fell into SHUTDOWN during precharge");
}

#[test]
fn imiev_ev_ecu_reaches_ready() {
    let mut sim = Simulation::imiev();
    sim.run(75_000_000);
    assert_eq!(sim.ecu_op_mode(), OP_MODE_READY, "operating mode did not reach READY");
}

#[test]
fn imiev_ev_ecu_reaches_and_holds_drive() {
    let mut sim = Simulation::imiev();
    sim.run(250_000_000);
    assert_eq!(sim.ecu_op_mode(), OP_MODE_DRIVE, "operating mode did not reach DRIVE");
    assert!(!sim.ecu_dtcs().iter().any(|d| d == "P1B2C"), "P1B2C stuck-before-drive watchdog confirmed");
}

#[test]
fn imiev_aps_test_pulse_does_not_confirm_p060b() {
    let mut sim = Simulation::imiev();
    sim.run(400_000_000);
    assert!(
        !sim.ecu_dtcs().iter().any(|d| d == "P060B"),
        "P060B confirmed: accelerator sensor 2 did not drop during the P1.0 test pulse"
    );
}

#[test]
fn imiev_full_throttle_holds_without_aps_overrange() {
    let mut sim = Simulation::imiev();
    sim.run(250_000_000);
    sim.set_gear(Gear::Drive);
    sim.set_pedal(100.0);
    sim.run(130_000_000); // well past the APS over-range debounce
    let dtcs = sim.ecu_dtcs();
    assert!(!dtcs.iter().any(|d| d == "P2123"), "P2123 (APS1 over-range) confirmed at full throttle");
    assert!(!dtcs.iter().any(|d| d == "P2128"), "P2128 (APS2 over-range) confirmed at full throttle");
}

#[test]
fn imiev_selecting_drive_engages_and_moves() {
    let mut sim = Simulation::imiev();
    sim.run(250_000_000);
    assert_eq!(sim.ecu_op_mode(), OP_MODE_DRIVE, "never reached DRIVE to test motion");
    let rpm = |sim: &Simulation| sim.bus().last(0x298).map_or(0, |f| u16::from_be_bytes([f.data[6], f.data[7]]) as i32 - 10_000);
    let rpm0 = rpm(&sim);
    sim.set_gear(Gear::Drive);
    sim.set_pedal(60.0);
    sim.run(20_000_000);
    assert_eq!(sim.ecu_op_mode(), OP_MODE_DRIVE, "selecting Drive collapsed the operating mode (handshake broke)");
    let rpm1 = rpm(&sim);
    assert!(rpm1 > rpm0, "vehicle did not accelerate after selecting Drive ({rpm0} -> {rpm1} rpm)");
}

#[test]
fn imiev_drive_gear_reaches_the_ev_ecu() {
    let mut sim = Simulation::imiev();
    sim.run(250_000_000);
    sim.set_gear(Gear::Drive);
    sim.set_pedal(60.0);
    sim.run(30_000_000);
    let gear = sim.bus().last(0x418).map(|f| f.data[0]);
    assert_eq!(gear, Some(b'D'), "the EV-ECU does not see the lever in D");
    let torque = sim.bus().last(0x285).map(|f| u16::from_be_bytes([f.data[0], f.data[1]]));
    assert!(torque.is_some_and(|t| t > 2000), "the EV-ECU commands no drive torque on 0x285 ({torque:?})");
}

#[test]
fn imiev_holds_drive_under_torque() {
    let mut sim = Simulation::imiev();
    sim.run(300_000_000);
    sim.set_gear(Gear::Drive);
    sim.set_pedal(40.0);
    sim.run(3 * CYCLES_PER_SECOND);
    let dtcs = sim.ecu_dtcs();
    assert_eq!(sim.ecu_op_mode(), 4, "dropped out of DRIVE under torque; confirmed: {dtcs:?}");
    for (dtc, why) in [
        ("P1A1A", "the inverter's reported torque disagrees with the command"),
        ("C200A", "the vacuum pump main relay's switched side doesn't follow it"),
        ("C200E", "the vacuum pump main relay's coil monitor doesn't follow it"),
    ] {
        assert_ecu_not_confirmed(&sim, dtc, why);
    }
    assert!(dtcs.is_empty(), "DTCs confirmed while driving: {dtcs:?}");
}

#[test]
fn imiev_idles_in_ready_without_dtcs() {
    let mut sim = Simulation::imiev();
    sim.run(14 * CYCLES_PER_SECOND);
    let dtcs = sim.ecu_dtcs();
    assert_eq!(sim.ecu_op_mode(), 4, "fell out of READY; confirmed: {dtcs:?}");
    for (dtc, why) in [
        ("P1B0B", "no vehicle speed from the ABS (0x215)"),
        ("P1B24", "no rear-left wheel speed from the ABS (0x208)"),
        ("U1110", "no heater/A-C compressor status (0x384)"),
        ("U111D", "the OBC power-supply relay is on, but there is no OBC on CAN"),
        ("P1A10", "the OBC power-supply relay's contact disagrees with it"),
        ("C2016", "the vacuum pump's motor sense is high with its control relays off"),
        ("C201A", "the vacuum pump's motor sense drops at once when it is switched off"),
        ("C2022", "the booster's vacuum does not move while the pump runs"),
    ] {
        assert_ecu_not_confirmed(&sim, dtc, why);
    }
    assert!(dtcs.is_empty(), "DTCs confirmed idling in READY: {dtcs:?}");
    let bmu = sim.bmu_dtcs();
    for (dtc, why) in [
        ("P1AB5", "CMU01 reports a temperature below -30 C (an unfed sensor reads -50)"),
        ("P1A8E", "CMU01's balancing status (b1 of its first response) is not 0"),
        ("P1AC3", "the battery cooling fan's feedback is high while it is at rest"),
        ("P1A44", "the leak detector's alarm (P9.4) is standing"),
        ("P1A46", "the leak detector's alarm ignores the BMU's self-test (P9.1)"),
    ] {
        assert_eq!(bmu_dtc(&sim, dtc) & CONFIRMED, 0, "{dtc} confirmed: {why}");
    }
}

#[test]
#[should_panic(expected = "the EV-ECU has no DTC P0000")]
fn ecu_dtc_rejects_a_code_the_firmware_lacks() {
    ecu_dtc(&Simulation::imiev(), "P0000");
}

#[test]
fn imiev_holds_every_drive_position() {
    let mut sim = Simulation::imiev();
    sim.run(300_000_000);
    sim.set_pedal(30.0);
    for (gear, name) in [(Gear::Eco, "B"), (Gear::Reverse, "R"), (Gear::Comfort, "C")] {
        sim.set_gear(gear);
        sim.run(2 * CYCLES_PER_SECOND);
        let dtcs = sim.ecu_dtcs();
        assert_eq!(sim.ecu_op_mode(), 4, "shut down in {name}; confirmed: {dtcs:?}");
        assert_ecu_not_confirmed(&sim, "P1A0F", &format!("{name}: the backlamp relay's switched side doesn't follow it"));
        assert!(dtcs.is_empty(), "DTCs confirmed in {name}: {dtcs:?}");
    }
}

#[test]
fn a_remote_bmu_runs_as_the_simulation_s_own() {
    const CYCLES: u64 = CYCLES_PER_SECOND;
    let ram = |s: &mh8106f::System| (0x80_0000u32..0x81_4000).step_by(4).map(|a| s.peek(a, 4)).collect::<Vec<_>>();
    let mut whole = Simulation::imiev();
    whole.run(CYCLES);
    let mut ecu = Simulation::imiev();
    let mut bmu = RemoteBmu::imiev();
    for _ in 0..CYCLES / BUS_PUMP_INTERVAL {
        ecu.run_ev_ecu_to_pump();
        bmu.run_to_pump();
        let tx = ecu.pump_with_remote_bmu(bmu.drain_tx());
        bmu.receive(&tx);
    }
    assert!(ram(ecu.ev_ecu().system()) == ram(whole.ev_ecu().system()), "the EV-ECU diverged");
    assert!(ram(bmu.node().system()) == ram(whole.bmu().system()), "the BMU diverged");
    assert!(bmu.node().system().cpu().pc == whole.bmu().system().cpu().pc);
}

fn with_ev_ecu(ev_ecu: &EvEcu) -> Simulation {
    Simulation::new(ev_ecu, &firmware::BMU_9499B11500)
}

fn with_bmu(bmu: &Bmu) -> Simulation {
    Simulation::new(&firmware::EV_ECU_9499A18206, bmu)
}

fn ev_ecu_completes_post(ev_ecu: &EvEcu) {
    let mut sim = with_ev_ecu(ev_ecu);
    sim.run(16_000_000);
    assert!(sim.bus().last(0x285).is_some(), "{} did not complete POST: no status on 0x285", ev_ecu.part);
}

fn ev_ecu_reaches_ready(mut sim: Simulation, part: &str) {
    sim.run(250_000_000);
    let mode = sim.ecu_op_mode();
    assert!(
        (OP_MODE_READY..=OP_MODE_DRIVE).contains(&mode),
        "{part}: operating mode {mode}; DTCs {:?}",
        sim.ecu_dtcs()
    );
}

fn bmu_broadcasts(bmu: &Bmu) {
    let mut sim = with_bmu(bmu);
    sim.run(45_000_000);
    for id in [0x373u16, 0x374, 0x375] {
        let f = sim.bus().last(id).unwrap_or_else(|| panic!("{} never sent {id:03x}", bmu.part));
        assert!(f.data.iter().any(|&b| b != 0), "{}: {id:03x} all zero", bmu.part);
    }
    let confirmed = sim.bmu_dtcs();
    assert!(!confirmed.iter().any(|code| code == "P1A51"), "{}: {confirmed:?}", bmu.part);
}

macro_rules! every_ev_ecu {
    ($($module:ident: $build:path),* $(,)?) => {$(
        mod $module {
            use super::*;

            #[test]
            fn completes_post() {
                ev_ecu_completes_post(&$build);
            }

            #[test]
            fn reaches_ready() {
                ev_ecu_reaches_ready(with_ev_ecu(&$build), $build.part);
            }
        }
    )*};
}

macro_rules! every_bmu {
    ($($module:ident: $build:path),* $(,)?) => {$(
        mod $module {
            use super::*;

            #[test]
            fn broadcasts_the_battery() {
                bmu_broadcasts(&$build);
            }
        }
    )*};
}

every_ev_ecu! {
    ev_ecu_9499a18206: firmware::EV_ECU_9499A18206,
    ev_ecu_9499b13103: firmware::EV_ECU_9499B13103,
    ev_ecu_9499a43902: firmware::EV_ECU_9499A43902,
}

every_bmu! {
    bmu_9499b11500: firmware::BMU_9499B11500,
    bmu_9499a84200: firmware::BMU_9499A84200,
}

#[test]
fn czero_pair_reaches_ready() {
    let sim = Simulation::new(&firmware::EV_ECU_9499A43902, &firmware::BMU_9499A84200);
    ev_ecu_reaches_ready(sim, "C-Zero pair");
}

fn drives_without_dtcs(mut sim: Simulation, part: &str) {
    sim.run(6 * CYCLES_PER_SECOND);
    let dtcs = sim.ecu_dtcs();
    assert_eq!(sim.ecu_op_mode(), OP_MODE_DRIVE, "{part}: fell out of READY; confirmed: {dtcs:?}");
    for (dtc, why) in [
        ("P0630", "no chassis number in the EEPROM"),
        ("P1B2E", "no key code registered over 0x591"),
        ("U1117", "the ETACS never answered the immobiliser's challenge"),
        ("P1B1B", "no brake-stroke sensor supply on ADC channel 14"),
        ("P1B32", "no chassis number from the ETACS on 0x29a"),
        ("P1B35", "no steering angle on 0x236"),
    ] {
        assert!(!dtcs.iter().any(|d| d == dtc), "{part}: {dtc}: {why}");
    }
    assert!(dtcs.is_empty(), "{part}: DTCs confirmed idling: {dtcs:?}");
    assert_eq!(sim.bus().last(0x101).map(|f| f.data[0]), Some(0x04), "{part}: immobiliser not unlocked");

    sim.set_gear(Gear::Drive);
    sim.set_pedal(40.0);
    sim.run(3 * CYCLES_PER_SECOND);
    let dtcs = sim.ecu_dtcs();
    assert_eq!(sim.ecu_op_mode(), OP_MODE_DRIVE, "{part}: dropped out of DRIVE; confirmed: {dtcs:?}");
    assert!(dtcs.is_empty(), "{part}: DTCs confirmed driving: {dtcs:?}");
    let kmh = sim.bus().last(0x412).map_or(0, |f| f.data[1]);
    assert!(kmh >= 5, "{part}: only {kmh} km/h after 3 s at 40% pedal");
}

#[test]
fn czero_pair_drives_without_dtcs() {
    drives_without_dtcs(Simulation::new(&firmware::EV_ECU_9499A43902, &firmware::BMU_9499A84200), "C-Zero pair");
}

#[test]
fn ev_ecu_9499b13103_drives_without_dtcs() {
    drives_without_dtcs(with_ev_ecu(&firmware::EV_ECU_9499B13103), "9499B13103");
}

#[test]
fn every_build_is_in_the_registry() {
    let mut parts: Vec<&str> = firmware::EV_ECUS.iter().map(|b| b.part).chain(firmware::BMUS.iter().map(|b| b.part)).collect();
    parts.sort();
    parts.dedup();
    assert_eq!(parts.len(), firmware::EV_ECUS.len() + firmware::BMUS.len());
    for b in firmware::EV_ECUS {
        assert_eq!(b.image.len(), 0x10_0000, "{}", b.part);
        assert!(b.image.windows(10).any(|w| w == b.part.as_bytes()), "{} not in its image", b.part);
    }
    for b in firmware::BMUS {
        assert_eq!(b.image.len(), 0x10_0000, "{}", b.part);
    }
}
