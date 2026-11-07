//! Where an EV-ECU firmware build keeps data

use crate::dtc::DtcTable;

#[derive(Debug, Clone, Copy)]
pub struct EvEcuLayout {
    pub ic2_startup_state: u32,
    pub mode_status_code: u32,
    pub operating_mode: u32,
    pub torque_request: Option<u32>,
    pub dtc: DtcTable,
}

pub const EV_ECU_2015_10_14: EvEcuLayout = EvEcuLayout {
    ic2_startup_state: 0x0080_825a,
    mode_status_code: 0x0080_e595,
    operating_mode: 0x0080_dd4e,
    torque_request: Some(0x0080_d384),
    dtc: DtcTable { base: 0x0080_4a00, count: 216, table: 0x000e_4ae2 },
};

pub const EV_ECU_2015_05_28: EvEcuLayout = EvEcuLayout {
    ic2_startup_state: 0x0080_82d2,
    mode_status_code: 0x0080_da21,
    operating_mode: 0x0080_ce38,
    torque_request: None,
    dtc: DtcTable { base: 0x0080_4a00, count: 225, table: 0x000e_4b3a },
};

pub const EV_ECU_2014_05_12: EvEcuLayout = EvEcuLayout {
    mode_status_code: 0x0080_dae9,
    operating_mode: 0x0080_c72c,
    torque_request: Some(0x0080_c6de),
    dtc: DtcTable { base: 0x0080_4a00, count: 222, table: 0x000e_4b3a },
    ..EV_ECU_2015_05_28
};

pub const BMU_2010_05_10_DTC: DtcTable = DtcTable { base: 0x0080_4900, count: 183, table: 0x000e_0eaa };
