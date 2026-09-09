//! Microwire (93C86) serial EEPROM. Bit-banged, no SPI.

use mh8106f::System;

use crate::{CanBus, Part};

const P7DATA: u32 = 0x0080_0707;

const CS: u8 = 0x80;
const SK: u8 = 0x40;
const DI: u8 = 0x20;
const DO: u8 = 0x10;

const NWORDS: usize = 1024;
const ADDR_BITS: u32 = 10;

#[derive(PartialEq, Clone, Copy)]
enum Phase {
    Idle,
    Command, // shifting start + opcode + address (+ data for write)
    ReadData,
}

pub struct Eeprom {
    store: [u16; NWORDS],
    prev_cs: bool,
    prev_sk: bool,
    do_bit: bool,
    write_enabled: bool,

    phase: Phase,
    bits_seen: u32, // bits shifted this frame since the start bit
    shreg: u32,     // accumulated opcode+address bits
    got_start: bool,

    opcode: u8,
    addr: u16,    // 10-bit word address (auto-increments on sequential read)
    data_in: u16, // 16-bit data being shifted for a write
    out_word: u16,
    out_idx: u32, // 0 => dummy leading 0; 1..=16 => data bit index
}

impl Default for Eeprom {
    fn default() -> Self {
        Eeprom {
            store: Self::initial_image(),
            prev_cs: false,
            prev_sk: false,
            do_bit: false,
            write_enabled: false,
            phase: Phase::Idle,
            bits_seen: 0,
            shreg: 0,
            got_start: false,
            opcode: 0,
            addr: 0,
            data_in: 0,
            out_word: 0,
            out_idx: 0,
        }
    }
}

impl Eeprom {
    /// A valid, CRC-consistent NVM image.
    fn initial_image() -> [u16; NWORDS] {
        let mut s = [0u16; NWORDS];
        s[14] = 0x97ee;
        s[15] = 0x0e01; // Block A CRC32
        s[62] = 0xe764;
        s[63] = 0xad43; // Block B CRC32
        s[510] = 0x1c76;
        s[511] = 0xc138; // Block C1 CRC32
        s[1022] = 0x61f5;
        s[1023] = 0xdde6; // Block C2 CRC32
        s
    }

    fn start_frame(&mut self) {
        self.phase = Phase::Command;
        self.bits_seen = 0;
        self.shreg = 0;
        self.got_start = false;
        self.out_idx = 0;
    }

    fn clock_bit(&mut self, di: bool) {
        match self.phase {
            Phase::Idle => {}
            Phase::Command => {
                if !self.got_start {
                    // Wait for the start bit (a 1).
                    if di {
                        self.got_start = true;
                        self.bits_seen = 0;
                        self.shreg = 0;
                    }
                    return;
                }
                self.shreg = (self.shreg << 1) | (di as u32);
                self.bits_seen += 1;
                if self.bits_seen == 2 {
                    self.opcode = (self.shreg & 0x3) as u8;
                }
                // Full command received: opcode(2) + address(ADDR_BITS).
                if self.bits_seen == 2 + ADDR_BITS {
                    let addr = (self.shreg & ((1 << ADDR_BITS) - 1)) as u16;
                    self.addr = addr;
                    self.opcode = ((self.shreg >> ADDR_BITS) & 0x3) as u8;
                    match self.opcode {
                        0b10 => {
                            // READ: present dummy 0 now, stream data on next edges.
                            self.out_word = self.store[(addr as usize) % NWORDS];
                            self.out_idx = 0;
                            self.do_bit = false; // dummy leading 0
                            self.phase = Phase::ReadData;
                        }
                        0b01 => self.data_in = 0, // WRITE: 16 data bits follow
                        0b11 => {
                            // ERASE addr -> 0xFFFF
                            if self.write_enabled {
                                self.store[(addr as usize) % NWORDS] = 0xffff;
                            }
                            self.do_bit = true; // ready
                            self.phase = Phase::Idle;
                        }
                        0b00 => {
                            // EWEN/EWDS/ERAL/WRAL selected by the top address bits.
                            match (addr >> (ADDR_BITS - 2)) & 0x3 {
                                0b11 => self.write_enabled = true,  // EWEN
                                0b00 => self.write_enabled = false, // EWDS
                                0b10 => {
                                    if self.write_enabled {
                                        self.store = [0xffff; NWORDS]; // ERAL
                                    }
                                }
                                _ => {}
                            }
                            self.do_bit = true;
                            self.phase = Phase::Idle;
                        }
                        _ => {}
                    }
                } else if self.opcode == 0b01 && self.bits_seen > 2 + ADDR_BITS {
                    // WRITE data phase: collect 16 bits after the command.
                    self.data_in = (self.data_in << 1) | (di as u16);
                    if self.bits_seen == 2 + ADDR_BITS + 16 {
                        if self.write_enabled {
                            self.store[(self.addr as usize) % NWORDS] = self.data_in;
                        }
                        self.do_bit = true; // ready after write
                        self.phase = Phase::Idle;
                    }
                }
            }
            Phase::ReadData => {
                if self.out_idx < 16 {
                    self.do_bit = (self.out_word >> (15 - self.out_idx)) & 1 != 0;
                    self.out_idx += 1;
                } else {
                    self.addr = self.addr.wrapping_add(1) & ((1 << ADDR_BITS) - 1);
                    self.out_word = self.store[(self.addr as usize) % NWORDS];
                    self.do_bit = (self.out_word >> 15) & 1 != 0;
                    self.out_idx = 1;
                }
            }
        }
    }
}

impl Part for Eeprom {
    fn update(&mut self, chip: &mut System, _bus: &CanBus) {
        let v = chip.gpio_output(P7DATA);
        let cs = v & CS != 0;
        let sk = v & SK != 0;
        let di = v & DI != 0;

        if cs && !self.prev_cs {
            self.start_frame();
        } else if !cs && self.prev_cs {
            self.phase = Phase::Idle;
        }
        if cs && sk && !self.prev_sk {
            self.clock_bit(di);
        }
        self.prev_cs = cs;
        self.prev_sk = sk;

        chip.set_gpio_input(P7DATA, DO, if self.do_bit { DO } else { 0 });
    }
}
