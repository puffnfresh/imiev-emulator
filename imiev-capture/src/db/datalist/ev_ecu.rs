//! The data items of the EV-ECU.

use crate::db::units::Unit;
use crate::item::{At, Conv, Item, Source, Value};
use crate::kwp::Ecu;

/// A/C power relay (diag)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ACPowerRelayDiag(pub bool);

pub(crate) static AC_POWER_RELAY_DIAG: [Item; 2] = [
    Item { name: "A/C power relay (diag)", pos: 43, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "A/C power relay (diag)", pos: 3, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ACPowerRelayDiag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &AC_POWER_RELAY_DIAG[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x01), item: &AC_POWER_RELAY_DIAG[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// A/C power relay (monitor)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ACPowerRelayMonitor(pub bool);

pub(crate) static AC_POWER_RELAY_MONITOR: [Item; 1] = [
    Item { name: "A/C power relay (monitor)", pos: 29, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ACPowerRelayMonitor {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &AC_POWER_RELAY_MONITOR[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// A/C relay
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ACRelay(pub bool);

pub(crate) static AC_RELAY: [Item; 2] = [
    Item { name: "A/C relay", pos: 45, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "A/C relay", pos: 3, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ACRelay {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &AC_RELAY[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x02), item: &AC_RELAY[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// ABS/ASC operation flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AbsAscOperationFlag(pub bool);

pub(crate) static ABS_ASC_OPERATION_FLAG: [Item; 1] = [
    Item { name: "ABS/ASC operation flag", pos: 67, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for AbsAscOperationFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &ABS_ASC_OPERATION_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Accelerator(control value)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AcceleratorControlValue(pub crate::db::units::Volts);

pub(crate) static ACCELERATOR_CONTROL_VALUE: [Item; 1] = [
    Item { name: "Accelerator(control value)", pos: 10, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.02, offset: 0.0, decimals: 2 } },
];

impl crate::item::Reading for AcceleratorControlValue {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x22), item: &ACCELERATOR_CONTROL_VALUE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Accumulated minute
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct AccumulatedMinute(pub crate::db::units::Minutes);

pub(crate) static ACCUMULATED_MINUTE: [Item; 1] = [
    Item { name: "Accumulated minute", pos: 7, len: 2, bits: None, unit: Unit::Minutes, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for AccumulatedMinute {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000, 0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &ACCUMULATED_MINUTE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Minutes(n)))
    }
}

/// Air bag collision signal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AirBagCollisionSignal(pub bool);

pub(crate) static AIR_BAG_COLLISION_SIGNAL: [Item; 1] = [
    Item { name: "Air bag collision signal", pos: 6, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for AirBagCollisionSignal {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300], source: Source::Record(0x01), item: &AIR_BAG_COLLISION_SIGNAL[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Air bag communication state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AirBagCommunicationState(pub bool);

pub(crate) static AIR_BAG_COMMUNICATION_STATE: [Item; 1] = [
    Item { name: "Air bag communication state", pos: 2, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "NG"), (1, 1, "OK")]) },
];

impl crate::item::Reading for AirBagCommunicationState {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300], source: Source::Record(0x23), item: &AIR_BAG_COMMUNICATION_STATE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "OK" => Some(Self(true)),
            "NG" => Some(Self(false)),
            _ => None,
        }
    }
}

/// APS1 power supply voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aps1PowerSupplyVoltage(pub crate::db::units::Volts);

pub(crate) static APS1_POWER_SUPPLY_VOLTAGE: [Item; 1] = [
    Item { name: "APS1 power supply voltage", pos: 2, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.02, offset: 0.0, decimals: 2 } },
];

impl crate::item::Reading for Aps1PowerSupplyVoltage {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x03), item: &APS1_POWER_SUPPLY_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// APS1 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aps1Voltage(pub crate::db::units::Volts);

pub(crate) static APS1_VOLTAGE: [Item; 1] = [
    Item { name: "APS1 voltage", pos: 5, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.02, offset: 0.0, decimals: 2 } },
];

impl crate::item::Reading for Aps1Voltage {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x03), item: &APS1_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// APS2 power supply voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aps2PowerSupplyVoltage(pub crate::db::units::Volts);

pub(crate) static APS2_POWER_SUPPLY_VOLTAGE: [Item; 1] = [
    Item { name: "APS2 power supply voltage", pos: 3, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.02, offset: 0.0, decimals: 2 } },
];

impl crate::item::Reading for Aps2PowerSupplyVoltage {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x03), item: &APS2_POWER_SUPPLY_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// APS2 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aps2Voltage(pub crate::db::units::Volts);

pub(crate) static APS2_VOLTAGE: [Item; 1] = [
    Item { name: "APS2 voltage", pos: 6, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.02, offset: 0.0, decimals: 2 } },
];

impl crate::item::Reading for Aps2Voltage {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x03), item: &APS2_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// APS (for control)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ApsForControl(pub crate::db::units::Volts);

pub(crate) static APS_FOR_CONTROL: [Item; 1] = [
    Item { name: "APS (for control)", pos: 20, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.02, offset: 0.0, decimals: 2 } },
];

impl crate::item::Reading for ApsForControl {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000, 0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &APS_FOR_CONTROL[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// APS(main)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ApsMain(pub crate::db::units::Volts);

pub(crate) static APS_MAIN: [Item; 1] = [
    Item { name: "APS(main)", pos: 30, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.02, offset: 0.0, decimals: 2 } },
];

impl crate::item::Reading for ApsMain {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &APS_MAIN[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// APS(main) power supply voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ApsMainPowerSupplyVoltage(pub crate::db::units::Volts);

pub(crate) static APS_MAIN_POWER_SUPPLY_VOLTAGE: [Item; 1] = [
    Item { name: "APS(main) power supply voltage", pos: 27, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.02, offset: 0.0, decimals: 2 } },
];

impl crate::item::Reading for ApsMainPowerSupplyVoltage {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &APS_MAIN_POWER_SUPPLY_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// APS(sub)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ApsSub(pub crate::db::units::Volts);

pub(crate) static APS_SUB: [Item; 1] = [
    Item { name: "APS(sub)", pos: 31, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.02, offset: 0.0, decimals: 2 } },
];

impl crate::item::Reading for ApsSub {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &APS_SUB[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// APS(sub) power supply voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ApsSubPowerSupplyVoltage(pub crate::db::units::Volts);

pub(crate) static APS_SUB_POWER_SUPPLY_VOLTAGE: [Item; 1] = [
    Item { name: "APS(sub) power supply voltage", pos: 28, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.02, offset: 0.0, decimals: 2 } },
];

impl crate::item::Reading for ApsSubPowerSupplyVoltage {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &APS_SUB_POWER_SUPPLY_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// B/P stroke.S initial learn flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BPStrokeSInitialLearnFlag(pub bool);

pub(crate) static BP_STROKE_S_INITIAL_LEARN_FLAG: [Item; 1] = [
    Item { name: "B/P stroke.S initial learn flag", pos: 7, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for BPStrokeSInitialLearnFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x0004, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &BP_STROKE_S_INITIAL_LEARN_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// B/P stroke sensor Equipment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BPStrokeSensorEquipment(pub bool);

pub(crate) static BP_STROKE_SENSOR_EQUIPMENT: [Item; 1] = [
    Item { name: "B/P stroke sensor Equipment", pos: 8, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "Not present"), (1, 1, "Present")]) },
];

impl crate::item::Reading for BPStrokeSensorEquipment {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &BP_STROKE_SENSOR_EQUIPMENT[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "Present" => Some(Self(true)),
            "Not present" => Some(Self(false)),
            _ => None,
        }
    }
}

/// B/P stroke sensor installation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BPStrokeSensorInstallation(pub bool);

pub(crate) static BP_STROKE_SENSOR_INSTALLATION: [Item; 1] = [
    Item { name: "B/P stroke sensor installation", pos: 8, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "NG"), (1, 1, "OK")]) },
];

impl crate::item::Reading for BPStrokeSensorInstallation {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x0004, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &BP_STROKE_SENSOR_INSTALLATION[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "OK" => Some(Self(true)),
            "NG" => Some(Self(false)),
            _ => None,
        }
    }
}

/// B/P stroke sensor power supply
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BPStrokeSensorPowerSupply(pub crate::db::units::Volts);

pub(crate) static BP_STROKE_SENSOR_POWER_SUPPLY: [Item; 2] = [
    Item { name: "B/P stroke sensor power supply", pos: 13, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.02, offset: 0.0, decimals: 2 } },
    Item { name: "B/P stroke sensor power supply", pos: 12, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.02, offset: 0.0, decimals: 2 } },
];

impl crate::item::Reading for BPStrokeSensorPowerSupply {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x0004, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x03), item: &BP_STROKE_SENSOR_POWER_SUPPLY[0] },
        At { versions: &[0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &BP_STROKE_SENSOR_POWER_SUPPLY[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// B/P stroke sensor voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BPStrokeSensorVoltage(pub crate::db::units::Volts);

pub(crate) static BP_STROKE_SENSOR_VOLTAGE: [Item; 1] = [
    Item { name: "B/P stroke sensor voltage", pos: 12, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.02, offset: 0.0, decimals: 2 } },
];

impl crate::item::Reading for BPStrokeSensorVoltage {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x0004, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x03), item: &BP_STROKE_SENSOR_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Back lamp relay
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackLampRelay(pub bool);

pub(crate) static BACK_LAMP_RELAY: [Item; 1] = [
    Item { name: "Back lamp relay", pos: 3, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for BackLampRelay {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x02), item: &BACK_LAMP_RELAY[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Back up lamp relay (diag)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackUpLampRelayDiag(pub bool);

pub(crate) static BACK_UP_LAMP_RELAY_DIAG: [Item; 2] = [
    Item { name: "Back up lamp relay (diag)", pos: 43, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Back up lamp relay (diag)", pos: 3, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for BackUpLampRelayDiag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &BACK_UP_LAMP_RELAY_DIAG[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x01), item: &BACK_UP_LAMP_RELAY_DIAG[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Back up lamp relay (monitor)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BackUpLampRelayMonitor(pub bool);

pub(crate) static BACK_UP_LAMP_RELAY_MONITOR: [Item; 1] = [
    Item { name: "Back up lamp relay (monitor)", pos: 29, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for BackUpLampRelayMonitor {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &BACK_UP_LAMP_RELAY_MONITOR[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Battery cooling request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BatteryCoolingRequest(pub bool);

pub(crate) static BATTERY_COOLING_REQUEST: [Item; 1] = [
    Item { name: "Battery cooling request", pos: 8, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "Not Request"), (1, 1, "Request")]) },
];

impl crate::item::Reading for BatteryCoolingRequest {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &BATTERY_COOLING_REQUEST[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "Request" => Some(Self(true)),
            "Not Request" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Battery cooling request flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BatteryCoolingRequestFlag(pub bool);

pub(crate) static BATTERY_COOLING_REQUEST_FLAG: [Item; 1] = [
    Item { name: "Battery cooling request flag", pos: 52, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for BatteryCoolingRequestFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &BATTERY_COOLING_REQUEST_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Battery current
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BatteryCurrent(pub crate::db::units::Amps);

pub(crate) static BATTERY_CURRENT: [Item; 1] = [
    Item { name: "Battery current", pos: 71, len: 2, bits: None, unit: Unit::Amps, conv: Conv::Linear { factor: 0.01, offset: -327.0, decimals: 2 } },
];

impl crate::item::Reading for BatteryCurrent {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &BATTERY_CURRENT[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Amps(n)))
    }
}

/// Battery Total voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BatteryTotalVoltage(pub crate::db::units::Volts);

pub(crate) static BATTERY_TOTAL_VOLTAGE: [Item; 1] = [
    Item { name: "Battery Total voltage", pos: 3, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 2.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for BatteryTotalVoltage {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x24), item: &BATTERY_TOTAL_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// BBU active flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BbuActiveFlag(pub bool);

pub(crate) static BBU_ACTIVE_FLAG: [Item; 1] = [
    Item { name: "BBU active flag", pos: 8, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for BbuActiveFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &BBU_ACTIVE_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// BMU:Battery balancer active
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BmuBatteryBalancerActive(pub bool);

pub(crate) static BMU_BATTERY_BALANCER_ACTIVE: [Item; 1] = [
    Item { name: "BMU:Battery balancer active", pos: 49, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for BmuBatteryBalancerActive {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &BMU_BATTERY_BALANCER_ACTIVE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// BMU:Battery cooling mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BmuBatteryCoolingMode(pub bool);

pub(crate) static BMU_BATTERY_COOLING_MODE: [Item; 1] = [
    Item { name: "BMU:Battery cooling mode", pos: 49, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for BmuBatteryCoolingMode {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &BMU_BATTERY_COOLING_MODE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// BMU:Battery total capacity
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BmuBatteryTotalCapacity(pub crate::db::units::KilowattHours);

pub(crate) static BMU_BATTERY_TOTAL_CAPACITY: [Item; 1] = [
    Item { name: "BMU:Battery total capacity", pos: 55, len: 2, bits: None, unit: Unit::KilowattHours, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for BmuBatteryTotalCapacity {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &BMU_BATTERY_TOTAL_CAPACITY[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::KilowattHours(n)))
    }
}

/// BMU:Battery upper limit voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BmuBatteryUpperLimitVoltage(pub crate::db::units::Volts);

pub(crate) static BMU_BATTERY_UPPER_LIMIT_VOLTAGE: [Item; 1] = [
    Item { name: "BMU:Battery upper limit voltage", pos: 53, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for BmuBatteryUpperLimitVoltage {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &BMU_BATTERY_UPPER_LIMIT_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// BMU:Battery usage permit flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BmuBatteryUsagePermitFlag(pub bool);

pub(crate) static BMU_BATTERY_USAGE_PERMIT_FLAG: [Item; 1] = [
    Item { name: "BMU:Battery usage permit flag", pos: 35, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for BmuBatteryUsagePermitFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &BMU_BATTERY_USAGE_PERMIT_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// BMU:BBU activation flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BmuBbuActivationFlag(pub bool);

pub(crate) static BMU_BBU_ACTIVATION_FLAG: [Item; 1] = [
    Item { name: "BMU:BBU activation flag", pos: 36, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for BmuBbuActivationFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &BMU_BBU_ACTIVATION_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// BMU:Cell temperature high flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BmuCellTemperatureHighFlag(pub bool);

pub(crate) static BMU_CELL_TEMPERATURE_HIGH_FLAG: [Item; 1] = [
    Item { name: "BMU:Cell temperature high flag", pos: 60, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for BmuCellTemperatureHighFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &BMU_CELL_TEMPERATURE_HIGH_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// BMU:Charge max time
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BmuChargeMaxTime(pub crate::db::units::Seconds);

pub(crate) static BMU_CHARGE_MAX_TIME: [Item; 1] = [
    Item { name: "BMU:Charge max time", pos: 57, len: 1, bits: None, unit: Unit::Seconds, conv: Conv::Linear { factor: 10.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for BmuChargeMaxTime {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &BMU_CHARGE_MAX_TIME[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Seconds(n)))
    }
}

/// BMU:Charge request flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BmuChargeRequestFlag(pub bool);

pub(crate) static BMU_CHARGE_REQUEST_FLAG: [Item; 1] = [
    Item { name: "BMU:Charge request flag", pos: 37, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for BmuChargeRequestFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &BMU_CHARGE_REQUEST_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// BMU:Charge status
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BmuChargeStatus(pub crate::db::units::Percent);

pub(crate) static BMU_CHARGE_STATUS: [Item; 1] = [
    Item { name: "BMU:Charge status", pos: 13, len: 1, bits: None, unit: Unit::Percent, conv: Conv::Linear { factor: 0.5, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for BmuChargeStatus {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000, 0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &BMU_CHARGE_STATUS[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Percent(n)))
    }
}

/// BMU:Charge voltage upper limit
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BmuChargeVoltageUpperLimit(pub crate::db::units::Volts);

pub(crate) static BMU_CHARGE_VOLTAGE_UPPER_LIMIT: [Item; 1] = [
    Item { name: "BMU:Charge voltage upper limit", pos: 58, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for BmuChargeVoltageUpperLimit {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &BMU_CHARGE_VOLTAGE_UPPER_LIMIT[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// BMU:Current difference flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BmuCurrentDifferenceFlag(pub bool);

pub(crate) static BMU_CURRENT_DIFFERENCE_FLAG: [Item; 1] = [
    Item { name: "BMU:Current difference flag", pos: 60, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for BmuCurrentDifferenceFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &BMU_CURRENT_DIFFERENCE_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// BMU:Low battery voltage flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BmuLowBatteryVoltageFlag(pub bool);

pub(crate) static BMU_LOW_BATTERY_VOLTAGE_FLAG: [Item; 1] = [
    Item { name: "BMU:Low battery voltage flag", pos: 60, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for BmuLowBatteryVoltageFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &BMU_LOW_BATTERY_VOLTAGE_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// BMU:On board charge command flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BmuOnBoardChargeCommandFlag(pub bool);

pub(crate) static BMU_ON_BOARD_CHARGE_COMMAND_FLAG: [Item; 1] = [
    Item { name: "BMU:On board charge command flag", pos: 36, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for BmuOnBoardChargeCommandFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &BMU_ON_BOARD_CHARGE_COMMAND_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// BMU:On board charge control flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BmuOnBoardChargeControlFlag(pub bool);

pub(crate) static BMU_ON_BOARD_CHARGE_CONTROL_FLAG: [Item; 1] = [
    Item { name: "BMU:On board charge control flag", pos: 36, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for BmuOnBoardChargeControlFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &BMU_ON_BOARD_CHARGE_CONTROL_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// BMU:Power supply keep REQ. flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BmuPowerSupplyKeepReqFlag(pub bool);

pub(crate) static BMU_POWER_SUPPLY_KEEP_REQ_FLAG: [Item; 1] = [
    Item { name: "BMU:Power supply keep REQ. flag", pos: 36, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for BmuPowerSupplyKeepReqFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &BMU_POWER_SUPPLY_KEEP_REQ_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// BMU:Power supply keep request
/// BMU power supply keep request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BmuPowerSupplyKeepRequest(pub bool);

pub(crate) static BMU_POWER_SUPPLY_KEEP_REQUEST: [Item; 2] = [
    Item { name: "BMU:Power supply keep request", pos: 49, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "BMU power supply keep request", pos: 8, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "Not Request"), (1, 1, "Request")]) },
];

impl crate::item::Reading for BmuPowerSupplyKeepRequest {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &BMU_POWER_SUPPLY_KEEP_REQUEST[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &BMU_POWER_SUPPLY_KEEP_REQUEST[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "Request" | "ON" => Some(Self(true)),
            "Not Request" | "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// BMU:Quick charge control flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BmuQuickChargeControlFlag(pub bool);

pub(crate) static BMU_QUICK_CHARGE_CONTROL_FLAG: [Item; 1] = [
    Item { name: "BMU:Quick charge control flag", pos: 35, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for BmuQuickChargeControlFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &BMU_QUICK_CHARGE_CONTROL_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// BMU:READY
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BmuReady(pub bool);

pub(crate) static BMU_READY: [Item; 2] = [
    Item { name: "BMU:READY", pos: 49, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "BMU:READY", pos: 8, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for BmuReady {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &BMU_READY[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &BMU_READY[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// BMU:Total battery voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BmuTotalBatteryVoltage(pub crate::db::units::Volts);

pub(crate) static BMU_TOTAL_BATTERY_VOLTAGE: [Item; 1] = [
    Item { name: "BMU:Total battery voltage", pos: 22, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 2.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for BmuTotalBatteryVoltage {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000, 0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &BMU_TOTAL_BATTERY_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// BMU:Vehicle abnormal flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BmuVehicleAbnormalFlag(pub bool);

pub(crate) static BMU_VEHICLE_ABNORMAL_FLAG: [Item; 1] = [
    Item { name: "BMU:Vehicle abnormal flag", pos: 60, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for BmuVehicleAbnormalFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &BMU_VEHICLE_ABNORMAL_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// BMU:Vehicle shift location
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BmuVehicleShiftLocation(pub bool);

pub(crate) static BMU_VEHICLE_SHIFT_LOCATION: [Item; 1] = [
    Item { name: "BMU:Vehicle shift location", pos: 60, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "P"), (1, 1, "Not P")]) },
];

impl crate::item::Reading for BmuVehicleShiftLocation {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &BMU_VEHICLE_SHIFT_LOCATION[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "Not P" => Some(Self(true)),
            "P" => Some(Self(false)),
            _ => None,
        }
    }
}

/// BMU:Voltage difference flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BmuVoltageDifferenceFlag(pub bool);

pub(crate) static BMU_VOLTAGE_DIFFERENCE_FLAG: [Item; 1] = [
    Item { name: "BMU:Voltage difference flag", pos: 60, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for BmuVoltageDifferenceFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &BMU_VOLTAGE_DIFFERENCE_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// BPS1 with or without flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bps1WithOrWithoutFlag(pub bool);

pub(crate) static BPS1_WITH_OR_WITHOUT_FLAG: [Item; 1] = [
    Item { name: "BPS1 with or without flag", pos: 3, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Bps1WithOrWithoutFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x25), item: &BPS1_WITH_OR_WITHOUT_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// BPSS 0 point learning off-set
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bpss0PointLearningOffSet(pub crate::db::units::Millimetres);

pub(crate) static BPSS0_POINT_LEARNING_OFF_SET: [Item; 1] = [
    Item { name: "BPSS 0 point learning off-set", pos: 2, len: 1, bits: None, unit: Unit::Millimetres, conv: Conv::Linear { factor: 0.2, offset: -25.6, decimals: 1 } },
];

impl crate::item::Reading for Bpss0PointLearningOffSet {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x25), item: &BPSS0_POINT_LEARNING_OFF_SET[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Millimetres(n)))
    }
}

/// Brake pedal stroke sensor
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BrakePedalStrokeSensor(pub crate::db::units::Volts);

pub(crate) static BRAKE_PEDAL_STROKE_SENSOR: [Item; 1] = [
    Item { name: "Brake pedal stroke sensor", pos: 21, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.02, offset: 0.0, decimals: 2 } },
];

impl crate::item::Reading for BrakePedalStrokeSensor {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &BRAKE_PEDAL_STROKE_SENSOR[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Brake press. sensor power supply
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BrakePressSensorPowerSupply(pub crate::db::units::Volts);

pub(crate) static BRAKE_PRESS_SENSOR_POWER_SUPPLY: [Item; 1] = [
    Item { name: "Brake press. sensor power supply", pos: 4, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.05, offset: 0.0, decimals: 2 } },
];

impl crate::item::Reading for BrakePressSensorPowerSupply {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x03), item: &BRAKE_PRESS_SENSOR_POWER_SUPPLY[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Brake press. sensor voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BrakePressSensorVoltage(pub crate::db::units::Volts);

pub(crate) static BRAKE_PRESS_SENSOR_VOLTAGE: [Item; 1] = [
    Item { name: "Brake press. sensor voltage", pos: 7, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.02, offset: 0.0, decimals: 2 } },
];

impl crate::item::Reading for BrakePressSensorVoltage {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x03), item: &BRAKE_PRESS_SENSOR_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Brake pressure sensor
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BrakePressureSensor(pub crate::db::units::Volts);

pub(crate) static BRAKE_PRESSURE_SENSOR: [Item; 1] = [
    Item { name: "Brake pressure sensor", pos: 32, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.02, offset: 0.0, decimals: 2 } },
];

impl crate::item::Reading for BrakePressureSensor {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100], source: Source::FreezeFrame, item: &BRAKE_PRESSURE_SENSOR[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Brake pressure sensor 1
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BrakePressureSensor1(pub crate::db::units::MegaPascals);

pub(crate) static BRAKE_PRESSURE_SENSOR1: [Item; 1] = [
    Item { name: "Brake pressure sensor 1", pos: 25, len: 1, bits: None, unit: Unit::MegaPascals, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for BrakePressureSensor1 {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &BRAKE_PRESSURE_SENSOR1[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::MegaPascals(n)))
    }
}

/// Brake pressure sensor(physical)
/// Brake pressure sensor(Physical)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BrakePressureSensorPhysical(pub crate::db::units::MegaPascals);

pub(crate) static BRAKE_PRESSURE_SENSOR_PHYSICAL: [Item; 2] = [
    Item { name: "Brake pressure sensor(physical)", pos: 25, len: 1, bits: None, unit: Unit::MegaPascals, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
    Item { name: "Brake pressure sensor(Physical)", pos: 15, len: 1, bits: None, unit: Unit::MegaPascals, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for BrakePressureSensorPhysical {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &BRAKE_PRESSURE_SENSOR_PHYSICAL[0] },
        At { versions: &[0x0200, 0x0300, 0x0400], source: Source::Record(0x03), item: &BRAKE_PRESSURE_SENSOR_PHYSICAL[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::MegaPascals(n)))
    }
}

/// Brake PRS. sensor power supply
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BrakePrsSensorPowerSupply(pub crate::db::units::Volts);

pub(crate) static BRAKE_PRS_SENSOR_POWER_SUPPLY: [Item; 1] = [
    Item { name: "Brake PRS. sensor power supply", pos: 29, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.05, offset: 0.0, decimals: 2 } },
];

impl crate::item::Reading for BrakePrsSensorPowerSupply {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &BRAKE_PRS_SENSOR_POWER_SUPPLY[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Brake sensor stroke value
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BrakeSensorStrokeValue(pub crate::db::units::Millimetres);

pub(crate) static BRAKE_SENSOR_STROKE_VALUE: [Item; 1] = [
    Item { name: "Brake sensor stroke value", pos: 14, len: 1, bits: None, unit: Unit::Millimetres, conv: Conv::Linear { factor: 1.0, offset: -128.0, decimals: 0 } },
];

impl crate::item::Reading for BrakeSensorStrokeValue {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x0004, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x03), item: &BRAKE_SENSOR_STROKE_VALUE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Millimetres(n)))
    }
}

/// Brake SW
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct BrakeSw(pub bool);

pub(crate) static BRAKE_SW: [Item; 3] = [
    Item { name: "Brake SW", pos: 28, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Brake SW", pos: 42, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Brake SW", pos: 2, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for BrakeSw {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &BRAKE_SW[0] },
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &BRAKE_SW[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x01), item: &BRAKE_SW[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Brake torque
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BrakeTorque(pub crate::db::units::NewtonMetres);

pub(crate) static BRAKE_TORQUE: [Item; 1] = [
    Item { name: "Brake torque", pos: 9, len: 1, bits: None, unit: Unit::NewtonMetres, conv: Conv::Linear { factor: 0.5, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for BrakeTorque {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x20), item: &BRAKE_TORQUE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::NewtonMetres(n)))
    }
}

/// Cell temperature high flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CellTemperatureHighFlag(pub bool);

pub(crate) static CELL_TEMPERATURE_HIGH_FLAG: [Item; 2] = [
    Item { name: "Cell temperature high flag", pos: 47, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Cell temperature high flag", pos: 5, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CellTemperatureHighFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &CELL_TEMPERATURE_HIGH_FLAG[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &CELL_TEMPERATURE_HIGH_FLAG[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Charge contactor
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChargeContactor(pub bool);

pub(crate) static CHARGE_CONTACTOR: [Item; 1] = [
    Item { name: "Charge contactor", pos: 30, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ChargeContactor {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &CHARGE_CONTACTOR[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Charge contactor (diag)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChargeContactorDiag(pub bool);

pub(crate) static CHARGE_CONTACTOR_DIAG: [Item; 3] = [
    Item { name: "Charge contactor (diag)", pos: 29, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Charge contactor (diag)", pos: 43, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Charge contactor (diag)", pos: 3, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ChargeContactorDiag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &CHARGE_CONTACTOR_DIAG[0] },
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &CHARGE_CONTACTOR_DIAG[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x01), item: &CHARGE_CONTACTOR_DIAG[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Charge contactor output
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChargeContactorOutput(pub bool);

pub(crate) static CHARGE_CONTACTOR_OUTPUT: [Item; 1] = [
    Item { name: "Charge contactor output", pos: 2, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ChargeContactorOutput {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x02), item: &CHARGE_CONTACTOR_OUTPUT[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Charge contactor weld flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChargeContactorWeldFlag(pub bool);

pub(crate) static CHARGE_CONTACTOR_WELD_FLAG: [Item; 1] = [
    Item { name: "Charge contactor weld flag", pos: 2, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ChargeContactorWeldFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &CHARGE_CONTACTOR_WELD_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Charge current judgment flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChargeCurrentJudgmentFlag(pub bool);

pub(crate) static CHARGE_CURRENT_JUDGMENT_FLAG: [Item; 1] = [
    Item { name: "Charge current judgment flag", pos: 34, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ChargeCurrentJudgmentFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &CHARGE_CURRENT_JUDGMENT_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Charge permission flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChargePermissionFlag(pub bool);

pub(crate) static CHARGE_PERMISSION_FLAG: [Item; 3] = [
    Item { name: "Charge permission flag", pos: 33, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Charge permission flag", pos: 47, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Charge permission flag", pos: 3, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ChargePermissionFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &CHARGE_PERMISSION_FLAG[0] },
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &CHARGE_PERMISSION_FLAG[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &CHARGE_PERMISSION_FLAG[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Charging A/C function
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChargingACFunction(pub bool);

pub(crate) static CHARGING_AC_FUNCTION: [Item; 1] = [
    Item { name: "Charging A/C function", pos: 2, len: 1, bits: None, unit: Unit::None, conv: Conv::Labels(&[(0, 0, "Disable"), (1, 1, "Enable")]) },
];

impl crate::item::Reading for ChargingACFunction {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x0400], source: Source::Record(0x28), item: &CHARGING_AC_FUNCTION[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "Enable" => Some(Self(true)),
            "Disable" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Charging A/C function customize
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChargingACFunctionCustomize(pub bool);

pub(crate) static CHARGING_AC_FUNCTION_CUSTOMIZE: [Item; 1] = [
    Item { name: "Charging A/C function customize", pos: 11, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "Not supported"), (1, 1, "Supported")]) },
];

impl crate::item::Reading for ChargingACFunctionCustomize {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x0400], source: Source::Record(0x24), item: &CHARGING_AC_FUNCTION_CUSTOMIZE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "Supported" => Some(Self(true)),
            "Not supported" => Some(Self(false)),
            _ => None,
        }
    }
}

/// COMP & HEATER power supply relay
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompHeaterPowerSupplyRelay(pub bool);

pub(crate) static COMP_HEATER_POWER_SUPPLY_RELAY: [Item; 1] = [
    Item { name: "COMP & HEATER power supply relay", pos: 31, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CompHeaterPowerSupplyRelay {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &COMP_HEATER_POWER_SUPPLY_RELAY[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// COMP&HTR:A/C power keep request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompHtrACPowerKeepRequest(pub bool);

pub(crate) static COMP_HTR_AC_POWER_KEEP_REQUEST: [Item; 1] = [
    Item { name: "COMP&HTR:A/C power keep request", pos: 35, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CompHtrACPowerKeepRequest {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &COMP_HTR_AC_POWER_KEEP_REQUEST[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// COMP&HTR:A/C supply keep request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompHtrACSupplyKeepRequest(pub bool);

pub(crate) static COMP_HTR_AC_SUPPLY_KEEP_REQUEST: [Item; 2] = [
    Item { name: "COMP&HTR:A/C supply keep request", pos: 48, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "COMP&HTR:A/C supply keep request", pos: 6, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CompHtrACSupplyKeepRequest {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &COMP_HTR_AC_SUPPLY_KEEP_REQUEST[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &COMP_HTR_AC_SUPPLY_KEEP_REQUEST[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// COMP&HTR:Pre A/C control
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CompHtrPreACControl(pub bool);

pub(crate) static COMP_HTR_PRE_AC_CONTROL: [Item; 1] = [
    Item { name: "COMP&HTR:Pre A/C control", pos: 50, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CompHtrPreACControl {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005], source: Source::FreezeFrame, item: &COMP_HTR_PRE_AC_CONTROL[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Condenser charge timeout flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CondenserChargeTimeoutFlag(pub bool);

pub(crate) static CONDENSER_CHARGE_TIMEOUT_FLAG: [Item; 1] = [
    Item { name: "Condenser charge timeout flag", pos: 3, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CondenserChargeTimeoutFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &CONDENSER_CHARGE_TIMEOUT_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Condenser discharge request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CondenserDischargeRequest(pub bool);

pub(crate) static CONDENSER_DISCHARGE_REQUEST: [Item; 1] = [
    Item { name: "Condenser discharge request", pos: 52, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CondenserDischargeRequest {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &CONDENSER_DISCHARGE_REQUEST[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Condenser discharge timeout flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CondenserDischargeTimeoutFlag(pub bool);

pub(crate) static CONDENSER_DISCHARGE_TIMEOUT_FLAG: [Item; 1] = [
    Item { name: "Condenser discharge timeout flag", pos: 3, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CondenserDischargeTimeoutFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &CONDENSER_DISCHARGE_TIMEOUT_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Condenser over volt protect flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CondenserOverVoltProtectFlag(pub bool);

pub(crate) static CONDENSER_OVER_VOLT_PROTECT_FLAG: [Item; 1] = [
    Item { name: "Condenser over volt protect flag", pos: 34, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CondenserOverVoltProtectFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &CONDENSER_OVER_VOLT_PROTECT_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Condenser voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CondenserVoltage(pub crate::db::units::Volts);

pub(crate) static CONDENSER_VOLTAGE: [Item; 1] = [
    Item { name: "Condenser voltage", pos: 4, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 2.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for CondenserVoltage {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x24), item: &CONDENSER_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Control power supply
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ControlPowerSupply(pub crate::db::units::Volts);

pub(crate) static CONTROL_POWER_SUPPLY: [Item; 3] = [
    Item { name: "Control power supply", pos: 33, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
    Item { name: "Control power supply", pos: 10, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.02, offset: 0.0, decimals: 2 } },
    Item { name: "Control power supply", pos: 12, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for ControlPowerSupply {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &CONTROL_POWER_SUPPLY[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x03), item: &CONTROL_POWER_SUPPLY[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x20), item: &CONTROL_POWER_SUPPLY[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Control power voltage low flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControlPowerVoltageLowFlag(pub bool);

pub(crate) static CONTROL_POWER_VOLTAGE_LOW_FLAG: [Item; 1] = [
    Item { name: "Control power voltage low flag", pos: 33, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ControlPowerVoltageLowFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &CONTROL_POWER_VOLTAGE_LOW_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Control power voltage low(P0562)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ControlPowerVoltageLowP0562(pub bool);

pub(crate) static CONTROL_POWER_VOLTAGE_LOW_P0562: [Item; 2] = [
    Item { name: "Control power voltage low(P0562)", pos: 47, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "Not Active"), (1, 1, "Active")]) },
    Item { name: "Control power voltage low(P0562)", pos: 4, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ControlPowerVoltageLowP0562 {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &CONTROL_POWER_VOLTAGE_LOW_P0562[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &CONTROL_POWER_VOLTAGE_LOW_P0562[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "Active" | "ON" => Some(Self(true)),
            "Not Active" | "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Coolant water temperature
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CoolantWaterTemperature(pub crate::db::units::Celsius);

pub(crate) static COOLANT_WATER_TEMPERATURE: [Item; 3] = [
    Item { name: "Coolant water temperature", pos: 27, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -40.0, decimals: 0 } },
    Item { name: "Coolant water temperature", pos: 35, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -40.0, decimals: 0 } },
    Item { name: "Coolant water temperature", pos: 6, len: 1, bits: None, unit: Unit::Celsius, conv: Conv::Linear { factor: 1.0, offset: -40.0, decimals: 0 } },
];

impl crate::item::Reading for CoolantWaterTemperature {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &COOLANT_WATER_TEMPERATURE[0] },
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &COOLANT_WATER_TEMPERATURE[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x24), item: &COOLANT_WATER_TEMPERATURE[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Celsius(n)))
    }
}

/// Crash blocking judgment flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CrashBlockingJudgmentFlag(pub bool);

pub(crate) static CRASH_BLOCKING_JUDGMENT_FLAG: [Item; 1] = [
    Item { name: "Crash blocking judgment flag", pos: 52, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for CrashBlockingJudgmentFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &CRASH_BLOCKING_JUDGMENT_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Creap torque
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct CreapTorque(pub crate::db::units::NewtonMetres);

pub(crate) static CREAP_TORQUE: [Item; 1] = [
    Item { name: "Creap torque", pos: 8, len: 1, bits: None, unit: Unit::NewtonMetres, conv: Conv::Linear { factor: 0.5, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for CreapTorque {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x20), item: &CREAP_TORQUE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::NewtonMetres(n)))
    }
}

/// 12V DC/DC converter shutdown
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DcDcConverterShutdown12v(pub bool);

pub(crate) static DC_DC_CONVERTER_SHUTDOWN12V: [Item; 3] = [
    Item { name: "12V DC/DC converter shutdown", pos: 31, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "ON"), (1, 1, "OFF")]) },
    Item { name: "12V DC/DC converter shutdown", pos: 45, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "ON"), (1, 1, "OFF")]) },
    Item { name: "12V DC/DC converter shutdown", pos: 2, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "ON"), (1, 1, "OFF")]) },
];

impl crate::item::Reading for DcDcConverterShutdown12v {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &DC_DC_CONVERTER_SHUTDOWN12V[0] },
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &DC_DC_CONVERTER_SHUTDOWN12V[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x02), item: &DC_DC_CONVERTER_SHUTDOWN12V[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// 12V DC/DC F/B signal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DcDcFBSignal12v(pub bool);

pub(crate) static DC_DC_FB_SIGNAL12V: [Item; 3] = [
    Item { name: "12V DC/DC F/B signal", pos: 28, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "12V DC/DC F/B signal", pos: 42, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "12V DC/DC F/B signal", pos: 2, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for DcDcFBSignal12v {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &DC_DC_FB_SIGNAL12V[0] },
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &DC_DC_FB_SIGNAL12V[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x01), item: &DC_DC_FB_SIGNAL12V[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// DIAG control terminal connection
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DiagControlTerminalConnection(pub bool);

pub(crate) static DIAG_CONTROL_TERMINAL_CONNECTION: [Item; 1] = [
    Item { name: "DIAG control terminal connection", pos: 3, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for DiagControlTerminalConnection {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x01), item: &DIAG_CONTROL_TERMINAL_CONNECTION[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Driving torque
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DrivingTorque(pub crate::db::units::NewtonMetres);

pub(crate) static DRIVING_TORQUE: [Item; 1] = [
    Item { name: "Driving torque", pos: 17, len: 1, bits: None, unit: Unit::NewtonMetres, conv: Conv::Linear { factor: 2.0, offset: -200.0, decimals: 0 } },
];

impl crate::item::Reading for DrivingTorque {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &DRIVING_TORQUE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::NewtonMetres(n)))
    }
}

/// Driving torque (control value)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DrivingTorqueControlValue(pub crate::db::units::NewtonMetres);

pub(crate) static DRIVING_TORQUE_CONTROL_VALUE: [Item; 1] = [
    Item { name: "Driving torque (control value)", pos: 17, len: 1, bits: None, unit: Unit::NewtonMetres, conv: Conv::Linear { factor: 2.0, offset: -200.0, decimals: 0 } },
];

impl crate::item::Reading for DrivingTorqueControlValue {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &DRIVING_TORQUE_CONTROL_VALUE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::NewtonMetres(n)))
    }
}

/// Elapsed time
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ElapsedTime(pub crate::db::units::Minutes);

pub(crate) static ELAPSED_TIME: [Item; 1] = [
    Item { name: "Elapsed time", pos: 10, len: 2, bits: None, unit: Unit::Minutes, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for ElapsedTime {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000, 0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &ELAPSED_TIME[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Minutes(n)))
    }
}

/// EV mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvMode(pub u32);

pub(crate) static EV_MODE: [Item; 1] = [
    Item { name: "EV mode", pos: 2, len: 1, bits: None, unit: Unit::None, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for EvMode {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x20), item: &EV_MODE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.count().map(|n| Self(n))
    }
}

/// EV sub mode1
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvSubMode1(pub u32);

pub(crate) static EV_SUB_MODE1: [Item; 1] = [
    Item { name: "EV sub mode1", pos: 3, len: 1, bits: None, unit: Unit::None, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for EvSubMode1 {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x20), item: &EV_SUB_MODE1[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.count().map(|n| Self(n))
    }
}

/// EV sub mode2
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EvSubMode2(pub u32);

pub(crate) static EV_SUB_MODE2: [Item; 1] = [
    Item { name: "EV sub mode2", pos: 5, len: 1, bits: None, unit: Unit::None, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for EvSubMode2 {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x20), item: &EV_SUB_MODE2[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.count().map(|n| Self(n))
    }
}

/// Gain(cell temperature)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GainCellTemperature(pub crate::db::units::Percent);

pub(crate) static GAIN_CELL_TEMPERATURE: [Item; 1] = [
    Item { name: "Gain(cell temperature)", pos: 7, len: 1, bits: None, unit: Unit::Percent, conv: Conv::Linear { factor: 0.5, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for GainCellTemperature {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x22), item: &GAIN_CELL_TEMPERATURE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Percent(n)))
    }
}

/// Gain(Highest cell temperature)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GainHighestCellTemperature(pub crate::db::units::Percent);

pub(crate) static GAIN_HIGHEST_CELL_TEMPERATURE: [Item; 1] = [
    Item { name: "Gain(Highest cell temperature)", pos: 4, len: 1, bits: None, unit: Unit::Percent, conv: Conv::Linear { factor: 0.5, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for GainHighestCellTemperature {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x22), item: &GAIN_HIGHEST_CELL_TEMPERATURE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Percent(n)))
    }
}

/// Gain(Highest cell voltage)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GainHighestCellVoltage(pub crate::db::units::Percent);

pub(crate) static GAIN_HIGHEST_CELL_VOLTAGE: [Item; 1] = [
    Item { name: "Gain(Highest cell voltage)", pos: 2, len: 1, bits: None, unit: Unit::Percent, conv: Conv::Linear { factor: 0.5, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for GainHighestCellVoltage {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x22), item: &GAIN_HIGHEST_CELL_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Percent(n)))
    }
}

/// Gain(IGBT temperature)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GainIgbtTemperature(pub crate::db::units::Percent);

pub(crate) static GAIN_IGBT_TEMPERATURE: [Item; 1] = [
    Item { name: "Gain(IGBT temperature)", pos: 6, len: 1, bits: None, unit: Unit::Percent, conv: Conv::Linear { factor: 0.5, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for GainIgbtTemperature {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x21), item: &GAIN_IGBT_TEMPERATURE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Percent(n)))
    }
}

/// Gain(Lowest cell voltage)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GainLowestCellVoltage(pub crate::db::units::Percent);

pub(crate) static GAIN_LOWEST_CELL_VOLTAGE: [Item; 1] = [
    Item { name: "Gain(Lowest cell voltage)", pos: 3, len: 1, bits: None, unit: Unit::Percent, conv: Conv::Linear { factor: 0.5, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for GainLowestCellVoltage {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x22), item: &GAIN_LOWEST_CELL_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Percent(n)))
    }
}

/// Gain(motor coil temperature)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GainMotorCoilTemperature(pub crate::db::units::Percent);

pub(crate) static GAIN_MOTOR_COIL_TEMPERATURE: [Item; 1] = [
    Item { name: "Gain(motor coil temperature)", pos: 5, len: 1, bits: None, unit: Unit::Percent, conv: Conv::Linear { factor: 0.5, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for GainMotorCoilTemperature {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x21), item: &GAIN_MOTOR_COIL_TEMPERATURE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Percent(n)))
    }
}

/// Gain(motor temperature)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GainMotorTemperature(pub crate::db::units::Percent);

pub(crate) static GAIN_MOTOR_TEMPERATURE: [Item; 1] = [
    Item { name: "Gain(motor temperature)", pos: 6, len: 1, bits: None, unit: Unit::Percent, conv: Conv::Linear { factor: 0.5, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for GainMotorTemperature {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x22), item: &GAIN_MOTOR_TEMPERATURE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Percent(n)))
    }
}

/// Gain(Shift)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GainShift(pub crate::db::units::Percent);

pub(crate) static GAIN_SHIFT: [Item; 1] = [
    Item { name: "Gain(Shift)", pos: 11, len: 1, bits: None, unit: Unit::Percent, conv: Conv::Linear { factor: 0.5, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for GainShift {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x20), item: &GAIN_SHIFT[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Percent(n)))
    }
}

/// Gain(speed)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GainSpeed(pub crate::db::units::Percent);

pub(crate) static GAIN_SPEED: [Item; 1] = [
    Item { name: "Gain(speed)", pos: 10, len: 1, bits: None, unit: Unit::Percent, conv: Conv::Linear { factor: 0.5, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for GainSpeed {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x20), item: &GAIN_SPEED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Percent(n)))
    }
}

/// Gain(total voltage)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct GainTotalVoltage(pub crate::db::units::Percent);

pub(crate) static GAIN_TOTAL_VOLTAGE: [Item; 1] = [
    Item { name: "Gain(total voltage)", pos: 5, len: 1, bits: None, unit: Unit::Percent, conv: Conv::Linear { factor: 0.5, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for GainTotalVoltage {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x22), item: &GAIN_TOTAL_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Percent(n)))
    }
}

/// Idle judgment flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdleJudgmentFlag(pub bool);

pub(crate) static IDLE_JUDGMENT_FLAG: [Item; 1] = [
    Item { name: "Idle judgment flag", pos: 2, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for IdleJudgmentFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &IDLE_JUDGMENT_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// IG OFF historical flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IgOffHistoricalFlag(pub bool);

pub(crate) static IG_OFF_HISTORICAL_FLAG: [Item; 1] = [
    Item { name: "IG OFF historical flag", pos: 32, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for IgOffHistoricalFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &IG_OFF_HISTORICAL_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// IG ST historical flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IgStHistoricalFlag(pub bool);

pub(crate) static IG_ST_HISTORICAL_FLAG: [Item; 1] = [
    Item { name: "IG ST historical flag", pos: 32, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for IgStHistoricalFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &IG_ST_HISTORICAL_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// IG ST signal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IgStSignal(pub bool);

pub(crate) static IG_ST_SIGNAL: [Item; 1] = [
    Item { name: "IG ST signal", pos: 28, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for IgStSignal {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &IG_ST_SIGNAL[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// IG SW signal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IgSwSignal(pub bool);

pub(crate) static IG_SW_SIGNAL: [Item; 1] = [
    Item { name: "IG SW signal", pos: 28, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for IgSwSignal {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &IG_SW_SIGNAL[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// IGBT abnormal temperature flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IgbtAbnormalTemperatureFlag(pub bool);

pub(crate) static IGBT_ABNORMAL_TEMPERATURE_FLAG: [Item; 1] = [
    Item { name: "IGBT abnormal temperature flag", pos: 51, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for IgbtAbnormalTemperatureFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &IGBT_ABNORMAL_TEMPERATURE_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// IGCT control power supply relay
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IgctControlPowerSupplyRelay(pub bool);

pub(crate) static IGCT_CONTROL_POWER_SUPPLY_RELAY: [Item; 1] = [
    Item { name: "IGCT control power supply relay", pos: 2, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for IgctControlPowerSupplyRelay {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x02), item: &IGCT_CONTROL_POWER_SUPPLY_RELAY[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Immobilizer Equipment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImmobilizerEquipment(pub bool);

pub(crate) static IMMOBILIZER_EQUIPMENT: [Item; 1] = [
    Item { name: "Immobilizer Equipment", pos: 7, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "Not present"), (1, 1, "Present")]) },
];

impl crate::item::Reading for ImmobilizerEquipment {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &IMMOBILIZER_EQUIPMENT[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "Present" => Some(Self(true)),
            "Not present" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Immobilizer verification accord
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImmobilizerVerificationAccord(pub bool);

pub(crate) static IMMOBILIZER_VERIFICATION_ACCORD: [Item; 1] = [
    Item { name: "Immobilizer verification accord", pos: 52, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "NG"), (1, 1, "OK")]) },
];

impl crate::item::Reading for ImmobilizerVerificationAccord {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &IMMOBILIZER_VERIFICATION_ACCORD[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "OK" => Some(Self(true)),
            "NG" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Immobilizer/VIN verification
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ImmobilizerVinVerification(pub bool);

pub(crate) static IMMOBILIZER_VIN_VERIFICATION: [Item; 1] = [
    Item { name: "Immobilizer/VIN verification", pos: 51, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "NG"), (1, 1, "OK")]) },
];

impl crate::item::Reading for ImmobilizerVinVerification {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &IMMOBILIZER_VIN_VERIFICATION[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "OK" => Some(Self(true)),
            "NG" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Inhibitor shift position D
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InhibitorShiftPositionD(pub bool);

pub(crate) static INHIBITOR_SHIFT_POSITION_D: [Item; 1] = [
    Item { name: "Inhibitor shift position D", pos: 6, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for InhibitorShiftPositionD {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x01), item: &INHIBITOR_SHIFT_POSITION_D[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Inhibitor shift position N
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InhibitorShiftPositionN(pub bool);

pub(crate) static INHIBITOR_SHIFT_POSITION_N: [Item; 1] = [
    Item { name: "Inhibitor shift position N", pos: 6, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for InhibitorShiftPositionN {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x01), item: &INHIBITOR_SHIFT_POSITION_N[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Inhibitor shift position P
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InhibitorShiftPositionP(pub bool);

pub(crate) static INHIBITOR_SHIFT_POSITION_P: [Item; 1] = [
    Item { name: "Inhibitor shift position P", pos: 5, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for InhibitorShiftPositionP {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x01), item: &INHIBITOR_SHIFT_POSITION_P[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Inhibitor shift position R
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InhibitorShiftPositionR(pub bool);

pub(crate) static INHIBITOR_SHIFT_POSITION_R: [Item; 1] = [
    Item { name: "Inhibitor shift position R", pos: 5, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for InhibitorShiftPositionR {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x01), item: &INHIBITOR_SHIFT_POSITION_R[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Inhibitor SW :B1
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InhibitorSwB1(pub bool);

pub(crate) static INHIBITOR_SW_B1: [Item; 1] = [
    Item { name: "Inhibitor SW :B1", pos: 51, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for InhibitorSwB1 {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001], source: Source::FreezeFrame, item: &INHIBITOR_SW_B1[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Inhibitor SW :B2
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InhibitorSwB2(pub bool);

pub(crate) static INHIBITOR_SW_B2: [Item; 1] = [
    Item { name: "Inhibitor SW :B2", pos: 51, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for InhibitorSwB2 {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001], source: Source::FreezeFrame, item: &INHIBITOR_SW_B2[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Inhibitor SW :B(JPN)/C(EU)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InhibitorSwBJpnCEu(pub bool);

pub(crate) static INHIBITOR_SW_B_JPN_C_EU: [Item; 2] = [
    Item { name: "Inhibitor SW :B(JPN)/C(EU)", pos: 51, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Inhibitor SW :B(JPN)/C(EU)", pos: 6, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for InhibitorSwBJpnCEu {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005], source: Source::FreezeFrame, item: &INHIBITOR_SW_B_JPN_C_EU[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005], source: Source::Record(0x01), item: &INHIBITOR_SW_B_JPN_C_EU[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Inhibitor SW :D
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InhibitorSwD(pub bool);

pub(crate) static INHIBITOR_SW_D: [Item; 1] = [
    Item { name: "Inhibitor SW :D", pos: 51, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for InhibitorSwD {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &INHIBITOR_SW_D[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Inhibitor SW :E(JPN)/B(EU)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InhibitorSwEJpnBEu(pub bool);

pub(crate) static INHIBITOR_SW_E_JPN_B_EU: [Item; 2] = [
    Item { name: "Inhibitor SW :E(JPN)/B(EU)", pos: 51, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Inhibitor SW :E(JPN)/B(EU)", pos: 6, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for InhibitorSwEJpnBEu {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005], source: Source::FreezeFrame, item: &INHIBITOR_SW_E_JPN_B_EU[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005], source: Source::Record(0x01), item: &INHIBITOR_SW_E_JPN_B_EU[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Inhibitor SW :N
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InhibitorSwN(pub bool);

pub(crate) static INHIBITOR_SW_N: [Item; 1] = [
    Item { name: "Inhibitor SW :N", pos: 51, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for InhibitorSwN {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &INHIBITOR_SW_N[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Inhibitor SW :P
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InhibitorSwP(pub bool);

pub(crate) static INHIBITOR_SW_P: [Item; 1] = [
    Item { name: "Inhibitor SW :P", pos: 51, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for InhibitorSwP {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &INHIBITOR_SW_P[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Inhibitor SW :R
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InhibitorSwR(pub bool);

pub(crate) static INHIBITOR_SW_R: [Item; 1] = [
    Item { name: "Inhibitor SW :R", pos: 51, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for InhibitorSwR {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &INHIBITOR_SW_R[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Interlock SW
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InterlockSw(pub bool);

pub(crate) static INTERLOCK_SW: [Item; 3] = [
    Item { name: "Interlock SW", pos: 28, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Interlock SW", pos: 42, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Interlock SW", pos: 2, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for InterlockSw {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &INTERLOCK_SW[0] },
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &INTERLOCK_SW[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x01), item: &INTERLOCK_SW[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Limp-home change flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LimpHomeChangeFlag(pub bool);

pub(crate) static LIMP_HOME_CHANGE_FLAG: [Item; 1] = [
    Item { name: "Limp-home change flag", pos: 6, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for LimpHomeChangeFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &LIMP_HOME_CHANGE_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Main contactor [-]
/// Main contactor(-)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MainContactorMinus(pub bool);

pub(crate) static MAIN_CONTACTOR_MINUS: [Item; 3] = [
    Item { name: "Main contactor [-]", pos: 31, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Main contactor [-]", pos: 45, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Main contactor(-)", pos: 2, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for MainContactorMinus {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &MAIN_CONTACTOR_MINUS[0] },
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &MAIN_CONTACTOR_MINUS[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x02), item: &MAIN_CONTACTOR_MINUS[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Main contactor[-] (diag)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MainContactorMinusDiag(pub bool);

pub(crate) static MAIN_CONTACTOR_MINUS_DIAG: [Item; 2] = [
    Item { name: "Main contactor[-] (diag)", pos: 43, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Main contactor[-] (diag)", pos: 3, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for MainContactorMinusDiag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &MAIN_CONTACTOR_MINUS_DIAG[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x01), item: &MAIN_CONTACTOR_MINUS_DIAG[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Main contactor [-] (monitor)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MainContactorMinusMonitor(pub bool);

pub(crate) static MAIN_CONTACTOR_MINUS_MONITOR: [Item; 1] = [
    Item { name: "Main contactor [-] (monitor)", pos: 29, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for MainContactorMinusMonitor {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &MAIN_CONTACTOR_MINUS_MONITOR[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Main contactor (-) weld flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MainContactorMinusWeldFlag(pub bool);

pub(crate) static MAIN_CONTACTOR_MINUS_WELD_FLAG: [Item; 1] = [
    Item { name: "Main contactor (-) weld flag", pos: 3, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for MainContactorMinusWeldFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &MAIN_CONTACTOR_MINUS_WELD_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Main contactor [+]
/// Main contactor(+)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MainContactorPlus(pub bool);

pub(crate) static MAIN_CONTACTOR_PLUS: [Item; 3] = [
    Item { name: "Main contactor [+]", pos: 30, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Main contactor [+]", pos: 44, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Main contactor(+)", pos: 2, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for MainContactorPlus {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &MAIN_CONTACTOR_PLUS[0] },
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &MAIN_CONTACTOR_PLUS[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x02), item: &MAIN_CONTACTOR_PLUS[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Main contactor[+] (diag)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MainContactorPlusDiag(pub bool);

pub(crate) static MAIN_CONTACTOR_PLUS_DIAG: [Item; 2] = [
    Item { name: "Main contactor[+] (diag)", pos: 43, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Main contactor[+] (diag)", pos: 3, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for MainContactorPlusDiag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &MAIN_CONTACTOR_PLUS_DIAG[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x01), item: &MAIN_CONTACTOR_PLUS_DIAG[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Main contactor [+] (monitor)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MainContactorPlusMonitor(pub bool);

pub(crate) static MAIN_CONTACTOR_PLUS_MONITOR: [Item; 1] = [
    Item { name: "Main contactor [+] (monitor)", pos: 29, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for MainContactorPlusMonitor {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &MAIN_CONTACTOR_PLUS_MONITOR[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Main contactor (+) weld flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MainContactorPlusWeldFlag(pub bool);

pub(crate) static MAIN_CONTACTOR_PLUS_WELD_FLAG: [Item; 1] = [
    Item { name: "Main contactor (+) weld flag", pos: 3, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for MainContactorPlusWeldFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &MAIN_CONTACTOR_PLUS_WELD_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Main contactor (pre-charge)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MainContactorPreCharge(pub bool);

pub(crate) static MAIN_CONTACTOR_PRE_CHARGE: [Item; 1] = [
    Item { name: "Main contactor (pre-charge)", pos: 44, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for MainContactorPreCharge {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &MAIN_CONTACTOR_PRE_CHARGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// MAX cell temp. protection flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaxCellTempProtectionFlag(pub bool);

pub(crate) static MAX_CELL_TEMP_PROTECTION_FLAG: [Item; 1] = [
    Item { name: "MAX cell temp. protection flag", pos: 33, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for MaxCellTempProtectionFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &MAX_CELL_TEMP_PROTECTION_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// MAX cell voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaxCellVoltage(pub crate::db::units::Volts);

pub(crate) static MAX_CELL_VOLTAGE: [Item; 3] = [
    Item { name: "MAX cell voltage", pos: 36, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.01, offset: 2.3, decimals: 2 } },
    Item { name: "MAX cell voltage", pos: 36, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.01, offset: 2.1, decimals: 2 } },
    Item { name: "MAX cell voltage", pos: 36, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.02, offset: 0.0, decimals: 2 } },
];

impl crate::item::Reading for MaxCellVoltage {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003], source: Source::FreezeFrame, item: &MAX_CELL_VOLTAGE[0] },
        At { versions: &[0x0004, 0x0005, 0x0100], source: Source::FreezeFrame, item: &MAX_CELL_VOLTAGE[1] },
        At { versions: &[0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &MAX_CELL_VOLTAGE[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// MAX cell voltage protection flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MaxCellVoltageProtectionFlag(pub bool);

pub(crate) static MAX_CELL_VOLTAGE_PROTECTION_FLAG: [Item; 3] = [
    Item { name: "MAX cell voltage protection flag", pos: 33, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "MAX cell voltage protection flag", pos: 47, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "MAX cell voltage protection flag", pos: 4, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for MaxCellVoltageProtectionFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &MAX_CELL_VOLTAGE_PROTECTION_FLAG[0] },
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &MAX_CELL_VOLTAGE_PROTECTION_FLAG[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &MAX_CELL_VOLTAGE_PROTECTION_FLAG[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Maximum in put battery torque
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaximumInPutBatteryTorque(pub crate::db::units::NewtonMetres);

pub(crate) static MAXIMUM_IN_PUT_BATTERY_TORQUE: [Item; 1] = [
    Item { name: "Maximum in put battery torque", pos: 3, len: 1, bits: None, unit: Unit::NewtonMetres, conv: Conv::Linear { factor: 2.0, offset: -200.0, decimals: 0 } },
];

impl crate::item::Reading for MaximumInPutBatteryTorque {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x21), item: &MAXIMUM_IN_PUT_BATTERY_TORQUE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::NewtonMetres(n)))
    }
}

/// Maximum out put battery torque
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaximumOutPutBatteryTorque(pub crate::db::units::NewtonMetres);

pub(crate) static MAXIMUM_OUT_PUT_BATTERY_TORQUE: [Item; 1] = [
    Item { name: "Maximum out put battery torque", pos: 4, len: 1, bits: None, unit: Unit::NewtonMetres, conv: Conv::Linear { factor: 2.0, offset: -200.0, decimals: 0 } },
];

impl crate::item::Reading for MaximumOutPutBatteryTorque {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x21), item: &MAXIMUM_OUT_PUT_BATTERY_TORQUE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::NewtonMetres(n)))
    }
}

/// MCU:Condenser voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct McuCondenserVoltage(pub crate::db::units::Volts);

pub(crate) static MCU_CONDENSER_VOLTAGE: [Item; 1] = [
    Item { name: "MCU:Condenser voltage", pos: 23, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 2.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for McuCondenserVoltage {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000, 0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &MCU_CONDENSER_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// MCU:Discharge request (F/B)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct McuDischargeRequestFB(pub bool);

pub(crate) static MCU_DISCHARGE_REQUEST_FB: [Item; 1] = [
    Item { name: "MCU:Discharge request (F/B)", pos: 48, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for McuDischargeRequestFB {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &MCU_DISCHARGE_REQUEST_FB[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// MCU:Motor speed
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct McuMotorSpeed(pub crate::db::units::Rpm);

pub(crate) static MCU_MOTOR_SPEED: [Item; 1] = [
    Item { name: "MCU:Motor speed", pos: 18, len: 2, bits: None, unit: Unit::Rpm, conv: Conv::Linear { factor: 1.0, offset: -10000.0, decimals: 0 } },
];

impl crate::item::Reading for McuMotorSpeed {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &MCU_MOTOR_SPEED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Rpm(n)))
    }
}

/// MCU:Motor torque inhibition flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct McuMotorTorqueInhibitionFlag(pub bool);

pub(crate) static MCU_MOTOR_TORQUE_INHIBITION_FLAG: [Item; 1] = [
    Item { name: "MCU:Motor torque inhibition flag", pos: 35, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for McuMotorTorqueInhibitionFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &MCU_MOTOR_TORQUE_INHIBITION_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// MCU:Power supply keep REQ.flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct McuPowerSupplyKeepReqFlag(pub bool);

pub(crate) static MCU_POWER_SUPPLY_KEEP_REQ_FLAG: [Item; 1] = [
    Item { name: "MCU:Power supply keep REQ.flag", pos: 35, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for McuPowerSupplyKeepReqFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &MCU_POWER_SUPPLY_KEEP_REQ_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// MCU:Power supply keep request
/// MCU power supply keep request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct McuPowerSupplyKeepRequest(pub bool);

pub(crate) static MCU_POWER_SUPPLY_KEEP_REQUEST: [Item; 2] = [
    Item { name: "MCU:Power supply keep request", pos: 49, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "MCU power supply keep request", pos: 7, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "Not Request"), (1, 1, "Request")]) },
];

impl crate::item::Reading for McuPowerSupplyKeepRequest {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &MCU_POWER_SUPPLY_KEEP_REQUEST[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &MCU_POWER_SUPPLY_KEEP_REQUEST[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "Request" | "ON" => Some(Self(true)),
            "Not Request" | "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// MCU:READY
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct McuReady(pub bool);

pub(crate) static MCU_READY: [Item; 3] = [
    Item { name: "MCU:READY", pos: 35, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "MCU:READY", pos: 48, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "MCU:READY", pos: 6, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for McuReady {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &MCU_READY[0] },
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &MCU_READY[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &MCU_READY[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// MCU_Ready timeout flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct McuReadyTimeoutFlag(pub bool);

pub(crate) static MCU_READY_TIMEOUT_FLAG: [Item; 1] = [
    Item { name: "MCU_Ready timeout flag", pos: 3, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for McuReadyTimeoutFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &MCU_READY_TIMEOUT_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// MCU shutdown
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct McuShutdown(pub bool);

pub(crate) static MCU_SHUTDOWN: [Item; 1] = [
    Item { name: "MCU shutdown", pos: 7, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for McuShutdown {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &MCU_SHUTDOWN[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// MCU shutdown F/B
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct McuShutdownFB(pub bool);

pub(crate) static MCU_SHUTDOWN_FB: [Item; 1] = [
    Item { name: "MCU shutdown F/B", pos: 7, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for McuShutdownFB {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &MCU_SHUTDOWN_FB[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// MCU:Shutdown (F/B) flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct McuShutdownFBFlag(pub bool);

pub(crate) static MCU_SHUTDOWN_FB_FLAG: [Item; 1] = [
    Item { name: "MCU:Shutdown (F/B) flag", pos: 35, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for McuShutdownFBFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &MCU_SHUTDOWN_FB_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// MCU:Shutdown flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct McuShutdownFlag(pub bool);

pub(crate) static MCU_SHUTDOWN_FLAG: [Item; 1] = [
    Item { name: "MCU:Shutdown flag", pos: 35, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for McuShutdownFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &MCU_SHUTDOWN_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// MCU shutdown permission flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct McuShutdownPermissionFlag(pub bool);

pub(crate) static MCU_SHUTDOWN_PERMISSION_FLAG: [Item; 1] = [
    Item { name: "MCU shutdown permission flag", pos: 2, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for McuShutdownPermissionFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &MCU_SHUTDOWN_PERMISSION_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// MCU:Shutdown request (F/B)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct McuShutdownRequestFB(pub bool);

pub(crate) static MCU_SHUTDOWN_REQUEST_FB: [Item; 1] = [
    Item { name: "MCU:Shutdown request (F/B)", pos: 49, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for McuShutdownRequestFB {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &MCU_SHUTDOWN_REQUEST_FB[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// MIN cell voltage protection flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MinCellVoltageProtectionFlag(pub bool);

pub(crate) static MIN_CELL_VOLTAGE_PROTECTION_FLAG: [Item; 1] = [
    Item { name: "MIN cell voltage protection flag", pos: 34, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for MinCellVoltageProtectionFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &MIN_CELL_VOLTAGE_PROTECTION_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Minimum cell voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MinimumCellVoltage(pub crate::db::units::Volts);

pub(crate) static MINIMUM_CELL_VOLTAGE: [Item; 3] = [
    Item { name: "Minimum cell voltage", pos: 37, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.01, offset: 2.3, decimals: 2 } },
    Item { name: "Minimum cell voltage", pos: 37, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.01, offset: 2.1, decimals: 2 } },
    Item { name: "Minimum cell voltage", pos: 37, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.02, offset: 0.0, decimals: 2 } },
];

impl crate::item::Reading for MinimumCellVoltage {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003], source: Source::FreezeFrame, item: &MINIMUM_CELL_VOLTAGE[0] },
        At { versions: &[0x0004, 0x0005, 0x0100], source: Source::FreezeFrame, item: &MINIMUM_CELL_VOLTAGE[1] },
        At { versions: &[0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &MINIMUM_CELL_VOLTAGE[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Mode transition priority
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ModeTransitionPriority(pub u32);

pub(crate) static MODE_TRANSITION_PRIORITY: [Item; 2] = [
    Item { name: "Mode transition priority", pos: 16, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
    Item { name: "Mode transition priority", pos: 16, len: 1, bits: None, unit: Unit::None, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for ModeTransitionPriority {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &MODE_TRANSITION_PRIORITY[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.count().map(|n| Self(n))
    }
}

/// Motor inverter shutdown
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MotorInverterShutdown(pub bool);

pub(crate) static MOTOR_INVERTER_SHUTDOWN: [Item; 3] = [
    Item { name: "Motor inverter shutdown", pos: 31, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "ON"), (1, 1, "OFF")]) },
    Item { name: "Motor inverter shutdown", pos: 45, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "ON"), (1, 1, "OFF")]) },
    Item { name: "Motor inverter shutdown", pos: 2, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "ON"), (1, 1, "OFF")]) },
];

impl crate::item::Reading for MotorInverterShutdown {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &MOTOR_INVERTER_SHUTDOWN[0] },
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &MOTOR_INVERTER_SHUTDOWN[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x02), item: &MOTOR_INVERTER_SHUTDOWN[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Motor lock determination flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MotorLockDeterminationFlag(pub bool);

pub(crate) static MOTOR_LOCK_DETERMINATION_FLAG: [Item; 1] = [
    Item { name: "Motor lock determination flag", pos: 34, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for MotorLockDeterminationFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &MOTOR_LOCK_DETERMINATION_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Motor lock judgment flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MotorLockJudgmentFlag(pub bool);

pub(crate) static MOTOR_LOCK_JUDGMENT_FLAG: [Item; 2] = [
    Item { name: "Motor lock judgment flag", pos: 48, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Motor lock judgment flag", pos: 6, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for MotorLockJudgmentFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &MOTOR_LOCK_JUDGMENT_FLAG[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &MOTOR_LOCK_JUDGMENT_FLAG[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Motor RPM
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MotorRpm(pub crate::db::units::Rpm);

pub(crate) static MOTOR_RPM: [Item; 1] = [
    Item { name: "Motor RPM", pos: 18, len: 2, bits: None, unit: Unit::Rpm, conv: Conv::Linear { factor: 1.0, offset: -10000.0, decimals: 0 } },
];

impl crate::item::Reading for MotorRpm {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &MOTOR_RPM[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Rpm(n)))
    }
}

/// Motor speed
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MotorSpeed(pub crate::db::units::Rpm);

pub(crate) static MOTOR_SPEED: [Item; 1] = [
    Item { name: "Motor speed", pos: 8, len: 2, bits: None, unit: Unit::Rpm, conv: Conv::Linear { factor: 1.0, offset: -10000.0, decimals: 0 } },
];

impl crate::item::Reading for MotorSpeed {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x22), item: &MOTOR_SPEED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Rpm(n)))
    }
}

/// Motor system heatup
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MotorSystemHeatup(pub bool);

pub(crate) static MOTOR_SYSTEM_HEATUP: [Item; 1] = [
    Item { name: "Motor system heatup", pos: 5, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for MotorSystemHeatup {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &MOTOR_SYSTEM_HEATUP[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Motor temperature abnormal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MotorTemperatureAbnormal(pub bool);

pub(crate) static MOTOR_TEMPERATURE_ABNORMAL: [Item; 1] = [
    Item { name: "Motor temperature abnormal", pos: 7, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "Normal"), (1, 1, "Abnormal")]) },
];

impl crate::item::Reading for MotorTemperatureAbnormal {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &MOTOR_TEMPERATURE_ABNORMAL[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "Abnormal" => Some(Self(true)),
            "Normal" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Motor torque F/B
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MotorTorqueFB(pub crate::db::units::NewtonMetres);

pub(crate) static MOTOR_TORQUE_FB: [Item; 1] = [
    Item { name: "Motor torque F/B", pos: 69, len: 2, bits: None, unit: Unit::NewtonMetres, conv: Conv::Linear { factor: 0.1, offset: -200.0, decimals: 1 } },
];

impl crate::item::Reading for MotorTorqueFB {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &MOTOR_TORQUE_FB[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::NewtonMetres(n)))
    }
}

/// Negative pressure sensor
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NegativePressureSensor(pub crate::db::units::Volts);

pub(crate) static NEGATIVE_PRESSURE_SENSOR: [Item; 1] = [
    Item { name: "Negative pressure sensor", pos: 26, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.02, offset: 0.0, decimals: 2 } },
];

impl crate::item::Reading for NegativePressureSensor {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &NEGATIVE_PRESSURE_SENSOR[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// OBC:A/C power judgement flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObcACPowerJudgementFlag(pub bool);

pub(crate) static OBC_AC_POWER_JUDGEMENT_FLAG: [Item; 1] = [
    Item { name: "OBC:A/C power judgement flag", pos: 36, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ObcACPowerJudgementFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &OBC_AC_POWER_JUDGEMENT_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// OBC:CHG. power keep request flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObcChgPowerKeepRequestFlag(pub bool);

pub(crate) static OBC_CHG_POWER_KEEP_REQUEST_FLAG: [Item; 1] = [
    Item { name: "OBC:CHG. power keep request flag", pos: 36, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ObcChgPowerKeepRequestFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &OBC_CHG_POWER_KEEP_REQUEST_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// OBC connecting signal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObcConnectingSignal {
    /// No Signal
    NoSignal,
    /// Release ON
    ReleaseOn,
    /// Release OFF
    ReleaseOff,
    /// Undefined
    Undefined,
}

pub(crate) static OBC_CONNECTING_SIGNAL: [Item; 1] = [
    Item { name: "OBC connecting signal", pos: 9, len: 1, bits: None, unit: Unit::None, conv: Conv::Labels(&[(0, 0, "No Signal"), (1, 1, "Release ON"), (2, 2, "Release OFF"), (3, 255, "Undefined")]) },
];

impl crate::item::Reading for ObcConnectingSignal {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x24), item: &OBC_CONNECTING_SIGNAL[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "No Signal" => Some(Self::NoSignal),
            "Release ON" => Some(Self::ReleaseOn),
            "Release OFF" => Some(Self::ReleaseOff),
            "Undefined" => Some(Self::Undefined),
            _ => None,
        }
    }
}

/// OBC connecting signal voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObcConnectingSignalVoltage(pub crate::db::units::Volts);

pub(crate) static OBC_CONNECTING_SIGNAL_VOLTAGE: [Item; 1] = [
    Item { name: "OBC connecting signal voltage", pos: 8, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.02, offset: 0.0, decimals: 2 } },
];

impl crate::item::Reading for ObcConnectingSignalVoltage {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x24), item: &OBC_CONNECTING_SIGNAL_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// OBC connection judgment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObcConnectionJudgment(pub bool);

pub(crate) static OBC_CONNECTION_JUDGMENT: [Item; 1] = [
    Item { name: "OBC connection judgment", pos: 4, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ObcConnectionJudgment {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &OBC_CONNECTION_JUDGMENT[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// OBC connector joint signal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObcConnectorJointSignal {
    /// No Signal
    NoSignal,
    /// Release ON
    ReleaseOn,
    /// Release OFF
    ReleaseOff,
    /// Undefined
    Undefined,
}

pub(crate) static OBC_CONNECTOR_JOINT_SIGNAL: [Item; 1] = [
    Item { name: "OBC connector joint signal", pos: 76, len: 1, bits: None, unit: Unit::None, conv: Conv::Labels(&[(0, 0, "No Signal"), (1, 1, "Release ON"), (2, 2, "Release OFF"), (3, 255, "Undefined")]) },
];

impl crate::item::Reading for ObcConnectorJointSignal {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &OBC_CONNECTOR_JOINT_SIGNAL[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "No Signal" => Some(Self::NoSignal),
            "Release ON" => Some(Self::ReleaseOn),
            "Release OFF" => Some(Self::ReleaseOff),
            "Undefined" => Some(Self::Undefined),
            _ => None,
        }
    }
}

/// OBC connector joint signal volt.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ObcConnectorJointSignalVolt(pub crate::db::units::Volts);

pub(crate) static OBC_CONNECTOR_JOINT_SIGNAL_VOLT: [Item; 1] = [
    Item { name: "OBC connector joint signal volt.", pos: 75, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.02, offset: 0.0, decimals: 2 } },
];

impl crate::item::Reading for ObcConnectorJointSignalVolt {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &OBC_CONNECTOR_JOINT_SIGNAL_VOLT[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// OBC control status flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObcControlStatusFlag(pub bool);

pub(crate) static OBC_CONTROL_STATUS_FLAG: [Item; 1] = [
    Item { name: "OBC control status flag", pos: 5, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ObcControlStatusFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &OBC_CONTROL_STATUS_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// OBC:Cooling request flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObcCoolingRequestFlag(pub bool);

pub(crate) static OBC_COOLING_REQUEST_FLAG: [Item; 1] = [
    Item { name: "OBC:Cooling request flag", pos: 36, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ObcCoolingRequestFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &OBC_COOLING_REQUEST_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// OBC:OBC cooling request flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObcObcCoolingRequestFlag(pub bool);

pub(crate) static OBC_OBC_COOLING_REQUEST_FLAG: [Item; 1] = [
    Item { name: "OBC:OBC cooling request flag", pos: 49, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ObcObcCoolingRequestFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &OBC_OBC_COOLING_REQUEST_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// OBC:On board charging flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObcOnBoardChargingFlag(pub bool);

pub(crate) static OBC_ON_BOARD_CHARGING_FLAG: [Item; 2] = [
    Item { name: "OBC:On board charging flag", pos: 36, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "OBC:On board charging flag", pos: 49, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ObcOnBoardChargingFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &OBC_ON_BOARD_CHARGING_FLAG[0] },
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &OBC_ON_BOARD_CHARGING_FLAG[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// OBC pilot signal control
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObcPilotSignalControl(pub bool);

pub(crate) static OBC_PILOT_SIGNAL_CONTROL: [Item; 1] = [
    Item { name: "OBC pilot signal control", pos: 11, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "Not Request"), (1, 1, "Request")]) },
];

impl crate::item::Reading for ObcPilotSignalControl {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x24), item: &OBC_PILOT_SIGNAL_CONTROL[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "Request" => Some(Self(true)),
            "Not Request" => Some(Self(false)),
            _ => None,
        }
    }
}

/// OBC:Power supply keep request
/// OBC power supply keep request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObcPowerSupplyKeepRequest(pub bool);

pub(crate) static OBC_POWER_SUPPLY_KEEP_REQUEST: [Item; 2] = [
    Item { name: "OBC:Power supply keep request", pos: 50, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "OBC power supply keep request", pos: 8, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "Not Request"), (1, 1, "Request")]) },
];

impl crate::item::Reading for ObcPowerSupplyKeepRequest {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &OBC_POWER_SUPPLY_KEEP_REQUEST[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &OBC_POWER_SUPPLY_KEEP_REQUEST[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "Request" | "ON" => Some(Self(true)),
            "Not Request" | "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// OBC power supply relay
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObcPowerSupplyRelay(pub bool);

pub(crate) static OBC_POWER_SUPPLY_RELAY: [Item; 3] = [
    Item { name: "OBC power supply relay", pos: 31, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "OBC power supply relay", pos: 45, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "OBC power supply relay", pos: 3, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ObcPowerSupplyRelay {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &OBC_POWER_SUPPLY_RELAY[0] },
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &OBC_POWER_SUPPLY_RELAY[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x02), item: &OBC_POWER_SUPPLY_RELAY[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// OBC power supply relay (diag)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObcPowerSupplyRelayDiag(pub bool);

pub(crate) static OBC_POWER_SUPPLY_RELAY_DIAG: [Item; 2] = [
    Item { name: "OBC power supply relay (diag)", pos: 43, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "OBC power supply relay (diag)", pos: 3, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ObcPowerSupplyRelayDiag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &OBC_POWER_SUPPLY_RELAY_DIAG[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x01), item: &OBC_POWER_SUPPLY_RELAY_DIAG[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// OBC power supply relay (monitor)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObcPowerSupplyRelayMonitor(pub bool);

pub(crate) static OBC_POWER_SUPPLY_RELAY_MONITOR: [Item; 1] = [
    Item { name: "OBC power supply relay (monitor)", pos: 29, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ObcPowerSupplyRelayMonitor {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &OBC_POWER_SUPPLY_RELAY_MONITOR[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// OBC variation information
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObcVariationInformation(pub u32);

pub(crate) static OBC_VARIATION_INFORMATION: [Item; 1] = [
    Item { name: "OBC variation information", pos: 10, len: 1, bits: None, unit: Unit::None, conv: Conv::Raw },
];

impl crate::item::Reading for ObcVariationInformation {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x24), item: &OBC_VARIATION_INFORMATION[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.raw().map(|n| Self(n))
    }
}

/// Odometer
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Odometer(pub crate::db::units::Km);

pub(crate) static ODOMETER: [Item; 1] = [
    Item { name: "Odometer", pos: 5, len: 2, bits: None, unit: Unit::Km, conv: Conv::Linear { factor: 6.4, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for Odometer {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000, 0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &ODOMETER[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Km(n)))
    }
}

/// On board charge complete flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OnBoardChargeCompleteFlag(pub bool);

pub(crate) static ON_BOARD_CHARGE_COMPLETE_FLAG: [Item; 1] = [
    Item { name: "On board charge complete flag", pos: 33, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for OnBoardChargeCompleteFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &ON_BOARD_CHARGE_COMPLETE_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// On board charge connecting flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OnBoardChargeConnectingFlag(pub bool);

pub(crate) static ON_BOARD_CHARGE_CONNECTING_FLAG: [Item; 1] = [
    Item { name: "On board charge connecting flag", pos: 4, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for OnBoardChargeConnectingFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x01), item: &ON_BOARD_CHARGE_CONNECTING_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// On board charge full flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OnBoardChargeFullFlag(pub bool);

pub(crate) static ON_BOARD_CHARGE_FULL_FLAG: [Item; 1] = [
    Item { name: "On board charge full flag", pos: 52, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for OnBoardChargeFullFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &ON_BOARD_CHARGE_FULL_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// On board charge permission flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OnBoardChargePermissionFlag(pub bool);

pub(crate) static ON_BOARD_CHARGE_PERMISSION_FLAG: [Item; 3] = [
    Item { name: "On board charge permission flag", pos: 33, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "On board charge permission flag", pos: 47, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "On board charge permission flag", pos: 4, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for OnBoardChargePermissionFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &ON_BOARD_CHARGE_PERMISSION_FLAG[0] },
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &ON_BOARD_CHARGE_PERMISSION_FLAG[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &ON_BOARD_CHARGE_PERMISSION_FLAG[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// On board charge start signal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OnBoardChargeStartSignal(pub bool);

pub(crate) static ON_BOARD_CHARGE_START_SIGNAL: [Item; 3] = [
    Item { name: "On board charge start signal", pos: 30, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "On board charge start signal", pos: 44, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "On board charge start signal", pos: 4, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for OnBoardChargeStartSignal {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &ON_BOARD_CHARGE_START_SIGNAL[0] },
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &ON_BOARD_CHARGE_START_SIGNAL[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x01), item: &ON_BOARD_CHARGE_START_SIGNAL[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// On board charge stop flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OnBoardChargeStopFlag(pub bool);

pub(crate) static ON_BOARD_CHARGE_STOP_FLAG: [Item; 4] = [
    Item { name: "On board charge stop flag", pos: 34, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "On board charge stop flag", pos: 48, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "On board charge stop flag", pos: 52, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "On board charge stop flag", pos: 6, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for OnBoardChargeStopFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &ON_BOARD_CHARGE_STOP_FLAG[0] },
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &ON_BOARD_CHARGE_STOP_FLAG[1] },
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &ON_BOARD_CHARGE_STOP_FLAG[2] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &ON_BOARD_CHARGE_STOP_FLAG[3] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// On board charge stop his. flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OnBoardChargeStopHisFlag(pub bool);

pub(crate) static ON_BOARD_CHARGE_STOP_HIS_FLAG: [Item; 2] = [
    Item { name: "On board charge stop his. flag", pos: 47, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "On board charge stop his. flag", pos: 4, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for OnBoardChargeStopHisFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &ON_BOARD_CHARGE_STOP_HIS_FLAG[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &ON_BOARD_CHARGE_STOP_HIS_FLAG[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// On board charger connect flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OnBoardChargerConnectFlag(pub bool);

pub(crate) static ON_BOARD_CHARGER_CONNECT_FLAG: [Item; 1] = [
    Item { name: "On board charger connect flag", pos: 44, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for OnBoardChargerConnectFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &ON_BOARD_CHARGER_CONNECT_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// On-Board charger cooling request
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OnBoardChargerCoolingRequest(pub bool);

pub(crate) static ON_BOARD_CHARGER_COOLING_REQUEST: [Item; 1] = [
    Item { name: "On-Board charger cooling request", pos: 8, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "Not Request"), (1, 1, "Request")]) },
];

impl crate::item::Reading for OnBoardChargerCoolingRequest {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &ON_BOARD_CHARGER_COOLING_REQUEST[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "Request" => Some(Self(true)),
            "Not Request" => Some(Self(false)),
            _ => None,
        }
    }
}

/// On-Board charger function
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OnBoardChargerFunction(pub bool);

pub(crate) static ON_BOARD_CHARGER_FUNCTION: [Item; 1] = [
    Item { name: "On-Board charger function", pos: 7, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for OnBoardChargerFunction {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &ON_BOARD_CHARGER_FUNCTION[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// On board CHG. connecting signal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OnBoardChgConnectingSignal(pub bool);

pub(crate) static ON_BOARD_CHG_CONNECTING_SIGNAL: [Item; 1] = [
    Item { name: "On board CHG. connecting signal", pos: 30, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for OnBoardChgConnectingSignal {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &ON_BOARD_CHG_CONNECTING_SIGNAL[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Operation mode
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OperationMode(pub u32);

pub(crate) static OPERATION_MODE: [Item; 2] = [
    Item { name: "Operation mode", pos: 14, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
    Item { name: "Operation mode", pos: 14, len: 1, bits: None, unit: Unit::None, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for OperationMode {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &OPERATION_MODE[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.count().map(|n| Self(n))
    }
}

/// Order of mode transfer priority
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrderOfModeTransferPriority(pub u32);

pub(crate) static ORDER_OF_MODE_TRANSFER_PRIORITY: [Item; 1] = [
    Item { name: "Order of mode transfer priority", pos: 4, len: 1, bits: None, unit: Unit::None, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for OrderOfModeTransferPriority {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x20), item: &ORDER_OF_MODE_TRANSFER_PRIORITY[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.count().map(|n| Self(n))
    }
}

/// Paddle shift position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PaddleShiftPosition {
    /// 0
    N0,
    /// 1
    N1,
    /// 2
    N2,
    /// 3
    N3,
    /// 4
    N4,
    /// 5
    N5,
    /// No request
    NoRequest,
    /// -
    Unknown,
}

pub(crate) static PADDLE_SHIFT_POSITION: [Item; 1] = [
    Item { name: "Paddle shift position", pos: 4, len: 1, bits: Some((3, 3)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "0"), (1, 1, "1"), (2, 2, "2"), (3, 3, "3"), (4, 4, "4"), (5, 5, "5"), (6, 6, "No request"), (7, 7, "-")]) },
];

impl crate::item::Reading for PaddleShiftPosition {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x0400], source: Source::Record(0x25), item: &PADDLE_SHIFT_POSITION[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "0" => Some(Self::N0),
            "1" => Some(Self::N1),
            "2" => Some(Self::N2),
            "3" => Some(Self::N3),
            "4" => Some(Self::N4),
            "5" => Some(Self::N5),
            "No request" => Some(Self::NoRequest),
            "-" => Some(Self::Unknown),
            _ => None,
        }
    }
}

/// Paddle shift switch (DOWN)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PaddleShiftSwitchDown(pub bool);

pub(crate) static PADDLE_SHIFT_SWITCH_DOWN: [Item; 1] = [
    Item { name: "Paddle shift switch (DOWN)", pos: 4, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for PaddleShiftSwitchDown {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x0400], source: Source::Record(0x25), item: &PADDLE_SHIFT_SWITCH_DOWN[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Paddle shift switch (UP)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PaddleShiftSwitchUp(pub bool);

pub(crate) static PADDLE_SHIFT_SWITCH_UP: [Item; 1] = [
    Item { name: "Paddle shift switch (UP)", pos: 4, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for PaddleShiftSwitchUp {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x0400], source: Source::Record(0x25), item: &PADDLE_SHIFT_SWITCH_UP[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Parking brake SW
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParkingBrakeSw(pub bool);

pub(crate) static PARKING_BRAKE_SW: [Item; 3] = [
    Item { name: "Parking brake SW", pos: 28, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Parking brake SW", pos: 42, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Parking brake SW", pos: 2, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ParkingBrakeSw {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &PARKING_BRAKE_SW[0] },
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &PARKING_BRAKE_SW[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x01), item: &PARKING_BRAKE_SW[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Power SW cycle
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PowerSwCycle(pub u32);

pub(crate) static POWER_SW_CYCLE: [Item; 1] = [
    Item { name: "Power SW cycle", pos: 9, len: 1, bits: None, unit: Unit::None, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for PowerSwCycle {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000, 0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &POWER_SW_CYCLE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.count().map(|n| Self(n))
    }
}

/// Power SW(OFF) historical flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PowerSwOffHistoricalFlag(pub bool);

pub(crate) static POWER_SW_OFF_HISTORICAL_FLAG: [Item; 2] = [
    Item { name: "Power SW(OFF) historical flag", pos: 46, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Power SW(OFF) historical flag", pos: 2, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for PowerSwOffHistoricalFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &POWER_SW_OFF_HISTORICAL_FLAG[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &POWER_SW_OFF_HISTORICAL_FLAG[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Power SW signal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PowerSwSignal(pub bool);

pub(crate) static POWER_SW_SIGNAL: [Item; 2] = [
    Item { name: "Power SW signal", pos: 42, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Power SW signal", pos: 2, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for PowerSwSignal {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &POWER_SW_SIGNAL[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x01), item: &POWER_SW_SIGNAL[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Power SW(ST) historical flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PowerSwStHistoricalFlag(pub bool);

pub(crate) static POWER_SW_ST_HISTORICAL_FLAG: [Item; 2] = [
    Item { name: "Power SW(ST) historical flag", pos: 46, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Power SW(ST) historical flag", pos: 2, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for PowerSwStHistoricalFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &POWER_SW_ST_HISTORICAL_FLAG[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &POWER_SW_ST_HISTORICAL_FLAG[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Power SW(ST) signal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PowerSwStSignal(pub bool);

pub(crate) static POWER_SW_ST_SIGNAL: [Item; 2] = [
    Item { name: "Power SW(ST) signal", pos: 42, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Power SW(ST) signal", pos: 2, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for PowerSwStSignal {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &POWER_SW_ST_SIGNAL[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x01), item: &POWER_SW_ST_SIGNAL[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Pre A/C active flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PreACActiveFlag(pub bool);

pub(crate) static PRE_AC_ACTIVE_FLAG: [Item; 2] = [
    Item { name: "Pre A/C active flag", pos: 44, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Pre A/C active flag", pos: 4, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for PreACActiveFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005], source: Source::FreezeFrame, item: &PRE_AC_ACTIVE_FLAG[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005], source: Source::Record(0x01), item: &PRE_AC_ACTIVE_FLAG[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Pre A/C control start signal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PreACControlStartSignal(pub bool);

pub(crate) static PRE_AC_CONTROL_START_SIGNAL: [Item; 1] = [
    Item { name: "Pre A/C control start signal", pos: 30, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for PreACControlStartSignal {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &PRE_AC_CONTROL_START_SIGNAL[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Q.Charge ContactorCircuit status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QChargeContactorcircuitStatus(pub bool);

pub(crate) static Q_CHARGE_CONTACTORCIRCUIT_STATUS: [Item; 1] = [
    Item { name: "Q.Charge ContactorCircuit status", pos: 3, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "Lo voltage"), (1, 1, "Hi voltage")]) },
];

impl crate::item::Reading for QChargeContactorcircuitStatus {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x0100, 0x0200], source: Source::Record(0x25), item: &Q_CHARGE_CONTACTORCIRCUIT_STATUS[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "Hi voltage" => Some(Self(true)),
            "Lo voltage" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Q.CHG contactor(-) circuit state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QChgContactorMinusCircuitState(pub bool);

pub(crate) static Q_CHG_CONTACTOR_MINUS_CIRCUIT_STATE: [Item; 1] = [
    Item { name: "Q.CHG contactor(-) circuit state", pos: 3, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "Lo voltage"), (1, 1, "Hi voltage")]) },
];

impl crate::item::Reading for QChgContactorMinusCircuitState {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x0300, 0x0400], source: Source::Record(0x25), item: &Q_CHG_CONTACTOR_MINUS_CIRCUIT_STATE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "Hi voltage" => Some(Self(true)),
            "Lo voltage" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Q.CHG contactor(+) circuit state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QChgContactorPlusCircuitState(pub bool);

pub(crate) static Q_CHG_CONTACTOR_PLUS_CIRCUIT_STATE: [Item; 1] = [
    Item { name: "Q.CHG contactor(+) circuit state", pos: 3, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "Lo voltage"), (1, 1, "Hi voltage")]) },
];

impl crate::item::Reading for QChgContactorPlusCircuitState {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x0300, 0x0400], source: Source::Record(0x25), item: &Q_CHG_CONTACTOR_PLUS_CIRCUIT_STATE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "Hi voltage" => Some(Self(true)),
            "Lo voltage" => Some(Self(false)),
            _ => None,
        }
    }
}

/// QC:Abnormal judgment upper limit
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QcAbnormalJudgmentUpperLimit(pub crate::db::units::Volts);

pub(crate) static QC_ABNORMAL_JUDGMENT_UPPER_LIMIT: [Item; 1] = [
    Item { name: "QC:Abnormal judgment upper limit", pos: 64, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for QcAbnormalJudgmentUpperLimit {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &QC_ABNORMAL_JUDGMENT_UPPER_LIMIT[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// QC:Battery abnormal flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QcBatteryAbnormalFlag(pub bool);

pub(crate) static QC_BATTERY_ABNORMAL_FLAG: [Item; 1] = [
    Item { name: "QC:Battery abnormal flag", pos: 67, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for QcBatteryAbnormalFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &QC_BATTERY_ABNORMAL_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// QC:Charge remaining time
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QcChargeRemainingTime(pub crate::db::units::Seconds);

pub(crate) static QC_CHARGE_REMAINING_TIME: [Item; 1] = [
    Item { name: "QC:Charge remaining time", pos: 68, len: 1, bits: None, unit: Unit::Seconds, conv: Conv::Linear { factor: 10.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for QcChargeRemainingTime {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &QC_CHARGE_REMAINING_TIME[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Seconds(n)))
    }
}

/// QC:Charger abnormal flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QcChargerAbnormalFlag(pub bool);

pub(crate) static QC_CHARGER_ABNORMAL_FLAG: [Item; 1] = [
    Item { name: "QC:Charger abnormal flag", pos: 67, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for QcChargerAbnormalFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &QC_CHARGER_ABNORMAL_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// QC:Charger connector rock flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QcChargerConnectorRockFlag(pub bool);

pub(crate) static QC_CHARGER_CONNECTOR_ROCK_FLAG: [Item; 1] = [
    Item { name: "QC:Charger connector rock flag", pos: 67, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for QcChargerConnectorRockFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &QC_CHARGER_CONNECTOR_ROCK_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// QC:Charger state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QcChargerState(pub bool);

pub(crate) static QC_CHARGER_STATE: [Item; 1] = [
    Item { name: "QC:Charger state", pos: 67, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "NG"), (1, 1, "OK")]) },
];

impl crate::item::Reading for QcChargerState {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &QC_CHARGER_STATE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "OK" => Some(Self(true)),
            "NG" => Some(Self(false)),
            _ => None,
        }
    }
}

/// QC:Output possible current
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QcOutputPossibleCurrent(pub crate::db::units::Amps);

pub(crate) static QC_OUTPUT_POSSIBLE_CURRENT: [Item; 1] = [
    Item { name: "QC:Output possible current", pos: 63, len: 1, bits: None, unit: Unit::Amps, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for QcOutputPossibleCurrent {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &QC_OUTPUT_POSSIBLE_CURRENT[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Amps(n)))
    }
}

/// QC:Output possible voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QcOutputPossibleVoltage(pub crate::db::units::Volts);

pub(crate) static QC_OUTPUT_POSSIBLE_VOLTAGE: [Item; 1] = [
    Item { name: "QC:Output possible voltage", pos: 61, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for QcOutputPossibleVoltage {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &QC_OUTPUT_POSSIBLE_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// QC:PIN
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QcPin(pub u32);

pub(crate) static QC_PIN: [Item; 1] = [
    Item { name: "QC:PIN", pos: 66, len: 1, bits: None, unit: Unit::None, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for QcPin {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &QC_PIN[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.count().map(|n| Self(n))
    }
}

/// QC:Vehicle abnormal flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QcVehicleAbnormalFlag(pub bool);

pub(crate) static QC_VEHICLE_ABNORMAL_FLAG: [Item; 1] = [
    Item { name: "QC:Vehicle abnormal flag", pos: 67, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for QcVehicleAbnormalFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &QC_VEHICLE_ABNORMAL_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Quick charge complete flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuickChargeCompleteFlag(pub bool);

pub(crate) static QUICK_CHARGE_COMPLETE_FLAG: [Item; 1] = [
    Item { name: "Quick charge complete flag", pos: 33, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for QuickChargeCompleteFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &QUICK_CHARGE_COMPLETE_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Quick charge connecting signal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuickChargeConnectingSignal(pub bool);

pub(crate) static QUICK_CHARGE_CONNECTING_SIGNAL: [Item; 1] = [
    Item { name: "Quick charge connecting signal", pos: 29, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for QuickChargeConnectingSignal {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &QUICK_CHARGE_CONNECTING_SIGNAL[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Quick charge contactor (-)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuickChargeContactorMinus(pub bool);

pub(crate) static QUICK_CHARGE_CONTACTOR_MINUS: [Item; 1] = [
    Item { name: "Quick charge contactor (-)", pos: 4, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for QuickChargeContactorMinus {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x0300, 0x0400], source: Source::Record(0x02), item: &QUICK_CHARGE_CONTACTOR_MINUS[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Quick charge contactor (+)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuickChargeContactorPlus(pub bool);

pub(crate) static QUICK_CHARGE_CONTACTOR_PLUS: [Item; 1] = [
    Item { name: "Quick charge contactor (+)", pos: 4, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for QuickChargeContactorPlus {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x0300, 0x0400], source: Source::Record(0x02), item: &QUICK_CHARGE_CONTACTOR_PLUS[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Quick charge contactor relay
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuickChargeContactorRelay(pub bool);

pub(crate) static QUICK_CHARGE_CONTACTOR_RELAY: [Item; 2] = [
    Item { name: "Quick charge contactor relay", pos: 46, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Quick charge contactor relay", pos: 3, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for QuickChargeContactorRelay {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200], source: Source::FreezeFrame, item: &QUICK_CHARGE_CONTACTOR_RELAY[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200], source: Source::Record(0x02), item: &QUICK_CHARGE_CONTACTOR_RELAY[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Quick charge current
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuickChargeCurrent(pub crate::db::units::Amps);

pub(crate) static QUICK_CHARGE_CURRENT: [Item; 1] = [
    Item { name: "Quick charge current", pos: 41, len: 1, bits: None, unit: Unit::Amps, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for QuickChargeCurrent {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &QUICK_CHARGE_CURRENT[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Amps(n)))
    }
}

/// Quick charge current (request)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuickChargeCurrentRequest(pub crate::db::units::Amps);

pub(crate) static QUICK_CHARGE_CURRENT_REQUEST: [Item; 1] = [
    Item { name: "Quick charge current (request)", pos: 38, len: 1, bits: None, unit: Unit::Amps, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for QuickChargeCurrentRequest {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &QUICK_CHARGE_CURRENT_REQUEST[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Amps(n)))
    }
}

/// Quick charge current stop flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuickChargeCurrentStopFlag(pub bool);

pub(crate) static QUICK_CHARGE_CURRENT_STOP_FLAG: [Item; 1] = [
    Item { name: "Quick charge current stop flag", pos: 52, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for QuickChargeCurrentStopFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &QUICK_CHARGE_CURRENT_STOP_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Quick Charge current timeout
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuickChargeCurrentTimeout(pub bool);

pub(crate) static QUICK_CHARGE_CURRENT_TIMEOUT: [Item; 1] = [
    Item { name: "Quick Charge current timeout", pos: 4, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for QuickChargeCurrentTimeout {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &QUICK_CHARGE_CURRENT_TIMEOUT[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Quick charge full flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuickChargeFullFlag(pub bool);

pub(crate) static QUICK_CHARGE_FULL_FLAG: [Item; 1] = [
    Item { name: "Quick charge full flag", pos: 52, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for QuickChargeFullFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &QUICK_CHARGE_FULL_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Quick charge output voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct QuickChargeOutputVoltage(pub crate::db::units::Volts);

pub(crate) static QUICK_CHARGE_OUTPUT_VOLTAGE: [Item; 1] = [
    Item { name: "Quick charge output voltage", pos: 39, len: 2, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for QuickChargeOutputVoltage {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &QUICK_CHARGE_OUTPUT_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Quick charge permission flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuickChargePermissionFlag(pub bool);

pub(crate) static QUICK_CHARGE_PERMISSION_FLAG: [Item; 3] = [
    Item { name: "Quick charge permission flag", pos: 33, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Quick charge permission flag", pos: 47, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Quick charge permission flag", pos: 3, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for QuickChargePermissionFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &QUICK_CHARGE_PERMISSION_FLAG[0] },
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &QUICK_CHARGE_PERMISSION_FLAG[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &QUICK_CHARGE_PERMISSION_FLAG[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Quick charge permission signal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuickChargePermissionSignal(pub bool);

pub(crate) static QUICK_CHARGE_PERMISSION_SIGNAL: [Item; 3] = [
    Item { name: "Quick charge permission signal", pos: 32, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Quick charge permission signal", pos: 46, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Quick charge permission signal", pos: 4, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for QuickChargePermissionSignal {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &QUICK_CHARGE_PERMISSION_SIGNAL[0] },
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &QUICK_CHARGE_PERMISSION_SIGNAL[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x02), item: &QUICK_CHARGE_PERMISSION_SIGNAL[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Quick charge start signal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuickChargeStartSignal(pub bool);

pub(crate) static QUICK_CHARGE_START_SIGNAL: [Item; 3] = [
    Item { name: "Quick charge start signal", pos: 30, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Quick charge start signal", pos: 44, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Quick charge start signal", pos: 4, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for QuickChargeStartSignal {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &QUICK_CHARGE_START_SIGNAL[0] },
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &QUICK_CHARGE_START_SIGNAL[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x01), item: &QUICK_CHARGE_START_SIGNAL[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Quick charge stop flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuickChargeStopFlag(pub bool);

pub(crate) static QUICK_CHARGE_STOP_FLAG: [Item; 3] = [
    Item { name: "Quick charge stop flag", pos: 34, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Quick charge stop flag", pos: 48, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Quick charge stop flag", pos: 6, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for QuickChargeStopFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &QUICK_CHARGE_STOP_FLAG[0] },
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &QUICK_CHARGE_STOP_FLAG[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &QUICK_CHARGE_STOP_FLAG[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Quick charge stop history flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuickChargeStopHistoryFlag(pub bool);

pub(crate) static QUICK_CHARGE_STOP_HISTORY_FLAG: [Item; 2] = [
    Item { name: "Quick charge stop history flag", pos: 47, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Quick charge stop history flag", pos: 4, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for QuickChargeStopHistoryFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &QUICK_CHARGE_STOP_HISTORY_FLAG[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &QUICK_CHARGE_STOP_HISTORY_FLAG[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Quick charger connect. judgment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuickChargerConnectJudgment(pub bool);

pub(crate) static QUICK_CHARGER_CONNECT_JUDGMENT: [Item; 1] = [
    Item { name: "Quick charger connect. judgment", pos: 4, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for QuickChargerConnectJudgment {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &QUICK_CHARGER_CONNECT_JUDGMENT[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Quick charger connecting flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuickChargerConnectingFlag(pub bool);

pub(crate) static QUICK_CHARGER_CONNECTING_FLAG: [Item; 2] = [
    Item { name: "Quick charger connecting flag", pos: 43, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Quick charger connecting flag", pos: 4, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for QuickChargerConnectingFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &QUICK_CHARGER_CONNECTING_FLAG[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x01), item: &QUICK_CHARGER_CONNECTING_FLAG[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Quick charger cont. status flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuickChargerContStatusFlag(pub bool);

pub(crate) static QUICK_CHARGER_CONT_STATUS_FLAG: [Item; 1] = [
    Item { name: "Quick charger cont. status flag", pos: 5, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for QuickChargerContStatusFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &QUICK_CHARGER_CONT_STATUS_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Quick charger contactor [-]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuickChargerContactorMinus(pub bool);

pub(crate) static QUICK_CHARGER_CONTACTOR_MINUS: [Item; 1] = [
    Item { name: "Quick charger contactor [-]", pos: 44, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for QuickChargerContactorMinus {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x0300, 0x0400], source: Source::FreezeFrame, item: &QUICK_CHARGER_CONTACTOR_MINUS[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Quick charger contactor [+]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuickChargerContactorPlus(pub bool);

pub(crate) static QUICK_CHARGER_CONTACTOR_PLUS: [Item; 1] = [
    Item { name: "Quick charger contactor [+]", pos: 44, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for QuickChargerContactorPlus {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x0300, 0x0400], source: Source::FreezeFrame, item: &QUICK_CHARGER_CONTACTOR_PLUS[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Quick charger function
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuickChargerFunction(pub bool);

pub(crate) static QUICK_CHARGER_FUNCTION: [Item; 1] = [
    Item { name: "Quick charger function", pos: 7, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for QuickChargerFunction {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &QUICK_CHARGER_FUNCTION[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Quick charger power relay
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuickChargerPowerRelay(pub bool);

pub(crate) static QUICK_CHARGER_POWER_RELAY: [Item; 1] = [
    Item { name: "Quick charger power relay", pos: 29, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for QuickChargerPowerRelay {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &QUICK_CHARGER_POWER_RELAY[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Quick charger start-up signal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuickChargerStartUpSignal(pub bool);

pub(crate) static QUICK_CHARGER_START_UP_SIGNAL: [Item; 2] = [
    Item { name: "Quick charger start-up signal", pos: 43, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Quick charger start-up signal", pos: 4, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for QuickChargerStartUpSignal {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &QUICK_CHARGER_START_UP_SIGNAL[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x01), item: &QUICK_CHARGER_START_UP_SIGNAL[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Quick CHG. contactor CONT. relay
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct QuickChgContactorContRelay(pub bool);

pub(crate) static QUICK_CHG_CONTACTOR_CONT_RELAY: [Item; 1] = [
    Item { name: "Quick CHG. contactor CONT. relay", pos: 32, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for QuickChgContactorContRelay {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &QUICK_CHG_CONTACTOR_CONT_RELAY[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Radiator fan1 relay
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RadiatorFan1Relay(pub bool);

pub(crate) static RADIATOR_FAN1_RELAY: [Item; 3] = [
    Item { name: "Radiator fan1 relay", pos: 31, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Radiator fan1 relay", pos: 45, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Radiator fan1 relay", pos: 2, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for RadiatorFan1Relay {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &RADIATOR_FAN1_RELAY[0] },
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200], source: Source::FreezeFrame, item: &RADIATOR_FAN1_RELAY[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0400], source: Source::Record(0x02), item: &RADIATOR_FAN1_RELAY[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Radiator fan1 relay (diag)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RadiatorFan1RelayDiag(pub bool);

pub(crate) static RADIATOR_FAN1_RELAY_DIAG: [Item; 2] = [
    Item { name: "Radiator fan1 relay (diag)", pos: 42, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Radiator fan1 relay (diag)", pos: 2, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for RadiatorFan1RelayDiag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200], source: Source::FreezeFrame, item: &RADIATOR_FAN1_RELAY_DIAG[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0400], source: Source::Record(0x01), item: &RADIATOR_FAN1_RELAY_DIAG[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Radiator fan1 relay (monitor)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RadiatorFan1RelayMonitor(pub bool);

pub(crate) static RADIATOR_FAN1_RELAY_MONITOR: [Item; 1] = [
    Item { name: "Radiator fan1 relay (monitor)", pos: 28, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for RadiatorFan1RelayMonitor {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &RADIATOR_FAN1_RELAY_MONITOR[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Radiator fan2 relay
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RadiatorFan2Relay(pub bool);

pub(crate) static RADIATOR_FAN2_RELAY: [Item; 3] = [
    Item { name: "Radiator fan2 relay", pos: 31, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Radiator fan2 relay", pos: 45, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Radiator fan2 relay", pos: 2, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for RadiatorFan2Relay {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &RADIATOR_FAN2_RELAY[0] },
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &RADIATOR_FAN2_RELAY[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x02), item: &RADIATOR_FAN2_RELAY[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Radiator fan2 relay (diag)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RadiatorFan2RelayDiag(pub bool);

pub(crate) static RADIATOR_FAN2_RELAY_DIAG: [Item; 2] = [
    Item { name: "Radiator fan2 relay (diag)", pos: 42, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Radiator fan2 relay (diag)", pos: 2, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for RadiatorFan2RelayDiag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &RADIATOR_FAN2_RELAY_DIAG[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x01), item: &RADIATOR_FAN2_RELAY_DIAG[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Radiator fan2 relay (monitor)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RadiatorFan2RelayMonitor(pub bool);

pub(crate) static RADIATOR_FAN2_RELAY_MONITOR: [Item; 1] = [
    Item { name: "Radiator fan2 relay (monitor)", pos: 28, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for RadiatorFan2RelayMonitor {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &RADIATOR_FAN2_RELAY_MONITOR[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// READY
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Ready(pub bool);

pub(crate) static READY: [Item; 1] = [
    Item { name: "READY", pos: 32, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for Ready {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &READY[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// READY lamp
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ReadyLamp(pub bool);

pub(crate) static READY_LAMP: [Item; 2] = [
    Item { name: "READY lamp", pos: 46, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "READY lamp", pos: 2, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ReadyLamp {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &READY_LAMP[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &READY_LAMP[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Shift information
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShiftInformation {
    /// -
    Unknown,
    /// P
    P,
    /// R
    R,
    /// N
    N,
    /// D
    D,
    /// E
    E,
    /// B
    B,
    /// BL
    Bl,
    /// D0
    D0,
    /// D1
    D1,
    /// D2
    D2,
    /// D3
    D3,
    /// D4
    D4,
    /// D5
    D5,
    /// E0
    E0,
    /// E1
    E1,
    /// E2
    E2,
    /// E3
    E3,
    /// E4
    E4,
    /// E5
    E5,
    /// B0
    B0,
    /// B1
    B1,
    /// B2
    B2,
    /// B3
    B3,
    /// B4
    B4,
    /// B5
    B5,
}

pub(crate) static SHIFT_INFORMATION: [Item; 1] = [
    Item { name: "Shift information", pos: 32, len: 1, bits: None, unit: Unit::None, conv: Conv::Labels(&[(0, 0, "-"), (1, 1, "P"), (2, 2, "R"), (3, 3, "N"), (4, 4, "D"), (5, 5, "E"), (6, 6, "B"), (16, 16, "BL"), (17, 17, "D0"), (18, 18, "D1"), (19, 19, "D2"), (20, 20, "D3"), (21, 21, "D4"), (22, 22, "D5"), (23, 23, "E0"), (24, 24, "E1"), (25, 25, "E2"), (26, 26, "E3"), (27, 27, "E4"), (28, 28, "E5"), (29, 29, "B0"), (30, 30, "B1"), (31, 31, "B2"), (32, 32, "B3"), (33, 33, "B4"), (34, 34, "B5")]) },
];

impl crate::item::Reading for ShiftInformation {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x0400], source: Source::FreezeFrame, item: &SHIFT_INFORMATION[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "-" => Some(Self::Unknown),
            "P" => Some(Self::P),
            "R" => Some(Self::R),
            "N" => Some(Self::N),
            "D" => Some(Self::D),
            "E" => Some(Self::E),
            "B" => Some(Self::B),
            "BL" => Some(Self::Bl),
            "D0" => Some(Self::D0),
            "D1" => Some(Self::D1),
            "D2" => Some(Self::D2),
            "D3" => Some(Self::D3),
            "D4" => Some(Self::D4),
            "D5" => Some(Self::D5),
            "E0" => Some(Self::E0),
            "E1" => Some(Self::E1),
            "E2" => Some(Self::E2),
            "E3" => Some(Self::E3),
            "E4" => Some(Self::E4),
            "E5" => Some(Self::E5),
            "B0" => Some(Self::B0),
            "B1" => Some(Self::B1),
            "B2" => Some(Self::B2),
            "B3" => Some(Self::B3),
            "B4" => Some(Self::B4),
            "B5" => Some(Self::B5),
            _ => None,
        }
    }
}

/// Shift/Inhibitor SW fail counter
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShiftInhibitorSwFailCounter(pub u32);

pub(crate) static SHIFT_INHIBITOR_SW_FAIL_COUNTER: [Item; 1] = [
    Item { name: "Shift/Inhibitor SW fail counter", pos: 12, len: 1, bits: None, unit: Unit::None, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for ShiftInhibitorSwFailCounter {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000, 0x8001, 0x8003, 0x0003, 0x0004, 0x0005], source: Source::FreezeFrame, item: &SHIFT_INHIBITOR_SW_FAIL_COUNTER[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.count().map(|n| Self(n))
    }
}

/// Shift "P" timeout flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShiftPTimeoutFlag(pub bool);

pub(crate) static SHIFT_P_TIMEOUT_FLAG: [Item; 1] = [
    Item { name: "Shift \"P\" timeout flag", pos: 3, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ShiftPTimeoutFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &SHIFT_P_TIMEOUT_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Shift position
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShiftPosition {
    /// Undefined
    Undefined,
    /// P
    P,
    /// R
    R,
    /// N
    N,
    /// D
    D,
    /// E(JPN) /B(EU)
    EJpnBEu,
    /// B(JPN) /C(EU)
    BJpnCEu,
}

pub(crate) static SHIFT_POSITION: [Item; 2] = [
    Item { name: "Shift position", pos: 24, len: 1, bits: None, unit: Unit::None, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
    Item { name: "Shift position", pos: 24, len: 1, bits: None, unit: Unit::None, conv: Conv::Labels(&[(0, 0, "Undefined"), (1, 1, "P"), (2, 2, "R"), (3, 3, "N"), (4, 4, "D"), (5, 5, "E(JPN) /B(EU)"), (6, 6, "B(JPN) /C(EU)"), (7, 255, "Undefined")]) },
];

impl crate::item::Reading for ShiftPosition {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &SHIFT_POSITION[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "Undefined" => Some(Self::Undefined),
            "P" => Some(Self::P),
            "R" => Some(Self::R),
            "N" => Some(Self::N),
            "D" => Some(Self::D),
            "E(JPN) /B(EU)" => Some(Self::EJpnBEu),
            "B(JPN) /C(EU)" => Some(Self::BJpnCEu),
            _ => None,
        }
    }
}

/// Shift position(control value)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShiftPositionControlValue(pub u32);

pub(crate) static SHIFT_POSITION_CONTROL_VALUE: [Item; 1] = [
    Item { name: "Shift position(control value)", pos: 5, len: 1, bits: None, unit: Unit::None, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for ShiftPositionControlValue {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x24), item: &SHIFT_POSITION_CONTROL_VALUE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.count().map(|n| Self(n))
    }
}

/// Shift position D
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShiftPositionD(pub bool);

pub(crate) static SHIFT_POSITION_D: [Item; 1] = [
    Item { name: "Shift position D", pos: 5, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ShiftPositionD {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x01), item: &SHIFT_POSITION_D[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Shift position N
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShiftPositionN(pub bool);

pub(crate) static SHIFT_POSITION_N: [Item; 1] = [
    Item { name: "Shift position N", pos: 5, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ShiftPositionN {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x01), item: &SHIFT_POSITION_N[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Shift position P
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShiftPositionP(pub bool);

pub(crate) static SHIFT_POSITION_P: [Item; 1] = [
    Item { name: "Shift position P", pos: 5, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ShiftPositionP {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x01), item: &SHIFT_POSITION_P[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Shift position R
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShiftPositionR(pub bool);

pub(crate) static SHIFT_POSITION_R: [Item; 1] = [
    Item { name: "Shift position R", pos: 5, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ShiftPositionR {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x01), item: &SHIFT_POSITION_R[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Shift position SW :B1
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShiftPositionSwB1(pub bool);

pub(crate) static SHIFT_POSITION_SW_B1: [Item; 1] = [
    Item { name: "Shift position SW :B1", pos: 50, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ShiftPositionSwB1 {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001], source: Source::FreezeFrame, item: &SHIFT_POSITION_SW_B1[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Shift position SW :B2
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShiftPositionSwB2(pub bool);

pub(crate) static SHIFT_POSITION_SW_B2: [Item; 1] = [
    Item { name: "Shift position SW :B2", pos: 50, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ShiftPositionSwB2 {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001], source: Source::FreezeFrame, item: &SHIFT_POSITION_SW_B2[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Shift position SW :B(JPN)/C(EU)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShiftPositionSwBJpnCEu(pub bool);

pub(crate) static SHIFT_POSITION_SW_B_JPN_C_EU: [Item; 2] = [
    Item { name: "Shift position SW :B(JPN)/C(EU)", pos: 50, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Shift position SW :B(JPN)/C(EU)", pos: 5, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ShiftPositionSwBJpnCEu {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &SHIFT_POSITION_SW_B_JPN_C_EU[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x01), item: &SHIFT_POSITION_SW_B_JPN_C_EU[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Shift position SW :D
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShiftPositionSwD(pub bool);

pub(crate) static SHIFT_POSITION_SW_D: [Item; 1] = [
    Item { name: "Shift position SW :D", pos: 50, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ShiftPositionSwD {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &SHIFT_POSITION_SW_D[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Shift position SW :E(JPN)/B(EU)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShiftPositionSwEJpnBEu(pub bool);

pub(crate) static SHIFT_POSITION_SW_E_JPN_B_EU: [Item; 2] = [
    Item { name: "Shift position SW :E(JPN)/B(EU)", pos: 50, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Shift position SW :E(JPN)/B(EU)", pos: 5, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ShiftPositionSwEJpnBEu {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &SHIFT_POSITION_SW_E_JPN_B_EU[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x01), item: &SHIFT_POSITION_SW_E_JPN_B_EU[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Shift position SW :N
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShiftPositionSwN(pub bool);

pub(crate) static SHIFT_POSITION_SW_N: [Item; 1] = [
    Item { name: "Shift position SW :N", pos: 50, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ShiftPositionSwN {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &SHIFT_POSITION_SW_N[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Shift position SW :P
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShiftPositionSwP(pub bool);

pub(crate) static SHIFT_POSITION_SW_P: [Item; 2] = [
    Item { name: "Shift position SW :P", pos: 24, len: 1, bits: None, unit: Unit::None, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
    Item { name: "Shift position SW :P", pos: 50, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ShiftPositionSwP {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &SHIFT_POSITION_SW_P[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Shift position SW :R
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShiftPositionSwR(pub bool);

pub(crate) static SHIFT_POSITION_SW_R: [Item; 1] = [
    Item { name: "Shift position SW :R", pos: 50, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for ShiftPositionSwR {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &SHIFT_POSITION_SW_R[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// SKC write complete flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SkcWriteCompleteFlag(pub bool);

pub(crate) static SKC_WRITE_COMPLETE_FLAG: [Item; 1] = [
    Item { name: "SKC write complete flag", pos: 67, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "NG"), (1, 1, "OK")]) },
];

impl crate::item::Reading for SkcWriteCompleteFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &SKC_WRITE_COMPLETE_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "OK" => Some(Self(true)),
            "NG" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Speed flag for radiator fan stop
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SpeedFlagForRadiatorFanStop(pub bool);

pub(crate) static SPEED_FLAG_FOR_RADIATOR_FAN_STOP: [Item; 1] = [
    Item { name: "Speed flag for radiator fan stop", pos: 2, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for SpeedFlagForRadiatorFanStop {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &SPEED_FLAG_FOR_RADIATOR_FAN_STOP[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Starting OK communicate complete
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StartingOkCommunicateComplete(pub bool);

pub(crate) static STARTING_OK_COMMUNICATE_COMPLETE: [Item; 1] = [
    Item { name: "Starting OK communicate complete", pos: 67, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "NG"), (1, 1, "OK")]) },
];

impl crate::item::Reading for StartingOkCommunicateComplete {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &STARTING_OK_COMMUNICATE_COMPLETE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "OK" => Some(Self(true)),
            "NG" => Some(Self(false)),
            _ => None,
        }
    }
}

/// State of charge(control value)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct StateOfChargeControlValue(pub crate::db::units::Percent);

pub(crate) static STATE_OF_CHARGE_CONTROL_VALUE: [Item; 1] = [
    Item { name: "State of charge(control value)", pos: 2, len: 1, bits: None, unit: Unit::Percent, conv: Conv::Linear { factor: 0.5, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for StateOfChargeControlValue {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x24), item: &STATE_OF_CHARGE_CONTROL_VALUE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Percent(n)))
    }
}

/// Sub mode
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SubMode(pub crate::db::units::Volts);

pub(crate) static SUB_MODE: [Item; 1] = [
    Item { name: "Sub mode", pos: 15, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for SubMode {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &SUB_MODE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Sub mode 1
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubMode1(pub u32);

pub(crate) static SUB_MODE1: [Item; 1] = [
    Item { name: "Sub mode 1", pos: 15, len: 1, bits: None, unit: Unit::None, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for SubMode1 {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &SUB_MODE1[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.count().map(|n| Self(n))
    }
}

/// Sub mode 2
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SubMode2(pub u32);

pub(crate) static SUB_MODE2: [Item; 1] = [
    Item { name: "Sub mode 2", pos: 34, len: 1, bits: None, unit: Unit::None, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for SubMode2 {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &SUB_MODE2[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.count().map(|n| Self(n))
    }
}

/// TCL_OFF SW
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TclOffSw(pub bool);

pub(crate) static TCL_OFF_SW: [Item; 1] = [
    Item { name: "TCL_OFF SW", pos: 3, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for TclOffSw {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x01), item: &TCL_OFF_SW[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Timer charge flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimerChargeFlag(pub bool);

pub(crate) static TIMER_CHARGE_FLAG: [Item; 3] = [
    Item { name: "Timer charge flag", pos: 44, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "ON"), (1, 1, "OFF")]) },
    Item { name: "Timer charge flag", pos: 44, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Timer charge flag", pos: 4, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "ON"), (1, 1, "OFF")]) },
];

impl crate::item::Reading for TimerChargeFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001], source: Source::FreezeFrame, item: &TIMER_CHARGE_FLAG[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005], source: Source::FreezeFrame, item: &TIMER_CHARGE_FLAG[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005], source: Source::Record(0x01), item: &TIMER_CHARGE_FLAG[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Timer charge start signal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimerChargeStartSignal(pub bool);

pub(crate) static TIMER_CHARGE_START_SIGNAL: [Item; 1] = [
    Item { name: "Timer charge start signal", pos: 30, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for TimerChargeStartSignal {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &TIMER_CHARGE_START_SIGNAL[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Timer reserve flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimerReserveFlag(pub bool);

pub(crate) static TIMER_RESERVE_FLAG: [Item; 3] = [
    Item { name: "Timer reserve flag", pos: 44, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "ON"), (1, 1, "OFF")]) },
    Item { name: "Timer reserve flag", pos: 44, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Timer reserve flag", pos: 4, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "ON"), (1, 1, "OFF")]) },
];

impl crate::item::Reading for TimerReserveFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001], source: Source::FreezeFrame, item: &TIMER_RESERVE_FLAG[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005], source: Source::FreezeFrame, item: &TIMER_RESERVE_FLAG[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005], source: Source::Record(0x01), item: &TIMER_RESERVE_FLAG[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Timer reserve signal
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TimerReserveSignal(pub bool);

pub(crate) static TIMER_RESERVE_SIGNAL: [Item; 1] = [
    Item { name: "Timer reserve signal", pos: 30, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for TimerReserveSignal {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &TIMER_RESERVE_SIGNAL[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Torque before restraint for bat.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TorqueBeforeRestraintForBat(pub crate::db::units::NewtonMetres);

pub(crate) static TORQUE_BEFORE_RESTRAINT_FOR_BAT: [Item; 1] = [
    Item { name: "Torque before restraint for bat.", pos: 2, len: 1, bits: None, unit: Unit::NewtonMetres, conv: Conv::Linear { factor: 2.0, offset: -200.0, decimals: 1 } },
];

impl crate::item::Reading for TorqueBeforeRestraintForBat {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x21), item: &TORQUE_BEFORE_RESTRAINT_FOR_BAT[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::NewtonMetres(n)))
    }
}

/// Torque(calculation value)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TorqueCalculationValue(pub crate::db::units::NewtonMetres);

pub(crate) static TORQUE_CALCULATION_VALUE: [Item; 1] = [
    Item { name: "Torque(calculation value)", pos: 7, len: 1, bits: None, unit: Unit::NewtonMetres, conv: Conv::Linear { factor: 2.0, offset: -200.0, decimals: 0 } },
];

impl crate::item::Reading for TorqueCalculationValue {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x21), item: &TORQUE_CALCULATION_VALUE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::NewtonMetres(n)))
    }
}

/// Torque(control value)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TorqueControlValue(pub crate::db::units::NewtonMetres);

pub(crate) static TORQUE_CONTROL_VALUE: [Item; 1] = [
    Item { name: "Torque(control value)", pos: 8, len: 1, bits: None, unit: Unit::NewtonMetres, conv: Conv::Linear { factor: 2.0, offset: -200.0, decimals: 0 } },
];

impl crate::item::Reading for TorqueControlValue {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x21), item: &TORQUE_CONTROL_VALUE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::NewtonMetres(n)))
    }
}

/// Torque inhibition (battery) flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TorqueInhibitionBatteryFlag(pub bool);

pub(crate) static TORQUE_INHIBITION_BATTERY_FLAG: [Item; 3] = [
    Item { name: "Torque inhibition (battery) flag", pos: 34, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Torque inhibition (battery) flag", pos: 48, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Torque inhibition (battery) flag", pos: 5, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for TorqueInhibitionBatteryFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &TORQUE_INHIBITION_BATTERY_FLAG[0] },
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &TORQUE_INHIBITION_BATTERY_FLAG[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &TORQUE_INHIBITION_BATTERY_FLAG[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Torque inhibition gain(battery)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TorqueInhibitionGainBattery(pub bool);

pub(crate) static TORQUE_INHIBITION_GAIN_BATTERY: [Item; 1] = [
    Item { name: "Torque inhibition gain(battery)", pos: 52, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for TorqueInhibitionGainBattery {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001], source: Source::FreezeFrame, item: &TORQUE_INHIBITION_GAIN_BATTERY[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Torque inhibition gain(motor)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TorqueInhibitionGainMotor(pub bool);

pub(crate) static TORQUE_INHIBITION_GAIN_MOTOR: [Item; 1] = [
    Item { name: "Torque inhibition gain(motor)", pos: 51, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for TorqueInhibitionGainMotor {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001], source: Source::FreezeFrame, item: &TORQUE_INHIBITION_GAIN_MOTOR[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Torque inhibition (motor) flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TorqueInhibitionMotorFlag(pub bool);

pub(crate) static TORQUE_INHIBITION_MOTOR_FLAG: [Item; 3] = [
    Item { name: "Torque inhibition (motor) flag", pos: 34, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Torque inhibition (motor) flag", pos: 48, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Torque inhibition (motor) flag", pos: 5, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for TorqueInhibitionMotorFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &TORQUE_INHIBITION_MOTOR_FLAG[0] },
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &TORQUE_INHIBITION_MOTOR_FLAG[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &TORQUE_INHIBITION_MOTOR_FLAG[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// V-CAN status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VCanStatus(pub u32);

pub(crate) static V_CAN_STATUS: [Item; 1] = [
    Item { name: "V-CAN status", pos: 6, len: 1, bits: None, unit: Unit::None, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for VCanStatus {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x20), item: &V_CAN_STATUS[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.count().map(|n| Self(n))
    }
}

/// Vacuum P.failsafe relay(monitor)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VacuumPFailsafeRelayMonitor(pub bool);

pub(crate) static VACUUM_P_FAILSAFE_RELAY_MONITOR: [Item; 1] = [
    Item { name: "Vacuum P.failsafe relay(monitor)", pos: 39, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for VacuumPFailsafeRelayMonitor {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &VACUUM_P_FAILSAFE_RELAY_MONITOR[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Vacuum pump 1 relay
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VacuumPump1Relay(pub bool);

pub(crate) static VACUUM_PUMP1_RELAY: [Item; 3] = [
    Item { name: "Vacuum pump 1 relay", pos: 32, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Vacuum pump 1 relay", pos: 46, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Vacuum pump 1 relay", pos: 3, len: 1, bits: Some((1, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for VacuumPump1Relay {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &VACUUM_PUMP1_RELAY[0] },
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &VACUUM_PUMP1_RELAY[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x02), item: &VACUUM_PUMP1_RELAY[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Vacuum pump 2 relay
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VacuumPump2Relay(pub bool);

pub(crate) static VACUUM_PUMP2_RELAY: [Item; 3] = [
    Item { name: "Vacuum pump 2 relay", pos: 32, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Vacuum pump 2 relay", pos: 46, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Vacuum pump 2 relay", pos: 3, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for VacuumPump2Relay {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &VACUUM_PUMP2_RELAY[0] },
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &VACUUM_PUMP2_RELAY[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x02), item: &VACUUM_PUMP2_RELAY[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Vacuum pump buzzer (CAN request)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VacuumPumpBuzzerCanRequest(pub bool);

pub(crate) static VACUUM_PUMP_BUZZER_CAN_REQUEST: [Item; 1] = [
    Item { name: "Vacuum pump buzzer (CAN request)", pos: 39, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for VacuumPumpBuzzerCanRequest {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &VACUUM_PUMP_BUZZER_CAN_REQUEST[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Vacuum pump failsafe relay
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VacuumPumpFailsafeRelay(pub bool);

pub(crate) static VACUUM_PUMP_FAILSAFE_RELAY: [Item; 3] = [
    Item { name: "Vacuum pump failsafe relay", pos: 32, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Vacuum pump failsafe relay", pos: 46, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Vacuum pump failsafe relay", pos: 3, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for VacuumPumpFailsafeRelay {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &VACUUM_PUMP_FAILSAFE_RELAY[0] },
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &VACUUM_PUMP_FAILSAFE_RELAY[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x02), item: &VACUUM_PUMP_FAILSAFE_RELAY[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Vacuum pump motor (monitor)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VacuumPumpMotorMonitor(pub bool);

pub(crate) static VACUUM_PUMP_MOTOR_MONITOR: [Item; 1] = [
    Item { name: "Vacuum pump motor (monitor)", pos: 39, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for VacuumPumpMotorMonitor {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &VACUUM_PUMP_MOTOR_MONITOR[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Vacuum pump negative PRS. sensor
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VacuumPumpNegativePrsSensor(pub crate::db::units::Volts);

pub(crate) static VACUUM_PUMP_NEGATIVE_PRS_SENSOR: [Item; 1] = [
    Item { name: "Vacuum pump negative PRS. sensor", pos: 38, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for VacuumPumpNegativePrsSensor {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &VACUUM_PUMP_NEGATIVE_PRS_SENSOR[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// Vacuum pump relay 1 (diag)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VacuumPumpRelay1Diag(pub bool);

pub(crate) static VACUUM_PUMP_RELAY1_DIAG: [Item; 1] = [
    Item { name: "Vacuum pump relay 1 (diag)", pos: 39, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for VacuumPumpRelay1Diag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &VACUUM_PUMP_RELAY1_DIAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Vacuum pump relay 2 (diag)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VacuumPumpRelay2Diag(pub bool);

pub(crate) static VACUUM_PUMP_RELAY2_DIAG: [Item; 1] = [
    Item { name: "Vacuum pump relay 2 (diag)", pos: 39, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for VacuumPumpRelay2Diag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &VACUUM_PUMP_RELAY2_DIAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Vehicle acceleration
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VehicleAcceleration(pub crate::db::units::MetresPerSecondSquared);

pub(crate) static VEHICLE_ACCELERATION: [Item; 1] = [
    Item { name: "Vehicle acceleration", pos: 73, len: 2, bits: None, unit: Unit::MetresPerSecondSquared, conv: Conv::Linear { factor: 0.01, offset: -100.0, decimals: 2 } },
];

impl crate::item::Reading for VehicleAcceleration {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &VEHICLE_ACCELERATION[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::MetresPerSecondSquared(n)))
    }
}

/// Vehicle speed
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VehicleSpeed(pub crate::db::units::KmPerHour);

pub(crate) static VEHICLE_SPEED: [Item; 2] = [
    Item { name: "Vehicle speed", pos: 21, len: 1, bits: None, unit: Unit::KmPerHour, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
    Item { name: "Vehicle speed", pos: 11, len: 1, bits: None, unit: Unit::KmPerHour, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for VehicleSpeed {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000, 0x8001, 0x8003, 0x0003, 0x0004, 0x0005], source: Source::FreezeFrame, item: &VEHICLE_SPEED[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x22), item: &VEHICLE_SPEED[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::KmPerHour(n)))
    }
}

/// Vehicle speed(signed)
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VehicleSpeedSigned(pub crate::db::units::KmPerHour);

pub(crate) static VEHICLE_SPEED_SIGNED: [Item; 1] = [
    Item { name: "Vehicle speed(signed)", pos: 7, len: 1, bits: None, unit: Unit::KmPerHour, conv: Conv::Linear { factor: 0.5, offset: -60.0, decimals: 1 } },
];

impl crate::item::Reading for VehicleSpeedSigned {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x20), item: &VEHICLE_SPEED_SIGNED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::KmPerHour(n)))
    }
}

/// VIN verification accord
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VinVerificationAccord(pub bool);

pub(crate) static VIN_VERIFICATION_ACCORD: [Item; 1] = [
    Item { name: "VIN verification accord", pos: 60, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "NG"), (1, 1, "OK")]) },
];

impl crate::item::Reading for VinVerificationAccord {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &VIN_VERIFICATION_ACCORD[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "OK" => Some(Self(true)),
            "NG" => Some(Self(false)),
            _ => None,
        }
    }
}

/// VIN write complete flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VinWriteCompleteFlag(pub bool);

pub(crate) static VIN_WRITE_COMPLETE_FLAG: [Item; 1] = [
    Item { name: "VIN write complete flag", pos: 60, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "NG"), (1, 1, "OK")]) },
];

impl crate::item::Reading for VinWriteCompleteFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &VIN_WRITE_COMPLETE_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "OK" => Some(Self(true)),
            "NG" => Some(Self(false)),
            _ => None,
        }
    }
}

/// VP:Brake W/L drive state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VpBrakeWLDriveState(pub bool);

pub(crate) static VP_BRAKE_WL_DRIVE_STATE: [Item; 1] = [
    Item { name: "VP:Brake W/L drive state", pos: 3, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for VpBrakeWLDriveState {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x40), item: &VP_BRAKE_WL_DRIVE_STATE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// VP Brake warning lamp (CAN req.)
/// VP:Brake warning lamp (CAN req.)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VpBrakeWarningLampCanReq(pub bool);

pub(crate) static VP_BRAKE_WARNING_LAMP_CAN_REQ: [Item; 3] = [
    Item { name: "VP Brake warning lamp (CAN req.)", pos: 39, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "VP:Brake warning lamp (CAN req.)", pos: 54, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "VP:Brake warning lamp (CAN req.)", pos: 77, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for VpBrakeWarningLampCanReq {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &VP_BRAKE_WARNING_LAMP_CAN_REQ[0] },
        At { versions: &[0x8001], source: Source::FreezeFrame, item: &VP_BRAKE_WARNING_LAMP_CAN_REQ[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &VP_BRAKE_WARNING_LAMP_CAN_REQ[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// VP:Buzzer (CAN request)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VpBuzzerCanRequest(pub bool);

pub(crate) static VP_BUZZER_CAN_REQUEST: [Item; 2] = [
    Item { name: "VP:Buzzer (CAN request)", pos: 54, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "VP:Buzzer (CAN request)", pos: 77, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for VpBuzzerCanRequest {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001], source: Source::FreezeFrame, item: &VP_BUZZER_CAN_REQUEST[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &VP_BUZZER_CAN_REQUEST[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// VP:Buzzer drive state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VpBuzzerDriveState(pub bool);

pub(crate) static VP_BUZZER_DRIVE_STATE: [Item; 1] = [
    Item { name: "VP:Buzzer drive state", pos: 3, len: 1, bits: Some((3, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for VpBuzzerDriveState {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x40), item: &VP_BUZZER_DRIVE_STATE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// VP:Control relay1 (diag)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VpControlRelay1Diag(pub bool);

pub(crate) static VP_CONTROL_RELAY1_DIAG: [Item; 2] = [
    Item { name: "VP:Control relay1 (diag)", pos: 54, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "VP:Control relay1 (diag)", pos: 77, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for VpControlRelay1Diag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001], source: Source::FreezeFrame, item: &VP_CONTROL_RELAY1_DIAG[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &VP_CONTROL_RELAY1_DIAG[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// VP:Control relay 1 state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VpControlRelay1State(pub bool);

pub(crate) static VP_CONTROL_RELAY1_STATE: [Item; 1] = [
    Item { name: "VP:Control relay 1 state", pos: 3, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for VpControlRelay1State {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x40), item: &VP_CONTROL_RELAY1_STATE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// VP:Control relay2 (diag)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VpControlRelay2Diag(pub bool);

pub(crate) static VP_CONTROL_RELAY2_DIAG: [Item; 2] = [
    Item { name: "VP:Control relay2 (diag)", pos: 54, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "VP:Control relay2 (diag)", pos: 77, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for VpControlRelay2Diag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001], source: Source::FreezeFrame, item: &VP_CONTROL_RELAY2_DIAG[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &VP_CONTROL_RELAY2_DIAG[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// VP:Control relay 2 state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VpControlRelay2State(pub bool);

pub(crate) static VP_CONTROL_RELAY2_STATE: [Item; 1] = [
    Item { name: "VP:Control relay 2 state", pos: 3, len: 1, bits: Some((5, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for VpControlRelay2State {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x40), item: &VP_CONTROL_RELAY2_STATE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// VP:Fail safe relay
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VpFailSafeRelay(pub bool);

pub(crate) static VP_FAIL_SAFE_RELAY: [Item; 2] = [
    Item { name: "VP:Fail safe relay", pos: 54, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "VP:Fail safe relay", pos: 77, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for VpFailSafeRelay {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001], source: Source::FreezeFrame, item: &VP_FAIL_SAFE_RELAY[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &VP_FAIL_SAFE_RELAY[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// VP:Failsafe relay state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VpFailsafeRelayState(pub bool);

pub(crate) static VP_FAILSAFE_RELAY_STATE: [Item; 1] = [
    Item { name: "VP:Failsafe relay state", pos: 3, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for VpFailsafeRelayState {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x40), item: &VP_FAILSAFE_RELAY_STATE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// VP Master cylinder PRS. sensor
/// VP:Master cylinder PRS. sensor
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VpMasterCylinderPrsSensor(pub crate::db::units::Volts);

pub(crate) static VP_MASTER_CYLINDER_PRS_SENSOR: [Item; 4] = [
    Item { name: "VP Master cylinder PRS. sensor", pos: 41, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
    Item { name: "VP:Master cylinder PRS. sensor", pos: 56, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
    Item { name: "VP:Master cylinder PRS. sensor", pos: 79, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
    Item { name: "VP:Master cylinder PRS. sensor", pos: 5, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for VpMasterCylinderPrsSensor {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &VP_MASTER_CYLINDER_PRS_SENSOR[0] },
        At { versions: &[0x8001], source: Source::FreezeFrame, item: &VP_MASTER_CYLINDER_PRS_SENSOR[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &VP_MASTER_CYLINDER_PRS_SENSOR[2] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x40), item: &VP_MASTER_CYLINDER_PRS_SENSOR[3] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// VP:Motor drive state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VpMotorDriveState(pub bool);

pub(crate) static VP_MOTOR_DRIVE_STATE: [Item; 1] = [
    Item { name: "VP:Motor drive state", pos: 3, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for VpMotorDriveState {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x40), item: &VP_MOTOR_DRIVE_STATE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// VP:Motor (monitor)
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VpMotorMonitor(pub bool);

pub(crate) static VP_MOTOR_MONITOR: [Item; 2] = [
    Item { name: "VP:Motor (monitor)", pos: 54, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "VP:Motor (monitor)", pos: 77, len: 1, bits: Some((4, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for VpMotorMonitor {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001], source: Source::FreezeFrame, item: &VP_MOTOR_MONITOR[0] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &VP_MOTOR_MONITOR[1] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// VP:Negative presser sensor
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VpNegativePresserSensor(pub crate::db::units::Volts);

pub(crate) static VP_NEGATIVE_PRESSER_SENSOR: [Item; 1] = [
    Item { name: "VP:Negative presser sensor", pos: 53, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for VpNegativePresserSensor {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8001], source: Source::FreezeFrame, item: &VP_NEGATIVE_PRESSER_SENSOR[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// VP:Negative pressure sensor
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VpNegativePressureSensor(pub crate::db::units::Volts);

pub(crate) static VP_NEGATIVE_PRESSURE_SENSOR: [Item; 1] = [
    Item { name: "VP:Negative pressure sensor", pos: 2, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.1, offset: 0.0, decimals: 1 } },
];

impl crate::item::Reading for VpNegativePressureSensor {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x40), item: &VP_NEGATIVE_PRESSURE_SENSOR[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// VP negative pressure sensor vol.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VpNegativePressureSensorVol(pub crate::db::units::Volts);

pub(crate) static VP_NEGATIVE_PRESSURE_SENSOR_VOL: [Item; 1] = [
    Item { name: "VP negative pressure sensor vol.", pos: 8, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.02, offset: 0.0, decimals: 2 } },
];

impl crate::item::Reading for VpNegativePressureSensorVol {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x03), item: &VP_NEGATIVE_PRESSURE_SENSOR_VOL[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// VP relay2 voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VpRelay2Voltage(pub crate::db::units::Volts);

pub(crate) static VP_RELAY2_VOLTAGE: [Item; 1] = [
    Item { name: "VP relay2 voltage", pos: 11, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.02, offset: 0.0, decimals: 2 } },
];

impl crate::item::Reading for VpRelay2Voltage {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x03), item: &VP_RELAY2_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// VP Vehicle speed
/// VP:Vehicle speed
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct VpVehicleSpeed(pub crate::db::units::KmPerHour);

pub(crate) static VP_VEHICLE_SPEED: [Item; 4] = [
    Item { name: "VP Vehicle speed", pos: 40, len: 1, bits: None, unit: Unit::KmPerHour, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
    Item { name: "VP:Vehicle speed", pos: 55, len: 1, bits: None, unit: Unit::KmPerHour, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
    Item { name: "VP:Vehicle speed", pos: 78, len: 1, bits: None, unit: Unit::KmPerHour, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
    Item { name: "VP:Vehicle speed", pos: 4, len: 1, bits: None, unit: Unit::KmPerHour, conv: Conv::Linear { factor: 1.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for VpVehicleSpeed {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &VP_VEHICLE_SPEED[0] },
        At { versions: &[0x8001], source: Source::FreezeFrame, item: &VP_VEHICLE_SPEED[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &VP_VEHICLE_SPEED[2] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x40), item: &VP_VEHICLE_SPEED[3] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::KmPerHour(n)))
    }
}

/// Water pump relay
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WaterPumpRelay(pub bool);

pub(crate) static WATER_PUMP_RELAY: [Item; 3] = [
    Item { name: "Water pump relay", pos: 31, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Water pump relay", pos: 45, len: 1, bits: Some((2, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
    Item { name: "Water pump relay", pos: 3, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for WaterPumpRelay {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8000], source: Source::FreezeFrame, item: &WATER_PUMP_RELAY[0] },
        At { versions: &[0x8001, 0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::FreezeFrame, item: &WATER_PUMP_RELAY[1] },
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x02), item: &WATER_PUMP_RELAY[2] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// Water temperature sensor voltage
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WaterTemperatureSensorVoltage(pub crate::db::units::Volts);

pub(crate) static WATER_TEMPERATURE_SENSOR_VOLTAGE: [Item; 1] = [
    Item { name: "Water temperature sensor voltage", pos: 9, len: 1, bits: None, unit: Unit::Volts, conv: Conv::Linear { factor: 0.02, offset: 0.0, decimals: 2 } },
];

impl crate::item::Reading for WaterTemperatureSensorVoltage {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x03), item: &WATER_TEMPERATURE_SENSOR_VOLTAGE[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Volts(n)))
    }
}

/// WP cooling condition flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WpCoolingConditionFlag(pub bool);

pub(crate) static WP_COOLING_CONDITION_FLAG: [Item; 1] = [
    Item { name: "WP cooling condition flag", pos: 6, len: 1, bits: Some((7, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for WpCoolingConditionFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &WP_COOLING_CONDITION_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// WP Intermittence drive flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WpIntermittenceDriveFlag(pub bool);

pub(crate) static WP_INTERMITTENCE_DRIVE_FLAG: [Item; 1] = [
    Item { name: "WP Intermittence drive flag", pos: 5, len: 1, bits: Some((0, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for WpIntermittenceDriveFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &WP_INTERMITTENCE_DRIVE_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// WP low temp. intermittent flag
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WpLowTempIntermittentFlag(pub bool);

pub(crate) static WP_LOW_TEMP_INTERMITTENT_FLAG: [Item; 1] = [
    Item { name: "WP low temp. intermittent flag", pos: 6, len: 1, bits: Some((6, 1)), unit: Unit::None, conv: Conv::Labels(&[(0, 0, "OFF"), (1, 1, "ON")]) },
];

impl crate::item::Reading for WpLowTempIntermittentFlag {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x23), item: &WP_LOW_TEMP_INTERMITTENT_FLAG[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        match value.label()? {
            "ON" => Some(Self(true)),
            "OFF" => Some(Self(false)),
            _ => None,
        }
    }
}

/// WP speed
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct WpSpeed(pub crate::db::units::Rpm);

pub(crate) static WP_SPEED: [Item; 1] = [
    Item { name: "WP speed", pos: 7, len: 1, bits: None, unit: Unit::Rpm, conv: Conv::Linear { factor: 50.0, offset: 0.0, decimals: 0 } },
];

impl crate::item::Reading for WpSpeed {
    const ECU: Ecu = Ecu::EvEcu;
    const AT: &'static [At] = &[
        At { versions: &[0x8003, 0x0003, 0x0004, 0x0005, 0x0100, 0x0200, 0x0300, 0x0400], source: Source::Record(0x24), item: &WP_SPEED[0] },
    ];
    fn decode(value: &Value) -> Option<Self> {
        value.number().map(|n| Self(crate::db::units::Rpm(n)))
    }
}
