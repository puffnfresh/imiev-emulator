//! End-to-end tests: drive a whole [`Simulation`].

use super::*;
use crate::dtc::Code;

const IC2_STARTUP_STATE: u32 = 0x0080_825a; // 0->4->0xFFFF (POST done)
const EV_ECU_OPERATING_MODE: u32 = 0x0080_dd4e; // 0 REST/2 PRECHARGE/3 READY/4 DRIVE/5 SHUTDOWN
const OP_MODE_PRECHARGE: u32 = 2;
const CONFIRMED: u8 = 0x02;

fn ecu_dtc(sim: &Simulation, label: &str) -> u8 {
    dtc::status(sim.ev_ecu().system(), &dtc::ECU_DTC, Code::parse(label))
        .unwrap_or_else(|| panic!("the EV-ECU has no DTC {label}"))
}

fn bmu_dtc(sim: &Simulation, label: &str) -> u8 {
    dtc::status(sim.bmu().system(), &dtc::BMU_DTC, Code::parse(label))
        .unwrap_or_else(|| panic!("the BMU has no DTC {label}"))
}

fn confirmed_ecu_dtcs(sim: &Simulation) -> Vec<Code> {
    sim.ecu_dtcs().into_iter().filter(|d| d.1).map(|d| d.0).collect()
}

fn assert_ecu_not_confirmed(sim: &Simulation, label: &str, why: &str) {
    assert_eq!(ecu_dtc(sim, label) & CONFIRMED, 0, "{label} confirmed: {why}");
}

#[test]
fn frame_truncates_to_can_limit() {
    let f = frame(0x374, &[9; 12]);
    assert_eq!(f.dlc, 8);
    assert_eq!(f.data, [9; 8]);
}

#[test]
fn imiev_ev_ecu_completes_post() {
    let mut sim = Simulation::imiev();
    sim.run(8_000_000); // POST completes by ~4M; stay clear of the later precharge-timeout reset
    let ss = sim.ev_ecu().system().peek(IC2_STARTUP_STATE, 2);
    assert_eq!(ss, 0xffff, "ECU did not complete POST in the stock co-sim");
}

#[test]
fn imiev_ev_ecu_reaches_precharge() {
    let mut sim = Simulation::imiev();
    let mut reached_precharge = false;
    for _ in 0..160 {
        sim.run(100_000);
        if sim.ev_ecu().system().peek(EV_ECU_OPERATING_MODE, 1) == OP_MODE_PRECHARGE {
            reached_precharge = true;
            break;
        }
    }
    assert!(reached_precharge, "ECU never reached PRECHARGE (mode machine stalled in REST)");
}

#[test]
fn imiev_ev_ecu_scheduler_runs_and_condenser_ramps() {
    let mut sim = Simulation::imiev();
    let mut reached_precharge = false;
    let mut ramped = false;
    for _ in 0..400 {
        sim.run(200_000);
        let e = sim.ev_ecu().system();
        if e.peek(EV_ECU_OPERATING_MODE, 1) == OP_MODE_PRECHARGE {
            reached_precharge = true;
        }
        let cf60 = f32::from_bits(e.peek(0x0080_cf60, 4));
        if cf60 > 5.0 && e.peek(0x0080_c088, 1) == 1 {
            ramped = true;
            break;
        }
    }
    assert!(reached_precharge, "EV-ECU never reached PRECHARGE");
    assert!(ramped, "scheduler never ran: cf60 never smoothed up / condenser data never valid");
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
    assert_eq!(
        bmu_dtc(&sim, "P1A51") & CONFIRMED,
        0,
        "P1A51 confirmed — the BMU EEPROM NVM check failed"
    );
}

#[test]
fn imiev_bmu_records_cmu_cell_voltage() {
    const CELL_V: u32 = 0x0080_7f37; // board array entry 0, cell voltage (BE, raw)
    let mut sim = Simulation::imiev();
    sim.run(16_000_000);
    let recorded = sim.bmu().system().peek(CELL_V, 2);
    assert_eq!(recorded, 0x0140, "BMU did not record the 3.7V cells the CMUs reported");
}

#[test]
fn imiev_bmu_reaches_cmu_valid() {
    const VALIDITY_FLAG9: u32 = 0x0080_bef9;
    const CMU_DATA_VALID: u32 = 0x0080_bfbe;
    const CMU_COMMS_HEALTHY: u32 = 0x0080_d6be;
    const CMU_VALID: u32 = 0x0080_befa;
    let mut sim = Simulation::imiev();
    sim.run(45_000_000);
    let bmu = sim.bmu().system();
    assert_eq!(bmu.peek(VALIDITY_FLAG9, 1), 1, "sensor validity gate never latched");
    assert_eq!(bmu.peek(CMU_DATA_VALID, 1), 1, "cmu_data_valid never latched");
    assert_eq!(bmu.peek(CMU_COMMS_HEALTHY, 1), 1, "cmu_comms_healthy never latched");
    assert_eq!(bmu.peek(CMU_VALID, 1), 1, "cmu_valid never latched");
}

#[test]
fn imiev_ev_ecu_holds_precharge_without_false_undervoltage() {
    const OP_MODE_SHUTDOWN: u32 = 5;
    let mut sim = Simulation::imiev();
    sim.run(8_000_000); // through POST + into precharge, while cf60 is still ramping
    assert_ecu_not_confirmed(&sim, "P0562", "false-latched while the condenser was still precharging");
    assert_ne!(
        sim.ev_ecu().system().peek(EV_ECU_OPERATING_MODE, 1),
        OP_MODE_SHUTDOWN,
        "ECU fell into SHUTDOWN during precharge"
    );
}

#[test]
fn imiev_ev_ecu_reaches_ready() {
    const MODE_SUBSTATE: u32 = 0x0080_e590; // dispatcher verdict: 3 = READY
    const DATA_VALID_LATCH: u32 = 0x0080_d81e;
    const DRIVE_STATE_READY: u32 = 0x0080_d5a5;
    const PRECHARGE_MASTER_STATE: u32 = 0x0080_e5ac; // 6 = HV-active
    const OP_MODE_READY: u32 = 3;
    let mut sim = Simulation::imiev();
    sim.run(75_000_000);
    let e = sim.ev_ecu().system();
    assert_eq!(e.peek(DATA_VALID_LATCH, 1), 1, "data_valid never latched");
    assert_eq!(e.peek(DRIVE_STATE_READY, 1), 1, "drive_state_ready never debounced high");
    assert_eq!(e.peek(MODE_SUBSTATE, 1), OP_MODE_READY, "dispatcher did not declare READY");
    assert_eq!(e.peek(EV_ECU_OPERATING_MODE, 1), OP_MODE_READY, "operating_mode did not reach READY");
    assert_eq!(e.peek(PRECHARGE_MASTER_STATE, 1), 6, "precharge master did not reach HV-active");
}

#[test]
fn imiev_ev_ecu_reaches_and_holds_drive() {
    const OP_MODE_DRIVE: u32 = 4;
    let mut sim = Simulation::imiev();
    sim.run(250_000_000);
    let e = sim.ev_ecu().system();
    assert_eq!(e.peek(EV_ECU_OPERATING_MODE, 1), OP_MODE_DRIVE, "operating_mode did not reach DRIVE");
    assert_ne!(ecu_dtc(&sim, "P1B2C"), 0x82, "P1B2C stuck-before-drive watchdog confirmed");
}

#[test]
fn imiev_aps_test_pulse_does_not_confirm_p060b() {
    let mut sim = Simulation::imiev();
    sim.run(400_000_000);
    assert_ecu_not_confirmed(&sim, "P060B", "accelerator sensor 2 did not drop during the P1.0 test pulse");
}

#[test]
fn imiev_full_throttle_holds_without_aps_overrange() {
    let mut sim = Simulation::imiev();
    sim.run(250_000_000);
    sim.set_gear(Gear::Drive);
    sim.set_pedal(100.0);
    sim.run(130_000_000); // well past the APS over-range debounce
    assert_ecu_not_confirmed(&sim, "P2123", "APS1 over-range at full throttle");
    assert_ecu_not_confirmed(&sim, "P2128", "APS2 over-range at full throttle");
}

#[test]
fn imiev_selecting_drive_engages_and_moves() {
    const OP_MODE_DRIVE: u32 = 4;
    let mut sim = Simulation::imiev();
    sim.run(250_000_000);
    assert_eq!(
        sim.ev_ecu().system().peek(EV_ECU_OPERATING_MODE, 1),
        OP_MODE_DRIVE,
        "never reached DRIVE to test motion"
    );
    let rpm = |sim: &Simulation| sim.bus().last(0x298).map_or(0, |f| u16::from_be_bytes([f.data[6], f.data[7]]) as i32 - 10_000);
    let rpm0 = rpm(&sim);
    sim.set_gear(Gear::Drive);
    sim.set_pedal(60.0);
    sim.run(20_000_000);
    assert_eq!(
        sim.ev_ecu().system().peek(EV_ECU_OPERATING_MODE, 1),
        OP_MODE_DRIVE,
        "selecting Drive collapsed the operating mode (handshake broke)"
    );
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
    let dtcs = confirmed_ecu_dtcs(&sim);
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
    let dtcs = confirmed_ecu_dtcs(&sim);
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
    let bmu: Vec<String> = sim.bmu_dtcs().into_iter().filter(|d| d.1).map(|d| d.0).collect();
    for (dtc, why) in [
        ("P1AB5", "CMU01 reports a temperature below -30 C (an unfed sensor reads -50)"),
        ("P1A8E", "CMU01's balancing status (b1 of its first response) is not 0"),
        ("P1AC3", "the battery cooling fan's feedback is high while it is at rest"),
    ] {
        assert!(!bmu.iter().any(|d| d == dtc), "{dtc}: {why}");
    }
}

#[test]
#[should_panic(expected = "the EV-ECU has no DTC P0000")]
fn ecu_dtc_rejects_a_code_the_firmware_lacks() {
    ecu_dtc(&Simulation::imiev(), "P0000");
}
