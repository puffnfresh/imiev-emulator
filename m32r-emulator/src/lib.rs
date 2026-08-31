//! M32R-FP interpreter, ported from the Ghidra SLEIGH spec:
//! https://github.com/bonybrown/imiev-hacking-tools

/// Memory bus the CPU talks to. Implementors add the MMIO peripheral stubs.
pub trait Bus {
    fn r8(&mut self, addr: u32) -> u8;
    fn w8(&mut self, addr: u32, v: u8);

    // Big-endian multi-byte access, expressed in terms of r8/w8 so MMIO stubs
    // (which key on byte address + size) see every access.
    fn r16(&mut self, a: u32) -> u16 {
        ((self.r8(a) as u16) << 8) | (self.r8(a.wrapping_add(1)) as u16)
    }
    fn r32(&mut self, a: u32) -> u32 {
        ((self.r16(a) as u32) << 16) | (self.r16(a.wrapping_add(2)) as u32)
    }
    fn w16(&mut self, a: u32, v: u16) {
        self.w8(a, hi_byte(v));
        self.w8(a.wrapping_add(1), lo_byte(v));
    }
    fn w32(&mut self, a: u32, v: u32) {
        self.w16(a, (v >> 16) as u16);
        self.w16(a.wrapping_add(2), v as u16);
    }
}

#[derive(Clone, Copy)]
pub struct Cpu {
    pub r: [u32; 16], // R0..R15 ; R13=FP, R15=SP
    pub pc: u32,
    pub c: bool, // condition (carry/borrow/compare) flag = PSW[0]
    pub psw: u32,
    pub cbr: u32,
    pub spi: u32,
    pub spu: u32,
    pub bpc: u32,
    pub fpsr: u32,
    pub acc: i64, // DSP accumulator (M32R 56-bit; kept sign-extended in an i64)
    pub halted: bool,
    pub in_eit: u32, // EIT (interrupt) nesting depth; 0 = mainline
}

#[inline]
fn sext8(v: u8) -> u32 {
    v as i8 as i32 as u32
}
#[inline]
fn sext16(v: u16) -> u32 {
    v as i16 as i32 as u32
}
#[inline]
fn sext24(v: u32) -> u32 {
    // sign-extend a 24-bit value held in the low 24 bits
    ((v << 8) as i32 >> 8) as u32
}
#[inline]
fn carry(a: u32, b: u32) -> bool {
    (a as u64 + b as u64) > 0xffff_ffff
}
#[inline]
fn sborrow_sub(a: u32, b: u32) -> bool {
    // signed overflow of a - b
    (a as i32).checked_sub(b as i32).is_none()
}

// Bit-field extraction --------------------------------------------------------

/// Extract `n` bits at offset `lo`.
#[inline]
fn field(lo: u32, n: u32, x: u32) -> u32 {
    (x >> lo) & ((1u32 << n) - 1)
}
#[inline]
fn hi_byte(x: u16) -> u8 {
    (x >> 8) as u8
}
#[inline]
fn lo_byte(x: u16) -> u8 {
    x as u8
}
#[inline]
fn hi_nib(x: u8) -> u8 {
    x >> 4
}
#[inline]
fn lo_nib(x: u8) -> u8 {
    x & 0xf
}

impl Default for Cpu {
    fn default() -> Self {
        Self::new()
    }
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            r: [0; 16],
            pc: 0,
            c: false,
            psw: 0,
            cbr: 0,
            spi: 0,
            spu: 0,
            bpc: 0,
            fpsr: 0,
            acc: 0,
            halted: false,
            in_eit: 0,
        }
    }

    fn read_cr(&self, i: u32) -> u32 {
        match i & 7 {
            0 => (self.psw & !1) | (self.c as u32),
            1 => self.cbr,
            2 => self.spi,
            3 => self.spu,
            6 => self.bpc,
            7 => self.fpsr,
            _ => 0,
        }
    }
    fn write_cr(&mut self, i: u32, v: u32) {
        match i & 7 {
            0 => {
                self.psw = v;
                self.c = (v & 1) != 0;
            }
            1 => self.cbr = v,
            2 => self.spi = v,
            3 => self.spu = v,
            6 => self.bpc = v,
            7 => self.fpsr = v,
            _ => {}
        }
    }

    pub fn insn_len(b0: u8) -> u32 {
        let op1 = b0 >> 4;
        match op1 {
            0x0..=0x7 => 2,
            0x8..=0xE => 4,
            0xF => {
                if b0 == 0xF0 {
                    2
                } else {
                    4
                }
            }
            _ => 2,
        }
    }

    pub fn interrupts_enabled(&self) -> bool {
        (self.psw & 0x40) != 0
    }

    /// Take an EIT (interrupt) to `vector`, the M32R way: save PC->BPC, back up the
    /// current SM/IE/C into BSM/BIE/BC, then clear IE (mask further interrupts).
    /// The matching `RTE` restores them.
    pub fn take_interrupt(&mut self, vector: u32) {
        self.bpc = self.pc;
        let ie = (self.psw >> 6) & 1;
        let sm = (self.psw >> 7) & 1;
        let c = self.c as u32;
        self.psw = (self.psw & !0x0000_C100) | (sm << 15) | (ie << 14) | (c << 8);
        self.psw &= !0x40; // IE = 0
        self.pc = vector;
        self.in_eit += 1;
    }

    /// Execute one instruction. Returns false only on a hard decode failure.
    pub fn step<B: Bus>(&mut self, bus: &mut B) -> bool {
        let pc = self.pc;
        let hw0 = bus.r16(pc);
        let b0 = hi_byte(hw0);
        let b1 = lo_byte(hw0);
        let op1 = hi_nib(b0) as u32;
        let rd = lo_nib(b0) as usize; // Rdest / Rsrc1
        let op3 = hi_nib(b1) as u32;
        let rs = lo_nib(b1) as usize; // Rsrc / Rsrc2
        let len = Cpu::insn_len(b0);
        // second halfword (immediate / rel) for 32-bit forms
        let hw1 = if len == 4 { bus.r16(pc.wrapping_add(2)) } else { 0 };
        let next = pc.wrapping_add(len);
        self.pc = next; // default; branches overwrite

        // R0 is a normal GPR on M32R (not hardwired zero).
        match op1 {
            0x0 => self.op1_0(op3, rd, rs),
            0x1 => self.op1_1(bus, b0, op3, rd, rs, pc),
            0x2 => self.op1_2(bus, op3, rd, rs),
            0x3 => {
                // DSP 16x16 multiply / multiply-accumulate into the accumulator.
                // rd = Rsrc1, rs = Rsrc2 (both read); op3 selects the operation.
                // mul* replaces ACC; mac* adds to it. Product is placed <<16 so the
                // integer result is ACC[16:47] (read via mvfacmi).
                let a = self.r[rd];
                let b = self.r[rs];
                let hi = |x: u32| ((x >> 16) as i16) as i64; // high halfword, sign-extended
                let lo = |x: u32| (x as i16) as i64; // low halfword, sign-extended
                match op3 {
                    0 => self.acc = (hi(a) * hi(b)) << 16, // MULHI
                    1 => self.acc = (lo(a) * lo(b)) << 16, // MULLO
                    4 => self.acc = self.acc.wrapping_add((hi(a) * hi(b)) << 16), // MACHI
                    5 => self.acc = self.acc.wrapping_add((lo(a) * lo(b)) << 16), // MACLO
                    // MULWHI/MULWLO/MACWHI/MACWLO (word x halfword) not modeled yet.
                    _ => {}
                }
            }
            0x4 => {
                // ADDI Rdest, #simm8
                self.r[rd] = self.r[rd].wrapping_add(sext8(b1));
            }
            0x5 => {
                // op1=0x5 is shared between shift-immediate and the accumulator move
                // ops (mvfac*/mvtac*). The moves use byte1 values (0x70/0x71/0xf0/
                // 0xf1/0xf2) whose top 3 bits (the shift sub-op) are 3 or 7.
                match b1 {
                    0x70 => {
                        // MVTACHI Rsrc: ACC[32:63] = Rsrc, low half preserved
                        let lo = (self.acc as u64) & 0x0000_0000_ffff_ffff;
                        self.acc = (((self.r[rd] as u64) << 32) | lo) as i64;
                    }
                    0x71 => {
                        // MVTACLO Rsrc: ACC[0:31] = Rsrc, high half preserved
                        let hi = (self.acc as u64) & 0xffff_ffff_0000_0000;
                        self.acc = (hi | self.r[rd] as u64) as i64;
                    }
                    0xf0 => self.r[rd] = (self.acc >> 32) as u32, // MVFACHI -> ACC[32:63]
                    0xf1 => self.r[rd] = self.acc as u32,         // MVFACLO -> ACC[0:31]
                    0xf2 => self.r[rd] = (self.acc >> 16) as u32, // MVFACMI -> ACC[16:47]
                    _ => {
                        // shifts by imm5
                        let sub = b1 >> 5;
                        let imm5 = (b1 & 0x1f) as u32;
                        match sub {
                            0 => self.r[rd] >>= imm5 & 31,
                            1 => self.r[rd] = ((self.r[rd] as i32) >> (imm5 & 31)) as u32,
                            2 => self.r[rd] <<= imm5 & 31,
                            _ => {}
                        }
                    }
                }
            }
            0x6 => {
                // LDI Rdest, #simm8
                self.r[rd] = sext8(b1);
            }
            0x7 => self.op1_7(bus, b0, b1, pc),
            0x8 => self.op1_8(op3, rd, rs, hw1),
            0x9 => self.op1_9(op3, rd, rs, hw1),
            0xA => self.op1_a(bus, op3, b0, rd, rs, hw1),
            0xB => self.op1_b(op3, b0, rd, rs, hw1, pc),
            0xD => self.op1_d(bus, b0, b1, rd, rs, hw1),
            0xE => {
                // LD24 Rdest, #imm24
                let imm24 = ((b1 as u32) << 16) | (hw1 as u32);
                self.r[rd] = imm24 & 0x00FF_FFFF;
            }
            0xF => self.op1_f(b0, b1, hw1, pc),
            _ => {}
        }
        true
    }

    // op1=0 : register-register ALU / compare (16-bit)
    fn op1_0(&mut self, op3: u32, rd: usize, rs: usize) {
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
    fn op1_1<B: Bus>(&mut self, _bus: &mut B, b0: u8, op3: u32, rd: usize, rs: usize, pc: u32) {
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
                    self.r[14] = (pc & 0xFFFF_FFFC).wrapping_add(4);
                }
                self.pc = self.r[rs] & 0xFFFF_FFFC;
            }
            13 => {
                // RTE = full opcode 0x10D6: return from EIT. Restore PC from BPC and
                // the current PSW fields (SM/IE/C) from their backups (BSM/BIE/BC),
                // re-enabling interrupts. Without this, IE stays 0 after the first
                // interrupt and the machine never takes another one.
                if b0 == 0x10 {
                    self.pc = self.bpc;
                    let bsm = (self.psw >> 15) & 1;
                    let bie = (self.psw >> 14) & 1;
                    let bc = (self.psw >> 8) & 1;
                    self.psw = (self.psw & !0x00C0) | (bsm << 7) | (bie << 6);
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
    fn op1_2<B: Bus>(&mut self, bus: &mut B, op3: u32, rd: usize, rs: usize) {
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
    fn op1_7<B: Bus>(&mut self, _bus: &mut B, b0: u8, b1: u8, pc: u32) {
        let rel8 = (pc & 0xFFFF_FFFC).wrapping_add(sext8(b1) << 2);
        match b0 {
            0x70 => {}                                       // NOP
            0x71 => self.psw |= b1 as u32,                   // SETPSW
            0x72 => self.psw &= b1 as u32,                   // CLRPSW
            0x7C => if self.c { self.pc = rel8 },            // BC
            0x7D => if !self.c { self.pc = rel8 },           // BNC
            0x7E => {
                // BL
                self.r[14] = (pc & 0xFFFF_FFFC).wrapping_add(4);
                self.pc = rel8;
            }
            0x7F => self.pc = rel8, // BRA
            _ => {}
        }
    }

    // op1=8 : 3-operand ALU with imm16 / CMPI (32-bit)
    fn op1_8(&mut self, op3: u32, rd: usize, rs: usize, hw1: u16) {
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
    fn op1_9(&mut self, op3: u32, rd: usize, rs: usize, hw1: u16) {
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
    fn op1_a<B: Bus>(&mut self, bus: &mut B, op3: u32, b0: u8, rd: usize, rs: usize, hw1: u16) {
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
    fn op1_b(&mut self, op3: u32, _b0: u8, rd: usize, rs: usize, hw1: u16, pc: u32) {
        let target = (pc & 0xFFFF_FFFC).wrapping_add(sext16(hw1) << 2);
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
    fn op1_d<B: Bus>(&mut self, _bus: &mut B, b0: u8, b1: u8, rd: usize, _rs: usize, hw1: u16) {
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
            (0, 0) => self.r[frdest] = (a + b).to_bits(),                 // FADD
            (0, 0x4) => self.r[frdest] = (a - b).to_bits(),              // FSUB
            (0, 0xC) => self.r[frdest] = (a - b).to_bits(),              // FCMP (diff)
            (0, 0xD) => self.r[frdest] = (a < b) as u32,                 // FCMPE
            (1, 0) => self.r[frdest] = (a * b).to_bits(),               // FMUL
            (2, 0) => self.r[frdest] = (a / b).to_bits(),               // FDIV
            (3, 0) => {
                let d = f32::from_bits(self.r[frdest]);
                self.r[frdest] = (d + a * b).to_bits(); // FMADD
            }
            (3, 0x4) => {
                let d = f32::from_bits(self.r[frdest]);
                self.r[frdest] = (d - a * b).to_bits(); // FMSUB
            }
            (4, 0) => self.r[frdest] = ((a as i32) as f32).to_bits(),   // ITOF
            (4, 0x4) => self.r[frdest] = ((self.r[frsrc]) as f32).to_bits(), // UTOF
            (4, 0x8) => self.r[frdest] = (a as i32) as u32,             // FTOI
            (4, 0xC) => self.r[frdest] = (a as i32) as u32,             // FTOS
            _ => {}
        }
    }

    // op1=F : NOP (0xF0) or REL24 branches (32-bit)
    fn op1_f(&mut self, b0: u8, b1: u8, hw1: u16, pc: u32) {
        if b0 == 0xF0 {
            return; // NOP
        }
        let disp = sext24(((b1 as u32) << 16) | (hw1 as u32));
        let target = (pc & 0xFFFF_FFFC).wrapping_add(disp << 2);
        match b0 {
            0xFC => if self.c { self.pc = target },  // BC
            0xFD => if !self.c { self.pc = target }, // BNC
            0xFE => {
                // BL
                self.r[14] = (pc & 0xFFFF_FFFC).wrapping_add(4);
                self.pc = target;
            }
            0xFF => self.pc = target, // BRA
            _ => {}
        }
    }
}

#[cfg(test)]
mod rom_tests {
    use super::*;

    const RESULT_SLOT: u32 = 0x0080_0000;
    const MEM_SIZE: usize = 0x0082_0000;

    struct FlatBus {
        mem: Vec<u8>,
    }

    impl FlatBus {
        fn new(rom: &[u8]) -> FlatBus {
            let mut mem = vec![0u8; MEM_SIZE];
            mem[..rom.len()].copy_from_slice(rom);
            FlatBus { mem }
        }
    }

    impl Bus for FlatBus {
        fn r8(&mut self, addr: u32) -> u8 {
            *self.mem.get(addr as usize).unwrap_or(&0)
        }
        fn w8(&mut self, addr: u32, v: u8) {
            if let Some(b) = self.mem.get_mut(addr as usize) {
                *b = v;
            }
        }
    }

    fn run_rom(rom: &[u8]) -> u32 {
        let mut bus = FlatBus::new(rom);
        let mut cpu = Cpu::new();
        cpu.pc = 0;
        const MAX_STEPS: u64 = 50_000_000;
        for _ in 0..MAX_STEPS {
            let pc_before = cpu.pc;
            if !cpu.step(&mut bus) {
                panic!("decode failure at pc={:#010x}", pc_before);
            }
            // Done if program is spinning
            if cpu.pc == pc_before {
                return bus.r32(RESULT_SLOT);
            }
        }
        panic!("ROM did not terminate within {MAX_STEPS} steps");
    }

    macro_rules! rom_test {
        ($name:ident, $file:literal) => {
            #[test]
            fn $name() {
                let result = run_rom(include_bytes!(concat!("testdata/", $file)));
                assert_eq!(result, 0, "check #{result} failed");
            }
        };
    }

    rom_test!(test_arith, "test_arith.bin");
    rom_test!(test_branch, "test_branch.bin");
    rom_test!(test_carry, "test_carry.bin");
    rom_test!(test_dsp, "test_dsp.bin");
    rom_test!(test_mem, "test_mem.bin");
    rom_test!(test_mul, "test_mul.bin");
}
