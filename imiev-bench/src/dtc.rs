//! Diagnostic trouble codes

use mh8106f::System;

pub struct DtcTable {
    pub base: u32,
    pub count: u32,
    pub table: u32,
}

pub const ECU_DTC: DtcTable = DtcTable { base: 0x0080_4a00, count: 216, table: 0x000e_4ae2 };
pub const BMU_DTC: DtcTable = DtcTable { base: 0x0080_4900, count: 181, table: 0x000e_0eaa };

pub fn decode_dtc(code: u32) -> String {
    let letter = ['P', 'C', 'B', 'U'][((code >> 14) & 3) as usize];
    let d1 = (code >> 12) & 3;
    format!("{letter}{d1}{:03X}", code & 0x0fff)
}

pub fn scan(sys: &System, spec: &DtcTable) -> Vec<(String, bool)> {
    let mut out = Vec::new();
    for i in 0..spec.count {
        let st = sys.peek(spec.base + i, 1);
        if st & 3 != 0 {
            out.push((decode_dtc(sys.peek(spec.table + 2 * i, 2)), st & 2 != 0));
        }
    }
    out
}
