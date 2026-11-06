//! Diagnostic trouble codes

use mh8106f::System;

pub struct DtcTable {
    pub base: u32,
    pub count: u32,
    pub table: u32,
}

pub const ECU_DTC: DtcTable = DtcTable { base: 0x0080_4a00, count: 216, table: 0x000e_4ae2 };
pub const BMU_DTC: DtcTable = DtcTable { base: 0x0080_4900, count: 181, table: 0x000e_0eaa };

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub struct Code(pub u16);

impl Code {
    pub const fn parse(label: &str) -> Code {
        let b = label.as_bytes();
        assert!(b.len() == 5, "a DTC label is five characters");
        let cat: u16 = match b[0] {
            b'P' => 0,
            b'C' => 1,
            b'B' => 2,
            b'U' => 3,
            _ => panic!("a DTC label starts with P, C, B or U"),
        };
        let mut n: u16 = 0;
        let mut i = 1;
        while i < 5 {
            let d = match b[i] {
                b'0'..=b'9' => b[i] - b'0',
                b'A'..=b'F' => b[i] - b'A' + 10,
                _ => panic!("a DTC label's number is upper-case hex"),
            };
            n = n << 4 | d as u16;
            i += 1;
        }
        assert!(n <= 0x3fff, "a DTC number's first digit is 0-3");
        Code(cat << 14 | n)
    }
}

impl core::fmt::Display for Code {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let cat = b"PCBU"[(self.0 >> 14) as usize] as char;
        write!(f, "{cat}{:04X}", self.0 & 0x3fff)
    }
}

impl core::fmt::Debug for Code {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(self, f)
    }
}

pub fn decode_dtc(code: u32) -> Code {
    Code(code as u16)
}

pub fn status(sys: &System, spec: &DtcTable, code: Code) -> Option<u8> {
    (0..spec.count)
        .find(|i| decode_dtc(sys.peek(spec.table + 2 * i, 2)) == code)
        .map(|i| sys.peek(spec.base + i, 1) as u8)
}

pub fn scan(sys: &System, spec: &DtcTable) -> Vec<(Code, bool)> {
    let mut out = Vec::new();
    for i in 0..spec.count {
        let st = sys.peek(spec.base + i, 1);
        if st & 3 != 0 {
            out.push((decode_dtc(sys.peek(spec.table + 2 * i, 2)), st & 2 != 0));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn code_round_trips_its_label() {
        for label in ["P0562", "P1A1A", "C200A", "B1234", "U1802", "U3FFF"] {
            assert_eq!(Code::parse(label).to_string(), label);
        }
        assert_eq!(Code::parse("U1802"), Code(0xd802));
        assert_eq!(Code::parse("C200A"), Code(0x600a));
    }
}
