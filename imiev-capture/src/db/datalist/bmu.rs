//! The data items of the BMU.

use crate::db::units::Unit;
use crate::item::{At, Conv, Item, Source, Value};
use crate::kwp::Ecu;

/// Accumulated minute
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AccumulatedMinute(pub crate::db::units::Minutes);

pub(crate) static ACCUMULATED_MINUTE: [Item; 1] = [
    Item { name: "Accumulated minute", pos: 7, len: 2, bits: None, unit: Unit::Minutes, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for AccumulatedMinute {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0101, 0x0104, 0x0010, 0x0011], source: Source::FreezeFrame, item: &ACCUMULATED_MINUTE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Minutes(n)))
    }
}

/// Auto capa.measured ODO his.1
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AutoCapaMeasuredOdoHis1(pub crate::db::units::Km);

pub(crate) static AUTO_CAPA_MEASURED_ODO_HIS1: [Item; 1] = [
    Item { name: "Auto capa.measured ODO his.1", pos: 2, len: 2, bits: None, unit: Unit::Km, conv: Conv::Linear { factor: 6.4, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for AutoCapaMeasuredOdoHis1 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0002, 0x0003, 0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x15), item: &AUTO_CAPA_MEASURED_ODO_HIS1[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Km(n)))
    }
}

/// Auto capa.measured ODO his.10
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AutoCapaMeasuredOdoHis10(pub crate::db::units::Km);

pub(crate) static AUTO_CAPA_MEASURED_ODO_HIS10: [Item; 1] = [
    Item { name: "Auto capa.measured ODO his.10", pos: 20, len: 2, bits: None, unit: Unit::Km, conv: Conv::Linear { factor: 6.4, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for AutoCapaMeasuredOdoHis10 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0002, 0x0003, 0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x15), item: &AUTO_CAPA_MEASURED_ODO_HIS10[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Km(n)))
    }
}

/// Auto capa.measured ODO his.2
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AutoCapaMeasuredOdoHis2(pub crate::db::units::Km);

pub(crate) static AUTO_CAPA_MEASURED_ODO_HIS2: [Item; 1] = [
    Item { name: "Auto capa.measured ODO his.2", pos: 4, len: 2, bits: None, unit: Unit::Km, conv: Conv::Linear { factor: 6.4, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for AutoCapaMeasuredOdoHis2 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0002, 0x0003, 0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x15), item: &AUTO_CAPA_MEASURED_ODO_HIS2[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Km(n)))
    }
}

/// Auto capa.measured ODO his.3
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AutoCapaMeasuredOdoHis3(pub crate::db::units::Km);

pub(crate) static AUTO_CAPA_MEASURED_ODO_HIS3: [Item; 1] = [
    Item { name: "Auto capa.measured ODO his.3", pos: 6, len: 2, bits: None, unit: Unit::Km, conv: Conv::Linear { factor: 6.4, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for AutoCapaMeasuredOdoHis3 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0002, 0x0003, 0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x15), item: &AUTO_CAPA_MEASURED_ODO_HIS3[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Km(n)))
    }
}

/// Auto capa.measured ODO his.4
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AutoCapaMeasuredOdoHis4(pub crate::db::units::Km);

pub(crate) static AUTO_CAPA_MEASURED_ODO_HIS4: [Item; 1] = [
    Item { name: "Auto capa.measured ODO his.4", pos: 8, len: 2, bits: None, unit: Unit::Km, conv: Conv::Linear { factor: 6.4, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for AutoCapaMeasuredOdoHis4 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0002, 0x0003, 0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x15), item: &AUTO_CAPA_MEASURED_ODO_HIS4[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Km(n)))
    }
}

/// Auto capa.measured ODO his.5
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AutoCapaMeasuredOdoHis5(pub crate::db::units::Km);

pub(crate) static AUTO_CAPA_MEASURED_ODO_HIS5: [Item; 1] = [
    Item { name: "Auto capa.measured ODO his.5", pos: 10, len: 2, bits: None, unit: Unit::Km, conv: Conv::Linear { factor: 6.4, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for AutoCapaMeasuredOdoHis5 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0002, 0x0003, 0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x15), item: &AUTO_CAPA_MEASURED_ODO_HIS5[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Km(n)))
    }
}

/// Auto capa.measured ODO his.6
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AutoCapaMeasuredOdoHis6(pub crate::db::units::Km);

pub(crate) static AUTO_CAPA_MEASURED_ODO_HIS6: [Item; 1] = [
    Item { name: "Auto capa.measured ODO his.6", pos: 12, len: 2, bits: None, unit: Unit::Km, conv: Conv::Linear { factor: 6.4, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for AutoCapaMeasuredOdoHis6 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0002, 0x0003, 0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x15), item: &AUTO_CAPA_MEASURED_ODO_HIS6[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Km(n)))
    }
}

/// Auto capa.measured ODO his.7
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AutoCapaMeasuredOdoHis7(pub crate::db::units::Km);

pub(crate) static AUTO_CAPA_MEASURED_ODO_HIS7: [Item; 1] = [
    Item { name: "Auto capa.measured ODO his.7", pos: 14, len: 2, bits: None, unit: Unit::Km, conv: Conv::Linear { factor: 6.4, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for AutoCapaMeasuredOdoHis7 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0002, 0x0003, 0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x15), item: &AUTO_CAPA_MEASURED_ODO_HIS7[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Km(n)))
    }
}

/// Auto capa.measured ODO his.8
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AutoCapaMeasuredOdoHis8(pub crate::db::units::Km);

pub(crate) static AUTO_CAPA_MEASURED_ODO_HIS8: [Item; 1] = [
    Item { name: "Auto capa.measured ODO his.8", pos: 16, len: 2, bits: None, unit: Unit::Km, conv: Conv::Linear { factor: 6.4, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for AutoCapaMeasuredOdoHis8 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0002, 0x0003, 0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x15), item: &AUTO_CAPA_MEASURED_ODO_HIS8[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Km(n)))
    }
}

/// Auto capa.measured ODO his.9
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AutoCapaMeasuredOdoHis9(pub crate::db::units::Km);

pub(crate) static AUTO_CAPA_MEASURED_ODO_HIS9: [Item; 1] = [
    Item { name: "Auto capa.measured ODO his.9", pos: 18, len: 2, bits: None, unit: Unit::Km, conv: Conv::Linear { factor: 6.4, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for AutoCapaMeasuredOdoHis9 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0002, 0x0003, 0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x15), item: &AUTO_CAPA_MEASURED_ODO_HIS9[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Km(n)))
    }
}

/// Auto capa.measured result his.1
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AutoCapaMeasuredResultHis1(pub crate::db::units::AmpHours);

pub(crate) static AUTO_CAPA_MEASURED_RESULT_HIS1: [Item; 1] = [
    Item { name: "Auto capa.measured result his.1", pos: 2, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for AutoCapaMeasuredResultHis1 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0002, 0x0003, 0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x14), item: &AUTO_CAPA_MEASURED_RESULT_HIS1[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Auto capa.measured result his.10
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AutoCapaMeasuredResultHis10(pub crate::db::units::AmpHours);

pub(crate) static AUTO_CAPA_MEASURED_RESULT_HIS10: [Item; 1] = [
    Item { name: "Auto capa.measured result his.10", pos: 20, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for AutoCapaMeasuredResultHis10 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0002, 0x0003, 0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x14), item: &AUTO_CAPA_MEASURED_RESULT_HIS10[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Auto capa.measured result his.2
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AutoCapaMeasuredResultHis2(pub crate::db::units::AmpHours);

pub(crate) static AUTO_CAPA_MEASURED_RESULT_HIS2: [Item; 1] = [
    Item { name: "Auto capa.measured result his.2", pos: 4, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for AutoCapaMeasuredResultHis2 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0002, 0x0003, 0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x14), item: &AUTO_CAPA_MEASURED_RESULT_HIS2[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Auto capa.measured result his.3
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AutoCapaMeasuredResultHis3(pub crate::db::units::AmpHours);

pub(crate) static AUTO_CAPA_MEASURED_RESULT_HIS3: [Item; 1] = [
    Item { name: "Auto capa.measured result his.3", pos: 6, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for AutoCapaMeasuredResultHis3 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0002, 0x0003, 0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x14), item: &AUTO_CAPA_MEASURED_RESULT_HIS3[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Auto capa.measured result his.4
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AutoCapaMeasuredResultHis4(pub crate::db::units::AmpHours);

pub(crate) static AUTO_CAPA_MEASURED_RESULT_HIS4: [Item; 1] = [
    Item { name: "Auto capa.measured result his.4", pos: 8, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for AutoCapaMeasuredResultHis4 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0002, 0x0003, 0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x14), item: &AUTO_CAPA_MEASURED_RESULT_HIS4[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Auto capa.measured result his.5
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AutoCapaMeasuredResultHis5(pub crate::db::units::AmpHours);

pub(crate) static AUTO_CAPA_MEASURED_RESULT_HIS5: [Item; 1] = [
    Item { name: "Auto capa.measured result his.5", pos: 10, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for AutoCapaMeasuredResultHis5 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0002, 0x0003, 0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x14), item: &AUTO_CAPA_MEASURED_RESULT_HIS5[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Auto capa.measured result his.6
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AutoCapaMeasuredResultHis6(pub crate::db::units::AmpHours);

pub(crate) static AUTO_CAPA_MEASURED_RESULT_HIS6: [Item; 1] = [
    Item { name: "Auto capa.measured result his.6", pos: 12, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for AutoCapaMeasuredResultHis6 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0002, 0x0003, 0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x14), item: &AUTO_CAPA_MEASURED_RESULT_HIS6[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Auto capa.measured result his.7
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AutoCapaMeasuredResultHis7(pub crate::db::units::AmpHours);

pub(crate) static AUTO_CAPA_MEASURED_RESULT_HIS7: [Item; 1] = [
    Item { name: "Auto capa.measured result his.7", pos: 14, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for AutoCapaMeasuredResultHis7 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0002, 0x0003, 0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x14), item: &AUTO_CAPA_MEASURED_RESULT_HIS7[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Auto capa.measured result his.8
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AutoCapaMeasuredResultHis8(pub crate::db::units::AmpHours);

pub(crate) static AUTO_CAPA_MEASURED_RESULT_HIS8: [Item; 1] = [
    Item { name: "Auto capa.measured result his.8", pos: 16, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for AutoCapaMeasuredResultHis8 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0002, 0x0003, 0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x14), item: &AUTO_CAPA_MEASURED_RESULT_HIS8[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Auto capa.measured result his.9
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AutoCapaMeasuredResultHis9(pub crate::db::units::AmpHours);

pub(crate) static AUTO_CAPA_MEASURED_RESULT_HIS9: [Item; 1] = [
    Item { name: "Auto capa.measured result his.9", pos: 18, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for AutoCapaMeasuredResultHis9 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0002, 0x0003, 0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x14), item: &AUTO_CAPA_MEASURED_RESULT_HIS9[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Average cell voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AverageCellVoltage(pub crate::db::units::Volts);

pub(crate) static AVERAGE_CELL_VOLTAGE: [Item; 1] = [
    Item { name: "Average cell voltage", pos: 50, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for AverageCellVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0010, 0x0011], source: Source::Record(0x01), item: &AVERAGE_CELL_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// BAT.capa. secular deterioration
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BatCapaSecularDeterioration(pub crate::db::units::Percent);

pub(crate) static BAT_CAPA_SECULAR_DETERIORATION: [Item; 1] = [
    Item { name: "BAT.capa. secular deterioration", pos: 22, len: 4, bits: None, unit: Unit::Percent, conv: Conv::Linear { factor: 1e-07, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for BatCapaSecularDeterioration {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x11), item: &BAT_CAPA_SECULAR_DETERIORATION[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Percent(n)))
    }
}

/// BAT.capacity cycle deterioration
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BatCapacityCycleDeterioration(pub crate::db::units::Percent);

pub(crate) static BAT_CAPACITY_CYCLE_DETERIORATION: [Item; 1] = [
    Item { name: "BAT.capacity cycle deterioration", pos: 26, len: 4, bits: None, unit: Unit::Percent, conv: Conv::Linear { factor: 1e-07, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for BatCapacityCycleDeterioration {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x11), item: &BAT_CAPACITY_CYCLE_DETERIORATION[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Percent(n)))
    }
}

/// BAT.cell voltage difference MAX
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BatCellVoltageDifferenceMax(pub crate::db::units::Volts);

pub(crate) static BAT_CELL_VOLTAGE_DIFFERENCE_MAX: [Item; 2] = [
    Item { name: "BAT.cell voltage difference MAX", pos: 43, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 0.0, decimals: 2 } },
    Item { name: "BAT.cell voltage difference MAX", pos: 35, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for BatCellVoltageDifferenceMax {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x01), item: &BAT_CELL_VOLTAGE_DIFFERENCE_MAX[0] },
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x01), item: &BAT_CELL_VOLTAGE_DIFFERENCE_MAX[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// BAT.current HI control value
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BatCurrentHiControlValue(pub crate::db::units::Amps);

pub(crate) static BAT_CURRENT_HI_CONTROL_VALUE: [Item; 1] = [
    Item { name: "BAT.current HI control value", pos: 18, len: 2, bits: None, unit: Unit::Amps, conv: Conv::Linear { factor: 1.0, offset: -350.0, decimals: 0 } },
];

impl crate::item::Reading for BatCurrentHiControlValue {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0101, 0x0104, 0x0010, 0x0011], source: Source::Record(0x01), item: &BAT_CURRENT_HI_CONTROL_VALUE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Amps(n)))
    }
}

/// BAT.current HI sensor
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BatCurrentHiSensor(pub crate::db::units::Volts);

pub(crate) static BAT_CURRENT_HI_SENSOR: [Item; 1] = [
    Item { name: "BAT.current HI sensor", pos: 22, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.01, offset: 0.0, decimals: 2 } },
];

impl crate::item::Reading for BatCurrentHiSensor {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0101, 0x0104, 0x0010, 0x0011], source: Source::Record(0x01), item: &BAT_CURRENT_HI_SENSOR[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// BAT.current LO control value
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BatCurrentLoControlValue(pub crate::db::units::Amps);

pub(crate) static BAT_CURRENT_LO_CONTROL_VALUE: [Item; 1] = [
    Item { name: "BAT.current LO control value", pos: 20, len: 2, bits: None, unit: Unit::Amps, conv: Conv::Linear { factor: 0.1, offset: -30.0, decimals: 1 } },
];

impl crate::item::Reading for BatCurrentLoControlValue {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0101, 0x0104, 0x0010, 0x0011], source: Source::Record(0x01), item: &BAT_CURRENT_LO_CONTROL_VALUE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Amps(n)))
    }
}

/// BAT.current LO sensor
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BatCurrentLoSensor(pub crate::db::units::Volts);

pub(crate) static BAT_CURRENT_LO_SENSOR: [Item; 1] = [
    Item { name: "BAT.current LO sensor", pos: 24, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.01, offset: 0.0, decimals: 2 } },
];

impl crate::item::Reading for BatCurrentLoSensor {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0101, 0x0104, 0x0010, 0x0011], source: Source::Record(0x01), item: &BAT_CURRENT_LO_SENSOR[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Battery cell maximum voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BatteryCellMaximumVoltage(pub crate::db::units::Volts);

pub(crate) static BATTERY_CELL_MAXIMUM_VOLTAGE: [Item; 2] = [
    Item { name: "Battery cell maximum voltage", pos: 4, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
    Item { name: "Battery cell maximum voltage", pos: 4, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for BatteryCellMaximumVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x01), item: &BATTERY_CELL_MAXIMUM_VOLTAGE[0] },
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x01), item: &BATTERY_CELL_MAXIMUM_VOLTAGE[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Battery cell minimum voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BatteryCellMinimumVoltage(pub crate::db::units::Volts);

pub(crate) static BATTERY_CELL_MINIMUM_VOLTAGE: [Item; 2] = [
    Item { name: "Battery cell minimum voltage", pos: 7, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
    Item { name: "Battery cell minimum voltage", pos: 7, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for BatteryCellMinimumVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x01), item: &BATTERY_CELL_MINIMUM_VOLTAGE[0] },
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x01), item: &BATTERY_CELL_MINIMUM_VOLTAGE[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Battery cooling fan PWM output
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BatteryCoolingFanPwmOutput(pub crate::db::units::Percent);

pub(crate) static BATTERY_COOLING_FAN_PWM_OUTPUT: [Item; 1] = [
    Item { name: "Battery cooling fan PWM output", pos: 41, len: 1, bits: None, unit: Unit::Percent, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for BatteryCoolingFanPwmOutput {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0010, 0x0011], source: Source::Record(0x01), item: &BATTERY_COOLING_FAN_PWM_OUTPUT[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Percent(n)))
    }
}

/// Battery cooling fan relay output
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BatteryCoolingFanRelayOutput(pub bool);

pub(crate) static BATTERY_COOLING_FAN_RELAY_OUTPUT: [Item; 1] = [
    Item { name: "Battery cooling fan relay output", pos: 26, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for BatteryCoolingFanRelayOutput {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0101], source: Source::Record(0x01), item: &BATTERY_COOLING_FAN_RELAY_OUTPUT[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Battery cooling fan rotations
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BatteryCoolingFanRotations(pub crate::db::units::Rpm);

pub(crate) static BATTERY_COOLING_FAN_ROTATIONS: [Item; 1] = [
    Item { name: "Battery cooling fan rotations", pos: 42, len: 1, bits: None, unit: Unit::Rpm, conv: Conv::Linear { factor: 30.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for BatteryCoolingFanRotations {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0010, 0x0011], source: Source::Record(0x01), item: &BATTERY_COOLING_FAN_ROTATIONS[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Rpm(n)))
    }
}

/// Battery current
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BatteryCurrent(pub crate::db::units::Amps);

pub(crate) static BATTERY_CURRENT: [Item; 2] = [
    Item { name: "Battery current", pos: 16, len: 2, bits: None, unit: Unit::Amps, conv: Conv::Linear { factor: 1.0, offset: -350.0, decimals: 0 } },
    Item { name: "Battery current", pos: 16, len: 2, bits: None, unit: Unit::Amps, conv: Conv::Linear { factor: 1.0, offset: -400.0, decimals: 0 } },
];

impl crate::item::Reading for BatteryCurrent {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0101, 0x0104], source: Source::Record(0x01), item: &BATTERY_CURRENT[0] },
        At { versions: &[0x0010, 0x0011], source: Source::Record(0x01), item: &BATTERY_CURRENT[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Amps(n)))
    }
}

/// Battery current capacity
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BatteryCurrentCapacity(pub crate::db::units::AmpHours);

pub(crate) static BATTERY_CURRENT_CAPACITY: [Item; 1] = [
    Item { name: "Battery current capacity", pos: 29, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for BatteryCurrentCapacity {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0101, 0x0104, 0x0010, 0x0011], source: Source::Record(0x01), item: &BATTERY_CURRENT_CAPACITY[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Battery maximum input power
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BatteryMaximumInputPower(pub crate::db::units::Kilowatts);

pub(crate) static BATTERY_MAXIMUM_INPUT_POWER: [Item; 1] = [
    Item { name: "Battery maximum input power", pos: 33, len: 1, bits: None, unit: Unit::Kilowatts, conv: Conv::Linear { factor: 0.25, offset: 0.0, decimals: 2 } },
];

impl crate::item::Reading for BatteryMaximumInputPower {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0101, 0x0104, 0x0010, 0x0011], source: Source::Record(0x01), item: &BATTERY_MAXIMUM_INPUT_POWER[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Kilowatts(n)))
    }
}

/// Battery maximum output power
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BatteryMaximumOutputPower(pub crate::db::units::Kilowatts);

pub(crate) static BATTERY_MAXIMUM_OUTPUT_POWER: [Item; 1] = [
    Item { name: "Battery maximum output power", pos: 34, len: 1, bits: None, unit: Unit::Kilowatts, conv: Conv::Linear { factor: 0.25, offset: 0.0, decimals: 2 } },
];

impl crate::item::Reading for BatteryMaximumOutputPower {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0101, 0x0104, 0x0010, 0x0011], source: Source::Record(0x01), item: &BATTERY_MAXIMUM_OUTPUT_POWER[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Kilowatts(n)))
    }
}

/// Battery remaining capacity
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BatteryRemainingCapacity(pub crate::db::units::AmpHours);

pub(crate) static BATTERY_REMAINING_CAPACITY: [Item; 1] = [
    Item { name: "Battery remaining capacity", pos: 31, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for BatteryRemainingCapacity {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0101, 0x0104, 0x0010, 0x0011], source: Source::Record(0x01), item: &BATTERY_REMAINING_CAPACITY[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Battery total voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BatteryTotalVoltage(pub crate::db::units::Volts);

pub(crate) static BATTERY_TOTAL_VOLTAGE: [Item; 1] = [
    Item { name: "Battery total voltage", pos: 10, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for BatteryTotalVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0101, 0x0104, 0x0010, 0x0011], source: Source::Record(0x01), item: &BATTERY_TOTAL_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Capa.measure discharge T/out his
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapaMeasureDischargeTOutHis(pub bool);

pub(crate) static CAPA_MEASURE_DISCHARGE_T_OUT_HIS: [Item; 1] = [
    Item { name: "Capa.measure discharge T/out his", pos: 2, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "Not present"), (1, 1, "Present")]) },
];

impl crate::item::Reading for CapaMeasureDischargeTOutHis {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x17), item: &CAPA_MEASURE_DISCHARGE_T_OUT_HIS[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "Present" => Some(Self(true)),
            "Not present" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Capa.measured BAT.heating his.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapaMeasuredBatHeatingHis(pub bool);

pub(crate) static CAPA_MEASURED_BAT_HEATING_HIS: [Item; 1] = [
    Item { name: "Capa.measured BAT.heating his.", pos: 2, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "Not present"), (1, 1, "Present")]) },
];

impl crate::item::Reading for CapaMeasuredBatHeatingHis {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x17), item: &CAPA_MEASURED_BAT_HEATING_HIS[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "Present" => Some(Self(true)),
            "Not present" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Capa.measured charge cancel his.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapaMeasuredChargeCancelHis(pub bool);

pub(crate) static CAPA_MEASURED_CHARGE_CANCEL_HIS: [Item; 1] = [
    Item { name: "Capa.measured charge cancel his.", pos: 2, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "Not present"), (1, 1, "Present")]) },
];

impl crate::item::Reading for CapaMeasuredChargeCancelHis {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x17), item: &CAPA_MEASURED_CHARGE_CANCEL_HIS[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "Present" => Some(Self(true)),
            "Not present" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Capa. measured discharge current
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapaMeasuredDischargeCurrent(pub bool);

pub(crate) static CAPA_MEASURED_DISCHARGE_CURRENT: [Item; 1] = [
    Item { name: "Capa. measured discharge current", pos: 2, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "NG/Not performed"), (1, 1, "OK")]) },
];

impl crate::item::Reading for CapaMeasuredDischargeCurrent {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0002, 0x0003, 0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x16), item: &CAPA_MEASURED_DISCHARGE_CURRENT[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "OK" => Some(Self(true)),
            "NG/Not performed" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Capa.measured fail history
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapaMeasuredFailHistory(pub bool);

pub(crate) static CAPA_MEASURED_FAIL_HISTORY: [Item; 1] = [
    Item { name: "Capa.measured fail history", pos: 2, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "Not present"), (1, 1, "Present")]) },
];

impl crate::item::Reading for CapaMeasuredFailHistory {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x17), item: &CAPA_MEASURED_FAIL_HISTORY[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "Present" => Some(Self(true)),
            "Not present" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Capa.measured high temp.history
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapaMeasuredHighTempHistory(pub bool);

pub(crate) static CAPA_MEASURED_HIGH_TEMP_HISTORY: [Item; 1] = [
    Item { name: "Capa.measured high temp.history", pos: 2, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "Not present"), (1, 1, "Present")]) },
];

impl crate::item::Reading for CapaMeasuredHighTempHistory {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x17), item: &CAPA_MEASURED_HIGH_TEMP_HISTORY[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "Present" => Some(Self(true)),
            "Not present" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Capa.measured low temp.history
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapaMeasuredLowTempHistory(pub bool);

pub(crate) static CAPA_MEASURED_LOW_TEMP_HISTORY: [Item; 1] = [
    Item { name: "Capa.measured low temp.history", pos: 2, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "Not present"), (1, 1, "Present")]) },
];

impl crate::item::Reading for CapaMeasuredLowTempHistory {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x17), item: &CAPA_MEASURED_LOW_TEMP_HISTORY[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "Present" => Some(Self(true)),
            "Not present" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Capa.measured(MUT) comp.history
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapaMeasuredMutCompHistory(pub bool);

pub(crate) static CAPA_MEASURED_MUT_COMP_HISTORY: [Item; 1] = [
    Item { name: "Capa.measured(MUT) comp.history", pos: 2, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "Not present"), (1, 1, "Present")]) },
];

impl crate::item::Reading for CapaMeasuredMutCompHistory {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x17), item: &CAPA_MEASURED_MUT_COMP_HISTORY[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "Present" => Some(Self(true)),
            "Not present" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Capa.measured Pre A/C history
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CapaMeasuredPreACHistory(pub bool);

pub(crate) static CAPA_MEASURED_PRE_AC_HISTORY: [Item; 1] = [
    Item { name: "Capa.measured Pre A/C history", pos: 2, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "Not present"), (1, 1, "Present")]) },
];

impl crate::item::Reading for CapaMeasuredPreACHistory {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x17), item: &CAPA_MEASURED_PRE_AC_HISTORY[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "Present" => Some(Self(true)),
            "Not present" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 000 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell000BalancerDrive(pub bool);

pub(crate) static CELL000_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 000 Balancer drive", pos: 2, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell000BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL000_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 000 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell000Voltage(pub crate::db::units::Volts);

pub(crate) static CELL000_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 000 voltage", pos: 2, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell000Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL000_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 001 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell001BalancerDrive(pub bool);

pub(crate) static CELL001_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 001 Balancer drive", pos: 2, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell001BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL001_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 001 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell001Voltage(pub crate::db::units::Volts);

pub(crate) static CELL001_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 001 voltage", pos: 4, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell001Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL001_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 002 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell002BalancerDrive(pub bool);

pub(crate) static CELL002_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 002 Balancer drive", pos: 2, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell002BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL002_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 002 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell002Voltage(pub crate::db::units::Volts);

pub(crate) static CELL002_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 002 voltage", pos: 6, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell002Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL002_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 003 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell003BalancerDrive(pub bool);

pub(crate) static CELL003_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 003 Balancer drive", pos: 2, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell003BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL003_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 003 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell003Voltage(pub crate::db::units::Volts);

pub(crate) static CELL003_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 003 voltage", pos: 8, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell003Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL003_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 004 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell004BalancerDrive(pub bool);

pub(crate) static CELL004_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 004 Balancer drive", pos: 2, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell004BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL004_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 004 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell004Voltage(pub crate::db::units::Volts);

pub(crate) static CELL004_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 004 voltage", pos: 10, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell004Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL004_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 005 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell005BalancerDrive(pub bool);

pub(crate) static CELL005_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 005 Balancer drive", pos: 2, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell005BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL005_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 005 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell005Voltage(pub crate::db::units::Volts);

pub(crate) static CELL005_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 005 voltage", pos: 12, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell005Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL005_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 006 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell006BalancerDrive(pub bool);

pub(crate) static CELL006_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 006 Balancer drive", pos: 2, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell006BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL006_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 006 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell006Voltage(pub crate::db::units::Volts);

pub(crate) static CELL006_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 006 voltage", pos: 14, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell006Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL006_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 007 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell007BalancerDrive(pub bool);

pub(crate) static CELL007_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 007 Balancer drive", pos: 2, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell007BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL007_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 007 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell007Voltage(pub crate::db::units::Volts);

pub(crate) static CELL007_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 007 voltage", pos: 16, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell007Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL007_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 008 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell008BalancerDrive(pub bool);

pub(crate) static CELL008_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 008 Balancer drive", pos: 3, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell008BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL008_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 008 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell008Voltage(pub crate::db::units::Volts);

pub(crate) static CELL008_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 008 voltage", pos: 18, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell008Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL008_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 009 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell009BalancerDrive(pub bool);

pub(crate) static CELL009_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 009 Balancer drive", pos: 3, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell009BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL009_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 009 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell009Voltage(pub crate::db::units::Volts);

pub(crate) static CELL009_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 009 voltage", pos: 20, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell009Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL009_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 010 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell010BalancerDrive(pub bool);

pub(crate) static CELL010_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 010 Balancer drive", pos: 3, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell010BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL010_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 010 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell010Voltage(pub crate::db::units::Volts);

pub(crate) static CELL010_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 010 voltage", pos: 22, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell010Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL010_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 011 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell011BalancerDrive(pub bool);

pub(crate) static CELL011_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 011 Balancer drive", pos: 3, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell011BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL011_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 011 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell011Voltage(pub crate::db::units::Volts);

pub(crate) static CELL011_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 011 voltage", pos: 24, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell011Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL011_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 012 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell012BalancerDrive(pub bool);

pub(crate) static CELL012_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 012 Balancer drive", pos: 3, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell012BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL012_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 012 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell012Voltage(pub crate::db::units::Volts);

pub(crate) static CELL012_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 012 voltage", pos: 26, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell012Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL012_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 013 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell013BalancerDrive(pub bool);

pub(crate) static CELL013_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 013 Balancer drive", pos: 3, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell013BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL013_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 013 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell013Voltage(pub crate::db::units::Volts);

pub(crate) static CELL013_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 013 voltage", pos: 28, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell013Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL013_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 014 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell014BalancerDrive(pub bool);

pub(crate) static CELL014_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 014 Balancer drive", pos: 3, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell014BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL014_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 014 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell014Voltage(pub crate::db::units::Volts);

pub(crate) static CELL014_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 014 voltage", pos: 30, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell014Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL014_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 015 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell015BalancerDrive(pub bool);

pub(crate) static CELL015_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 015 Balancer drive", pos: 3, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell015BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL015_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 015 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell015Voltage(pub crate::db::units::Volts);

pub(crate) static CELL015_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 015 voltage", pos: 32, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell015Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL015_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 016 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell016BalancerDrive(pub bool);

pub(crate) static CELL016_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 016 Balancer drive", pos: 4, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell016BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL016_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 016 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell016Voltage(pub crate::db::units::Volts);

pub(crate) static CELL016_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 016 voltage", pos: 34, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell016Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL016_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 017 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell017BalancerDrive(pub bool);

pub(crate) static CELL017_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 017 Balancer drive", pos: 4, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell017BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL017_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 017 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell017Voltage(pub crate::db::units::Volts);

pub(crate) static CELL017_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 017 voltage", pos: 36, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell017Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL017_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 018 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell018BalancerDrive(pub bool);

pub(crate) static CELL018_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 018 Balancer drive", pos: 4, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell018BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL018_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 018 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell018Voltage(pub crate::db::units::Volts);

pub(crate) static CELL018_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 018 voltage", pos: 38, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell018Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL018_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 019 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell019BalancerDrive(pub bool);

pub(crate) static CELL019_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 019 Balancer drive", pos: 4, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell019BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL019_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 019 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell019Voltage(pub crate::db::units::Volts);

pub(crate) static CELL019_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 019 voltage", pos: 40, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell019Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL019_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 020 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell020BalancerDrive(pub bool);

pub(crate) static CELL020_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 020 Balancer drive", pos: 4, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell020BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL020_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 020 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell020Voltage(pub crate::db::units::Volts);

pub(crate) static CELL020_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 020 voltage", pos: 42, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell020Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL020_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 021 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell021BalancerDrive(pub bool);

pub(crate) static CELL021_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 021 Balancer drive", pos: 4, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell021BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL021_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 021 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell021Voltage(pub crate::db::units::Volts);

pub(crate) static CELL021_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 021 voltage", pos: 44, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell021Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL021_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 022 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell022BalancerDrive(pub bool);

pub(crate) static CELL022_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 022 Balancer drive", pos: 4, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell022BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL022_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 022 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell022Voltage(pub crate::db::units::Volts);

pub(crate) static CELL022_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 022 voltage", pos: 46, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell022Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL022_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 023 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell023BalancerDrive(pub bool);

pub(crate) static CELL023_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 023 Balancer drive", pos: 4, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell023BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL023_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 023 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell023Voltage(pub crate::db::units::Volts);

pub(crate) static CELL023_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 023 voltage", pos: 48, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell023Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL023_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 024 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell024BalancerDrive(pub bool);

pub(crate) static CELL024_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 024 Balancer drive", pos: 5, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell024BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL024_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 024 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell024Voltage(pub crate::db::units::Volts);

pub(crate) static CELL024_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 024 voltage", pos: 50, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell024Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL024_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 025 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell025BalancerDrive(pub bool);

pub(crate) static CELL025_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 025 Balancer drive", pos: 5, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell025BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL025_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 025 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell025Voltage(pub crate::db::units::Volts);

pub(crate) static CELL025_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 025 voltage", pos: 52, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell025Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL025_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 026 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell026BalancerDrive(pub bool);

pub(crate) static CELL026_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 026 Balancer drive", pos: 5, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell026BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL026_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 026 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell026Voltage(pub crate::db::units::Volts);

pub(crate) static CELL026_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 026 voltage", pos: 54, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell026Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL026_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 027 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell027BalancerDrive(pub bool);

pub(crate) static CELL027_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 027 Balancer drive", pos: 5, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell027BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL027_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 027 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell027Voltage(pub crate::db::units::Volts);

pub(crate) static CELL027_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 027 voltage", pos: 56, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell027Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL027_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 028 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell028BalancerDrive(pub bool);

pub(crate) static CELL028_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 028 Balancer drive", pos: 5, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell028BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL028_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 028 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell028Voltage(pub crate::db::units::Volts);

pub(crate) static CELL028_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 028 voltage", pos: 58, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell028Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL028_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 029 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell029BalancerDrive(pub bool);

pub(crate) static CELL029_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 029 Balancer drive", pos: 5, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell029BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL029_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 029 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell029Voltage(pub crate::db::units::Volts);

pub(crate) static CELL029_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 029 voltage", pos: 60, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell029Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL029_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 030 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell030BalancerDrive(pub bool);

pub(crate) static CELL030_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 030 Balancer drive", pos: 5, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell030BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL030_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 030 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell030Voltage(pub crate::db::units::Volts);

pub(crate) static CELL030_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 030 voltage", pos: 62, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell030Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL030_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 031 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell031BalancerDrive(pub bool);

pub(crate) static CELL031_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 031 Balancer drive", pos: 5, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell031BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL031_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 031 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell031Voltage(pub crate::db::units::Volts);

pub(crate) static CELL031_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 031 voltage", pos: 64, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell031Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL031_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 032 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell032BalancerDrive(pub bool);

pub(crate) static CELL032_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 032 Balancer drive", pos: 6, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell032BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL032_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 032 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell032Voltage(pub crate::db::units::Volts);

pub(crate) static CELL032_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 032 voltage", pos: 66, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell032Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL032_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 033 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell033BalancerDrive(pub bool);

pub(crate) static CELL033_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 033 Balancer drive", pos: 6, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell033BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL033_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 033 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell033Voltage(pub crate::db::units::Volts);

pub(crate) static CELL033_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 033 voltage", pos: 68, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell033Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL033_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 034 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell034BalancerDrive(pub bool);

pub(crate) static CELL034_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 034 Balancer drive", pos: 6, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell034BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL034_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 034 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell034Voltage(pub crate::db::units::Volts);

pub(crate) static CELL034_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 034 voltage", pos: 70, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell034Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL034_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 035 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell035BalancerDrive(pub bool);

pub(crate) static CELL035_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 035 Balancer drive", pos: 6, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell035BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL035_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 035 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell035Voltage(pub crate::db::units::Volts);

pub(crate) static CELL035_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 035 voltage", pos: 72, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell035Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL035_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 036 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell036BalancerDrive(pub bool);

pub(crate) static CELL036_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 036 Balancer drive", pos: 6, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell036BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL036_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 036 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell036Voltage(pub crate::db::units::Volts);

pub(crate) static CELL036_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 036 voltage", pos: 74, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell036Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL036_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 037 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell037BalancerDrive(pub bool);

pub(crate) static CELL037_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 037 Balancer drive", pos: 6, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell037BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL037_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 037 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell037Voltage(pub crate::db::units::Volts);

pub(crate) static CELL037_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 037 voltage", pos: 76, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell037Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL037_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 038 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell038BalancerDrive(pub bool);

pub(crate) static CELL038_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 038 Balancer drive", pos: 6, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell038BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL038_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 038 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell038Voltage(pub crate::db::units::Volts);

pub(crate) static CELL038_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 038 voltage", pos: 78, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell038Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL038_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 039 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell039BalancerDrive(pub bool);

pub(crate) static CELL039_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 039 Balancer drive", pos: 6, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell039BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL039_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 039 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell039Voltage(pub crate::db::units::Volts);

pub(crate) static CELL039_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 039 voltage", pos: 80, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell039Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL039_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 040 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell040BalancerDrive(pub bool);

pub(crate) static CELL040_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 040 Balancer drive", pos: 7, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell040BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL040_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 040 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell040Voltage(pub crate::db::units::Volts);

pub(crate) static CELL040_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 040 voltage", pos: 82, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell040Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL040_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 041 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell041BalancerDrive(pub bool);

pub(crate) static CELL041_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 041 Balancer drive", pos: 7, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell041BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL041_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 041 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell041Voltage(pub crate::db::units::Volts);

pub(crate) static CELL041_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 041 voltage", pos: 84, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell041Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL041_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 042 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell042BalancerDrive(pub bool);

pub(crate) static CELL042_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 042 Balancer drive", pos: 7, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell042BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL042_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 042 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell042Voltage(pub crate::db::units::Volts);

pub(crate) static CELL042_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 042 voltage", pos: 86, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell042Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL042_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 043 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell043BalancerDrive(pub bool);

pub(crate) static CELL043_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 043 Balancer drive", pos: 7, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell043BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL043_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 043 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell043Voltage(pub crate::db::units::Volts);

pub(crate) static CELL043_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 043 voltage", pos: 88, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell043Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL043_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 044 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell044BalancerDrive(pub bool);

pub(crate) static CELL044_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 044 Balancer drive", pos: 7, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell044BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL044_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 044 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell044Voltage(pub crate::db::units::Volts);

pub(crate) static CELL044_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 044 voltage", pos: 90, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell044Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL044_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 045 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell045BalancerDrive(pub bool);

pub(crate) static CELL045_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 045 Balancer drive", pos: 7, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell045BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL045_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 045 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell045Voltage(pub crate::db::units::Volts);

pub(crate) static CELL045_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 045 voltage", pos: 92, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell045Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL045_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 046 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell046BalancerDrive(pub bool);

pub(crate) static CELL046_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 046 Balancer drive", pos: 7, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell046BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL046_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 046 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell046Voltage(pub crate::db::units::Volts);

pub(crate) static CELL046_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 046 voltage", pos: 94, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell046Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL046_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 047 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell047BalancerDrive(pub bool);

pub(crate) static CELL047_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 047 Balancer drive", pos: 7, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell047BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL047_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 047 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell047Voltage(pub crate::db::units::Volts);

pub(crate) static CELL047_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 047 voltage", pos: 96, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell047Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL047_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 048 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell048BalancerDrive(pub bool);

pub(crate) static CELL048_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 048 Balancer drive", pos: 8, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell048BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL048_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 048 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell048Voltage(pub crate::db::units::Volts);

pub(crate) static CELL048_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 048 voltage", pos: 98, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell048Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL048_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 049 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell049BalancerDrive(pub bool);

pub(crate) static CELL049_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 049 Balancer drive", pos: 8, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell049BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL049_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 049 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell049Voltage(pub crate::db::units::Volts);

pub(crate) static CELL049_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 049 voltage", pos: 100, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell049Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL049_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 050 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell050BalancerDrive(pub bool);

pub(crate) static CELL050_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 050 Balancer drive", pos: 8, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell050BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL050_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 050 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell050Voltage(pub crate::db::units::Volts);

pub(crate) static CELL050_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 050 voltage", pos: 102, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell050Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL050_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 051 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell051BalancerDrive(pub bool);

pub(crate) static CELL051_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 051 Balancer drive", pos: 8, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell051BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL051_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 051 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell051Voltage(pub crate::db::units::Volts);

pub(crate) static CELL051_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 051 voltage", pos: 104, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell051Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL051_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 052 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell052BalancerDrive(pub bool);

pub(crate) static CELL052_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 052 Balancer drive", pos: 8, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell052BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL052_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 052 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell052Voltage(pub crate::db::units::Volts);

pub(crate) static CELL052_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 052 voltage", pos: 106, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell052Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL052_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 053 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell053BalancerDrive(pub bool);

pub(crate) static CELL053_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 053 Balancer drive", pos: 8, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell053BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL053_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 053 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell053Voltage(pub crate::db::units::Volts);

pub(crate) static CELL053_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 053 voltage", pos: 108, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell053Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL053_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 054 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell054BalancerDrive(pub bool);

pub(crate) static CELL054_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 054 Balancer drive", pos: 8, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell054BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL054_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 054 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell054Voltage(pub crate::db::units::Volts);

pub(crate) static CELL054_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 054 voltage", pos: 110, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell054Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL054_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 055 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell055BalancerDrive(pub bool);

pub(crate) static CELL055_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 055 Balancer drive", pos: 8, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell055BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL055_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 055 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell055Voltage(pub crate::db::units::Volts);

pub(crate) static CELL055_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 055 voltage", pos: 112, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell055Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL055_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 056 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell056BalancerDrive(pub bool);

pub(crate) static CELL056_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 056 Balancer drive", pos: 9, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell056BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL056_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 056 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell056Voltage(pub crate::db::units::Volts);

pub(crate) static CELL056_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 056 voltage", pos: 114, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell056Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL056_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 057 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell057BalancerDrive(pub bool);

pub(crate) static CELL057_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 057 Balancer drive", pos: 9, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell057BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL057_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 057 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell057Voltage(pub crate::db::units::Volts);

pub(crate) static CELL057_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 057 voltage", pos: 116, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell057Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL057_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 058 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell058BalancerDrive(pub bool);

pub(crate) static CELL058_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 058 Balancer drive", pos: 9, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell058BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL058_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 058 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell058Voltage(pub crate::db::units::Volts);

pub(crate) static CELL058_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 058 voltage", pos: 118, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell058Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL058_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 059 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell059BalancerDrive(pub bool);

pub(crate) static CELL059_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 059 Balancer drive", pos: 9, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell059BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL059_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 059 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell059Voltage(pub crate::db::units::Volts);

pub(crate) static CELL059_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 059 voltage", pos: 120, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell059Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL059_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 060 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell060BalancerDrive(pub bool);

pub(crate) static CELL060_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 060 Balancer drive", pos: 9, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell060BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL060_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 060 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell060Voltage(pub crate::db::units::Volts);

pub(crate) static CELL060_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 060 voltage", pos: 122, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell060Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL060_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 061 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell061BalancerDrive(pub bool);

pub(crate) static CELL061_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 061 Balancer drive", pos: 9, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell061BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL061_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 061 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell061Voltage(pub crate::db::units::Volts);

pub(crate) static CELL061_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 061 voltage", pos: 124, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell061Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL061_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 062 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell062BalancerDrive(pub bool);

pub(crate) static CELL062_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 062 Balancer drive", pos: 9, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell062BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL062_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 062 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell062Voltage(pub crate::db::units::Volts);

pub(crate) static CELL062_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 062 voltage", pos: 126, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell062Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL062_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 063 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell063BalancerDrive(pub bool);

pub(crate) static CELL063_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 063 Balancer drive", pos: 9, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell063BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL063_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 063 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell063Voltage(pub crate::db::units::Volts);

pub(crate) static CELL063_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 063 voltage", pos: 128, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell063Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL063_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 064 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell064BalancerDrive(pub bool);

pub(crate) static CELL064_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 064 Balancer drive", pos: 10, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell064BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL064_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 064 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell064Voltage(pub crate::db::units::Volts);

pub(crate) static CELL064_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 064 voltage", pos: 130, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell064Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL064_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 065 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell065BalancerDrive(pub bool);

pub(crate) static CELL065_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 065 Balancer drive", pos: 10, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell065BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL065_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 065 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell065Voltage(pub crate::db::units::Volts);

pub(crate) static CELL065_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 065 voltage", pos: 132, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell065Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL065_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 066 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell066BalancerDrive(pub bool);

pub(crate) static CELL066_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 066 Balancer drive", pos: 10, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell066BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL066_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 066 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell066Voltage(pub crate::db::units::Volts);

pub(crate) static CELL066_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 066 voltage", pos: 134, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell066Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL066_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 067 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell067BalancerDrive(pub bool);

pub(crate) static CELL067_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 067 Balancer drive", pos: 10, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell067BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL067_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 067 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell067Voltage(pub crate::db::units::Volts);

pub(crate) static CELL067_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 067 voltage", pos: 136, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell067Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL067_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 068 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell068BalancerDrive(pub bool);

pub(crate) static CELL068_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 068 Balancer drive", pos: 10, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell068BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL068_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 068 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell068Voltage(pub crate::db::units::Volts);

pub(crate) static CELL068_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 068 voltage", pos: 138, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell068Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL068_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 069 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell069BalancerDrive(pub bool);

pub(crate) static CELL069_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 069 Balancer drive", pos: 10, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell069BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL069_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 069 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell069Voltage(pub crate::db::units::Volts);

pub(crate) static CELL069_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 069 voltage", pos: 140, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell069Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL069_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 070 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell070BalancerDrive(pub bool);

pub(crate) static CELL070_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 070 Balancer drive", pos: 10, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell070BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL070_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 070 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell070Voltage(pub crate::db::units::Volts);

pub(crate) static CELL070_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 070 voltage", pos: 142, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell070Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL070_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 071 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell071BalancerDrive(pub bool);

pub(crate) static CELL071_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 071 Balancer drive", pos: 10, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell071BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL071_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 071 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell071Voltage(pub crate::db::units::Volts);

pub(crate) static CELL071_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 071 voltage", pos: 144, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell071Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL071_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 072 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell072BalancerDrive(pub bool);

pub(crate) static CELL072_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 072 Balancer drive", pos: 11, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell072BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL072_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 072 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell072Voltage(pub crate::db::units::Volts);

pub(crate) static CELL072_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 072 voltage", pos: 146, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell072Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL072_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 073 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell073BalancerDrive(pub bool);

pub(crate) static CELL073_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 073 Balancer drive", pos: 11, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell073BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL073_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 073 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell073Voltage(pub crate::db::units::Volts);

pub(crate) static CELL073_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 073 voltage", pos: 148, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell073Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL073_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 074 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell074BalancerDrive(pub bool);

pub(crate) static CELL074_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 074 Balancer drive", pos: 11, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell074BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL074_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 074 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell074Voltage(pub crate::db::units::Volts);

pub(crate) static CELL074_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 074 voltage", pos: 150, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell074Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL074_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 075 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell075BalancerDrive(pub bool);

pub(crate) static CELL075_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 075 Balancer drive", pos: 11, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell075BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL075_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 075 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell075Voltage(pub crate::db::units::Volts);

pub(crate) static CELL075_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 075 voltage", pos: 152, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell075Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL075_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 076 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell076BalancerDrive(pub bool);

pub(crate) static CELL076_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 076 Balancer drive", pos: 11, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell076BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL076_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 076 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell076Voltage(pub crate::db::units::Volts);

pub(crate) static CELL076_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 076 voltage", pos: 154, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell076Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL076_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 077 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell077BalancerDrive(pub bool);

pub(crate) static CELL077_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 077 Balancer drive", pos: 11, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell077BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL077_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 077 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell077Voltage(pub crate::db::units::Volts);

pub(crate) static CELL077_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 077 voltage", pos: 156, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell077Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL077_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 078 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell078BalancerDrive(pub bool);

pub(crate) static CELL078_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 078 Balancer drive", pos: 11, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell078BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL078_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 078 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell078Voltage(pub crate::db::units::Volts);

pub(crate) static CELL078_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 078 voltage", pos: 158, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell078Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL078_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 079 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell079BalancerDrive(pub bool);

pub(crate) static CELL079_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 079 Balancer drive", pos: 11, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell079BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL079_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 079 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell079Voltage(pub crate::db::units::Volts);

pub(crate) static CELL079_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 079 voltage", pos: 160, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell079Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL079_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 080 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell080BalancerDrive(pub bool);

pub(crate) static CELL080_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 080 Balancer drive", pos: 12, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell080BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL080_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 080 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell080Voltage(pub crate::db::units::Volts);

pub(crate) static CELL080_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 080 voltage", pos: 162, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell080Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL080_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 081 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell081BalancerDrive(pub bool);

pub(crate) static CELL081_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 081 Balancer drive", pos: 12, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell081BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL081_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 081 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell081Voltage(pub crate::db::units::Volts);

pub(crate) static CELL081_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 081 voltage", pos: 164, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell081Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL081_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 082 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell082BalancerDrive(pub bool);

pub(crate) static CELL082_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 082 Balancer drive", pos: 12, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell082BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL082_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 082 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell082Voltage(pub crate::db::units::Volts);

pub(crate) static CELL082_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 082 voltage", pos: 166, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell082Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL082_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 083 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell083BalancerDrive(pub bool);

pub(crate) static CELL083_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 083 Balancer drive", pos: 12, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell083BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL083_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 083 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell083Voltage(pub crate::db::units::Volts);

pub(crate) static CELL083_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 083 voltage", pos: 168, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell083Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL083_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 084 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell084BalancerDrive(pub bool);

pub(crate) static CELL084_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 084 Balancer drive", pos: 12, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell084BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL084_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 084 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell084Voltage(pub crate::db::units::Volts);

pub(crate) static CELL084_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 084 voltage", pos: 170, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell084Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL084_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 085 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell085BalancerDrive(pub bool);

pub(crate) static CELL085_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 085 Balancer drive", pos: 12, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell085BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL085_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 085 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell085Voltage(pub crate::db::units::Volts);

pub(crate) static CELL085_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 085 voltage", pos: 172, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell085Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL085_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 086 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell086BalancerDrive(pub bool);

pub(crate) static CELL086_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 086 Balancer drive", pos: 12, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell086BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL086_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 086 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell086Voltage(pub crate::db::units::Volts);

pub(crate) static CELL086_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 086 voltage", pos: 174, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell086Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL086_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 087 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell087BalancerDrive(pub bool);

pub(crate) static CELL087_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 087 Balancer drive", pos: 12, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell087BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL087_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 087 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell087Voltage(pub crate::db::units::Volts);

pub(crate) static CELL087_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 087 voltage", pos: 176, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell087Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL087_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 088 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell088BalancerDrive(pub bool);

pub(crate) static CELL088_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 088 Balancer drive", pos: 13, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell088BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL088_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 088 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell088Voltage(pub crate::db::units::Volts);

pub(crate) static CELL088_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 088 voltage", pos: 178, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell088Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL088_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 089 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell089BalancerDrive(pub bool);

pub(crate) static CELL089_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 089 Balancer drive", pos: 13, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell089BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL089_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 089 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell089Voltage(pub crate::db::units::Volts);

pub(crate) static CELL089_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 089 voltage", pos: 180, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell089Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL089_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 090 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell090BalancerDrive(pub bool);

pub(crate) static CELL090_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 090 Balancer drive", pos: 13, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell090BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL090_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 090 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell090Voltage(pub crate::db::units::Volts);

pub(crate) static CELL090_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 090 voltage", pos: 182, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell090Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL090_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 091 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell091BalancerDrive(pub bool);

pub(crate) static CELL091_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 091 Balancer drive", pos: 13, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell091BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL091_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 091 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell091Voltage(pub crate::db::units::Volts);

pub(crate) static CELL091_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 091 voltage", pos: 184, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell091Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL091_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 092 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell092BalancerDrive(pub bool);

pub(crate) static CELL092_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 092 Balancer drive", pos: 13, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell092BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL092_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 092 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell092Voltage(pub crate::db::units::Volts);

pub(crate) static CELL092_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 092 voltage", pos: 186, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell092Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL092_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 093 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell093BalancerDrive(pub bool);

pub(crate) static CELL093_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 093 Balancer drive", pos: 13, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell093BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL093_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 093 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell093Voltage(pub crate::db::units::Volts);

pub(crate) static CELL093_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 093 voltage", pos: 188, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell093Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL093_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 094 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell094BalancerDrive(pub bool);

pub(crate) static CELL094_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 094 Balancer drive", pos: 13, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell094BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL094_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 094 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell094Voltage(pub crate::db::units::Volts);

pub(crate) static CELL094_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 094 voltage", pos: 190, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell094Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL094_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 095 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell095BalancerDrive(pub bool);

pub(crate) static CELL095_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 095 Balancer drive", pos: 13, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell095BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL095_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 095 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell095Voltage(pub crate::db::units::Volts);

pub(crate) static CELL095_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 095 voltage", pos: 192, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell095Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL095_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 096 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell096BalancerDrive(pub bool);

pub(crate) static CELL096_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 096 Balancer drive", pos: 14, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell096BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL096_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 096 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell096Voltage(pub crate::db::units::Volts);

pub(crate) static CELL096_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 096 voltage", pos: 194, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell096Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL096_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 097 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell097BalancerDrive(pub bool);

pub(crate) static CELL097_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 097 Balancer drive", pos: 14, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell097BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL097_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 097 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell097Voltage(pub crate::db::units::Volts);

pub(crate) static CELL097_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 097 voltage", pos: 196, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell097Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL097_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 098 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell098BalancerDrive(pub bool);

pub(crate) static CELL098_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 098 Balancer drive", pos: 14, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell098BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL098_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 098 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell098Voltage(pub crate::db::units::Volts);

pub(crate) static CELL098_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 098 voltage", pos: 198, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell098Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL098_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 099 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell099BalancerDrive(pub bool);

pub(crate) static CELL099_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 099 Balancer drive", pos: 14, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell099BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL099_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 099 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell099Voltage(pub crate::db::units::Volts);

pub(crate) static CELL099_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 099 voltage", pos: 200, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell099Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL099_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 100 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell100BalancerDrive(pub bool);

pub(crate) static CELL100_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 100 Balancer drive", pos: 14, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell100BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL100_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 100 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell100Voltage(pub crate::db::units::Volts);

pub(crate) static CELL100_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 100 voltage", pos: 202, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell100Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL100_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 101 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell101BalancerDrive(pub bool);

pub(crate) static CELL101_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 101 Balancer drive", pos: 14, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell101BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL101_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 101 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell101Voltage(pub crate::db::units::Volts);

pub(crate) static CELL101_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 101 voltage", pos: 204, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell101Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL101_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 102 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell102BalancerDrive(pub bool);

pub(crate) static CELL102_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 102 Balancer drive", pos: 14, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell102BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL102_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 102 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell102Voltage(pub crate::db::units::Volts);

pub(crate) static CELL102_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 102 voltage", pos: 206, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell102Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL102_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 103 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell103BalancerDrive(pub bool);

pub(crate) static CELL103_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 103 Balancer drive", pos: 14, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell103BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL103_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 103 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell103Voltage(pub crate::db::units::Volts);

pub(crate) static CELL103_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 103 voltage", pos: 208, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell103Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL103_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 104 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell104BalancerDrive(pub bool);

pub(crate) static CELL104_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 104 Balancer drive", pos: 15, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell104BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL104_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 104 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell104Voltage(pub crate::db::units::Volts);

pub(crate) static CELL104_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 104 voltage", pos: 210, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell104Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL104_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 105 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell105BalancerDrive(pub bool);

pub(crate) static CELL105_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 105 Balancer drive", pos: 15, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell105BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL105_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 105 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell105Voltage(pub crate::db::units::Volts);

pub(crate) static CELL105_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 105 voltage", pos: 212, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell105Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL105_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 106 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell106BalancerDrive(pub bool);

pub(crate) static CELL106_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 106 Balancer drive", pos: 15, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell106BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL106_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 106 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell106Voltage(pub crate::db::units::Volts);

pub(crate) static CELL106_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 106 voltage", pos: 214, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell106Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL106_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 107 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell107BalancerDrive(pub bool);

pub(crate) static CELL107_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 107 Balancer drive", pos: 15, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell107BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL107_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 107 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell107Voltage(pub crate::db::units::Volts);

pub(crate) static CELL107_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 107 voltage", pos: 216, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell107Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL107_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 108 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell108BalancerDrive(pub bool);

pub(crate) static CELL108_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 108 Balancer drive", pos: 15, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell108BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL108_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 108 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell108Voltage(pub crate::db::units::Volts);

pub(crate) static CELL108_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 108 voltage", pos: 218, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell108Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL108_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 109 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell109BalancerDrive(pub bool);

pub(crate) static CELL109_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 109 Balancer drive", pos: 15, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell109BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL109_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 109 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell109Voltage(pub crate::db::units::Volts);

pub(crate) static CELL109_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 109 voltage", pos: 220, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell109Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL109_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 110 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell110BalancerDrive(pub bool);

pub(crate) static CELL110_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 110 Balancer drive", pos: 15, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell110BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL110_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 110 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell110Voltage(pub crate::db::units::Volts);

pub(crate) static CELL110_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 110 voltage", pos: 222, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell110Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL110_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 111 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell111BalancerDrive(pub bool);

pub(crate) static CELL111_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 111 Balancer drive", pos: 15, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell111BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL111_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 111 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell111Voltage(pub crate::db::units::Volts);

pub(crate) static CELL111_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 111 voltage", pos: 224, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell111Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL111_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 112 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell112BalancerDrive(pub bool);

pub(crate) static CELL112_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 112 Balancer drive", pos: 16, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell112BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL112_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 112 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell112Voltage(pub crate::db::units::Volts);

pub(crate) static CELL112_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 112 voltage", pos: 226, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell112Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL112_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 113 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell113BalancerDrive(pub bool);

pub(crate) static CELL113_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 113 Balancer drive", pos: 16, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell113BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL113_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 113 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell113Voltage(pub crate::db::units::Volts);

pub(crate) static CELL113_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 113 voltage", pos: 228, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell113Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL113_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 114 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell114BalancerDrive(pub bool);

pub(crate) static CELL114_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 114 Balancer drive", pos: 16, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell114BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL114_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 114 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell114Voltage(pub crate::db::units::Volts);

pub(crate) static CELL114_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 114 voltage", pos: 230, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell114Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL114_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 115 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell115BalancerDrive(pub bool);

pub(crate) static CELL115_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 115 Balancer drive", pos: 16, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell115BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL115_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 115 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell115Voltage(pub crate::db::units::Volts);

pub(crate) static CELL115_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 115 voltage", pos: 232, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell115Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL115_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 116 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell116BalancerDrive(pub bool);

pub(crate) static CELL116_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 116 Balancer drive", pos: 16, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell116BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL116_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 116 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell116Voltage(pub crate::db::units::Volts);

pub(crate) static CELL116_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 116 voltage", pos: 234, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell116Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL116_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 117 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell117BalancerDrive(pub bool);

pub(crate) static CELL117_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 117 Balancer drive", pos: 16, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell117BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL117_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 117 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell117Voltage(pub crate::db::units::Volts);

pub(crate) static CELL117_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 117 voltage", pos: 236, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell117Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL117_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 118 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell118BalancerDrive(pub bool);

pub(crate) static CELL118_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 118 Balancer drive", pos: 16, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell118BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL118_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 118 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell118Voltage(pub crate::db::units::Volts);

pub(crate) static CELL118_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 118 voltage", pos: 238, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell118Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL118_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 119 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell119BalancerDrive(pub bool);

pub(crate) static CELL119_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 119 Balancer drive", pos: 16, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell119BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL119_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 119 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell119Voltage(pub crate::db::units::Volts);

pub(crate) static CELL119_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 119 voltage", pos: 240, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell119Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL119_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 120 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell120BalancerDrive(pub bool);

pub(crate) static CELL120_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 120 Balancer drive", pos: 17, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell120BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL120_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 120 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell120Voltage(pub crate::db::units::Volts);

pub(crate) static CELL120_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 120 voltage", pos: 242, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell120Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL120_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 121 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell121BalancerDrive(pub bool);

pub(crate) static CELL121_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 121 Balancer drive", pos: 17, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell121BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL121_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 121 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell121Voltage(pub crate::db::units::Volts);

pub(crate) static CELL121_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 121 voltage", pos: 244, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell121Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL121_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 122 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell122BalancerDrive(pub bool);

pub(crate) static CELL122_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 122 Balancer drive", pos: 17, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell122BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL122_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 122 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell122Voltage(pub crate::db::units::Volts);

pub(crate) static CELL122_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 122 voltage", pos: 246, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell122Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL122_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 123 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell123BalancerDrive(pub bool);

pub(crate) static CELL123_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 123 Balancer drive", pos: 17, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell123BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL123_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 123 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell123Voltage(pub crate::db::units::Volts);

pub(crate) static CELL123_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 123 voltage", pos: 248, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell123Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL123_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 124 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell124BalancerDrive(pub bool);

pub(crate) static CELL124_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 124 Balancer drive", pos: 17, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell124BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL124_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 124 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell124Voltage(pub crate::db::units::Volts);

pub(crate) static CELL124_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 124 voltage", pos: 250, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell124Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL124_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 125 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell125BalancerDrive(pub bool);

pub(crate) static CELL125_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 125 Balancer drive", pos: 17, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell125BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL125_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 125 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell125Voltage(pub crate::db::units::Volts);

pub(crate) static CELL125_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 125 voltage", pos: 252, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell125Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL125_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 126 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell126BalancerDrive(pub bool);

pub(crate) static CELL126_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 126 Balancer drive", pos: 17, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell126BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL126_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 126 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell126Voltage(pub crate::db::units::Volts);

pub(crate) static CELL126_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 126 voltage", pos: 254, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell126Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL126_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 127 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell127BalancerDrive(pub bool);

pub(crate) static CELL127_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 127 Balancer drive", pos: 17, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell127BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL127_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 127 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell127Voltage(pub crate::db::units::Volts);

pub(crate) static CELL127_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 127 voltage", pos: 256, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell127Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL127_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 128 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell128BalancerDrive(pub bool);

pub(crate) static CELL128_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 128 Balancer drive", pos: 18, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell128BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL128_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 128 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell128Voltage(pub crate::db::units::Volts);

pub(crate) static CELL128_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 128 voltage", pos: 258, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell128Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL128_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 129 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell129BalancerDrive(pub bool);

pub(crate) static CELL129_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 129 Balancer drive", pos: 18, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell129BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL129_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 129 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell129Voltage(pub crate::db::units::Volts);

pub(crate) static CELL129_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 129 voltage", pos: 260, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell129Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL129_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 130 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell130BalancerDrive(pub bool);

pub(crate) static CELL130_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 130 Balancer drive", pos: 18, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell130BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL130_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 130 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell130Voltage(pub crate::db::units::Volts);

pub(crate) static CELL130_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 130 voltage", pos: 262, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell130Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL130_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 131 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell131BalancerDrive(pub bool);

pub(crate) static CELL131_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 131 Balancer drive", pos: 18, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell131BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL131_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 131 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell131Voltage(pub crate::db::units::Volts);

pub(crate) static CELL131_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 131 voltage", pos: 264, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell131Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL131_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 132 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell132BalancerDrive(pub bool);

pub(crate) static CELL132_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 132 Balancer drive", pos: 18, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell132BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL132_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 132 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell132Voltage(pub crate::db::units::Volts);

pub(crate) static CELL132_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 132 voltage", pos: 266, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell132Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL132_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 133 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell133BalancerDrive(pub bool);

pub(crate) static CELL133_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 133 Balancer drive", pos: 18, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell133BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL133_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 133 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell133Voltage(pub crate::db::units::Volts);

pub(crate) static CELL133_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 133 voltage", pos: 268, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell133Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL133_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 134 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell134BalancerDrive(pub bool);

pub(crate) static CELL134_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 134 Balancer drive", pos: 18, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell134BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL134_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 134 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell134Voltage(pub crate::db::units::Volts);

pub(crate) static CELL134_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 134 voltage", pos: 270, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell134Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL134_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 135 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell135BalancerDrive(pub bool);

pub(crate) static CELL135_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 135 Balancer drive", pos: 18, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell135BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL135_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 135 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell135Voltage(pub crate::db::units::Volts);

pub(crate) static CELL135_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 135 voltage", pos: 272, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell135Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL135_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 136 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell136BalancerDrive(pub bool);

pub(crate) static CELL136_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 136 Balancer drive", pos: 19, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell136BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL136_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 136 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell136Voltage(pub crate::db::units::Volts);

pub(crate) static CELL136_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 136 voltage", pos: 274, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell136Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL136_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 137 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell137BalancerDrive(pub bool);

pub(crate) static CELL137_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 137 Balancer drive", pos: 19, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell137BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL137_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 137 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell137Voltage(pub crate::db::units::Volts);

pub(crate) static CELL137_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 137 voltage", pos: 276, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell137Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL137_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 138 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell138BalancerDrive(pub bool);

pub(crate) static CELL138_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 138 Balancer drive", pos: 19, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell138BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL138_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 138 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell138Voltage(pub crate::db::units::Volts);

pub(crate) static CELL138_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 138 voltage", pos: 278, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell138Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL138_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 139 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell139BalancerDrive(pub bool);

pub(crate) static CELL139_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 139 Balancer drive", pos: 19, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell139BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL139_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 139 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell139Voltage(pub crate::db::units::Volts);

pub(crate) static CELL139_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 139 voltage", pos: 280, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell139Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL139_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 140 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell140BalancerDrive(pub bool);

pub(crate) static CELL140_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 140 Balancer drive", pos: 19, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell140BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL140_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 140 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell140Voltage(pub crate::db::units::Volts);

pub(crate) static CELL140_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 140 voltage", pos: 282, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell140Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL140_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 141 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell141BalancerDrive(pub bool);

pub(crate) static CELL141_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 141 Balancer drive", pos: 19, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell141BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL141_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 141 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell141Voltage(pub crate::db::units::Volts);

pub(crate) static CELL141_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 141 voltage", pos: 284, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell141Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL141_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 142 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell142BalancerDrive(pub bool);

pub(crate) static CELL142_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 142 Balancer drive", pos: 19, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell142BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL142_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 142 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell142Voltage(pub crate::db::units::Volts);

pub(crate) static CELL142_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 142 voltage", pos: 286, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell142Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL142_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell 143 Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Cell143BalancerDrive(pub bool);

pub(crate) static CELL143_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "Cell 143 Balancer drive", pos: 19, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Cell143BalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x04), item: &CELL143_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Cell 143 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell143Voltage(pub crate::db::units::Volts);

pub(crate) static CELL143_VOLTAGE: [Item; 1] = [
    Item { name: "Cell 143 voltage", pos: 288, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for Cell143Voltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x02), item: &CELL143_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Cell capacity(measured) MAX(01)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCapacityMeasuredMax01(pub crate::db::units::AmpHours);

pub(crate) static CELL_CAPACITY_MEASURED_MAX01: [Item; 1] = [
    Item { name: "Cell capacity(measured) MAX(01)", pos: 2, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CellCapacityMeasuredMax01 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x14), item: &CELL_CAPACITY_MEASURED_MAX01[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Cell capacity(measured) MAX(02)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCapacityMeasuredMax02(pub crate::db::units::AmpHours);

pub(crate) static CELL_CAPACITY_MEASURED_MAX02: [Item; 1] = [
    Item { name: "Cell capacity(measured) MAX(02)", pos: 6, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CellCapacityMeasuredMax02 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x14), item: &CELL_CAPACITY_MEASURED_MAX02[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Cell capacity(measured) MAX(03)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCapacityMeasuredMax03(pub crate::db::units::AmpHours);

pub(crate) static CELL_CAPACITY_MEASURED_MAX03: [Item; 1] = [
    Item { name: "Cell capacity(measured) MAX(03)", pos: 10, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CellCapacityMeasuredMax03 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x14), item: &CELL_CAPACITY_MEASURED_MAX03[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Cell capacity(measured) MAX(04)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCapacityMeasuredMax04(pub crate::db::units::AmpHours);

pub(crate) static CELL_CAPACITY_MEASURED_MAX04: [Item; 1] = [
    Item { name: "Cell capacity(measured) MAX(04)", pos: 14, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CellCapacityMeasuredMax04 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x14), item: &CELL_CAPACITY_MEASURED_MAX04[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Cell capacity(measured) MAX(05)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCapacityMeasuredMax05(pub crate::db::units::AmpHours);

pub(crate) static CELL_CAPACITY_MEASURED_MAX05: [Item; 1] = [
    Item { name: "Cell capacity(measured) MAX(05)", pos: 18, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CellCapacityMeasuredMax05 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x14), item: &CELL_CAPACITY_MEASURED_MAX05[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Cell capacity(measured) MAX(06)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCapacityMeasuredMax06(pub crate::db::units::AmpHours);

pub(crate) static CELL_CAPACITY_MEASURED_MAX06: [Item; 1] = [
    Item { name: "Cell capacity(measured) MAX(06)", pos: 22, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CellCapacityMeasuredMax06 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x14), item: &CELL_CAPACITY_MEASURED_MAX06[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Cell capacity(measured) MAX(07)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCapacityMeasuredMax07(pub crate::db::units::AmpHours);

pub(crate) static CELL_CAPACITY_MEASURED_MAX07: [Item; 1] = [
    Item { name: "Cell capacity(measured) MAX(07)", pos: 26, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CellCapacityMeasuredMax07 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x14), item: &CELL_CAPACITY_MEASURED_MAX07[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Cell capacity(measured) MAX(08)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCapacityMeasuredMax08(pub crate::db::units::AmpHours);

pub(crate) static CELL_CAPACITY_MEASURED_MAX08: [Item; 1] = [
    Item { name: "Cell capacity(measured) MAX(08)", pos: 30, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CellCapacityMeasuredMax08 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x14), item: &CELL_CAPACITY_MEASURED_MAX08[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Cell capacity(measured) MAX(09)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCapacityMeasuredMax09(pub crate::db::units::AmpHours);

pub(crate) static CELL_CAPACITY_MEASURED_MAX09: [Item; 1] = [
    Item { name: "Cell capacity(measured) MAX(09)", pos: 34, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CellCapacityMeasuredMax09 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x14), item: &CELL_CAPACITY_MEASURED_MAX09[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Cell capacity(measured) MAX(10)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCapacityMeasuredMax10(pub crate::db::units::AmpHours);

pub(crate) static CELL_CAPACITY_MEASURED_MAX10: [Item; 1] = [
    Item { name: "Cell capacity(measured) MAX(10)", pos: 38, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CellCapacityMeasuredMax10 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x14), item: &CELL_CAPACITY_MEASURED_MAX10[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Cell capacity(measured) MIN(01)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCapacityMeasuredMin01(pub crate::db::units::AmpHours);

pub(crate) static CELL_CAPACITY_MEASURED_MIN01: [Item; 1] = [
    Item { name: "Cell capacity(measured) MIN(01)", pos: 4, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CellCapacityMeasuredMin01 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x14), item: &CELL_CAPACITY_MEASURED_MIN01[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Cell capacity(measured) MIN(02)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCapacityMeasuredMin02(pub crate::db::units::AmpHours);

pub(crate) static CELL_CAPACITY_MEASURED_MIN02: [Item; 1] = [
    Item { name: "Cell capacity(measured) MIN(02)", pos: 8, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CellCapacityMeasuredMin02 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x14), item: &CELL_CAPACITY_MEASURED_MIN02[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Cell capacity(measured) MIN(03)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCapacityMeasuredMin03(pub crate::db::units::AmpHours);

pub(crate) static CELL_CAPACITY_MEASURED_MIN03: [Item; 1] = [
    Item { name: "Cell capacity(measured) MIN(03)", pos: 12, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CellCapacityMeasuredMin03 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x14), item: &CELL_CAPACITY_MEASURED_MIN03[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Cell capacity(measured) MIN(04)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCapacityMeasuredMin04(pub crate::db::units::AmpHours);

pub(crate) static CELL_CAPACITY_MEASURED_MIN04: [Item; 1] = [
    Item { name: "Cell capacity(measured) MIN(04)", pos: 16, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CellCapacityMeasuredMin04 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x14), item: &CELL_CAPACITY_MEASURED_MIN04[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Cell capacity(measured) MIN(05)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCapacityMeasuredMin05(pub crate::db::units::AmpHours);

pub(crate) static CELL_CAPACITY_MEASURED_MIN05: [Item; 1] = [
    Item { name: "Cell capacity(measured) MIN(05)", pos: 20, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CellCapacityMeasuredMin05 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x14), item: &CELL_CAPACITY_MEASURED_MIN05[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Cell capacity(measured) MIN(06)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCapacityMeasuredMin06(pub crate::db::units::AmpHours);

pub(crate) static CELL_CAPACITY_MEASURED_MIN06: [Item; 1] = [
    Item { name: "Cell capacity(measured) MIN(06)", pos: 24, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CellCapacityMeasuredMin06 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x14), item: &CELL_CAPACITY_MEASURED_MIN06[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Cell capacity(measured) MIN(07)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCapacityMeasuredMin07(pub crate::db::units::AmpHours);

pub(crate) static CELL_CAPACITY_MEASURED_MIN07: [Item; 1] = [
    Item { name: "Cell capacity(measured) MIN(07)", pos: 28, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CellCapacityMeasuredMin07 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x14), item: &CELL_CAPACITY_MEASURED_MIN07[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Cell capacity(measured) MIN(08)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCapacityMeasuredMin08(pub crate::db::units::AmpHours);

pub(crate) static CELL_CAPACITY_MEASURED_MIN08: [Item; 1] = [
    Item { name: "Cell capacity(measured) MIN(08)", pos: 32, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CellCapacityMeasuredMin08 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x14), item: &CELL_CAPACITY_MEASURED_MIN08[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Cell capacity(measured) MIN(09)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCapacityMeasuredMin09(pub crate::db::units::AmpHours);

pub(crate) static CELL_CAPACITY_MEASURED_MIN09: [Item; 1] = [
    Item { name: "Cell capacity(measured) MIN(09)", pos: 36, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CellCapacityMeasuredMin09 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x14), item: &CELL_CAPACITY_MEASURED_MIN09[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Cell capacity(measured) MIN(10)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCapacityMeasuredMin10(pub crate::db::units::AmpHours);

pub(crate) static CELL_CAPACITY_MEASURED_MIN10: [Item; 1] = [
    Item { name: "Cell capacity(measured) MIN(10)", pos: 40, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CellCapacityMeasuredMin10 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x14), item: &CELL_CAPACITY_MEASURED_MIN10[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Cell capacity measured ODO (01)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCapacityMeasuredOdo01(pub crate::db::units::Km);

pub(crate) static CELL_CAPACITY_MEASURED_ODO01: [Item; 1] = [
    Item { name: "Cell capacity measured ODO (01)", pos: 2, len: 2, bits: None, unit: Unit::Km, conv: Conv::Linear { factor: 6.4, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for CellCapacityMeasuredOdo01 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x15), item: &CELL_CAPACITY_MEASURED_ODO01[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Km(n)))
    }
}

/// Cell capacity measured ODO (02)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCapacityMeasuredOdo02(pub crate::db::units::Km);

pub(crate) static CELL_CAPACITY_MEASURED_ODO02: [Item; 1] = [
    Item { name: "Cell capacity measured ODO (02)", pos: 4, len: 2, bits: None, unit: Unit::Km, conv: Conv::Linear { factor: 6.4, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for CellCapacityMeasuredOdo02 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x15), item: &CELL_CAPACITY_MEASURED_ODO02[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Km(n)))
    }
}

/// Cell capacity measured ODO (03)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCapacityMeasuredOdo03(pub crate::db::units::Km);

pub(crate) static CELL_CAPACITY_MEASURED_ODO03: [Item; 1] = [
    Item { name: "Cell capacity measured ODO (03)", pos: 6, len: 2, bits: None, unit: Unit::Km, conv: Conv::Linear { factor: 6.4, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for CellCapacityMeasuredOdo03 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x15), item: &CELL_CAPACITY_MEASURED_ODO03[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Km(n)))
    }
}

/// Cell capacity measured ODO (04)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCapacityMeasuredOdo04(pub crate::db::units::Km);

pub(crate) static CELL_CAPACITY_MEASURED_ODO04: [Item; 1] = [
    Item { name: "Cell capacity measured ODO (04)", pos: 8, len: 2, bits: None, unit: Unit::Km, conv: Conv::Linear { factor: 6.4, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for CellCapacityMeasuredOdo04 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x15), item: &CELL_CAPACITY_MEASURED_ODO04[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Km(n)))
    }
}

/// Cell capacity measured ODO (05)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCapacityMeasuredOdo05(pub crate::db::units::Km);

pub(crate) static CELL_CAPACITY_MEASURED_ODO05: [Item; 1] = [
    Item { name: "Cell capacity measured ODO (05)", pos: 10, len: 2, bits: None, unit: Unit::Km, conv: Conv::Linear { factor: 6.4, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for CellCapacityMeasuredOdo05 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x15), item: &CELL_CAPACITY_MEASURED_ODO05[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Km(n)))
    }
}

/// Cell capacity measured ODO (06)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCapacityMeasuredOdo06(pub crate::db::units::Km);

pub(crate) static CELL_CAPACITY_MEASURED_ODO06: [Item; 1] = [
    Item { name: "Cell capacity measured ODO (06)", pos: 12, len: 2, bits: None, unit: Unit::Km, conv: Conv::Linear { factor: 6.4, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for CellCapacityMeasuredOdo06 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x15), item: &CELL_CAPACITY_MEASURED_ODO06[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Km(n)))
    }
}

/// Cell capacity measured ODO (07)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCapacityMeasuredOdo07(pub crate::db::units::Km);

pub(crate) static CELL_CAPACITY_MEASURED_ODO07: [Item; 1] = [
    Item { name: "Cell capacity measured ODO (07)", pos: 14, len: 2, bits: None, unit: Unit::Km, conv: Conv::Linear { factor: 6.4, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for CellCapacityMeasuredOdo07 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x15), item: &CELL_CAPACITY_MEASURED_ODO07[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Km(n)))
    }
}

/// Cell capacity measured ODO (08)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCapacityMeasuredOdo08(pub crate::db::units::Km);

pub(crate) static CELL_CAPACITY_MEASURED_ODO08: [Item; 1] = [
    Item { name: "Cell capacity measured ODO (08)", pos: 16, len: 2, bits: None, unit: Unit::Km, conv: Conv::Linear { factor: 6.4, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for CellCapacityMeasuredOdo08 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x15), item: &CELL_CAPACITY_MEASURED_ODO08[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Km(n)))
    }
}

/// Cell capacity measured ODO (09)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCapacityMeasuredOdo09(pub crate::db::units::Km);

pub(crate) static CELL_CAPACITY_MEASURED_ODO09: [Item; 1] = [
    Item { name: "Cell capacity measured ODO (09)", pos: 18, len: 2, bits: None, unit: Unit::Km, conv: Conv::Linear { factor: 6.4, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for CellCapacityMeasuredOdo09 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x15), item: &CELL_CAPACITY_MEASURED_ODO09[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Km(n)))
    }
}

/// Cell capacity measured ODO (10)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCapacityMeasuredOdo10(pub crate::db::units::Km);

pub(crate) static CELL_CAPACITY_MEASURED_ODO10: [Item; 1] = [
    Item { name: "Cell capacity measured ODO (10)", pos: 20, len: 2, bits: None, unit: Unit::Km, conv: Conv::Linear { factor: 6.4, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for CellCapacityMeasuredOdo10 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x15), item: &CELL_CAPACITY_MEASURED_ODO10[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Km(n)))
    }
}

/// Cell current capacity dif. pert
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCurrentCapacityDifPert(pub crate::db::units::Percent);

pub(crate) static CELL_CURRENT_CAPACITY_DIF_PERT: [Item; 1] = [
    Item { name: "Cell current capacity dif. pert", pos: 6, len: 2, bits: None, unit: Unit::Percent, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CellCurrentCapacityDifPert {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x13), item: &CELL_CURRENT_CAPACITY_DIF_PERT[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Percent(n)))
    }
}

/// Cell current capacity difference
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCurrentCapacityDifference(pub crate::db::units::AmpHours);

pub(crate) static CELL_CURRENT_CAPACITY_DIFFERENCE: [Item; 2] = [
    Item { name: "Cell current capacity difference", pos: 4, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: -60.0, decimals: 1 } },
    Item { name: "Cell current capacity difference", pos: 4, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: -127.0, decimals: 1 } },
];

impl crate::item::Reading for CellCurrentCapacityDifference {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0002, 0x0003, 0x0004, 0x0101, 0x0104], source: Source::Record(0x13), item: &CELL_CURRENT_CAPACITY_DIFFERENCE[0] },
        At { versions: &[0x0010, 0x0011], source: Source::Record(0x13), item: &CELL_CURRENT_CAPACITY_DIFFERENCE[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Cell current capacity (measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CellCurrentCapacityMeasured(pub crate::db::units::AmpHours);

pub(crate) static CELL_CURRENT_CAPACITY_MEASURED: [Item; 1] = [
    Item { name: "Cell current capacity (measured)", pos: 2, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CellCurrentCapacityMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0002, 0x0003, 0x0004, 0x0101, 0x0104, 0x0010, 0x0011], source: Source::Record(0x13), item: &CELL_CURRENT_CAPACITY_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Cell number specification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CellNumberSpecification {
    /// 88 cell
    Cell88,
    /// 72 cell
    Cell72,
    /// 80 cell
    Cell80,
    /// 117 cell
    Cell117,
    /// -
    Unknown,
}

pub(crate) static CELL_NUMBER_SPECIFICATION: [Item; 2] = [
    Item { name: "Cell number specification", pos: 26, len: 1, bits: Some((4, 4)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "88 cell"), (1, 1, "72 cell")]) },
    Item { name: "Cell number specification", pos: 26, len: 1, bits: Some((4, 4)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "88 cell"), (1, 1, "80 cell"), (2, 2, "117 cell"), (3, 3, "80 cell"), (4, 4, "72 cell"), (5, 5, "88 cell"), (6, 7, "-")]) },
];

impl crate::item::Reading for CellNumberSpecification {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x01), item: &CELL_NUMBER_SPECIFICATION[0] },
        At { versions: &[0x0002, 0x0003, 0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x01), item: &CELL_NUMBER_SPECIFICATION[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "88 cell" => Some(Self::Cell88),
            "72 cell" => Some(Self::Cell72),
            "80 cell" => Some(Self::Cell80),
            "117 cell" => Some(Self::Cell117),
            "-" => Some(Self::Unknown),
            _ => None,
        }
    }
}

/// Charge current integrated value
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChargeCurrentIntegratedValue(pub crate::db::units::AmpHours);

pub(crate) static CHARGE_CURRENT_INTEGRATED_VALUE: [Item; 1] = [
    Item { name: "Charge current integrated value", pos: 44, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for ChargeCurrentIntegratedValue {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x01), item: &CHARGE_CURRENT_INTEGRATED_VALUE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Charge&Discharge integrated cur.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChargeDischargeIntegratedCur(pub crate::db::units::AmpHours);

pub(crate) static CHARGE_DISCHARGE_INTEGRATED_CUR: [Item; 1] = [
    Item { name: "Charge&Discharge integrated cur.", pos: 18, len: 4, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for ChargeDischargeIntegratedCur {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0004, 0x0104, 0x0010, 0x0011], source: Source::Record(0x11), item: &CHARGE_DISCHARGE_INTEGRATED_CUR[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Charge state(Control)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChargeStateControl(pub crate::db::units::Percent);

pub(crate) static CHARGE_STATE_CONTROL: [Item; 1] = [
    Item { name: "Charge state(Control)", pos: 2, len: 1, bits: None, unit: Unit::Percent, conv: Conv::Linear { factor: 0.5, offset: -5.0, decimals: 1 } },
];

impl crate::item::Reading for ChargeStateControl {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0101, 0x0104, 0x0010, 0x0011], source: Source::Record(0x01), item: &CHARGE_STATE_CONTROL[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Percent(n)))
    }
}

/// Charge state(Display)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ChargeStateDisplay(pub crate::db::units::Percent);

pub(crate) static CHARGE_STATE_DISPLAY: [Item; 1] = [
    Item { name: "Charge state(Display)", pos: 3, len: 1, bits: None, unit: Unit::Percent, conv: Conv::Linear { factor: 0.5, offset: -5.0, decimals: 1 } },
];

impl crate::item::Reading for ChargeStateDisplay {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0101, 0x0104, 0x0010, 0x0011], source: Source::Record(0x01), item: &CHARGE_STATE_DISPLAY[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Percent(n)))
    }
}

/// CMU drive relay output
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuDriveRelayOutput(pub bool);

pub(crate) static CMU_DRIVE_RELAY_OUTPUT: [Item; 1] = [
    Item { name: "CMU drive relay output", pos: 26, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuDriveRelayOutput {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001], source: Source::Record(0x01), item: &CMU_DRIVE_RELAY_OUTPUT[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID01 Cell A Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId01CellABalancerDrive(pub bool);

pub(crate) static CMU_ID01_CELL_A_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID01 Cell A Balancer drive", pos: 2, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId01CellABalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID01_CELL_A_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID01 Cell A capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId01CellACapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID01_CELL_A_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID01 Cell A capa.(measured)", pos: 2, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId01CellACapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID01_CELL_A_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID01 Cell A voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId01CellAVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID01_CELL_A_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID01 Cell A voltage", pos: 2, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId01CellAVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID01_CELL_A_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID01 Cell B Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId01CellBBalancerDrive(pub bool);

pub(crate) static CMU_ID01_CELL_B_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID01 Cell B Balancer drive", pos: 2, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId01CellBBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID01_CELL_B_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID01 Cell B capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId01CellBCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID01_CELL_B_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID01 Cell B capa.(measured)", pos: 4, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId01CellBCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID01_CELL_B_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID01 Cell B voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId01CellBVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID01_CELL_B_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID01 Cell B voltage", pos: 4, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId01CellBVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID01_CELL_B_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID01 Cell C Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId01CellCBalancerDrive(pub bool);

pub(crate) static CMU_ID01_CELL_C_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID01 Cell C Balancer drive", pos: 2, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId01CellCBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID01_CELL_C_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID01 Cell C capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId01CellCCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID01_CELL_C_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID01 Cell C capa.(measured)", pos: 6, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId01CellCCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID01_CELL_C_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID01 Cell C voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId01CellCVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID01_CELL_C_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID01 Cell C voltage", pos: 6, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId01CellCVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID01_CELL_C_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID01 Cell D Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId01CellDBalancerDrive(pub bool);

pub(crate) static CMU_ID01_CELL_D_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID01 Cell D Balancer drive", pos: 2, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId01CellDBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID01_CELL_D_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID01 Cell D capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId01CellDCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID01_CELL_D_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID01 Cell D capa.(measured)", pos: 8, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId01CellDCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID01_CELL_D_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID01 Cell D voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId01CellDVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID01_CELL_D_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID01 Cell D voltage", pos: 8, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId01CellDVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID01_CELL_D_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID01 Cell E Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId01CellEBalancerDrive(pub bool);

pub(crate) static CMU_ID01_CELL_E_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID01 Cell E Balancer drive", pos: 2, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId01CellEBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID01_CELL_E_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID01 Cell E capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId01CellECapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID01_CELL_E_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID01 Cell E capa.(measured)", pos: 10, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId01CellECapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID01_CELL_E_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID01 Cell E voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId01CellEVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID01_CELL_E_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID01 Cell E voltage", pos: 10, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId01CellEVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID01_CELL_E_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID01 Cell F Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId01CellFBalancerDrive(pub bool);

pub(crate) static CMU_ID01_CELL_F_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID01 Cell F Balancer drive", pos: 2, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId01CellFBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID01_CELL_F_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID01 Cell F capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId01CellFCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID01_CELL_F_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID01 Cell F capa.(measured)", pos: 12, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId01CellFCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID01_CELL_F_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID01 Cell F voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId01CellFVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID01_CELL_F_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID01 Cell F voltage", pos: 12, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId01CellFVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID01_CELL_F_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID01 Cell G Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId01CellGBalancerDrive(pub bool);

pub(crate) static CMU_ID01_CELL_G_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID01 Cell G Balancer drive", pos: 2, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId01CellGBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID01_CELL_G_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID01 Cell G capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId01CellGCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID01_CELL_G_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID01 Cell G capa.(measured)", pos: 14, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId01CellGCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID01_CELL_G_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID01 Cell G voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId01CellGVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID01_CELL_G_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID01 Cell G voltage", pos: 14, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId01CellGVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID01_CELL_G_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID01 Cell H Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId01CellHBalancerDrive(pub bool);

pub(crate) static CMU_ID01_CELL_H_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID01 Cell H Balancer drive", pos: 2, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId01CellHBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID01_CELL_H_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID01 Cell H capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId01CellHCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID01_CELL_H_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID01 Cell H capa.(measured)", pos: 16, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId01CellHCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID01_CELL_H_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID01 Cell H voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId01CellHVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID01_CELL_H_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID01 Cell H voltage", pos: 16, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId01CellHVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID01_CELL_H_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID01 Module temperature1
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId01ModuleTemperature1(pub crate::db::units::Celsius);

pub(crate) static CMU_ID01_MODULE_TEMPERATURE1: [Item; 1] = [
    Item { name: "CMU ID01 Module temperature1", pos: 2, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId01ModuleTemperature1 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID01_MODULE_TEMPERATURE1[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID01 Module temperature2
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId01ModuleTemperature2(pub crate::db::units::Celsius);

pub(crate) static CMU_ID01_MODULE_TEMPERATURE2: [Item; 1] = [
    Item { name: "CMU ID01 Module temperature2", pos: 3, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId01ModuleTemperature2 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID01_MODULE_TEMPERATURE2[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID01 Module temperature3
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId01ModuleTemperature3(pub crate::db::units::Celsius);

pub(crate) static CMU_ID01_MODULE_TEMPERATURE3: [Item; 1] = [
    Item { name: "CMU ID01 Module temperature3", pos: 4, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId01ModuleTemperature3 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID01_MODULE_TEMPERATURE3[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID01 Module temperature4
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId01ModuleTemperature4(pub crate::db::units::Celsius);

pub(crate) static CMU_ID01_MODULE_TEMPERATURE4: [Item; 1] = [
    Item { name: "CMU ID01 Module temperature4", pos: 5, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId01ModuleTemperature4 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID01_MODULE_TEMPERATURE4[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID01 Module temperature5
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId01ModuleTemperature5(pub crate::db::units::Celsius);

pub(crate) static CMU_ID01_MODULE_TEMPERATURE5: [Item; 1] = [
    Item { name: "CMU ID01 Module temperature5", pos: 6, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId01ModuleTemperature5 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID01_MODULE_TEMPERATURE5[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID01 Module temperature6
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId01ModuleTemperature6(pub crate::db::units::Celsius);

pub(crate) static CMU_ID01_MODULE_TEMPERATURE6: [Item; 1] = [
    Item { name: "CMU ID01 Module temperature6", pos: 7, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId01ModuleTemperature6 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID01_MODULE_TEMPERATURE6[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID02 Cell A Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId02CellABalancerDrive(pub bool);

pub(crate) static CMU_ID02_CELL_A_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID02 Cell A Balancer drive", pos: 3, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId02CellABalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID02_CELL_A_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID02 Cell A capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId02CellACapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID02_CELL_A_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID02 Cell A capa.(measured)", pos: 18, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId02CellACapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID02_CELL_A_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID02 Cell A voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId02CellAVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID02_CELL_A_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID02 Cell A voltage", pos: 18, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId02CellAVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID02_CELL_A_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID02 Cell B Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId02CellBBalancerDrive(pub bool);

pub(crate) static CMU_ID02_CELL_B_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID02 Cell B Balancer drive", pos: 3, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId02CellBBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID02_CELL_B_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID02 Cell B capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId02CellBCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID02_CELL_B_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID02 Cell B capa.(measured)", pos: 20, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId02CellBCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID02_CELL_B_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID02 Cell B voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId02CellBVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID02_CELL_B_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID02 Cell B voltage", pos: 20, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId02CellBVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID02_CELL_B_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID02 Cell C Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId02CellCBalancerDrive(pub bool);

pub(crate) static CMU_ID02_CELL_C_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID02 Cell C Balancer drive", pos: 3, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId02CellCBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID02_CELL_C_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID02 Cell C capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId02CellCCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID02_CELL_C_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID02 Cell C capa.(measured)", pos: 22, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId02CellCCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID02_CELL_C_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID02 Cell C voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId02CellCVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID02_CELL_C_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID02 Cell C voltage", pos: 22, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId02CellCVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID02_CELL_C_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID02 Cell D Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId02CellDBalancerDrive(pub bool);

pub(crate) static CMU_ID02_CELL_D_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID02 Cell D Balancer drive", pos: 3, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId02CellDBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID02_CELL_D_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID02 Cell D capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId02CellDCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID02_CELL_D_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID02 Cell D capa.(measured)", pos: 24, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId02CellDCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID02_CELL_D_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID02 Cell D voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId02CellDVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID02_CELL_D_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID02 Cell D voltage", pos: 24, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId02CellDVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID02_CELL_D_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID02 Cell E Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId02CellEBalancerDrive(pub bool);

pub(crate) static CMU_ID02_CELL_E_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID02 Cell E Balancer drive", pos: 3, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId02CellEBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID02_CELL_E_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID02 Cell E capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId02CellECapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID02_CELL_E_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID02 Cell E capa.(measured)", pos: 26, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId02CellECapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID02_CELL_E_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID02 Cell E voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId02CellEVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID02_CELL_E_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID02 Cell E voltage", pos: 26, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId02CellEVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID02_CELL_E_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID02 Cell F Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId02CellFBalancerDrive(pub bool);

pub(crate) static CMU_ID02_CELL_F_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID02 Cell F Balancer drive", pos: 3, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId02CellFBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID02_CELL_F_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID02 Cell F capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId02CellFCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID02_CELL_F_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID02 Cell F capa.(measured)", pos: 28, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId02CellFCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID02_CELL_F_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID02 Cell F voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId02CellFVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID02_CELL_F_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID02 Cell F voltage", pos: 28, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId02CellFVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID02_CELL_F_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID02 Cell G Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId02CellGBalancerDrive(pub bool);

pub(crate) static CMU_ID02_CELL_G_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID02 Cell G Balancer drive", pos: 3, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId02CellGBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID02_CELL_G_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID02 Cell G capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId02CellGCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID02_CELL_G_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID02 Cell G capa.(measured)", pos: 30, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId02CellGCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID02_CELL_G_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID02 Cell G voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId02CellGVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID02_CELL_G_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID02 Cell G voltage", pos: 30, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId02CellGVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID02_CELL_G_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID02 Cell H Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId02CellHBalancerDrive(pub bool);

pub(crate) static CMU_ID02_CELL_H_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID02 Cell H Balancer drive", pos: 3, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId02CellHBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID02_CELL_H_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID02 Cell H capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId02CellHCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID02_CELL_H_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID02 Cell H capa.(measured)", pos: 32, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId02CellHCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID02_CELL_H_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID02 Cell H voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId02CellHVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID02_CELL_H_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID02 Cell H voltage", pos: 32, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId02CellHVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID02_CELL_H_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID02 Module temperature1
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId02ModuleTemperature1(pub crate::db::units::Celsius);

pub(crate) static CMU_ID02_MODULE_TEMPERATURE1: [Item; 1] = [
    Item { name: "CMU ID02 Module temperature1", pos: 8, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId02ModuleTemperature1 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID02_MODULE_TEMPERATURE1[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID02 Module temperature2
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId02ModuleTemperature2(pub crate::db::units::Celsius);

pub(crate) static CMU_ID02_MODULE_TEMPERATURE2: [Item; 1] = [
    Item { name: "CMU ID02 Module temperature2", pos: 9, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId02ModuleTemperature2 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID02_MODULE_TEMPERATURE2[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID02 Module temperature3
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId02ModuleTemperature3(pub crate::db::units::Celsius);

pub(crate) static CMU_ID02_MODULE_TEMPERATURE3: [Item; 1] = [
    Item { name: "CMU ID02 Module temperature3", pos: 10, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId02ModuleTemperature3 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID02_MODULE_TEMPERATURE3[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID02 Module temperature4
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId02ModuleTemperature4(pub crate::db::units::Celsius);

pub(crate) static CMU_ID02_MODULE_TEMPERATURE4: [Item; 1] = [
    Item { name: "CMU ID02 Module temperature4", pos: 11, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId02ModuleTemperature4 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID02_MODULE_TEMPERATURE4[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID02 Module temperature5
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId02ModuleTemperature5(pub crate::db::units::Celsius);

pub(crate) static CMU_ID02_MODULE_TEMPERATURE5: [Item; 1] = [
    Item { name: "CMU ID02 Module temperature5", pos: 12, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId02ModuleTemperature5 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID02_MODULE_TEMPERATURE5[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID02 Module temperature6
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId02ModuleTemperature6(pub crate::db::units::Celsius);

pub(crate) static CMU_ID02_MODULE_TEMPERATURE6: [Item; 1] = [
    Item { name: "CMU ID02 Module temperature6", pos: 13, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId02ModuleTemperature6 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID02_MODULE_TEMPERATURE6[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID03 Cell A Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId03CellABalancerDrive(pub bool);

pub(crate) static CMU_ID03_CELL_A_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID03 Cell A Balancer drive", pos: 4, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId03CellABalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID03_CELL_A_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID03 Cell A capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId03CellACapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID03_CELL_A_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID03 Cell A capa.(measured)", pos: 34, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId03CellACapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID03_CELL_A_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID03 Cell A voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId03CellAVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID03_CELL_A_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID03 Cell A voltage", pos: 34, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId03CellAVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID03_CELL_A_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID03 Cell B Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId03CellBBalancerDrive(pub bool);

pub(crate) static CMU_ID03_CELL_B_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID03 Cell B Balancer drive", pos: 4, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId03CellBBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID03_CELL_B_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID03 Cell B capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId03CellBCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID03_CELL_B_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID03 Cell B capa.(measured)", pos: 36, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId03CellBCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID03_CELL_B_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID03 Cell B voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId03CellBVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID03_CELL_B_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID03 Cell B voltage", pos: 36, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId03CellBVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID03_CELL_B_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID03 Cell C Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId03CellCBalancerDrive(pub bool);

pub(crate) static CMU_ID03_CELL_C_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID03 Cell C Balancer drive", pos: 4, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId03CellCBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID03_CELL_C_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID03 Cell C capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId03CellCCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID03_CELL_C_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID03 Cell C capa.(measured)", pos: 38, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId03CellCCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID03_CELL_C_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID03 Cell C voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId03CellCVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID03_CELL_C_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID03 Cell C voltage", pos: 38, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId03CellCVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID03_CELL_C_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID03 Cell D Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId03CellDBalancerDrive(pub bool);

pub(crate) static CMU_ID03_CELL_D_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID03 Cell D Balancer drive", pos: 4, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId03CellDBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID03_CELL_D_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID03 Cell D capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId03CellDCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID03_CELL_D_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID03 Cell D capa.(measured)", pos: 40, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId03CellDCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID03_CELL_D_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID03 Cell D voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId03CellDVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID03_CELL_D_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID03 Cell D voltage", pos: 40, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId03CellDVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID03_CELL_D_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID03 Cell E Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId03CellEBalancerDrive(pub bool);

pub(crate) static CMU_ID03_CELL_E_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID03 Cell E Balancer drive", pos: 4, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId03CellEBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID03_CELL_E_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID03 Cell E capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId03CellECapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID03_CELL_E_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID03 Cell E capa.(measured)", pos: 42, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId03CellECapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID03_CELL_E_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID03 Cell E voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId03CellEVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID03_CELL_E_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID03 Cell E voltage", pos: 42, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId03CellEVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID03_CELL_E_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID03 Cell F Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId03CellFBalancerDrive(pub bool);

pub(crate) static CMU_ID03_CELL_F_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID03 Cell F Balancer drive", pos: 4, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId03CellFBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID03_CELL_F_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID03 Cell F capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId03CellFCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID03_CELL_F_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID03 Cell F capa.(measured)", pos: 44, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId03CellFCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID03_CELL_F_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID03 Cell F voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId03CellFVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID03_CELL_F_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID03 Cell F voltage", pos: 44, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId03CellFVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID03_CELL_F_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID03 Cell G Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId03CellGBalancerDrive(pub bool);

pub(crate) static CMU_ID03_CELL_G_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID03 Cell G Balancer drive", pos: 4, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId03CellGBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID03_CELL_G_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID03 Cell G capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId03CellGCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID03_CELL_G_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID03 Cell G capa.(measured)", pos: 46, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId03CellGCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID03_CELL_G_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID03 Cell G voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId03CellGVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID03_CELL_G_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID03 Cell G voltage", pos: 46, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId03CellGVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID03_CELL_G_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID03 Cell H Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId03CellHBalancerDrive(pub bool);

pub(crate) static CMU_ID03_CELL_H_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID03 Cell H Balancer drive", pos: 4, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId03CellHBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID03_CELL_H_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID03 Cell H capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId03CellHCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID03_CELL_H_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID03 Cell H capa.(measured)", pos: 48, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId03CellHCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID03_CELL_H_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID03 Cell H voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId03CellHVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID03_CELL_H_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID03 Cell H voltage", pos: 48, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId03CellHVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID03_CELL_H_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID03 Module temperature1
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId03ModuleTemperature1(pub crate::db::units::Celsius);

pub(crate) static CMU_ID03_MODULE_TEMPERATURE1: [Item; 1] = [
    Item { name: "CMU ID03 Module temperature1", pos: 14, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId03ModuleTemperature1 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID03_MODULE_TEMPERATURE1[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID03 Module temperature2
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId03ModuleTemperature2(pub crate::db::units::Celsius);

pub(crate) static CMU_ID03_MODULE_TEMPERATURE2: [Item; 1] = [
    Item { name: "CMU ID03 Module temperature2", pos: 15, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId03ModuleTemperature2 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID03_MODULE_TEMPERATURE2[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID03 Module temperature3
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId03ModuleTemperature3(pub crate::db::units::Celsius);

pub(crate) static CMU_ID03_MODULE_TEMPERATURE3: [Item; 1] = [
    Item { name: "CMU ID03 Module temperature3", pos: 16, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId03ModuleTemperature3 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID03_MODULE_TEMPERATURE3[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID03 Module temperature4
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId03ModuleTemperature4(pub crate::db::units::Celsius);

pub(crate) static CMU_ID03_MODULE_TEMPERATURE4: [Item; 1] = [
    Item { name: "CMU ID03 Module temperature4", pos: 17, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId03ModuleTemperature4 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID03_MODULE_TEMPERATURE4[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID03 Module temperature5
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId03ModuleTemperature5(pub crate::db::units::Celsius);

pub(crate) static CMU_ID03_MODULE_TEMPERATURE5: [Item; 1] = [
    Item { name: "CMU ID03 Module temperature5", pos: 18, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId03ModuleTemperature5 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID03_MODULE_TEMPERATURE5[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID03 Module temperature6
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId03ModuleTemperature6(pub crate::db::units::Celsius);

pub(crate) static CMU_ID03_MODULE_TEMPERATURE6: [Item; 1] = [
    Item { name: "CMU ID03 Module temperature6", pos: 19, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId03ModuleTemperature6 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID03_MODULE_TEMPERATURE6[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID04 Cell A Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId04CellABalancerDrive(pub bool);

pub(crate) static CMU_ID04_CELL_A_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID04 Cell A Balancer drive", pos: 5, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId04CellABalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID04_CELL_A_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID04 Cell A capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId04CellACapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID04_CELL_A_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID04 Cell A capa.(measured)", pos: 50, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId04CellACapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID04_CELL_A_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID04 Cell A voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId04CellAVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID04_CELL_A_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID04 Cell A voltage", pos: 50, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId04CellAVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID04_CELL_A_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID04 Cell B Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId04CellBBalancerDrive(pub bool);

pub(crate) static CMU_ID04_CELL_B_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID04 Cell B Balancer drive", pos: 5, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId04CellBBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID04_CELL_B_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID04 Cell B capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId04CellBCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID04_CELL_B_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID04 Cell B capa.(measured)", pos: 52, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId04CellBCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID04_CELL_B_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID04 Cell B voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId04CellBVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID04_CELL_B_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID04 Cell B voltage", pos: 52, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId04CellBVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID04_CELL_B_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID04 Cell C Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId04CellCBalancerDrive(pub bool);

pub(crate) static CMU_ID04_CELL_C_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID04 Cell C Balancer drive", pos: 5, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId04CellCBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID04_CELL_C_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID04 Cell C capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId04CellCCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID04_CELL_C_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID04 Cell C capa.(measured)", pos: 54, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId04CellCCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID04_CELL_C_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID04 Cell C voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId04CellCVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID04_CELL_C_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID04 Cell C voltage", pos: 54, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId04CellCVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID04_CELL_C_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID04 Cell D Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId04CellDBalancerDrive(pub bool);

pub(crate) static CMU_ID04_CELL_D_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID04 Cell D Balancer drive", pos: 5, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId04CellDBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID04_CELL_D_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID04 Cell D capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId04CellDCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID04_CELL_D_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID04 Cell D capa.(measured)", pos: 56, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId04CellDCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID04_CELL_D_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID04 Cell D voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId04CellDVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID04_CELL_D_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID04 Cell D voltage", pos: 56, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId04CellDVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID04_CELL_D_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID04 Cell E Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId04CellEBalancerDrive(pub bool);

pub(crate) static CMU_ID04_CELL_E_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID04 Cell E Balancer drive", pos: 5, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId04CellEBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID04_CELL_E_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID04 Cell E capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId04CellECapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID04_CELL_E_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID04 Cell E capa.(measured)", pos: 58, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId04CellECapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID04_CELL_E_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID04 Cell E voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId04CellEVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID04_CELL_E_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID04 Cell E voltage", pos: 58, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId04CellEVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID04_CELL_E_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID04 Cell F Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId04CellFBalancerDrive(pub bool);

pub(crate) static CMU_ID04_CELL_F_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID04 Cell F Balancer drive", pos: 5, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId04CellFBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID04_CELL_F_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID04 Cell F capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId04CellFCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID04_CELL_F_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID04 Cell F capa.(measured)", pos: 60, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId04CellFCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID04_CELL_F_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID04 Cell F voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId04CellFVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID04_CELL_F_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID04 Cell F voltage", pos: 60, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId04CellFVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID04_CELL_F_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID04 Cell G Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId04CellGBalancerDrive(pub bool);

pub(crate) static CMU_ID04_CELL_G_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID04 Cell G Balancer drive", pos: 5, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId04CellGBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID04_CELL_G_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID04 Cell G capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId04CellGCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID04_CELL_G_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID04 Cell G capa.(measured)", pos: 62, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId04CellGCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID04_CELL_G_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID04 Cell G voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId04CellGVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID04_CELL_G_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID04 Cell G voltage", pos: 62, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId04CellGVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID04_CELL_G_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID04 Cell H Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId04CellHBalancerDrive(pub bool);

pub(crate) static CMU_ID04_CELL_H_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID04 Cell H Balancer drive", pos: 5, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId04CellHBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID04_CELL_H_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID04 Cell H capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId04CellHCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID04_CELL_H_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID04 Cell H capa.(measured)", pos: 64, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId04CellHCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID04_CELL_H_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID04 Cell H voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId04CellHVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID04_CELL_H_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID04 Cell H voltage", pos: 64, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId04CellHVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID04_CELL_H_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID04 Module temperature1
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId04ModuleTemperature1(pub crate::db::units::Celsius);

pub(crate) static CMU_ID04_MODULE_TEMPERATURE1: [Item; 1] = [
    Item { name: "CMU ID04 Module temperature1", pos: 20, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId04ModuleTemperature1 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID04_MODULE_TEMPERATURE1[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID04 Module temperature2
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId04ModuleTemperature2(pub crate::db::units::Celsius);

pub(crate) static CMU_ID04_MODULE_TEMPERATURE2: [Item; 1] = [
    Item { name: "CMU ID04 Module temperature2", pos: 21, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId04ModuleTemperature2 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID04_MODULE_TEMPERATURE2[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID04 Module temperature3
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId04ModuleTemperature3(pub crate::db::units::Celsius);

pub(crate) static CMU_ID04_MODULE_TEMPERATURE3: [Item; 1] = [
    Item { name: "CMU ID04 Module temperature3", pos: 22, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId04ModuleTemperature3 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID04_MODULE_TEMPERATURE3[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID04 Module temperature4
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId04ModuleTemperature4(pub crate::db::units::Celsius);

pub(crate) static CMU_ID04_MODULE_TEMPERATURE4: [Item; 1] = [
    Item { name: "CMU ID04 Module temperature4", pos: 23, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId04ModuleTemperature4 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID04_MODULE_TEMPERATURE4[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID04 Module temperature5
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId04ModuleTemperature5(pub crate::db::units::Celsius);

pub(crate) static CMU_ID04_MODULE_TEMPERATURE5: [Item; 1] = [
    Item { name: "CMU ID04 Module temperature5", pos: 24, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId04ModuleTemperature5 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID04_MODULE_TEMPERATURE5[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID04 Module temperature6
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId04ModuleTemperature6(pub crate::db::units::Celsius);

pub(crate) static CMU_ID04_MODULE_TEMPERATURE6: [Item; 1] = [
    Item { name: "CMU ID04 Module temperature6", pos: 25, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId04ModuleTemperature6 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID04_MODULE_TEMPERATURE6[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID05 Cell A Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId05CellABalancerDrive(pub bool);

pub(crate) static CMU_ID05_CELL_A_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID05 Cell A Balancer drive", pos: 6, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId05CellABalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID05_CELL_A_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID05 Cell A capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId05CellACapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID05_CELL_A_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID05 Cell A capa.(measured)", pos: 66, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId05CellACapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID05_CELL_A_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID05 Cell A voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId05CellAVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID05_CELL_A_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID05 Cell A voltage", pos: 66, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId05CellAVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID05_CELL_A_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID05 Cell B Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId05CellBBalancerDrive(pub bool);

pub(crate) static CMU_ID05_CELL_B_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID05 Cell B Balancer drive", pos: 6, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId05CellBBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID05_CELL_B_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID05 Cell B capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId05CellBCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID05_CELL_B_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID05 Cell B capa.(measured)", pos: 68, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId05CellBCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID05_CELL_B_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID05 Cell B voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId05CellBVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID05_CELL_B_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID05 Cell B voltage", pos: 68, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId05CellBVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID05_CELL_B_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID05 Cell C Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId05CellCBalancerDrive(pub bool);

pub(crate) static CMU_ID05_CELL_C_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID05 Cell C Balancer drive", pos: 6, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId05CellCBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID05_CELL_C_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID05 Cell C capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId05CellCCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID05_CELL_C_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID05 Cell C capa.(measured)", pos: 70, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId05CellCCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID05_CELL_C_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID05 Cell C voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId05CellCVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID05_CELL_C_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID05 Cell C voltage", pos: 70, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId05CellCVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID05_CELL_C_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID05 Cell D Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId05CellDBalancerDrive(pub bool);

pub(crate) static CMU_ID05_CELL_D_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID05 Cell D Balancer drive", pos: 6, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId05CellDBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID05_CELL_D_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID05 Cell D capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId05CellDCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID05_CELL_D_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID05 Cell D capa.(measured)", pos: 72, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId05CellDCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID05_CELL_D_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID05 Cell D voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId05CellDVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID05_CELL_D_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID05 Cell D voltage", pos: 72, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId05CellDVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID05_CELL_D_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID05 Cell E Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId05CellEBalancerDrive(pub bool);

pub(crate) static CMU_ID05_CELL_E_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID05 Cell E Balancer drive", pos: 6, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId05CellEBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID05_CELL_E_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID05 Cell E capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId05CellECapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID05_CELL_E_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID05 Cell E capa.(measured)", pos: 74, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId05CellECapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID05_CELL_E_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID05 Cell E voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId05CellEVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID05_CELL_E_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID05 Cell E voltage", pos: 74, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId05CellEVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID05_CELL_E_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID05 Cell F Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId05CellFBalancerDrive(pub bool);

pub(crate) static CMU_ID05_CELL_F_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID05 Cell F Balancer drive", pos: 6, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId05CellFBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID05_CELL_F_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID05 Cell F capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId05CellFCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID05_CELL_F_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID05 Cell F capa.(measured)", pos: 76, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId05CellFCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID05_CELL_F_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID05 Cell F voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId05CellFVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID05_CELL_F_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID05 Cell F voltage", pos: 76, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId05CellFVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID05_CELL_F_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID05 Cell G Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId05CellGBalancerDrive(pub bool);

pub(crate) static CMU_ID05_CELL_G_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID05 Cell G Balancer drive", pos: 6, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId05CellGBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID05_CELL_G_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID05 Cell G capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId05CellGCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID05_CELL_G_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID05 Cell G capa.(measured)", pos: 78, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId05CellGCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID05_CELL_G_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID05 Cell G voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId05CellGVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID05_CELL_G_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID05 Cell G voltage", pos: 78, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId05CellGVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID05_CELL_G_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID05 Cell H Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId05CellHBalancerDrive(pub bool);

pub(crate) static CMU_ID05_CELL_H_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID05 Cell H Balancer drive", pos: 6, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId05CellHBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID05_CELL_H_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID05 Cell H capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId05CellHCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID05_CELL_H_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID05 Cell H capa.(measured)", pos: 80, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId05CellHCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID05_CELL_H_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID05 Cell H voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId05CellHVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID05_CELL_H_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID05 Cell H voltage", pos: 80, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId05CellHVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID05_CELL_H_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID05 Module temperature1
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId05ModuleTemperature1(pub crate::db::units::Celsius);

pub(crate) static CMU_ID05_MODULE_TEMPERATURE1: [Item; 1] = [
    Item { name: "CMU ID05 Module temperature1", pos: 26, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId05ModuleTemperature1 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID05_MODULE_TEMPERATURE1[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID05 Module temperature2
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId05ModuleTemperature2(pub crate::db::units::Celsius);

pub(crate) static CMU_ID05_MODULE_TEMPERATURE2: [Item; 1] = [
    Item { name: "CMU ID05 Module temperature2", pos: 27, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId05ModuleTemperature2 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID05_MODULE_TEMPERATURE2[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID05 Module temperature3
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId05ModuleTemperature3(pub crate::db::units::Celsius);

pub(crate) static CMU_ID05_MODULE_TEMPERATURE3: [Item; 1] = [
    Item { name: "CMU ID05 Module temperature3", pos: 28, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId05ModuleTemperature3 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID05_MODULE_TEMPERATURE3[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID05 Module temperature4
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId05ModuleTemperature4(pub crate::db::units::Celsius);

pub(crate) static CMU_ID05_MODULE_TEMPERATURE4: [Item; 1] = [
    Item { name: "CMU ID05 Module temperature4", pos: 29, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId05ModuleTemperature4 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID05_MODULE_TEMPERATURE4[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID05 Module temperature5
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId05ModuleTemperature5(pub crate::db::units::Celsius);

pub(crate) static CMU_ID05_MODULE_TEMPERATURE5: [Item; 1] = [
    Item { name: "CMU ID05 Module temperature5", pos: 30, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId05ModuleTemperature5 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID05_MODULE_TEMPERATURE5[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID05 Module temperature6
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId05ModuleTemperature6(pub crate::db::units::Celsius);

pub(crate) static CMU_ID05_MODULE_TEMPERATURE6: [Item; 1] = [
    Item { name: "CMU ID05 Module temperature6", pos: 31, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId05ModuleTemperature6 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID05_MODULE_TEMPERATURE6[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID06 Cell A Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId06CellABalancerDrive(pub bool);

pub(crate) static CMU_ID06_CELL_A_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID06 Cell A Balancer drive", pos: 7, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId06CellABalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID06_CELL_A_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID06 Cell A capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId06CellACapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID06_CELL_A_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID06 Cell A capa.(measured)", pos: 82, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId06CellACapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID06_CELL_A_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID06 Cell A voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId06CellAVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID06_CELL_A_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID06 Cell A voltage", pos: 82, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId06CellAVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID06_CELL_A_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID06 Cell B Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId06CellBBalancerDrive(pub bool);

pub(crate) static CMU_ID06_CELL_B_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID06 Cell B Balancer drive", pos: 7, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId06CellBBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID06_CELL_B_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID06 Cell B capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId06CellBCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID06_CELL_B_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID06 Cell B capa.(measured)", pos: 84, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId06CellBCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID06_CELL_B_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID06 Cell B voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId06CellBVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID06_CELL_B_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID06 Cell B voltage", pos: 84, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId06CellBVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID06_CELL_B_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID06 Cell C Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId06CellCBalancerDrive(pub bool);

pub(crate) static CMU_ID06_CELL_C_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID06 Cell C Balancer drive", pos: 7, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId06CellCBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID06_CELL_C_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID06 Cell C capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId06CellCCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID06_CELL_C_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID06 Cell C capa.(measured)", pos: 86, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId06CellCCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID06_CELL_C_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID06 Cell C voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId06CellCVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID06_CELL_C_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID06 Cell C voltage", pos: 86, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId06CellCVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID06_CELL_C_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID06 Cell D Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId06CellDBalancerDrive(pub bool);

pub(crate) static CMU_ID06_CELL_D_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID06 Cell D Balancer drive", pos: 7, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId06CellDBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID06_CELL_D_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID06 Cell D capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId06CellDCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID06_CELL_D_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID06 Cell D capa.(measured)", pos: 88, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId06CellDCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID06_CELL_D_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID06 Cell D voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId06CellDVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID06_CELL_D_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID06 Cell D voltage", pos: 88, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId06CellDVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID06_CELL_D_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID06 Module temperature1
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId06ModuleTemperature1(pub crate::db::units::Celsius);

pub(crate) static CMU_ID06_MODULE_TEMPERATURE1: [Item; 1] = [
    Item { name: "CMU ID06 Module temperature1", pos: 32, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId06ModuleTemperature1 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID06_MODULE_TEMPERATURE1[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID06 Module temperature2
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId06ModuleTemperature2(pub crate::db::units::Celsius);

pub(crate) static CMU_ID06_MODULE_TEMPERATURE2: [Item; 1] = [
    Item { name: "CMU ID06 Module temperature2", pos: 33, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId06ModuleTemperature2 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID06_MODULE_TEMPERATURE2[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID06 Module temperature3
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId06ModuleTemperature3(pub crate::db::units::Celsius);

pub(crate) static CMU_ID06_MODULE_TEMPERATURE3: [Item; 1] = [
    Item { name: "CMU ID06 Module temperature3", pos: 34, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId06ModuleTemperature3 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID06_MODULE_TEMPERATURE3[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID07 Cell A Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId07CellABalancerDrive(pub bool);

pub(crate) static CMU_ID07_CELL_A_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID07 Cell A Balancer drive", pos: 8, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId07CellABalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID07_CELL_A_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID07 Cell A capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId07CellACapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID07_CELL_A_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID07 Cell A capa.(measured)", pos: 90, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId07CellACapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID07_CELL_A_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID07 Cell A voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId07CellAVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID07_CELL_A_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID07 Cell A voltage", pos: 90, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId07CellAVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID07_CELL_A_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID07 Cell B Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId07CellBBalancerDrive(pub bool);

pub(crate) static CMU_ID07_CELL_B_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID07 Cell B Balancer drive", pos: 8, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId07CellBBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID07_CELL_B_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID07 Cell B capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId07CellBCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID07_CELL_B_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID07 Cell B capa.(measured)", pos: 92, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId07CellBCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID07_CELL_B_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID07 Cell B voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId07CellBVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID07_CELL_B_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID07 Cell B voltage", pos: 92, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId07CellBVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID07_CELL_B_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID07 Cell C Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId07CellCBalancerDrive(pub bool);

pub(crate) static CMU_ID07_CELL_C_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID07 Cell C Balancer drive", pos: 8, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId07CellCBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID07_CELL_C_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID07 Cell C capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId07CellCCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID07_CELL_C_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID07 Cell C capa.(measured)", pos: 94, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId07CellCCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID07_CELL_C_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID07 Cell C voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId07CellCVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID07_CELL_C_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID07 Cell C voltage", pos: 94, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId07CellCVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID07_CELL_C_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID07 Cell D Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId07CellDBalancerDrive(pub bool);

pub(crate) static CMU_ID07_CELL_D_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID07 Cell D Balancer drive", pos: 8, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId07CellDBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID07_CELL_D_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID07 Cell D capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId07CellDCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID07_CELL_D_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID07 Cell D capa.(measured)", pos: 96, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId07CellDCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID07_CELL_D_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID07 Cell D voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId07CellDVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID07_CELL_D_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID07 Cell D voltage", pos: 96, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId07CellDVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID07_CELL_D_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID07 Cell E Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId07CellEBalancerDrive(pub bool);

pub(crate) static CMU_ID07_CELL_E_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID07 Cell E Balancer drive", pos: 8, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId07CellEBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID07_CELL_E_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID07 Cell E capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId07CellECapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID07_CELL_E_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID07 Cell E capa.(measured)", pos: 98, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId07CellECapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID07_CELL_E_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID07 Cell E voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId07CellEVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID07_CELL_E_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID07 Cell E voltage", pos: 98, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId07CellEVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID07_CELL_E_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID07 Cell F Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId07CellFBalancerDrive(pub bool);

pub(crate) static CMU_ID07_CELL_F_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID07 Cell F Balancer drive", pos: 8, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId07CellFBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID07_CELL_F_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID07 Cell F capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId07CellFCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID07_CELL_F_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID07 Cell F capa.(measured)", pos: 100, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId07CellFCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID07_CELL_F_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID07 Cell F voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId07CellFVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID07_CELL_F_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID07 Cell F voltage", pos: 100, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId07CellFVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID07_CELL_F_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID07 Cell G Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId07CellGBalancerDrive(pub bool);

pub(crate) static CMU_ID07_CELL_G_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID07 Cell G Balancer drive", pos: 8, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId07CellGBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID07_CELL_G_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID07 Cell G capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId07CellGCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID07_CELL_G_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID07 Cell G capa.(measured)", pos: 102, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId07CellGCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID07_CELL_G_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID07 Cell G voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId07CellGVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID07_CELL_G_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID07 Cell G voltage", pos: 102, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId07CellGVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID07_CELL_G_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID07 Cell H Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId07CellHBalancerDrive(pub bool);

pub(crate) static CMU_ID07_CELL_H_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID07 Cell H Balancer drive", pos: 8, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId07CellHBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID07_CELL_H_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID07 Cell H capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId07CellHCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID07_CELL_H_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID07 Cell H capa.(measured)", pos: 104, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId07CellHCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID07_CELL_H_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID07 Cell H voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId07CellHVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID07_CELL_H_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID07 Cell H voltage", pos: 104, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId07CellHVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID07_CELL_H_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID07 Module temperature1
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId07ModuleTemperature1(pub crate::db::units::Celsius);

pub(crate) static CMU_ID07_MODULE_TEMPERATURE1: [Item; 1] = [
    Item { name: "CMU ID07 Module temperature1", pos: 35, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId07ModuleTemperature1 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID07_MODULE_TEMPERATURE1[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID07 Module temperature2
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId07ModuleTemperature2(pub crate::db::units::Celsius);

pub(crate) static CMU_ID07_MODULE_TEMPERATURE2: [Item; 1] = [
    Item { name: "CMU ID07 Module temperature2", pos: 36, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId07ModuleTemperature2 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID07_MODULE_TEMPERATURE2[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID07 Module temperature3
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId07ModuleTemperature3(pub crate::db::units::Celsius);

pub(crate) static CMU_ID07_MODULE_TEMPERATURE3: [Item; 1] = [
    Item { name: "CMU ID07 Module temperature3", pos: 37, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId07ModuleTemperature3 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID07_MODULE_TEMPERATURE3[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID07 Module temperature4
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId07ModuleTemperature4(pub crate::db::units::Celsius);

pub(crate) static CMU_ID07_MODULE_TEMPERATURE4: [Item; 1] = [
    Item { name: "CMU ID07 Module temperature4", pos: 38, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId07ModuleTemperature4 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID07_MODULE_TEMPERATURE4[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID07 Module temperature5
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId07ModuleTemperature5(pub crate::db::units::Celsius);

pub(crate) static CMU_ID07_MODULE_TEMPERATURE5: [Item; 1] = [
    Item { name: "CMU ID07 Module temperature5", pos: 39, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId07ModuleTemperature5 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID07_MODULE_TEMPERATURE5[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID07 Module temperature6
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId07ModuleTemperature6(pub crate::db::units::Celsius);

pub(crate) static CMU_ID07_MODULE_TEMPERATURE6: [Item; 1] = [
    Item { name: "CMU ID07 Module temperature6", pos: 40, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId07ModuleTemperature6 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID07_MODULE_TEMPERATURE6[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID08 Cell A Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId08CellABalancerDrive(pub bool);

pub(crate) static CMU_ID08_CELL_A_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID08 Cell A Balancer drive", pos: 9, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId08CellABalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID08_CELL_A_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID08 Cell A capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId08CellACapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID08_CELL_A_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID08 Cell A capa.(measured)", pos: 106, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId08CellACapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID08_CELL_A_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID08 Cell A voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId08CellAVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID08_CELL_A_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID08 Cell A voltage", pos: 106, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId08CellAVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID08_CELL_A_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID08 Cell B Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId08CellBBalancerDrive(pub bool);

pub(crate) static CMU_ID08_CELL_B_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID08 Cell B Balancer drive", pos: 9, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId08CellBBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID08_CELL_B_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID08 Cell B capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId08CellBCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID08_CELL_B_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID08 Cell B capa.(measured)", pos: 108, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId08CellBCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID08_CELL_B_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID08 Cell B voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId08CellBVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID08_CELL_B_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID08 Cell B voltage", pos: 108, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId08CellBVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID08_CELL_B_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID08 Cell C Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId08CellCBalancerDrive(pub bool);

pub(crate) static CMU_ID08_CELL_C_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID08 Cell C Balancer drive", pos: 9, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId08CellCBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID08_CELL_C_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID08 Cell C capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId08CellCCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID08_CELL_C_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID08 Cell C capa.(measured)", pos: 110, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId08CellCCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID08_CELL_C_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID08 Cell C voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId08CellCVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID08_CELL_C_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID08 Cell C voltage", pos: 110, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId08CellCVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID08_CELL_C_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID08 Cell D Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId08CellDBalancerDrive(pub bool);

pub(crate) static CMU_ID08_CELL_D_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID08 Cell D Balancer drive", pos: 9, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId08CellDBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID08_CELL_D_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID08 Cell D capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId08CellDCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID08_CELL_D_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID08 Cell D capa.(measured)", pos: 112, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId08CellDCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID08_CELL_D_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID08 Cell D voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId08CellDVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID08_CELL_D_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID08 Cell D voltage", pos: 112, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId08CellDVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID08_CELL_D_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID08 Cell E Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId08CellEBalancerDrive(pub bool);

pub(crate) static CMU_ID08_CELL_E_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID08 Cell E Balancer drive", pos: 9, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId08CellEBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID08_CELL_E_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID08 Cell E capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId08CellECapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID08_CELL_E_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID08 Cell E capa.(measured)", pos: 114, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId08CellECapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID08_CELL_E_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID08 Cell E voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId08CellEVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID08_CELL_E_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID08 Cell E voltage", pos: 114, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId08CellEVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID08_CELL_E_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID08 Cell F Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId08CellFBalancerDrive(pub bool);

pub(crate) static CMU_ID08_CELL_F_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID08 Cell F Balancer drive", pos: 9, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId08CellFBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID08_CELL_F_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID08 Cell F capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId08CellFCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID08_CELL_F_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID08 Cell F capa.(measured)", pos: 116, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId08CellFCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID08_CELL_F_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID08 Cell F voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId08CellFVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID08_CELL_F_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID08 Cell F voltage", pos: 116, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId08CellFVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID08_CELL_F_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID08 Cell G Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId08CellGBalancerDrive(pub bool);

pub(crate) static CMU_ID08_CELL_G_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID08 Cell G Balancer drive", pos: 9, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId08CellGBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID08_CELL_G_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID08 Cell G capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId08CellGCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID08_CELL_G_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID08 Cell G capa.(measured)", pos: 118, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId08CellGCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID08_CELL_G_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID08 Cell G voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId08CellGVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID08_CELL_G_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID08 Cell G voltage", pos: 118, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId08CellGVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID08_CELL_G_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID08 Cell H Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId08CellHBalancerDrive(pub bool);

pub(crate) static CMU_ID08_CELL_H_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID08 Cell H Balancer drive", pos: 9, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId08CellHBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID08_CELL_H_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID08 Cell H capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId08CellHCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID08_CELL_H_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID08 Cell H capa.(measured)", pos: 120, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId08CellHCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID08_CELL_H_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID08 Cell H voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId08CellHVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID08_CELL_H_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID08 Cell H voltage", pos: 120, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId08CellHVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID08_CELL_H_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID08 Module temperature1
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId08ModuleTemperature1(pub crate::db::units::Celsius);

pub(crate) static CMU_ID08_MODULE_TEMPERATURE1: [Item; 1] = [
    Item { name: "CMU ID08 Module temperature1", pos: 41, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId08ModuleTemperature1 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID08_MODULE_TEMPERATURE1[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID08 Module temperature2
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId08ModuleTemperature2(pub crate::db::units::Celsius);

pub(crate) static CMU_ID08_MODULE_TEMPERATURE2: [Item; 1] = [
    Item { name: "CMU ID08 Module temperature2", pos: 42, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId08ModuleTemperature2 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID08_MODULE_TEMPERATURE2[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID08 Module temperature3
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId08ModuleTemperature3(pub crate::db::units::Celsius);

pub(crate) static CMU_ID08_MODULE_TEMPERATURE3: [Item; 1] = [
    Item { name: "CMU ID08 Module temperature3", pos: 43, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId08ModuleTemperature3 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID08_MODULE_TEMPERATURE3[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID08 Module temperature4
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId08ModuleTemperature4(pub crate::db::units::Celsius);

pub(crate) static CMU_ID08_MODULE_TEMPERATURE4: [Item; 1] = [
    Item { name: "CMU ID08 Module temperature4", pos: 44, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId08ModuleTemperature4 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID08_MODULE_TEMPERATURE4[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID08 Module temperature5
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId08ModuleTemperature5(pub crate::db::units::Celsius);

pub(crate) static CMU_ID08_MODULE_TEMPERATURE5: [Item; 1] = [
    Item { name: "CMU ID08 Module temperature5", pos: 45, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId08ModuleTemperature5 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID08_MODULE_TEMPERATURE5[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID08 Module temperature6
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId08ModuleTemperature6(pub crate::db::units::Celsius);

pub(crate) static CMU_ID08_MODULE_TEMPERATURE6: [Item; 1] = [
    Item { name: "CMU ID08 Module temperature6", pos: 46, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId08ModuleTemperature6 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID08_MODULE_TEMPERATURE6[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID09 Cell A Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId09CellABalancerDrive(pub bool);

pub(crate) static CMU_ID09_CELL_A_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID09 Cell A Balancer drive", pos: 10, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId09CellABalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID09_CELL_A_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID09 Cell A capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId09CellACapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID09_CELL_A_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID09 Cell A capa.(measured)", pos: 122, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId09CellACapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID09_CELL_A_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID09 Cell A voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId09CellAVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID09_CELL_A_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID09 Cell A voltage", pos: 122, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId09CellAVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID09_CELL_A_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID09 Cell B Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId09CellBBalancerDrive(pub bool);

pub(crate) static CMU_ID09_CELL_B_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID09 Cell B Balancer drive", pos: 10, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId09CellBBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID09_CELL_B_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID09 Cell B capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId09CellBCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID09_CELL_B_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID09 Cell B capa.(measured)", pos: 124, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId09CellBCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID09_CELL_B_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID09 Cell B voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId09CellBVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID09_CELL_B_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID09 Cell B voltage", pos: 124, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId09CellBVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID09_CELL_B_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID09 Cell C Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId09CellCBalancerDrive(pub bool);

pub(crate) static CMU_ID09_CELL_C_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID09 Cell C Balancer drive", pos: 10, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId09CellCBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID09_CELL_C_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID09 Cell C capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId09CellCCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID09_CELL_C_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID09 Cell C capa.(measured)", pos: 126, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId09CellCCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID09_CELL_C_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID09 Cell C voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId09CellCVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID09_CELL_C_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID09 Cell C voltage", pos: 126, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId09CellCVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID09_CELL_C_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID09 Cell D Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId09CellDBalancerDrive(pub bool);

pub(crate) static CMU_ID09_CELL_D_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID09 Cell D Balancer drive", pos: 10, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId09CellDBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID09_CELL_D_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID09 Cell D capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId09CellDCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID09_CELL_D_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID09 Cell D capa.(measured)", pos: 128, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId09CellDCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID09_CELL_D_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID09 Cell D voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId09CellDVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID09_CELL_D_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID09 Cell D voltage", pos: 128, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId09CellDVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID09_CELL_D_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID09 Cell E Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId09CellEBalancerDrive(pub bool);

pub(crate) static CMU_ID09_CELL_E_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID09 Cell E Balancer drive", pos: 10, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId09CellEBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID09_CELL_E_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID09 Cell E capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId09CellECapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID09_CELL_E_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID09 Cell E capa.(measured)", pos: 130, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId09CellECapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID09_CELL_E_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID09 Cell E voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId09CellEVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID09_CELL_E_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID09 Cell E voltage", pos: 130, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId09CellEVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID09_CELL_E_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID09 Cell F Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId09CellFBalancerDrive(pub bool);

pub(crate) static CMU_ID09_CELL_F_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID09 Cell F Balancer drive", pos: 10, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId09CellFBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID09_CELL_F_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID09 Cell F capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId09CellFCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID09_CELL_F_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID09 Cell F capa.(measured)", pos: 132, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId09CellFCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID09_CELL_F_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID09 Cell F voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId09CellFVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID09_CELL_F_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID09 Cell F voltage", pos: 132, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId09CellFVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID09_CELL_F_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID09 Cell G Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId09CellGBalancerDrive(pub bool);

pub(crate) static CMU_ID09_CELL_G_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID09 Cell G Balancer drive", pos: 10, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId09CellGBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID09_CELL_G_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID09 Cell G capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId09CellGCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID09_CELL_G_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID09 Cell G capa.(measured)", pos: 134, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId09CellGCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID09_CELL_G_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID09 Cell G voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId09CellGVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID09_CELL_G_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID09 Cell G voltage", pos: 134, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId09CellGVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID09_CELL_G_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID09 Cell H Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId09CellHBalancerDrive(pub bool);

pub(crate) static CMU_ID09_CELL_H_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID09 Cell H Balancer drive", pos: 10, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId09CellHBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID09_CELL_H_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID09 Cell H capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId09CellHCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID09_CELL_H_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID09 Cell H capa.(measured)", pos: 136, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId09CellHCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID09_CELL_H_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID09 Cell H voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId09CellHVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID09_CELL_H_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID09 Cell H voltage", pos: 136, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId09CellHVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID09_CELL_H_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID09 Module temperature1
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId09ModuleTemperature1(pub crate::db::units::Celsius);

pub(crate) static CMU_ID09_MODULE_TEMPERATURE1: [Item; 1] = [
    Item { name: "CMU ID09 Module temperature1", pos: 47, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId09ModuleTemperature1 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID09_MODULE_TEMPERATURE1[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID09 Module temperature2
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId09ModuleTemperature2(pub crate::db::units::Celsius);

pub(crate) static CMU_ID09_MODULE_TEMPERATURE2: [Item; 1] = [
    Item { name: "CMU ID09 Module temperature2", pos: 48, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId09ModuleTemperature2 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID09_MODULE_TEMPERATURE2[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID09 Module temperature3
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId09ModuleTemperature3(pub crate::db::units::Celsius);

pub(crate) static CMU_ID09_MODULE_TEMPERATURE3: [Item; 1] = [
    Item { name: "CMU ID09 Module temperature3", pos: 49, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId09ModuleTemperature3 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID09_MODULE_TEMPERATURE3[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID09 Module temperature4
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId09ModuleTemperature4(pub crate::db::units::Celsius);

pub(crate) static CMU_ID09_MODULE_TEMPERATURE4: [Item; 1] = [
    Item { name: "CMU ID09 Module temperature4", pos: 50, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId09ModuleTemperature4 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID09_MODULE_TEMPERATURE4[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID09 Module temperature5
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId09ModuleTemperature5(pub crate::db::units::Celsius);

pub(crate) static CMU_ID09_MODULE_TEMPERATURE5: [Item; 1] = [
    Item { name: "CMU ID09 Module temperature5", pos: 51, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId09ModuleTemperature5 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID09_MODULE_TEMPERATURE5[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID09 Module temperature6
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId09ModuleTemperature6(pub crate::db::units::Celsius);

pub(crate) static CMU_ID09_MODULE_TEMPERATURE6: [Item; 1] = [
    Item { name: "CMU ID09 Module temperature6", pos: 52, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId09ModuleTemperature6 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID09_MODULE_TEMPERATURE6[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID10 Cell A Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId10CellABalancerDrive(pub bool);

pub(crate) static CMU_ID10_CELL_A_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID10 Cell A Balancer drive", pos: 11, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId10CellABalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID10_CELL_A_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID10 Cell A capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId10CellACapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID10_CELL_A_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID10 Cell A capa.(measured)", pos: 138, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId10CellACapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID10_CELL_A_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID10 Cell A voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId10CellAVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID10_CELL_A_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID10 Cell A voltage", pos: 138, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId10CellAVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID10_CELL_A_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID10 Cell B Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId10CellBBalancerDrive(pub bool);

pub(crate) static CMU_ID10_CELL_B_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID10 Cell B Balancer drive", pos: 11, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId10CellBBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID10_CELL_B_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID10 Cell B capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId10CellBCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID10_CELL_B_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID10 Cell B capa.(measured)", pos: 140, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId10CellBCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID10_CELL_B_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID10 Cell B voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId10CellBVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID10_CELL_B_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID10 Cell B voltage", pos: 140, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId10CellBVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID10_CELL_B_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID10 Cell C Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId10CellCBalancerDrive(pub bool);

pub(crate) static CMU_ID10_CELL_C_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID10 Cell C Balancer drive", pos: 11, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId10CellCBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID10_CELL_C_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID10 Cell C capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId10CellCCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID10_CELL_C_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID10 Cell C capa.(measured)", pos: 142, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId10CellCCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID10_CELL_C_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID10 Cell C voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId10CellCVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID10_CELL_C_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID10 Cell C voltage", pos: 142, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId10CellCVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID10_CELL_C_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID10 Cell D Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId10CellDBalancerDrive(pub bool);

pub(crate) static CMU_ID10_CELL_D_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID10 Cell D Balancer drive", pos: 11, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId10CellDBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID10_CELL_D_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID10 Cell D capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId10CellDCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID10_CELL_D_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID10 Cell D capa.(measured)", pos: 144, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId10CellDCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID10_CELL_D_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID10 Cell D voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId10CellDVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID10_CELL_D_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID10 Cell D voltage", pos: 144, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId10CellDVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID10_CELL_D_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID10 Cell E Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId10CellEBalancerDrive(pub bool);

pub(crate) static CMU_ID10_CELL_E_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID10 Cell E Balancer drive", pos: 11, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId10CellEBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID10_CELL_E_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID10 Cell E capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId10CellECapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID10_CELL_E_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID10 Cell E capa.(measured)", pos: 146, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId10CellECapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID10_CELL_E_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID10 Cell E voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId10CellEVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID10_CELL_E_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID10 Cell E voltage", pos: 146, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId10CellEVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID10_CELL_E_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID10 Cell F Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId10CellFBalancerDrive(pub bool);

pub(crate) static CMU_ID10_CELL_F_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID10 Cell F Balancer drive", pos: 11, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId10CellFBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID10_CELL_F_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID10 Cell F capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId10CellFCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID10_CELL_F_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID10 Cell F capa.(measured)", pos: 148, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId10CellFCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID10_CELL_F_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID10 Cell F voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId10CellFVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID10_CELL_F_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID10 Cell F voltage", pos: 148, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId10CellFVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID10_CELL_F_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID10 Cell G Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId10CellGBalancerDrive(pub bool);

pub(crate) static CMU_ID10_CELL_G_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID10 Cell G Balancer drive", pos: 11, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId10CellGBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID10_CELL_G_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID10 Cell G capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId10CellGCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID10_CELL_G_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID10 Cell G capa.(measured)", pos: 150, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId10CellGCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID10_CELL_G_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID10 Cell G voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId10CellGVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID10_CELL_G_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID10 Cell G voltage", pos: 150, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId10CellGVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID10_CELL_G_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID10 Cell H Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId10CellHBalancerDrive(pub bool);

pub(crate) static CMU_ID10_CELL_H_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID10 Cell H Balancer drive", pos: 11, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId10CellHBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID10_CELL_H_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID10 Cell H capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId10CellHCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID10_CELL_H_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID10 Cell H capa.(measured)", pos: 152, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId10CellHCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID10_CELL_H_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID10 Cell H voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId10CellHVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID10_CELL_H_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID10 Cell H voltage", pos: 152, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId10CellHVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID10_CELL_H_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID10 Module temperature1
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId10ModuleTemperature1(pub crate::db::units::Celsius);

pub(crate) static CMU_ID10_MODULE_TEMPERATURE1: [Item; 1] = [
    Item { name: "CMU ID10 Module temperature1", pos: 53, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId10ModuleTemperature1 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID10_MODULE_TEMPERATURE1[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID10 Module temperature2
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId10ModuleTemperature2(pub crate::db::units::Celsius);

pub(crate) static CMU_ID10_MODULE_TEMPERATURE2: [Item; 1] = [
    Item { name: "CMU ID10 Module temperature2", pos: 54, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId10ModuleTemperature2 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID10_MODULE_TEMPERATURE2[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID10 Module temperature3
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId10ModuleTemperature3(pub crate::db::units::Celsius);

pub(crate) static CMU_ID10_MODULE_TEMPERATURE3: [Item; 1] = [
    Item { name: "CMU ID10 Module temperature3", pos: 55, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId10ModuleTemperature3 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID10_MODULE_TEMPERATURE3[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID10 Module temperature4
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId10ModuleTemperature4(pub crate::db::units::Celsius);

pub(crate) static CMU_ID10_MODULE_TEMPERATURE4: [Item; 1] = [
    Item { name: "CMU ID10 Module temperature4", pos: 56, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId10ModuleTemperature4 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID10_MODULE_TEMPERATURE4[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID10 Module temperature5
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId10ModuleTemperature5(pub crate::db::units::Celsius);

pub(crate) static CMU_ID10_MODULE_TEMPERATURE5: [Item; 1] = [
    Item { name: "CMU ID10 Module temperature5", pos: 57, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId10ModuleTemperature5 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID10_MODULE_TEMPERATURE5[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID10 Module temperature6
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId10ModuleTemperature6(pub crate::db::units::Celsius);

pub(crate) static CMU_ID10_MODULE_TEMPERATURE6: [Item; 1] = [
    Item { name: "CMU ID10 Module temperature6", pos: 58, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId10ModuleTemperature6 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID10_MODULE_TEMPERATURE6[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID11 Cell A Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId11CellABalancerDrive(pub bool);

pub(crate) static CMU_ID11_CELL_A_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID11 Cell A Balancer drive", pos: 12, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId11CellABalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID11_CELL_A_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID11 Cell A capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId11CellACapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID11_CELL_A_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID11 Cell A capa.(measured)", pos: 154, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId11CellACapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID11_CELL_A_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID11 Cell A voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId11CellAVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID11_CELL_A_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID11 Cell A voltage", pos: 154, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId11CellAVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID11_CELL_A_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID11 Cell B Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId11CellBBalancerDrive(pub bool);

pub(crate) static CMU_ID11_CELL_B_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID11 Cell B Balancer drive", pos: 12, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId11CellBBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID11_CELL_B_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID11 Cell B capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId11CellBCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID11_CELL_B_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID11 Cell B capa.(measured)", pos: 156, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId11CellBCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID11_CELL_B_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID11 Cell B voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId11CellBVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID11_CELL_B_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID11 Cell B voltage", pos: 156, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId11CellBVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID11_CELL_B_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID11 Cell C Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId11CellCBalancerDrive(pub bool);

pub(crate) static CMU_ID11_CELL_C_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID11 Cell C Balancer drive", pos: 12, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId11CellCBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID11_CELL_C_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID11 Cell C capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId11CellCCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID11_CELL_C_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID11 Cell C capa.(measured)", pos: 158, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId11CellCCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID11_CELL_C_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID11 Cell C voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId11CellCVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID11_CELL_C_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID11 Cell C voltage", pos: 158, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId11CellCVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID11_CELL_C_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID11 Cell D Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId11CellDBalancerDrive(pub bool);

pub(crate) static CMU_ID11_CELL_D_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID11 Cell D Balancer drive", pos: 12, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId11CellDBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID11_CELL_D_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID11 Cell D capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId11CellDCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID11_CELL_D_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID11 Cell D capa.(measured)", pos: 160, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId11CellDCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID11_CELL_D_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID11 Cell D voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId11CellDVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID11_CELL_D_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID11 Cell D voltage", pos: 160, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId11CellDVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID11_CELL_D_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID11 Cell E Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId11CellEBalancerDrive(pub bool);

pub(crate) static CMU_ID11_CELL_E_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID11 Cell E Balancer drive", pos: 12, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId11CellEBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID11_CELL_E_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID11 Cell E capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId11CellECapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID11_CELL_E_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID11 Cell E capa.(measured)", pos: 162, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId11CellECapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID11_CELL_E_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID11 Cell E voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId11CellEVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID11_CELL_E_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID11 Cell E voltage", pos: 162, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId11CellEVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID11_CELL_E_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID11 Cell F Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId11CellFBalancerDrive(pub bool);

pub(crate) static CMU_ID11_CELL_F_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID11 Cell F Balancer drive", pos: 12, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId11CellFBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID11_CELL_F_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID11 Cell F capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId11CellFCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID11_CELL_F_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID11 Cell F capa.(measured)", pos: 164, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId11CellFCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID11_CELL_F_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID11 Cell F voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId11CellFVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID11_CELL_F_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID11 Cell F voltage", pos: 164, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId11CellFVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID11_CELL_F_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID11 Cell G Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId11CellGBalancerDrive(pub bool);

pub(crate) static CMU_ID11_CELL_G_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID11 Cell G Balancer drive", pos: 12, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId11CellGBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID11_CELL_G_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID11 Cell G capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId11CellGCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID11_CELL_G_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID11 Cell G capa.(measured)", pos: 166, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId11CellGCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID11_CELL_G_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID11 Cell G voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId11CellGVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID11_CELL_G_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID11 Cell G voltage", pos: 166, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId11CellGVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID11_CELL_G_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID11 Cell H Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId11CellHBalancerDrive(pub bool);

pub(crate) static CMU_ID11_CELL_H_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID11 Cell H Balancer drive", pos: 12, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId11CellHBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID11_CELL_H_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID11 Cell H capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId11CellHCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID11_CELL_H_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID11 Cell H capa.(measured)", pos: 168, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId11CellHCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID11_CELL_H_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID11 Cell H voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId11CellHVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID11_CELL_H_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID11 Cell H voltage", pos: 168, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId11CellHVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID11_CELL_H_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID11 Module temperature1
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId11ModuleTemperature1(pub crate::db::units::Celsius);

pub(crate) static CMU_ID11_MODULE_TEMPERATURE1: [Item; 1] = [
    Item { name: "CMU ID11 Module temperature1", pos: 59, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId11ModuleTemperature1 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID11_MODULE_TEMPERATURE1[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID11 Module temperature2
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId11ModuleTemperature2(pub crate::db::units::Celsius);

pub(crate) static CMU_ID11_MODULE_TEMPERATURE2: [Item; 1] = [
    Item { name: "CMU ID11 Module temperature2", pos: 60, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId11ModuleTemperature2 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID11_MODULE_TEMPERATURE2[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID11 Module temperature3
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId11ModuleTemperature3(pub crate::db::units::Celsius);

pub(crate) static CMU_ID11_MODULE_TEMPERATURE3: [Item; 1] = [
    Item { name: "CMU ID11 Module temperature3", pos: 61, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId11ModuleTemperature3 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID11_MODULE_TEMPERATURE3[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID11 Module temperature4
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId11ModuleTemperature4(pub crate::db::units::Celsius);

pub(crate) static CMU_ID11_MODULE_TEMPERATURE4: [Item; 1] = [
    Item { name: "CMU ID11 Module temperature4", pos: 62, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId11ModuleTemperature4 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID11_MODULE_TEMPERATURE4[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID11 Module temperature5
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId11ModuleTemperature5(pub crate::db::units::Celsius);

pub(crate) static CMU_ID11_MODULE_TEMPERATURE5: [Item; 1] = [
    Item { name: "CMU ID11 Module temperature5", pos: 63, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId11ModuleTemperature5 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID11_MODULE_TEMPERATURE5[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID11 Module temperature6
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId11ModuleTemperature6(pub crate::db::units::Celsius);

pub(crate) static CMU_ID11_MODULE_TEMPERATURE6: [Item; 1] = [
    Item { name: "CMU ID11 Module temperature6", pos: 64, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId11ModuleTemperature6 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID11_MODULE_TEMPERATURE6[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID12 Cell A Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId12CellABalancerDrive(pub bool);

pub(crate) static CMU_ID12_CELL_A_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID12 Cell A Balancer drive", pos: 13, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId12CellABalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID12_CELL_A_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID12 Cell A capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId12CellACapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID12_CELL_A_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID12 Cell A capa.(measured)", pos: 170, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId12CellACapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID12_CELL_A_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID12 Cell A voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId12CellAVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID12_CELL_A_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID12 Cell A voltage", pos: 170, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId12CellAVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID12_CELL_A_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID12 Cell B Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId12CellBBalancerDrive(pub bool);

pub(crate) static CMU_ID12_CELL_B_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID12 Cell B Balancer drive", pos: 13, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId12CellBBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID12_CELL_B_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID12 Cell B capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId12CellBCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID12_CELL_B_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID12 Cell B capa.(measured)", pos: 172, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId12CellBCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID12_CELL_B_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID12 Cell B voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId12CellBVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID12_CELL_B_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID12 Cell B voltage", pos: 172, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId12CellBVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID12_CELL_B_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID12 Cell C Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId12CellCBalancerDrive(pub bool);

pub(crate) static CMU_ID12_CELL_C_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID12 Cell C Balancer drive", pos: 13, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId12CellCBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID12_CELL_C_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID12 Cell C capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId12CellCCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID12_CELL_C_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID12 Cell C capa.(measured)", pos: 174, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId12CellCCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID12_CELL_C_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID12 Cell C voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId12CellCVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID12_CELL_C_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID12 Cell C voltage", pos: 174, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId12CellCVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID12_CELL_C_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID12 Cell D Balancer drive
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmuId12CellDBalancerDrive(pub bool);

pub(crate) static CMU_ID12_CELL_D_BALANCER_DRIVE: [Item; 1] = [
    Item { name: "CMU ID12 Cell D Balancer drive", pos: 13, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CmuId12CellDBalancerDrive {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x04), item: &CMU_ID12_CELL_D_BALANCER_DRIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// CMU ID12 Cell D capa.(measured)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId12CellDCapaMeasured(pub crate::db::units::AmpHours);

pub(crate) static CMU_ID12_CELL_D_CAPA_MEASURED: [Item; 1] = [
    Item { name: "CMU ID12 Cell D capa.(measured)", pos: 176, len: 2, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CmuId12CellDCapaMeasured {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0101], source: Source::Record(0x12), item: &CMU_ID12_CELL_D_CAPA_MEASURED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// CMU ID12 Cell D voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId12CellDVoltage(pub crate::db::units::Volts);

pub(crate) static CMU_ID12_CELL_D_VOLTAGE: [Item; 1] = [
    Item { name: "CMU ID12 Cell D voltage", pos: 176, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
];

impl crate::item::Reading for CmuId12CellDVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x02), item: &CMU_ID12_CELL_D_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// CMU ID12 Module temperature1
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId12ModuleTemperature1(pub crate::db::units::Celsius);

pub(crate) static CMU_ID12_MODULE_TEMPERATURE1: [Item; 1] = [
    Item { name: "CMU ID12 Module temperature1", pos: 65, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId12ModuleTemperature1 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID12_MODULE_TEMPERATURE1[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID12 Module temperature2
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId12ModuleTemperature2(pub crate::db::units::Celsius);

pub(crate) static CMU_ID12_MODULE_TEMPERATURE2: [Item; 1] = [
    Item { name: "CMU ID12 Module temperature2", pos: 66, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId12ModuleTemperature2 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID12_MODULE_TEMPERATURE2[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// CMU ID12 Module temperature3
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CmuId12ModuleTemperature3(pub crate::db::units::Celsius);

pub(crate) static CMU_ID12_MODULE_TEMPERATURE3: [Item; 1] = [
    Item { name: "CMU ID12 Module temperature3", pos: 67, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for CmuId12ModuleTemperature3 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x03), item: &CMU_ID12_MODULE_TEMPERATURE3[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Elapsed time
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElapsedTime(pub crate::db::units::Minutes);

pub(crate) static ELAPSED_TIME: [Item; 1] = [
    Item { name: "Elapsed time", pos: 10, len: 2, bits: None, unit: Unit::Minutes, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for ElapsedTime {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0101, 0x0104, 0x0010, 0x0011], source: Source::FreezeFrame, item: &ELAPSED_TIME[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Minutes(n)))
    }
}

/// IGCT power supply voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct IgctPowerSupplyVoltage(pub crate::db::units::Volts);

pub(crate) static IGCT_POWER_SUPPLY_VOLTAGE: [Item; 1] = [
    Item { name: "IGCT power supply voltage", pos: 40, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for IgctPowerSupplyVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0101, 0x0104, 0x0010, 0x0011], source: Source::Record(0x01), item: &IGCT_POWER_SUPPLY_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Ignition cycle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IgnitionCycle(pub u32);

pub(crate) static IGNITION_CYCLE: [Item; 1] = [
    Item { name: "Ignition cycle", pos: 9, len: 1, bits: None, unit: Unit::None, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for IgnitionCycle {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0101, 0x0104, 0x0010, 0x0011], source: Source::FreezeFrame, item: &IGNITION_CYCLE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.count().map(|n| Self(n))
    }
}

/// Internal resistance dif. MAX
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InternalResistanceDifMax(pub crate::db::units::Milliohms);

pub(crate) static INTERNAL_RESISTANCE_DIF_MAX: [Item; 1] = [
    Item { name: "Internal resistance dif. MAX", pos: 54, len: 1, bits: None, unit: Unit::Milliohms, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for InternalResistanceDifMax {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0010, 0x0011], source: Source::Record(0x01), item: &INTERNAL_RESISTANCE_DIF_MAX[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Milliohms(n)))
    }
}

/// Internal resistance maximum
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InternalResistanceMaximum(pub crate::db::units::Milliohms);

pub(crate) static INTERNAL_RESISTANCE_MAXIMUM: [Item; 1] = [
    Item { name: "Internal resistance maximum", pos: 27, len: 1, bits: None, unit: Unit::Milliohms, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for InternalResistanceMaximum {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0101, 0x0104, 0x0010, 0x0011], source: Source::Record(0x01), item: &INTERNAL_RESISTANCE_MAXIMUM[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Milliohms(n)))
    }
}

/// Internal resistance minimum
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct InternalResistanceMinimum(pub crate::db::units::Milliohms);

pub(crate) static INTERNAL_RESISTANCE_MINIMUM: [Item; 1] = [
    Item { name: "Internal resistance minimum", pos: 28, len: 1, bits: None, unit: Unit::Milliohms, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for InternalResistanceMinimum {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0101, 0x0104, 0x0010, 0x0011], source: Source::Record(0x01), item: &INTERNAL_RESISTANCE_MINIMUM[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Milliohms(n)))
    }
}

/// Leak detection flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeakDetectionFlag(pub bool);

pub(crate) static LEAK_DETECTION_FLAG: [Item; 1] = [
    Item { name: "Leak detection flag", pos: 26, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for LeakDetectionFlag {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0101, 0x0104], source: Source::Record(0x01), item: &LEAK_DETECTION_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Leak sensor output(Precheck)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LeakSensorOutputPrecheck(pub bool);

pub(crate) static LEAK_SENSOR_OUTPUT_PRECHECK: [Item; 1] = [
    Item { name: "Leak sensor output(Precheck)", pos: 26, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for LeakSensorOutputPrecheck {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0101, 0x0104, 0x0010, 0x0011], source: Source::Record(0x01), item: &LEAK_SENSOR_OUTPUT_PRECHECK[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Maximum temperature module ID
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaximumTemperatureModuleId(pub f64);

pub(crate) static MAXIMUM_TEMPERATURE_MODULE_ID: [Item; 2] = [
    Item { name: "Maximum temperature module ID", pos: 13, len: 1, bits: None, unit: Unit::None, conv: Conv::Linear { factor: 1.0, offset: 1.0, decimals: 0 } },
    Item { name: "Maximum temperature module ID", pos: 13, len: 1, bits: None, unit: Unit::None, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for MaximumTemperatureModuleId {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x01), item: &MAXIMUM_TEMPERATURE_MODULE_ID[0] },
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x01), item: &MAXIMUM_TEMPERATURE_MODULE_ID[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(n))
    }
}

/// Maximum voltage cell ID
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaximumVoltageCellId(pub f64);

pub(crate) static MAXIMUM_VOLTAGE_CELL_ID: [Item; 2] = [
    Item { name: "Maximum voltage cell ID", pos: 6, len: 1, bits: None, unit: Unit::None, conv: Conv::Linear { factor: 1.0, offset: 1.0, decimals: 0 } },
    Item { name: "Maximum voltage cell ID", pos: 6, len: 1, bits: None, unit: Unit::None, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for MaximumVoltageCellId {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x01), item: &MAXIMUM_VOLTAGE_CELL_ID[0] },
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x01), item: &MAXIMUM_VOLTAGE_CELL_ID[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(n))
    }
}

/// Minimum temperature module ID
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MinimumTemperatureModuleId(pub f64);

pub(crate) static MINIMUM_TEMPERATURE_MODULE_ID: [Item; 2] = [
    Item { name: "Minimum temperature module ID", pos: 15, len: 1, bits: None, unit: Unit::None, conv: Conv::Linear { factor: 1.0, offset: 1.0, decimals: 0 } },
    Item { name: "Minimum temperature module ID", pos: 15, len: 1, bits: None, unit: Unit::None, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for MinimumTemperatureModuleId {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x01), item: &MINIMUM_TEMPERATURE_MODULE_ID[0] },
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x01), item: &MINIMUM_TEMPERATURE_MODULE_ID[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(n))
    }
}

/// Minimum voltage cell ID
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MinimumVoltageCellId(pub f64);

pub(crate) static MINIMUM_VOLTAGE_CELL_ID: [Item; 2] = [
    Item { name: "Minimum voltage cell ID", pos: 9, len: 1, bits: None, unit: Unit::None, conv: Conv::Linear { factor: 1.0, offset: 1.0, decimals: 0 } },
    Item { name: "Minimum voltage cell ID", pos: 9, len: 1, bits: None, unit: Unit::None, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for MinimumVoltageCellId {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x01), item: &MINIMUM_VOLTAGE_CELL_ID[0] },
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x01), item: &MINIMUM_VOLTAGE_CELL_ID[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(n))
    }
}

/// Module maximum temperature
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleMaximumTemperature(pub crate::db::units::Celsius);

pub(crate) static MODULE_MAXIMUM_TEMPERATURE: [Item; 1] = [
    Item { name: "Module maximum temperature", pos: 12, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleMaximumTemperature {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0101, 0x0104, 0x0010, 0x0011], source: Source::Record(0x01), item: &MODULE_MAXIMUM_TEMPERATURE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module minimum temperature
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleMinimumTemperature(pub crate::db::units::Celsius);

pub(crate) static MODULE_MINIMUM_TEMPERATURE: [Item; 1] = [
    Item { name: "Module minimum temperature", pos: 14, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleMinimumTemperature {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0101, 0x0104, 0x0010, 0x0011], source: Source::Record(0x01), item: &MODULE_MINIMUM_TEMPERATURE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 00
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature00(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE00: [Item; 1] = [
    Item { name: "Module temperature 00", pos: 2, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature00 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE00[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 01
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature01(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE01: [Item; 1] = [
    Item { name: "Module temperature 01", pos: 3, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature01 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE01[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 02
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature02(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE02: [Item; 1] = [
    Item { name: "Module temperature 02", pos: 4, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature02 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE02[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 03
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature03(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE03: [Item; 1] = [
    Item { name: "Module temperature 03", pos: 5, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature03 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE03[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 04
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature04(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE04: [Item; 1] = [
    Item { name: "Module temperature 04", pos: 6, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature04 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE04[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 05
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature05(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE05: [Item; 1] = [
    Item { name: "Module temperature 05", pos: 7, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature05 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE05[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 06
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature06(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE06: [Item; 1] = [
    Item { name: "Module temperature 06", pos: 8, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature06 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE06[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 07
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature07(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE07: [Item; 1] = [
    Item { name: "Module temperature 07", pos: 9, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature07 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE07[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 08
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature08(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE08: [Item; 1] = [
    Item { name: "Module temperature 08", pos: 10, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature08 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE08[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 09
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature09(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE09: [Item; 1] = [
    Item { name: "Module temperature 09", pos: 11, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature09 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE09[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 10
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature10(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE10: [Item; 1] = [
    Item { name: "Module temperature 10", pos: 12, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature10 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE10[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 11
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature11(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE11: [Item; 1] = [
    Item { name: "Module temperature 11", pos: 13, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature11 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE11[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 12
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature12(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE12: [Item; 1] = [
    Item { name: "Module temperature 12", pos: 14, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature12 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE12[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 13
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature13(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE13: [Item; 1] = [
    Item { name: "Module temperature 13", pos: 15, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature13 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE13[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 14
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature14(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE14: [Item; 1] = [
    Item { name: "Module temperature 14", pos: 16, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature14 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE14[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 15
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature15(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE15: [Item; 1] = [
    Item { name: "Module temperature 15", pos: 17, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature15 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE15[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 16
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature16(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE16: [Item; 1] = [
    Item { name: "Module temperature 16", pos: 18, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature16 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE16[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 17
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature17(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE17: [Item; 1] = [
    Item { name: "Module temperature 17", pos: 19, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature17 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE17[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 18
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature18(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE18: [Item; 1] = [
    Item { name: "Module temperature 18", pos: 20, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature18 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE18[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 19
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature19(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE19: [Item; 1] = [
    Item { name: "Module temperature 19", pos: 21, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature19 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE19[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 20
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature20(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE20: [Item; 1] = [
    Item { name: "Module temperature 20", pos: 22, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature20 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE20[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 21
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature21(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE21: [Item; 1] = [
    Item { name: "Module temperature 21", pos: 23, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature21 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE21[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 22
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature22(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE22: [Item; 1] = [
    Item { name: "Module temperature 22", pos: 24, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature22 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE22[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 23
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature23(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE23: [Item; 1] = [
    Item { name: "Module temperature 23", pos: 25, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature23 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE23[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 24
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature24(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE24: [Item; 1] = [
    Item { name: "Module temperature 24", pos: 26, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature24 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE24[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 25
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature25(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE25: [Item; 1] = [
    Item { name: "Module temperature 25", pos: 27, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature25 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE25[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 26
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature26(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE26: [Item; 1] = [
    Item { name: "Module temperature 26", pos: 28, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature26 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE26[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 27
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature27(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE27: [Item; 1] = [
    Item { name: "Module temperature 27", pos: 29, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature27 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE27[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 28
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature28(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE28: [Item; 1] = [
    Item { name: "Module temperature 28", pos: 30, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature28 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE28[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 29
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature29(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE29: [Item; 1] = [
    Item { name: "Module temperature 29", pos: 31, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature29 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE29[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 30
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature30(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE30: [Item; 1] = [
    Item { name: "Module temperature 30", pos: 32, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature30 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE30[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 31
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature31(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE31: [Item; 1] = [
    Item { name: "Module temperature 31", pos: 33, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature31 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE31[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 32
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature32(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE32: [Item; 1] = [
    Item { name: "Module temperature 32", pos: 34, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature32 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE32[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 33
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature33(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE33: [Item; 1] = [
    Item { name: "Module temperature 33", pos: 35, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature33 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE33[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 34
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature34(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE34: [Item; 1] = [
    Item { name: "Module temperature 34", pos: 36, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature34 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE34[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 35
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature35(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE35: [Item; 1] = [
    Item { name: "Module temperature 35", pos: 37, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature35 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE35[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 36
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature36(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE36: [Item; 1] = [
    Item { name: "Module temperature 36", pos: 38, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature36 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE36[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 37
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature37(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE37: [Item; 1] = [
    Item { name: "Module temperature 37", pos: 39, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature37 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE37[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 38
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature38(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE38: [Item; 1] = [
    Item { name: "Module temperature 38", pos: 40, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature38 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE38[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 39
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature39(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE39: [Item; 1] = [
    Item { name: "Module temperature 39", pos: 41, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature39 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE39[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 40
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature40(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE40: [Item; 1] = [
    Item { name: "Module temperature 40", pos: 42, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature40 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE40[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 41
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature41(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE41: [Item; 1] = [
    Item { name: "Module temperature 41", pos: 43, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature41 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE41[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 42
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature42(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE42: [Item; 1] = [
    Item { name: "Module temperature 42", pos: 44, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature42 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE42[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 43
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature43(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE43: [Item; 1] = [
    Item { name: "Module temperature 43", pos: 45, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature43 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE43[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 44
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature44(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE44: [Item; 1] = [
    Item { name: "Module temperature 44", pos: 46, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature44 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE44[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 45
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature45(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE45: [Item; 1] = [
    Item { name: "Module temperature 45", pos: 47, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature45 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE45[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 46
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature46(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE46: [Item; 1] = [
    Item { name: "Module temperature 46", pos: 48, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature46 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE46[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 47
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature47(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE47: [Item; 1] = [
    Item { name: "Module temperature 47", pos: 49, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature47 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE47[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 48
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature48(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE48: [Item; 1] = [
    Item { name: "Module temperature 48", pos: 50, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature48 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE48[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 49
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature49(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE49: [Item; 1] = [
    Item { name: "Module temperature 49", pos: 51, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature49 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE49[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 50
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature50(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE50: [Item; 1] = [
    Item { name: "Module temperature 50", pos: 52, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature50 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE50[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 51
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature51(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE51: [Item; 1] = [
    Item { name: "Module temperature 51", pos: 53, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature51 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE51[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 52
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature52(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE52: [Item; 1] = [
    Item { name: "Module temperature 52", pos: 54, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature52 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE52[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 53
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature53(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE53: [Item; 1] = [
    Item { name: "Module temperature 53", pos: 55, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature53 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE53[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 54
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature54(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE54: [Item; 1] = [
    Item { name: "Module temperature 54", pos: 56, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature54 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE54[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 55
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature55(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE55: [Item; 1] = [
    Item { name: "Module temperature 55", pos: 57, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature55 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE55[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 56
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature56(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE56: [Item; 1] = [
    Item { name: "Module temperature 56", pos: 58, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature56 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE56[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 57
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature57(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE57: [Item; 1] = [
    Item { name: "Module temperature 57", pos: 59, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature57 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE57[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 58
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature58(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE58: [Item; 1] = [
    Item { name: "Module temperature 58", pos: 60, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature58 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE58[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 59
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature59(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE59: [Item; 1] = [
    Item { name: "Module temperature 59", pos: 61, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature59 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE59[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 60
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature60(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE60: [Item; 1] = [
    Item { name: "Module temperature 60", pos: 62, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature60 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE60[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 61
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature61(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE61: [Item; 1] = [
    Item { name: "Module temperature 61", pos: 63, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature61 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE61[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 62
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature62(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE62: [Item; 1] = [
    Item { name: "Module temperature 62", pos: 64, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature62 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE62[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 63
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature63(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE63: [Item; 1] = [
    Item { name: "Module temperature 63", pos: 65, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature63 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE63[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 64
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature64(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE64: [Item; 1] = [
    Item { name: "Module temperature 64", pos: 66, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature64 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE64[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 65
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature65(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE65: [Item; 1] = [
    Item { name: "Module temperature 65", pos: 67, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature65 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE65[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 66
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature66(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE66: [Item; 1] = [
    Item { name: "Module temperature 66", pos: 68, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature66 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE66[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 67
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature67(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE67: [Item; 1] = [
    Item { name: "Module temperature 67", pos: 69, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature67 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE67[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 68
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature68(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE68: [Item; 1] = [
    Item { name: "Module temperature 68", pos: 70, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature68 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE68[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 69
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature69(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE69: [Item; 1] = [
    Item { name: "Module temperature 69", pos: 71, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature69 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE69[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 70
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature70(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE70: [Item; 1] = [
    Item { name: "Module temperature 70", pos: 72, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature70 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE70[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Module temperature 71
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ModuleTemperature71(pub crate::db::units::Celsius);

pub(crate) static MODULE_TEMPERATURE71: [Item; 1] = [
    Item { name: "Module temperature 71", pos: 73, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -50.0, decimals: 0 } },
];

impl crate::item::Reading for ModuleTemperature71 {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x03), item: &MODULE_TEMPERATURE71[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// OBC integrated current value
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObcIntegratedCurrentValue(pub crate::db::units::KiloAmpHours);

pub(crate) static OBC_INTEGRATED_CURRENT_VALUE: [Item; 1] = [
    Item { name: "OBC integrated current value", pos: 6, len: 4, bits: None, unit: Unit::KiloAmpHours, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for ObcIntegratedCurrentValue {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0002, 0x0003, 0x0004, 0x0101, 0x0104, 0x0010, 0x0011], source: Source::Record(0x11), item: &OBC_INTEGRATED_CURRENT_VALUE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::KiloAmpHours(n)))
    }
}

/// Odometer
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Odometer(pub crate::db::units::Km);

pub(crate) static ODOMETER: [Item; 1] = [
    Item { name: "Odometer", pos: 5, len: 2, bits: None, unit: Unit::Km, conv: Conv::Linear { factor: 6.4, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for Odometer {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0101, 0x0104, 0x0010, 0x0011], source: Source::FreezeFrame, item: &ODOMETER[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Km(n)))
    }
}

/// OR power supply voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OrPowerSupplyVoltage(pub crate::db::units::Volts);

pub(crate) static OR_POWER_SUPPLY_VOLTAGE: [Item; 1] = [
    Item { name: "OR power supply voltage", pos: 39, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for OrPowerSupplyVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0003, 0x0004, 0x0101, 0x0104, 0x0010, 0x0011], source: Source::Record(0x01), item: &OR_POWER_SUPPLY_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Outside discharge integrated cur
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OutsideDischargeIntegratedCur(pub crate::db::units::AmpHours);

pub(crate) static OUTSIDE_DISCHARGE_INTEGRATED_CUR: [Item; 1] = [
    Item { name: "Outside discharge integrated cur", pos: 46, len: 4, bits: None, unit: Unit::AmpHours, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for OutsideDischargeIntegratedCur {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0010, 0x0011], source: Source::Record(0x01), item: &OUTSIDE_DISCHARGE_INTEGRATED_CUR[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::AmpHours(n)))
    }
}

/// Outside discharge time
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct OutsideDischargeTime(pub crate::db::units::Minutes);

pub(crate) static OUTSIDE_DISCHARGE_TIME: [Item; 1] = [
    Item { name: "Outside discharge time", pos: 44, len: 2, bits: None, unit: Unit::Minutes, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for OutsideDischargeTime {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0010, 0x0011], source: Source::Record(0x01), item: &OUTSIDE_DISCHARGE_TIME[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Minutes(n)))
    }
}

/// Quick charge integrated current
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuickChargeIntegratedCurrent(pub crate::db::units::KiloAmpHours);

pub(crate) static QUICK_CHARGE_INTEGRATED_CURRENT: [Item; 1] = [
    Item { name: "Quick charge integrated current", pos: 10, len: 4, bits: None, unit: Unit::KiloAmpHours, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for QuickChargeIntegratedCurrent {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0002, 0x0003, 0x0004, 0x0101, 0x0104, 0x0010, 0x0011], source: Source::Record(0x11), item: &QUICK_CHARGE_INTEGRATED_CURRENT[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::KiloAmpHours(n)))
    }
}

/// Resistance value
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ResistanceValue(pub crate::db::units::Kilohms);

pub(crate) static RESISTANCE_VALUE: [Item; 1] = [
    Item { name: "Resistance value", pos: 55, len: 2, bits: None, unit: Unit::Kilohms, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for ResistanceValue {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0011], source: Source::Record(0x01), item: &RESISTANCE_VALUE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Kilohms(n)))
    }
}

/// Stop time back up
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StopTimeBackUp(pub crate::db::units::Months);

pub(crate) static STOP_TIME_BACK_UP: [Item; 1] = [
    Item { name: "Stop time back up", pos: 14, len: 4, bits: None, unit: Unit::Months, conv: Conv::Linear { factor: 2.31481e-05, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for StopTimeBackUp {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0004, 0x0104], source: Source::Record(0x11), item: &STOP_TIME_BACK_UP[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Months(n)))
    }
}

/// Sum total time back up
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SumTotalTimeBackUp(pub crate::db::units::Months);

pub(crate) static SUM_TOTAL_TIME_BACK_UP: [Item; 1] = [
    Item { name: "Sum total time back up", pos: 2, len: 4, bits: None, unit: Unit::Months, conv: Conv::Linear { factor: 2.31481e-05, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for SumTotalTimeBackUp {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0001, 0x0002, 0x0003, 0x0004, 0x0101, 0x0104, 0x0010, 0x0011], source: Source::Record(0x11), item: &SUM_TOTAL_TIME_BACK_UP[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Months(n)))
    }
}

/// Target cell voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TargetCellVoltage(pub crate::db::units::Volts);

pub(crate) static TARGET_CELL_VOLTAGE: [Item; 2] = [
    Item { name: "Target cell voltage", pos: 37, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.005, offset: 2.1, decimals: 3 } },
    Item { name: "Target cell voltage", pos: 37, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for TargetCellVoltage {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0000, 0x0001, 0x0002, 0x0004, 0x0101, 0x0104], source: Source::Record(0x01), item: &TARGET_CELL_VOLTAGE[0] },
        At { versions: &[0x0003, 0x0010, 0x0011], source: Source::Record(0x01), item: &TARGET_CELL_VOLTAGE[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Voltage dif. fail judgment MAX
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VoltageDifFailJudgmentMax(pub crate::db::units::Volts);

pub(crate) static VOLTAGE_DIF_FAIL_JUDGMENT_MAX: [Item; 1] = [
    Item { name: "Voltage dif. fail judgment MAX", pos: 52, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.001, offset: 0.0, decimals: 3 } },
];

impl crate::item::Reading for VoltageDifFailJudgmentMax {
    const ECU: Ecu = Ecu::Bmu;
    const AT: &'static [At] = &[
        At { versions: &[0x0010, 0x0011], source: Source::Record(0x01), item: &VOLTAGE_DIF_FAIL_JUDGMENT_MAX[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}
