//! Representation of data items and variants available on an i-MiEV.

use crate::db::units::Unit;
use crate::db::variant;

use crate::kwp::Ecu;

pub use crate::db::dtc::DTC_DESCRIPTIONS;

#[derive(Debug)]
pub struct Item {
    pub name: &'static str,
    pub pos: u16,
    pub len: u8,
    pub bits: Option<(u8, u8)>,
    pub unit: Unit,
    pub conv: Conv,
}

#[derive(Debug)]
pub enum Conv {
    Linear { factor: f64, offset: f64, decimals: u8 },
    Labels(&'static [(u32, u32, &'static str)]),
    Raw,
}

#[derive(Debug)]
pub struct Variant {
    pub ecu: Ecu,
    pub diag_version: u16,
    pub freeze_frame: &'static [&'static Item],
    pub records: &'static [(u8, &'static [&'static Item])],
    pub actuators: &'static [(u8, &'static str)],
}

pub trait Reading: Sized {
    const ECU: Ecu;
    const AT: &'static [At];
    fn decode(value: &Value) -> Option<Self>;
}

#[derive(Debug)]
pub struct At {
    pub versions: &'static [u16],
    pub source: Source,
    pub item: &'static Item,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Value {
    Number(f64),
    Label(&'static str),
    Raw(u32),
}

impl Value {
    pub fn number(&self) -> Option<f64> {
        match *self {
            Value::Number(value) => Some(value),
            _ => None,
        }
    }

    pub fn count(&self) -> Option<u32> {
        match *self {
            Value::Number(value) if value >= 0.0 && value.fract() == 0.0 => Some(value as u32),
            Value::Raw(n) => Some(n),
            _ => None,
        }
    }

    pub fn label(&self) -> Option<&'static str> {
        match *self {
            Value::Label(l) => Some(l),
            _ => None,
        }
    }

    pub fn raw(&self) -> Option<u32> {
        match *self {
            Value::Raw(n) => Some(n),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Source {
    FreezeFrame,
    Record(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReadError {
    Missing,
    WrongAnswer,
    Undecodable,
}

impl Variant {
    pub fn freeze_frame(&self) -> impl Iterator<Item = &'static Item> {
        self.freeze_frame.iter().copied()
    }

    pub fn record(&self, lid: u8) -> impl Iterator<Item = &'static Item> {
        let items = self.records.iter().find(|(l, _)| *l == lid).map_or(&[][..], |(_, items)| items);
        items.iter().copied()
    }

    fn at<R: Reading>(&self) -> impl Iterator<Item = &'static At> {
        let mine = R::ECU == self.ecu;
        R::AT.iter().filter(move |at| mine && at.versions.contains(&self.diag_version))
    }

    pub fn locate<R: Reading>(&self) -> Option<Source> {
        self.at::<R>().map(|at| at.source).max_by_key(|s| matches!(s, Source::Record(_)))
    }

    pub fn get<R: Reading>(&self, pdu: &[u8]) -> Result<R, ReadError> {
        if self.locate::<R>().is_none() {
            return Err(ReadError::Missing);
        }
        let source = match pdu {
            [0x57, ..] => Source::FreezeFrame,
            [0x61, lid, ..] => Source::Record(*lid),
            _ => return Err(ReadError::WrongAnswer),
        };
        let at = self.at::<R>().find(|at| at.source == source).ok_or(ReadError::WrongAnswer)?;
        let value = at.item.decode(pdu).ok_or(ReadError::WrongAnswer)?;
        R::decode(&value).ok_or(ReadError::Undecodable)
    }
}

pub fn diag_version(pdu: &[u8]) -> Option<u16> {
    match pdu {
        [0x5a, 0x87, _, _, hi, lo, ..] => Some(u16::from_be_bytes([*hi, *lo])),
        _ => None,
    }
}

fn variants(ecu: Ecu) -> &'static [Variant] {
    match ecu {
        Ecu::EvEcu => variant::EV_ECU,
        Ecu::Bmu => variant::BMU,
        _ => &[],
    }
}

pub fn covers(ecu: Ecu) -> bool {
    !variants(ecu).is_empty()
}

pub fn variant(ecu: Ecu, diag_version: u16) -> Option<&'static Variant> {
    variants(ecu).iter().find(|v| v.diag_version == diag_version)
}

pub fn dtc_description(code: u16) -> Option<&'static str> {
    DTC_DESCRIPTIONS.binary_search_by_key(&code, |(c, _)| *c).ok().map(|i| DTC_DESCRIPTIONS[i].1)
}

impl Item {
    fn raw(&self, pdu: &[u8]) -> Option<u32> {
        let bytes = pdu.get(self.pos as usize..self.pos as usize + self.len as usize)?;
        let raw = bytes.iter().fold(0u32, |n, &b| n << 8 | u32::from(b));
        Some(match self.bits {
            Some((bit, count)) => (raw >> bit) & ((1 << count) - 1),
            None => raw,
        })
    }

    pub fn decode(&self, pdu: &[u8]) -> Option<Value> {
        let raw = self.raw(pdu)?;
        Some(match self.conv {
            Conv::Linear { factor, offset, .. } => Value::Number(f64::from(raw) * factor + offset),
            Conv::Labels(labels) => match labels.iter().find(|(lo, hi, _)| (*lo..=*hi).contains(&raw)) {
                Some((_, _, label)) => Value::Label(label),
                None => Value::Raw(raw),
            },
            Conv::Raw => Value::Raw(raw),
        })
    }

    pub fn value(&self, pdu: &[u8]) -> Option<String> {
        let unit = match self.unit {
            Unit::None => String::new(),
            u => format!(" {}", u.symbol()),
        };
        Some(match (self.decode(pdu)?, &self.conv) {
            (Value::Number(value), Conv::Linear { decimals, .. }) => format!("{:.*}{unit}", *decimals as usize, value),
            (Value::Number(value), _) => format!("{value}{unit}"),
            (Value::Label(label), _) => format!("{label}{unit}"),
            (Value::Raw(raw), Conv::Raw) => format!("{raw:0width$x}{unit}", width = 2 * self.len as usize),
            (Value::Raw(raw), _) => format!("{raw}{unit}"),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_emulated_ecus_have_tables() {
        let ev = variant(Ecu::EvEcu, diag_version(&[0x5a, 0x87, 0x04, 0x85, 0x00, 0x03, 0xff]).unwrap()).unwrap();
        assert_eq!(ev.freeze_frame().count(), 152);
        assert_eq!(ev.actuators.len(), 13);
        let bmu = variant(Ecu::Bmu, 0x0002).unwrap();
        assert_eq!(bmu.freeze_frame().map(|i| i.name).collect::<Vec<_>>(), ["Odometer", "Ignition cycle", "Elapsed time", "Accumulated minute"]);
        assert!(variant(Ecu::Heater, 0x0002).is_none());
        assert!(variant(Ecu::EvEcu, 0x1234).is_none());
    }

    #[test]
    fn a_real_ev_ecu_has_tables() {
        let mut pdu = vec![0x5a, 0x87, 0x04, 0x85, 0x01, 0x00, 0xff, 0x00, 0x06, 0x90, 0x13, 0x08];
        pdu.extend(b"9499B131  ");
        assert_eq!(diag_version(&pdu), Some(0x0100));
        let ev = variant(Ecu::EvEcu, 0x0100).unwrap();
        assert_eq!(ev.freeze_frame().count(), 150);
        assert_eq!(ev.actuators.len(), 13);
    }

    #[test]
    fn decodes_the_bench_u1802_freeze_frame() {
        let ev = variant(Ecu::EvEcu, 0x0003).unwrap();
        let pdu = [
            0x57, 0x01, 0xd8, 0x02, 0xe0, 0x37, 0xa5, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x26, 0x03, 0x04, 0x0b, 0x64, 0x27, 0x10, 0x24, 0xc8, 0xa3, 0x9d,
        ];
        let value = |name| ev.freeze_frame().find(|i| i.name == name).unwrap().value(&pdu);
        assert_eq!(value("Odometer").unwrap(), "91168 km");
        assert_eq!(value("BMU:Charge status").unwrap(), "19.0 %");
        assert_eq!(value("Operation mode").unwrap(), "3");
        assert_eq!(value("Driving torque").unwrap(), "0 Nm");
        assert_eq!(value("MCU:Motor speed").unwrap(), "0 r/min");
        assert_eq!(value("Shift position"), None, "past the end of this answer");
    }

    #[test]
    fn decodes_bits_and_labels() {
        let ev = variant(Ecu::EvEcu, 0x0003).unwrap();
        let brake = ev.record(0x01).find(|i| i.name == "Brake SW").unwrap();
        assert_eq!(brake.value(&[0x61, 0x01, 0b0010_0000]).unwrap(), "ON");
        assert_eq!(brake.value(&[0x61, 0x01, 0b1101_1111]).unwrap(), "OFF");
    }

    #[test]
    fn describes_dtcs() {
        assert!(DTC_DESCRIPTIONS.is_sorted_by_key(|(c, _)| *c));
        assert_eq!(dtc_description(0x1a15), Some("High voltage circuit (1)"));
        assert_eq!(dtc_description(0x1a51), Some("EEPROM fail"));
    }

    #[test]
    fn reads_typed_values_from_a_freeze_frame() {
        use crate::db::datalist::ev_ecu::*;
        use crate::db::units::*;
        let ev = variant(Ecu::EvEcu, 0x0003).unwrap();
        let pdu = [
            0x57, 0x01, 0xd8, 0x02, 0xe0, 0x37, 0xa5, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x26, 0x03, 0x04, 0x0b, 0x64, 0x27, 0x10, 0x24, 0xc8, 0xa3, 0x9d,
        ];
        assert_eq!(ev.get(&pdu), Ok(Odometer(Km(91168.0))));
        assert_eq!(ev.get(&pdu), Ok(BmuChargeStatus(Percent(19.0))));
        assert_eq!(ev.get(&pdu), Ok(OperationMode(3)));
        assert_eq!(ev.get(&pdu), Ok(DrivingTorque(NewtonMetres(0.0))));
        assert_eq!(ev.get(&pdu), Ok(McuMotorSpeed(Rpm(0.0))));
        assert_eq!(ev.get::<ShiftPosition>(&pdu), Err(ReadError::WrongAnswer), "past the end of this answer");
    }

    #[test]
    fn reads_typed_values_from_a_record() {
        use crate::db::datalist::ev_ecu::BrakeSw;
        let ev = variant(Ecu::EvEcu, 0x0100).unwrap();
        assert_eq!(ev.locate::<BrakeSw>(), Some(Source::Record(0x01)));
        assert_eq!(ev.get(&[0x61, 0x01, 0b0010_0000]), Ok(BrakeSw(true)));
        assert_eq!(ev.get(&[0x61, 0x01, 0b1101_1111]), Ok(BrakeSw(false)));
        assert_eq!(ev.get::<BrakeSw>(&[0x61, 0x02, 0xff]), Err(ReadError::WrongAnswer), "another record");
    }

    #[test]
    fn locates_readings() {
        use crate::db::datalist::{bmu, ev_ecu};
        let ev = variant(Ecu::EvEcu, 0x0100).unwrap();
        let bmu = variant(Ecu::Bmu, 0x0002).unwrap();
        assert_eq!(ev.locate::<ev_ecu::Odometer>(), Some(Source::FreezeFrame));
        assert_eq!(bmu.locate::<bmu::BatteryTotalVoltage>(), Some(Source::Record(0x01)));
        assert_eq!(bmu.locate::<ev_ecu::BatteryTotalVoltage>(), None, "another ECU's reading");
        assert_eq!(bmu.get::<ev_ecu::BrakeSw>(&[0x61, 0x01, 0xff]), Err(ReadError::Missing));
    }

    #[test]
    fn leaves_out_encodings_of_another_type() {
        use crate::db::datalist::ev_ecu::ShiftPosition;
        let old = variant(Ecu::EvEcu, 0x8001).unwrap();
        assert!(old.freeze_frame().any(|i| i.name == "Shift position"));
        assert_eq!(old.locate::<ShiftPosition>(), None);
        assert_eq!(variant(Ecu::EvEcu, 0x0100).unwrap().locate::<ShiftPosition>(), Some(Source::FreezeFrame));
    }
}
