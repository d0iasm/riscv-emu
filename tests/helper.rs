use rvemu::bus::DRAM_BASE;
use rvemu::cpu::{POINTER_TO_DTB, REGISTERS_COUNT};
use rvemu::dram::DRAM_SIZE;
use rvemu::emulator::Emulator;

pub const DEFAULT_SP: u64 = DRAM_BASE + DRAM_SIZE;

/// Create registers for x0-x31 with expected values.
pub fn create_xregs(non_zero_regs: Vec<(usize, u64)>) -> [u64; REGISTERS_COUNT] {
    let mut xregs = [0; REGISTERS_COUNT];

    // Based on XRegisters::new().
    xregs[2] = DEFAULT_SP;
    xregs[11] = POINTER_TO_DTB;

    for pair in non_zero_regs.iter() {
        xregs[pair.0] = pair.1;
    }
    xregs
}

/// Create registers for f0-f31 with expected values.
pub fn create_fregs(non_zero_regs: Vec<(usize, f64)>) -> [f64; REGISTERS_COUNT] {
    let mut fregs = [0.0; REGISTERS_COUNT];

    for pair in non_zero_regs.iter() {
        fregs[pair.0] = pair.1;
    }
    fregs
}

/// Start a test and check if the registers are expected.
pub fn run(
    emu: &mut Emulator,
    data: Vec<u8>,
    expected_xregs: &[u64; 32],
    expected_fregs: &[f64; 32],
) {
    let len = data.len() as u64;

    emu.is_debug = true;

    emu.initialize_dram(data);
    emu.initialize_pc(DRAM_BASE);

    emu.test_start(DRAM_BASE, DRAM_BASE + len);

    for (i, e) in expected_xregs.iter().enumerate() {
        assert_eq!(*e, emu.cpu.xregs.read(i as u64), "fails at {}", i);
    }
    for (i, e) in expected_fregs.iter().enumerate() {
        assert_eq!(
            (*e).to_bits(),
            emu.cpu.fregs.read(i as u64).to_bits(),
            "fails at {} expected {} but got {} ",
            i,
            *e,
            emu.cpu.fregs.read(i as u64)
        );
    }
}

pub fn nan_box(num: f32) -> f64 {
    let NAN_MASK = !(u32::MAX as u64);
    f64::from_bits(NAN_MASK | (num.to_bits() as u64))
}
