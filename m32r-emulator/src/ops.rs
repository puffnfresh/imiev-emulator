//! Per-opcode-group instruction handlers for [`Cpu`].

use super::*;

impl Cpu {
    // op1=0 : register-register ALU / compare (16-bit)
    pub(crate) fn op1_0(&mut self, op3: u32, rd: usize, rs: usize) {
        match op3 {
            0 => {
                // SUBV
                self.c = sborrow_sub(self.r[rd], self.r[rs]);
                self.r[rd] = self.r[rd].wrapping_sub(self.r[rs]);
            }
            1 => {
                // SUBX (subtract with borrow)
                let bin = self.c as u32;
                let t = self.r[rd].wrapping_sub(self.r[rs]);
                self.c = (self.r[rd] < self.r[rs]) || (t < bin);
                self.r[rd] = t.wrapping_sub(bin);
            }
            2 => self.r[rd] = self.r[rd].wrapping_sub(self.r[rs]), // SUB
            3 => self.r[rd] = 0u32.wrapping_sub(self.r[rs]),       // NEG
            4 => {
                // CMP  C = Rsrc1 s< Rsrc2
                self.c = (self.r[rd] as i32) < (self.r[rs] as i32);
                self.cbr = self.c as u32;
            }
            5 => {
                // CMPU
                self.c = self.r[rd] < self.r[rs];
                self.cbr = self.c as u32;
            }
            8 => {
                // ADDV
                self.c = carry(self.r[rd], self.r[rs]);
                self.r[rd] = self.r[rd].wrapping_add(self.r[rs]);
            }
            9 => {
                // ADDX (add with carry)
                let cin = self.c as u32;
                let tmp = self.r[rd].wrapping_add(self.r[rs]);
                let tmp2 = tmp.wrapping_add(cin);
                self.c = carry(self.r[rd], self.r[rs]) || carry(tmp, cin);
                self.r[rd] = tmp2;
            }
            10 => self.r[rd] = self.r[rd].wrapping_add(self.r[rs]), // ADD
            11 => self.r[rd] = !self.r[rs],                         // NOT
            12 => self.r[rd] &= self.r[rs],                         // AND
            13 => self.r[rd] ^= self.r[rs],                         // XOR
            14 => self.r[rd] |= self.r[rs],                         // OR
            15 => {
                // BTST #bitpos, Rsrc  -- bitpos = rd(&7)
                let bitpos = (rd & 7) as u32;
                let tmp = (self.r[rs] & 0xff) & (1u32 << (7 - bitpos));
                self.c = tmp != 0;
                self.cbr = (tmp != 0) as u32;
            }
            _ => {}
        }
    }

    // op1=1 : shifts / moves / control transfers (16-bit)
    // The jump/trap forms are distinguished by BOTH the full first byte AND
    // op3 -- e.g. 0x1083 is MV R0,R3 (op3=8), NOT RTE. Gate on op3.
    pub(crate) fn op1_1<B: Bus>(&mut self, _bus: &mut B, b0: u8, op3: u32, rd: usize, rs: usize, pc: u32) {
        match op3 {
            0 => self.r[rd] >>= self.r[rs] & 31,                                 // SRL
            2 => self.r[rd] = ((self.r[rd] as i32) >> (self.r[rs] & 31)) as u32, // SRA
            4 => self.r[rd] <<= self.r[rs] & 31,                                 // SLL
            6 => self.r[rd] = self.r[rd].wrapping_mul(self.r[rs]),               // MUL
            8 => self.r[rd] = self.r[rs],                                        // MV
            9 => self.r[rd] = self.read_cr(rs as u32 & 7),                       // MVFC
            10 => self.write_cr((b0 & 7) as u32, self.r[rs]),                    // MVTC
            12 => {
                // JL (0x1E) / JMP (0x1F, RET=JMP R14)
                if b0 == 0x1E {
                    self.r[14] = (pc & PC_ALIGN).wrapping_add(4);
                }
                self.pc = self.r[rs] & PC_ALIGN;
            }
            13 => {
                // RTE = full opcode 0x10D6: return from EIT. Restore PC from BPC and
                // the current PSW fields (SM/IE/C) from their backups (BSM/BIE/BC),
                // re-enabling interrupts. Without this, IE stays 0 after the first
                // interrupt and the machine never takes another one.
                if b0 == 0x10 {
                    self.pc = self.bpc;
                    let bsm = (self.psw >> PSW_BSM) & 1;
                    let bie = (self.psw >> PSW_BIE) & 1;
                    let bc = (self.psw >> PSW_BC) & 1;
                    self.psw = (self.psw & !(1 << PSW_IE)) | (bie << PSW_IE); // restore IE
                    self.set_sm(bsm); // restore SM, banking R15
                    self.c = bc != 0;
                    self.in_eit = self.in_eit.saturating_sub(1);
                }
            }
            15
                // TRAP #op4  (op12=0x10)
                if b0 == 0x10 => {
                    self.bpc = pc.wrapping_add(4);
                    self.c = false;
                }
            _ => {}
        }
    }

    // op1=2 : loads / stores register-indirect (16-bit)
    pub(crate) fn op1_2<B: Bus>(&mut self, bus: &mut B, op3: u32, rd: usize, rs: usize) {
        // store Rdest to @Rsrc; address & data are read before any `bump`, so a
        // post-inc (`store` then `bump`) sees the old Rsrc and a pre-adjust (`bump`
        // then `store`) sees the new one -- matching the M32R order when rd == rs.
        let store = |cpu: &mut Cpu, bus: &mut B, wr: fn(&mut B, u32, u32)| {
            wr(bus, cpu.r[rs], cpu.r[rd]);
        };
        // load into Rdest from @Rsrc, widening the fetched value via `rdr`.
        let load = |cpu: &mut Cpu, bus: &mut B, rdr: fn(&mut B, u32) -> u32| {
            let a = cpu.r[rs];
            cpu.r[rd] = rdr(bus, a);
        };
        let bump = |cpu: &mut Cpu, n: i32| cpu.r[rs] = cpu.r[rs].wrapping_add(n as u32);
        match op3 {
            0 => store(self, bus, |b, a, v| b.w8(a, v as u8)),    // STB
            2 => store(self, bus, |b, a, v| b.w16(a, v as u16)),  // STH
            3 => {
                store(self, bus, |b, a, v| b.w16(a, v as u16));
                bump(self, 2);
            } // STH postinc
            4 => store(self, bus, |b, a, v| b.w32(a, v)), // ST
            5 => store(self, bus, |b, a, v| b.w32(a, v)), // UNLOCK (store)
            6 => {
                bump(self, 4);
                store(self, bus, |b, a, v| b.w32(a, v));
            } // ST preinc
            7 => {
                bump(self, -4);
                store(self, bus, |b, a, v| b.w32(a, v));
            } // ST predec / PUSH
            8 => load(self, bus, |b, a| sext8(b.r8(a))),   // LDB
            9 => load(self, bus, |b, a| b.r8(a) as u32),   // LDUB
            10 => load(self, bus, |b, a| sext16(b.r16(a))), // LDH
            11 => load(self, bus, |b, a| b.r16(a) as u32), // LDUH
            12 => load(self, bus, |b, a| b.r32(a)),        // LD
            13 => load(self, bus, |b, a| b.r32(a)),        // LOCK
            14 => {
                load(self, bus, |b, a| b.r32(a));
                bump(self, 4);
            } // LD postinc / POP
            _ => {}
        }
    }

    // op1=7 : branches REL8 / PSW ops / NOP (16-bit)
    pub(crate) fn op1_7<B: Bus>(&mut self, _bus: &mut B, b0: u8, b1: u8, pc: u32) {
        let rel8 = (pc & PC_ALIGN).wrapping_add(sext8(b1) << 2);
        match b0 {
            0x70 => {}                                       // NOP
            0x71 => self.psw |= b1 as u32,                   // SETPSW
            0x72 => self.psw &= b1 as u32,                   // CLRPSW
            0x7C => if self.c { self.pc = rel8 },            // BC
            0x7D => if !self.c { self.pc = rel8 },           // BNC
            0x7E => {
                // BL
                self.r[14] = (pc & PC_ALIGN).wrapping_add(4);
                self.pc = rel8;
            }
            0x7F => self.pc = rel8, // BRA
            _ => {}
        }
    }

    // op1=8 : 3-operand ALU with imm16 / CMPI (32-bit)
    pub(crate) fn op1_8(&mut self, op3: u32, rd: usize, rs: usize, hw1: u16) {
        let simm = sext16(hw1);
        let imm = hw1 as u32;
        match op3 {
            4 => {
                // CMPI Rsrc, #simm16
                self.c = (self.r[rs] as i32) < (simm as i32);
                self.cbr = self.c as u32;
            }
            5 => {
                // CMPUI
                self.c = self.r[rs] < imm;
                self.cbr = self.c as u32;
            }
            8 => {
                // ADDV3
                self.c = carry(self.r[rs], simm);
                self.r[rd] = self.r[rs].wrapping_add(simm);
            }
            10 => self.r[rd] = self.r[rs].wrapping_add(simm), // ADD3
            12 => self.r[rd] = self.r[rs] & imm,              // AND3
            13 => self.r[rd] = self.r[rs] ^ imm,              // XOR3
            14 => self.r[rd] = self.r[rs] | imm,              // OR3
            _ => {}
        }
    }

    // op1=9 : DIV/REM group (imm16=0), shift-by-imm16, LDI16 (32-bit)
    pub(crate) fn op1_9(&mut self, op3: u32, rd: usize, rs: usize, hw1: u16) {
        match op3 {
            0 => {
                if self.r[rs] != 0 {
                    self.r[rd] = ((self.r[rd] as i32).wrapping_div(self.r[rs] as i32)) as u32;
                }
            } // DIV
            1 => {
                if let Some(q) = self.r[rd].checked_div(self.r[rs]) {
                    self.r[rd] = q;
                }
            } // DIVU
            2 => {
                if self.r[rs] != 0 {
                    self.r[rd] = ((self.r[rd] as i32).wrapping_rem(self.r[rs] as i32)) as u32;
                }
            } // REM
            3 => {
                if self.r[rs] != 0 {
                    self.r[rd] %= self.r[rs];
                }
            } // REMU
            8 => self.r[rd] = self.r[rs] >> ((hw1 as u32) & 31),                    // SRL3
            10 => self.r[rd] = ((self.r[rs] as i32) >> ((hw1 as u32) & 31)) as u32, // SRA3
            12 => self.r[rd] = self.r[rs] << ((hw1 as u32) & 31),                   // SLL3
            15 => self.r[rd] = sext16(hw1),                                         // LDI #simm16
            _ => {}
        }
    }

    // op1=A : loads/stores with rel16 offset, BSET/BCLR (32-bit)
    pub(crate) fn op1_a<B: Bus>(&mut self, bus: &mut B, op3: u32, b0: u8, rd: usize, rs: usize, hw1: u16) {
        let off = sext16(hw1);
        let addr = self.r[rs].wrapping_add(off);
        match op3 {
            0 => bus.w8(addr, self.r[rd] as u8),   // STB
            2 => bus.w16(addr, self.r[rd] as u16), // STH
            4 => bus.w32(addr, self.r[rd]),        // ST
            6 => {
                // BSET #bitpos, @(rel16,Rsrc)
                let bitpos = (b0 & 7) as u32;
                let v = bus.r8(addr) | (1u8 << (7 - bitpos));
                bus.w8(addr, v);
            }
            7 => {
                // BCLR
                let bitpos = (b0 & 7) as u32;
                let v = bus.r8(addr) & !(1u8 << (7 - bitpos));
                bus.w8(addr, v);
            }
            8 => self.r[rd] = sext8(bus.r8(addr)),    // LDB
            9 => self.r[rd] = bus.r8(addr) as u32,    // LDUB
            10 => self.r[rd] = sext16(bus.r16(addr)), // LDH
            11 => self.r[rd] = bus.r16(addr) as u32,  // LDUH
            12 => self.r[rd] = bus.r32(addr),         // LD
            _ => {}
        }
    }

    // op1=B : conditional branches with rel16 (32-bit)
    pub(crate) fn op1_b(&mut self, op3: u32, _b0: u8, rd: usize, rs: usize, hw1: u16, pc: u32) {
        let target = (pc & PC_ALIGN).wrapping_add(sext16(hw1) << 2);
        let take = match op3 {
            0 => self.r[rd] == self.r[rs],       // BEQ
            1 => self.r[rd] != self.r[rs],       // BNE
            8 => self.r[rs] == 0,                 // BEQZ
            9 => self.r[rs] != 0,                 // BNEZ
            10 => (self.r[rs] as i32) < 0,        // BLTZ
            11 => (self.r[rs] as i32) >= 0,       // BGEZ
            12 => (self.r[rs] as i32) <= 0,       // BLEZ
            13 => (self.r[rs] as i32) > 0,        // BGTZ
            _ => false,
        };
        if take {
            self.pc = target;
        }
    }

    // op1=D : SETH or FP op (32-bit)
    pub(crate) fn op1_d<B: Bus>(&mut self, _bus: &mut B, b0: u8, b1: u8, rd: usize, _rs: usize, hw1: u16) {
        if b1 == 0xC0 {
            // SETH Rdest, #imm16
            self.r[rd] = (hw1 as u32) << 16;
            return;
        }
        // FP op. Decode fields from the 32-bit word.
        let frsrc = lo_nib(b0) as usize;
        let frsrc2 = lo_nib(b1) as usize;
        let fop3 = field(12, 4, hw1 as u32);
        let frdest = field(8, 4, hw1 as u32) as usize;
        let fop4 = field(4, 4, hw1 as u32);
        let a = f32::from_bits(self.r[frsrc]);
        let b = f32::from_bits(self.r[frsrc2]);
        match (fop3, fop4) {
            (0, 0) => self.r[frdest] = (a + b).to_bits(),   // FADD
            (0, 0x4) => self.r[frdest] = (a - b).to_bits(), // FSUB
            (0, 0xC) | (0, 0xD) => {
                self.r[frdest] = if a.is_nan() || b.is_nan() {
                    0x7fc0_0000
                } else if a < b {
                    0x8000_0000
                } else if a > b {
                    1
                } else {
                    0
                };
            }
            (1, 0) => self.r[frdest] = (a * b).to_bits(), // FMUL
            (2, 0) => self.r[frdest] = (a / b).to_bits(), // FDIV
            (3, 0) => {
                let d = f32::from_bits(self.r[frdest]);
                self.r[frdest] = (d + a * b).to_bits(); // FMADD
            }
            (3, 0x4) => {
                let d = f32::from_bits(self.r[frdest]);
                self.r[frdest] = (d - a * b).to_bits(); // FMSUB
            }
            (4, 0) => self.r[frdest] = (self.r[frsrc] as i32 as f32).to_bits(), // ITOF
            (4, 0x4) => self.r[frdest] = (self.r[frsrc] as f32).to_bits(),      // UTOF
            (4, 0x8) => self.r[frdest] = a as i32 as u32,                       // FTOI
            (4, 0xC) => self.r[frdest] = a as i32 as u32,                       // FTOS
            _ => {}
        }
    }

    // op1=F : NOP (0xF0) or REL24 branches (32-bit)
    pub(crate) fn op1_f(&mut self, b0: u8, b1: u8, hw1: u16, pc: u32) {
        if b0 == 0xF0 {
            return; // NOP
        }
        let disp = sext24(((b1 as u32) << 16) | (hw1 as u32));
        let target = (pc & PC_ALIGN).wrapping_add(disp << 2);
        match b0 {
            0xFC => if self.c { self.pc = target },  // BC
            0xFD => if !self.c { self.pc = target }, // BNC
            0xFE => {
                // BL
                self.r[14] = (pc & PC_ALIGN).wrapping_add(4);
                self.pc = target;
            }
            0xFF => self.pc = target, // BRA
            _ => {}
        }
    }
}
