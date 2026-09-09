//! End-to-end tests: drive a whole [`Simulation`].

use super::*;

const IC2_STARTUP_STATE: u32 = 0x0080_825a; // 0->4->0xFFFF (POST done)
const EV_ECU_OPERATING_MODE: u32 = 0x0080_dd4e; // 0 REST/2 PRECHARGE/3 READY/4 DRIVE/5 SHUTDOWN
const OP_MODE_PRECHARGE: u32 = 2;

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
    const DTC_P0562_CONFIRMED: u32 = 0x0080_4a00; // fault-flag array idx 0, bit1 = confirmed
    const OP_MODE_SHUTDOWN: u32 = 5;
    let mut sim = Simulation::imiev();
    sim.run(8_000_000); // through POST + into precharge, while cf60 is still ramping
    let e = sim.ev_ecu().system();
    assert_eq!(
        e.peek(DTC_P0562_CONFIRMED, 1) & 0x02,
        0,
        "P0562 false-latched while the condenser was still precharging"
    );
    assert_ne!(
        e.peek(EV_ECU_OPERATING_MODE, 1),
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
    const DTC_P1B2C_FLAG: u32 = 0x0080_4abc; // 0x82 when the stuck-before-drive watchdog confirms
    let mut sim = Simulation::imiev();
    sim.run(250_000_000);
    let e = sim.ev_ecu().system();
    assert_eq!(e.peek(EV_ECU_OPERATING_MODE, 1), OP_MODE_DRIVE, "operating_mode did not reach DRIVE");
    assert_ne!(e.peek(DTC_P1B2C_FLAG, 1), 0x82, "P1B2C stuck-before-drive watchdog confirmed");
}
