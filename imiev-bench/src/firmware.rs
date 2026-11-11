//! Every EV-ECU and BMU image in the catalogue

pub struct EvEcu {
    pub part: &'static str,
    pub built: &'static str,
    pub from: &'static str,
    pub image: &'static [u8],
}

pub struct Bmu {
    pub part: &'static str,
    pub built: &'static str,
    pub from: &'static str,
    pub image: &'static [u8],
}

pub const EV_ECU_9499A18206: EvEcu = EvEcu {
    part: "9499A18206",
    built: "2015-10-14",
    from: "i-MiEV",
    image: include_bytes!("../../firmware/ev-ecu/9499A18206.bin"),
};

pub const EV_ECU_9499B13103: EvEcu = EvEcu {
    part: "9499B13103",
    built: "2015-05-28",
    from: "i-MiEV",
    image: include_bytes!("../../firmware/ev-ecu/9499B13103.bin"),
};

pub const EV_ECU_9499A43902: EvEcu = EvEcu {
    part: "9499A43902",
    built: "2014-05-12",
    from: "Citroën C-Zero 2011",
    image: include_bytes!("../../firmware/ev-ecu/9499A43902.bin"),
};

pub const BMU_9499B11500: Bmu = Bmu {
    part: "9499B11500",
    built: "2011-01-21",
    from: "i-MiEV",
    image: include_bytes!("../../firmware/bmu/9499B11500.bin"),
};

pub const BMU_9499A84200: Bmu = Bmu {
    part: "9499A84200",
    built: "2010-05-10",
    from: "Citroën C-Zero 2011",
    image: include_bytes!("../../firmware/bmu/9499A84200.bin"),
};

pub const EV_ECUS: [&EvEcu; 3] = [&EV_ECU_9499A18206, &EV_ECU_9499B13103, &EV_ECU_9499A43902];
pub const BMUS: [&Bmu; 2] = [&BMU_9499B11500, &BMU_9499A84200];
