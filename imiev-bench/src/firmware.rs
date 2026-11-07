//! Every EV-ECU and BMU image in the catalogue

use crate::dtc::DtcTable;
use crate::layout::{self, EvEcuLayout};

pub struct EvEcu {
    pub part: &'static str,
    pub built: &'static str,
    pub from: &'static str,
    pub image: &'static [u8],
    pub layout: EvEcuLayout,
}

pub struct Bmu {
    pub part: &'static str,
    pub built: &'static str,
    pub from: &'static str,
    pub image: &'static [u8],
    pub dtc: DtcTable,
}

pub const EV_ECU_9499A18206: EvEcu = EvEcu {
    part: "9499A18206",
    built: "2015-10-14",
    from: "i-MiEV",
    image: include_bytes!("../../firmware/ev-ecu/9499A18206.bin"),
    layout: layout::EV_ECU_2015_10_14,
};

pub const EV_ECU_9499B13103: EvEcu = EvEcu {
    part: "9499B13103",
    built: "2015-05-28",
    from: "i-MiEV",
    image: include_bytes!("../../firmware/ev-ecu/9499B13103.bin"),
    layout: layout::EV_ECU_2015_05_28,
};

pub const EV_ECU_9499A43902: EvEcu = EvEcu {
    part: "9499A43902",
    built: "2014-05-12",
    from: "Citroën C-Zero 2011",
    image: include_bytes!("../../firmware/ev-ecu/9499A43902.bin"),
    layout: layout::EV_ECU_2014_05_12,
};

pub const BMU_9499B11500: Bmu = Bmu {
    part: "9499B11500",
    built: "2011-01-21",
    from: "i-MiEV",
    image: include_bytes!("../../firmware/bmu/9499B11500.bin"),
    dtc: DtcTable { base: 0x0080_4900, count: 181, table: 0x000e_0eaa },
};

pub const BMU_9499A84200: Bmu = Bmu {
    part: "9499A84200",
    built: "2010-05-10",
    from: "Citroën C-Zero 2011",
    image: include_bytes!("../../firmware/bmu/9499A84200.bin"),
    dtc: layout::BMU_2010_05_10_DTC,
};

pub const EV_ECUS: [&EvEcu; 3] = [&EV_ECU_9499A18206, &EV_ECU_9499B13103, &EV_ECU_9499A43902];
pub const BMUS: [&Bmu; 2] = [&BMU_9499B11500, &BMU_9499A84200];
