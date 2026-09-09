//! ROM-driven conformance tests.

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

// R15 aliases the stack pointer selected by PSW.SM (0=>SPI, 1=>SPU).
#[test]
fn r15_aliases_spi_via_mvtc() {
    let mut cpu = Cpu::new();
    assert_eq!(cpu.sm(), 0, "reset selects the interrupt stack (SPI)");
    cpu.r[2] = 0x0081_2000;
    cpu.write_cr(2, cpu.r[2]); // MVTC R2,SPI
    assert_eq!(cpu.r[15], 0x0081_2000, "SPI write must reflect into R15");
    assert_eq!(cpu.read_cr(2), 0x0081_2000, "reading SPI returns live R15");
}

// Switching PSW.SM banks R15 between SPI and SPU without losing either value.
#[test]
fn sm_switch_banks_r15() {
    let mut cpu = Cpu::new();
    cpu.write_cr(2, 0x0081_2000); // SPI (active, SM=0)
    cpu.write_cr(3, 0x0090_0000); // SPU (inactive)
    assert_eq!(cpu.r[15], 0x0081_2000);
    cpu.set_sm(1); // switch to user stack
    assert_eq!(cpu.r[15], 0x0090_0000, "R15 now the SPU bank");
    cpu.r[15] = 0x0090_0010; // push on user stack
    cpu.set_sm(0); // back to interrupt stack
    assert_eq!(cpu.r[15], 0x0081_2000, "SPI preserved across the switch");
    assert_eq!(cpu.read_cr(3), 0x0090_0010, "SPU update preserved too");
}
