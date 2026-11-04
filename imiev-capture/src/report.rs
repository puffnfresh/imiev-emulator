//! Plain-text reports

use crate::bus::Bus;
use crate::item::{self, Item, Variant};
use crate::kwp::{self, describe, hex, Dtc, Ecu, Request};
use crate::openport::Transport;

pub fn ecus(name: &str) -> Result<Vec<Ecu>, String> {
    match name {
        "all" => Ok(Ecu::ALL.to_vec()),
        _ => Ecu::from_name(name).map(|e| vec![e]).ok_or(format!("unknown ECU `{name}`")),
    }
}

pub async fn info<T: Transport>(bus: &mut Bus<T>, ecus: &[Ecu]) -> String {
    let mut text = String::new();
    for &ecu in ecus {
        text += &format!("{}:\n", ecu.name());
        match bus.identification(ecu).await {
            Ok(lines) => lines.iter().for_each(|(label, id)| text += &format!("  {label}: {id}\n")),
            Err(_) => text += "  no answer\n",
        }
    }
    text
}

pub async fn dtcs<T: Transport>(bus: &mut Bus<T>, ecus: &[Ecu]) -> String {
    let mut text = String::new();
    for &ecu in ecus {
        text += &dtc_lines(bus, ecu, "stored").await.unwrap_or_else(|e| format!("{}: {e}\n", ecu.name()));
    }
    text
}

pub async fn clear<T: Transport>(bus: &mut Bus<T>, ecu: Ecu) -> Result<String, String> {
    if !ecu.can_clear() {
        return Err(format!("can't clear the {}'s DTCs", ecu.name()));
    }
    let mut text = dtc_lines(bus, ecu, "before").await?;
    match bus.clear_dtcs(ecu).await {
        Ok(()) => text += &format!("{}: cleared\n", ecu.name()),
        Err(e) => return Err(format!("{text}{e}")),
    }
    Ok(text + &dtc_lines(bus, ecu, "after").await.unwrap_or_else(|e| format!("{}: {e}\n", ecu.name())))
}

pub async fn dtc_lines<T: Transport>(bus: &mut Bus<T>, ecu: Ecu, label: &str) -> Result<String, String> {
    let dtcs = bus.dtcs(ecu).await?;
    if dtcs.is_empty() {
        return Ok(format!("{} {label}: no DTCs\n", ecu.name()));
    }
    let mut text = String::new();
    let variant = variant(bus, ecu, &mut text).await;
    for d in dtcs {
        text += &match item::dtc_description(d.code) {
            Some(description) => format!("{} {label}: {d} {description}\n", ecu.name()),
            None => format!("{} {label}: {d}\n", ecu.name()),
        };
        if ecu.can_read_dtc_record() {
            text += &freeze_frame(bus, ecu, d, variant).await?;
        }
    }
    Ok(text)
}

async fn freeze_frame<T: Transport>(bus: &mut Bus<T>, ecu: Ecu, dtc: Dtc, variant: Option<&Variant>) -> Result<String, String> {
    let pdu = bus.ask(ecu, Request::ReadDtcRecord(dtc.code)).await?;
    Ok(match (kwp::parse_dtc_record(dtc.code, &pdu), variant) {
        (None, _) => format!("  freeze frame: {} refused ReadDtcRecord: {}\n", ecu.name(), describe(&pdu)),
        (Some(None), _) => "  no freeze frame\n".into(),
        (Some(Some(record)), Some(variant)) => {
            format!("  freeze frame: status {:02x} ({})\n", record.status, kwp::state(record.status))
                + &items(variant.freeze_frame(), &pdu)
        }
        (Some(Some(record)), None) => {
            let mut text = format!("  freeze frame: {record}\n");
            for (i, row) in record.snapshot.chunks(16).enumerate() {
                text += &format!("    {:02x}: {}\n", i * 16, hex(row));
            }
            text
        }
    })
}

pub async fn data<T: Transport>(bus: &mut Bus<T>, ecus: &[Ecu], all: bool) -> String {
    let mut text = String::new();
    for &ecu in ecus {
        text += &records(bus, ecu, all).await.unwrap_or_else(|e| format!("{}: {e}\n", ecu.name()));
    }
    text
}

pub async fn records<T: Transport>(bus: &mut Bus<T>, ecu: Ecu, all: bool) -> Result<String, String> {
    let mut text = format!("{}:\n", ecu.name());
    let variant = variant(bus, ecu, &mut text).await;
    let lids: Vec<u8> = match variant {
        Some(v) if !all => v.records.iter().map(|(lid, _)| *lid).collect(),
        _ => (0..=0xff).collect(),
    };
    for lid in lids {
        let req = Request::ReadLocal(lid);
        let pdu = match bus.ask(ecu, req).await {
            Ok(pdu) => pdu,
            Err(e) if bus.lost => return Err(e),
            Err(e) => {
                text += &format!("  {lid:02x}: {e}\n");
                continue;
            }
        };
        match req.answer(&pdu) {
            Some([_, record @ ..]) => {
                text += &format!("  {lid:02x}: {}\n", hex(record));
                if let Some(variant) = variant {
                    text += &items(variant.record(lid), &pdu);
                }
            }
            _ if variant.is_some() && !all => text += &format!("  {lid:02x}: refused: {}\n", describe(&pdu)),
            _ => {}
        }
    }
    Ok(text)
}

pub async fn actuators<T: Transport>(bus: &mut Bus<T>, ecus: &[Ecu]) -> String {
    let mut text = String::new();
    for &ecu in ecus {
        let mut note = String::new();
        text += &match variant(bus, ecu, &mut note).await {
            Some(v) if v.actuators.is_empty() => format!("{}: no actuator tests\n", ecu.name()),
            Some(v) => {
                let header = format!("{} (diag version {:04x}):\n", ecu.name(), v.diag_version);
                v.actuators.iter().fold(header, |text, (lid, name)| text + &format!("  {lid:02x}: {name}\n"))
            }
            None => note + &format!("{}: no MUT-III tables for it\n", ecu.name()),
        };
    }
    text
}

fn items(items: impl Iterator<Item = &'static Item>, pdu: &[u8]) -> String {
    items.filter_map(|item| Some(format!("    {}: {}\n", item.name, item.value(pdu)?))).collect()
}

async fn variant<T: Transport>(bus: &mut Bus<T>, ecu: Ecu, text: &mut String) -> Option<&'static Variant> {
    if !item::covers(ecu) {
        return None;
    }
    let pdu = bus.ask(ecu, Request::Identify(0x87)).await.ok()?;
    let note = match item::diag_version(&pdu) {
        None => format!("no diag version in 1A 87 ({})", describe(&pdu)),
        Some(version) => match item::variant(ecu, version) {
            Some(variant) => return Some(variant),
            None => format!("diag version {version:04x} has no MUT-III tables"),
        },
    };
    *text += &format!("{}: {note}; showing raw bytes\n", ecu.name());
    None
}
