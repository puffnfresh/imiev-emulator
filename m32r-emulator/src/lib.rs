//! M32R-FP interpreter, ported from the Ghidra SLEIGH spec:
//! https://github.com/bonybrown/imiev-hacking-tools

mod ops;

#[cfg(test)]
mod tests;

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

// PSW (Processor Status Word) bit positions. On EIT the live SM/IE/C are copied
// into their backup slots (BSM/BIE/BC) and restored by RTE.
const PSW_C: u32 = 0; // condition / carry-borrow flag
const PSW_IE: u32 = 6; // interrupt enable
const PSW_SM: u32 = 7; // stack mode: 0 = SPI (interrupt), 1 = SPU (user)
const PSW_BC: u32 = 8; // backup of C
const PSW_BIE: u32 = 14; // backup of IE
const PSW_BSM: u32 = 15; // backup of SM

/// PC word-alignment mask: M32R branch/jump targets clear the low two bits.
const PC_ALIGN: u32 = !0b11;

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

    #[inline]
    fn sm(&self) -> u32 {
        (self.psw >> PSW_SM) & 1
    }

    fn read_cr(&self, i: u32) -> u32 {
        match i & 7 {
            0 => (self.psw & !(1 << PSW_C)) | (self.c as u32),
            1 => self.cbr,
            2 => if self.sm() == 0 { self.r[15] } else { self.spi }, // SPI
            3 => if self.sm() == 1 { self.r[15] } else { self.spu }, // SPU
            6 => self.bpc,
            7 => self.fpsr,
            _ => 0,
        }
    }
    fn write_cr(&mut self, i: u32, v: u32) {
        match i & 7 {
            0 => {
                self.psw = v;
                self.c = (v & (1 << PSW_C)) != 0;
            }
            1 => self.cbr = v,
            2 => {
                self.spi = v;
                if self.sm() == 0 {
                    self.r[15] = v;
                }
            }
            3 => {
                self.spu = v;
                if self.sm() == 1 {
                    self.r[15] = v;
                }
            }
            6 => self.bpc = v,
            7 => self.fpsr = v,
            _ => {}
        }
    }

    /// Switch PSW.SM, banking R15 between the SPI (interrupt) and SPU (user)
    /// stack pointers so R15 always holds the active bank.
    fn set_sm(&mut self, new_sm: u32) {
        let old = self.sm();
        if old == new_sm {
            return;
        }
        if old == 0 {
            self.spi = self.r[15];
        } else {
            self.spu = self.r[15];
        }
        self.psw = (self.psw & !(1 << PSW_SM)) | ((new_sm & 1) << PSW_SM);
        self.r[15] = if new_sm == 0 { self.spi } else { self.spu };
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
        (self.psw & (1 << PSW_IE)) != 0
    }

    /// Take an EIT (interrupt) to `vector`, the M32R way: save PC->BPC, back up the
    /// current SM/IE/C into BSM/BIE/BC, then clear IE (mask further interrupts).
    /// The matching `RTE` restores them.
    pub fn take_interrupt(&mut self, vector: u32) {
        self.bpc = self.pc;
        let ie = (self.psw >> PSW_IE) & 1;
        let sm = (self.psw >> PSW_SM) & 1;
        let c = self.c as u32;
        let backup_mask = (1 << PSW_BSM) | (1 << PSW_BIE) | (1 << PSW_BC);
        self.psw = (self.psw & !backup_mask) | (sm << PSW_BSM) | (ie << PSW_BIE) | (c << PSW_BC);
        self.psw &= !(1 << PSW_IE); // IE = 0
        self.set_sm(0); // EIT runs on the interrupt stack (SPI)
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
}
