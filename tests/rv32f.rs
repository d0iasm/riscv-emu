mod helper;

use rvemu::emulator::Emulator;

#[test]
fn flw_rd_offset_rs1() {
    let mut emu = Emulator::new();

    let data = vec![
        0x93, 0x0f, 0x20, 0x00, // addi x31, x0, 2
        0x13, 0x0f, 0x40, 0x00, // addi x30, x0, 4
        0x87, 0xaf, 0x0f, 0x00, // flw f31, 0(x31)
    ];
    let expected_xregs = helper::create_xregs(vec![(30, 4), (31, 2)]);
    let expected_fregs = helper::create_fregs(vec![]);

    // Fix f31
    //assert_eq!(0x0f130020, cpu.fregs.read(31).to_bits());
    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fsw_rs2_offset_rs1() {
    let mut emu = Emulator::new();

    let data = vec![
        0x93, 0x0f, 0x20, 0x00, // addi x31, x0, 2
        0x13, 0x0f, 0x40, 0x00, // addi x30, x0, 4
        0x27, 0xa0, 0xff, 0x01, // fsw f31, 0(x31)
        0x87, 0xaf, 0x0f, 0x00, // flw f31, 0(x31)
    ];
    let expected_xregs = helper::create_xregs(vec![(30, 4), (31, 2)]);
    let expected_fregs = helper::create_fregs(vec![]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fmadds_rd_rs1_rs2_rs3() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(28, -0.5);
    emu.cpu.fregs.write(29, 4.2);
    emu.cpu.fregs.write(30, 1.2);

    let _data = vec![
        0xc3, 0x0f, 0xdf, 0xe1, // fmadd.s f31, f30, f29, f28
    ];
    let _expected_xregs = helper::create_xregs(vec![]);
    let _expected_fregs = helper::create_fregs(vec![(31, 4.54), (30, 1.2), (29, 4.2), (28, -0.5)]);

    // TODO: fix floating point precision problem
    //helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fmsubs_rd_rs1_rs2_rs3() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(28, -0.5);
    emu.cpu.fregs.write(29, 4.2);
    emu.cpu.fregs.write(30, 1.2);

    let _data = vec![
        0xc7, 0x0f, 0xdf, 0xe1, // fmsub.s f31, f30, f29, f28
    ];
    let _expected_xregs = helper::create_xregs(vec![]);
    let _expected_fregs = helper::create_fregs(vec![(31, 5.54), (30, 1.2), (29, 4.2), (28, -0.5)]);

    // TODO: fix floating point precision problem
    //helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fnmadds_rd_rs1_rs2_rs3() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(28, -0.5);
    emu.cpu.fregs.write(29, 4.2);
    emu.cpu.fregs.write(30, 1.2);

    let _data = vec![
        0xcb, 0x0f, 0xdf, 0xe1, // fnmadd.s f31, f30, f29, f28
    ];
    let _expected_xregs = helper::create_xregs(vec![]);
    let _expected_fregs = helper::create_fregs(vec![(31, -5.54), (30, 1.2), (29, 4.2), (28, -0.5)]);

    // TODO: fix floating point precision problem
    //helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fnmsubs_rd_rs1_rs2_rs3() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(28, -0.5);
    emu.cpu.fregs.write(29, 4.2);
    emu.cpu.fregs.write(30, 1.2);

    let _data = vec![
        0xcf, 0x0f, 0xdf, 0xe1, // fnmsub.s f31, f30, f29, f28
    ];
    let _expected_xregs = helper::create_xregs(vec![]);
    let _expected_fregs = helper::create_fregs(vec![(31, -4.54), (30, 1.2), (29, 4.2), (28, -0.5)]);

    // TODO: fix floating point precision problem
    //helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fadds_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(29, 4.2);
    emu.cpu.fregs.write(30, 2.5);

    let _data = vec![
        0xd3, 0x0f, 0xdf, 0x01, // fadd.s f31, f30, f29
    ];
    let _expected_xregs = helper::create_xregs(vec![]);
    let _expected_fregs = helper::create_fregs(vec![(31, 6.7), (30, 2.5), (29, 4.2)]);

    // TODO: fix floating point precision problem
    //helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fsubs_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(29, 4.2);
    emu.cpu.fregs.write(30, 2.8);

    let _data = vec![
        0xd3, 0x0f, 0xdf, 0x09, // fsub.s f31, f30, f29
    ];
    let _expected_xregs = helper::create_xregs(vec![]);
    let _expected_fregs = helper::create_fregs(vec![(31, -1.4), (30, 2.8), (29, 4.2)]);

    // TODO: fix floating point precision problem
    //helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fmuls_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(29, 4.2);
    emu.cpu.fregs.write(30, -1.2);

    let _data = vec![
        0xd3, 0x0f, 0xdf, 0x11, // fmul.s f31, f30, f29
    ];
    let _expected_xregs = helper::create_xregs(vec![]);
    let _expected_fregs = helper::create_fregs(vec![(31, -5.04), (30, -1.2), (29, 4.2)]);

    // TODO: fix floating point precision problem
    //helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fdivs_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(29, -1.2);
    emu.cpu.fregs.write(30, 4.2);

    let _data = vec![
        0xd3, 0x0f, 0xdf, 0x19, // fdiv.s f31, f30, f29
    ];
    let _expected_xregs = helper::create_xregs(vec![]);
    let _expected_fregs = helper::create_fregs(vec![(31, -3.5), (30, 4.2), (29, -1.2)]);

    // TODO: fix floating point precision problem
    //helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fsgnjs_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(29, -1.2);
    emu.cpu.fregs.write(30, 4.2);

    let data = vec![
        0xd3, 0x0f, 0xdf, 0x21, // fsgnj.s f31, f30, f29
    ];
    let expected_xregs = helper::create_xregs(vec![]);
    let expected_fregs = helper::create_fregs(vec![(31, -4.2), (30, 4.2), (29, -1.2)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fsgnjns_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(29, -1.2);
    emu.cpu.fregs.write(30, 4.2);

    let data = vec![
        0xd3, 0x1f, 0xdf, 0x21, // fsgnjn.s f31, f30, f29
    ];
    let expected_xregs = helper::create_xregs(vec![]);
    let expected_fregs = helper::create_fregs(vec![(31, 4.2), (30, 4.2), (29, -1.2)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fsgnjxs_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(29, -1.2);
    emu.cpu.fregs.write(30, 4.2);

    let _data = vec![
        0xd3, 0x2f, 0xdf, 0x21, // fsgnjx.s f31, f30, f29
    ];
    let _expected_xregs = helper::create_xregs(vec![]);
    let _expected_fregs = helper::create_fregs(vec![(31, -4.2), (30, 4.2), (29, -1.2)]);

    // TODO: fix floating point precision problem
    //helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fmins_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(29, 4.2);
    emu.cpu.fregs.write(30, -1.2);

    let data = vec![
        0xd3, 0x0f, 0xdf, 0x29, // fmin.s f31, f30, f29
    ];
    let expected_xregs = helper::create_xregs(vec![]);
    let expected_fregs = helper::create_fregs(vec![(31, -1.2), (30, -1.2), (29, 4.2)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fmaxs_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(29, 4.2);
    emu.cpu.fregs.write(30, -1.2);

    let data = vec![
        0xd3, 0x1f, 0xdf, 0x29, // fmax.s f31, f30, f29
    ];
    let expected_xregs = helper::create_xregs(vec![]);
    let expected_fregs = helper::create_fregs(vec![(31, 4.2), (30, -1.2), (29, 4.2)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fsqrts_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(30, 4.2);

    let data = vec![
        0xd3, 0x0f, 0x0f, 0x58, // fmax.s f31, f30
    ];
    let expected_xregs = helper::create_xregs(vec![]);
    let expected_fregs = helper::create_fregs(vec![(31, 2.0493900775909424), (30, 4.2)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fles_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(29, 4.2);
    emu.cpu.fregs.write(30, 4.2);

    let data = vec![
        0xd3, 0x0f, 0xdf, 0xa1, // fle.s f31, f30, f29
    ];
    let expected_xregs = helper::create_xregs(vec![(31, 1)]);
    let expected_fregs = helper::create_fregs(vec![(30, 4.2), (29, 4.2)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn flts_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(29, 4.2);
    emu.cpu.fregs.write(30, -1.2);

    let data = vec![
        0xd3, 0x1f, 0xdf, 0xa1, // flt.s f31, f30, f29
    ];
    let expected_xregs = helper::create_xregs(vec![(31, 1)]);
    let expected_fregs = helper::create_fregs(vec![(30, -1.2), (29, 4.2)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn feqs_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(29, 4.2);
    emu.cpu.fregs.write(30, 4.2);

    let data = vec![
        0xd3, 0x2f, 0xdf, 0xa1, // feq.s f31, f30, f29
    ];
    let expected_xregs = helper::create_xregs(vec![(31, 1)]);
    let expected_fregs = helper::create_fregs(vec![(30, 4.2), (29, 4.2)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fcvtws_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(31, -4.2);

    let data = vec![
        0xd3, 0x8f, 0x0f, 0xc0, // fcvt.w.s x31, f31 (rm: 000)
    ];
    let expected_xregs = helper::create_xregs(vec![(31, -4 as i64 as u64)]);
    let expected_fregs = helper::create_fregs(vec![(31, -4.2)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fcvtwus_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(31, 4.2);

    let data = vec![
        0xd3, 0x8f, 0x1f, 0xc0, // fcvt.wu.s x31, f31 (rm: 000)
    ];
    let expected_xregs = helper::create_xregs(vec![(31, 4)]);
    let expected_fregs = helper::create_fregs(vec![(31, 4.2)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fcvtsw_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.xregs.write(31, -4 as i64 as u64);

    let data = vec![
        0xd3, 0x8f, 0x0f, 0xd0, // fcvt.s.w f31, x31 (rm: 000)
    ];
    let expected_xregs = helper::create_xregs(vec![(31, -4 as i64 as u64)]);
    let expected_fregs = helper::create_fregs(vec![(31, -4.0)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fcvtswu_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.xregs.write(31, 4);

    let data = vec![
        0xd3, 0x8f, 0x1f, 0xd0, // fcvt.s.wu f31, x31 (rm: 000)
    ];
    let expected_xregs = helper::create_xregs(vec![(31, 4)]);
    let expected_fregs = helper::create_fregs(vec![(31, 4.0)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fcvtls_rd_fs1() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(31, -1.0);

    let data = vec![
        0xd3, 0xff, 0x2f, 0xc0, // fcvt.l.s x31,f31 (rm: 000)
    ];
    let expected_xregs = helper::create_xregs(vec![(31, -1i64 as u64)]);
    let expected_fregs = helper::create_fregs(vec![(31, -1.0)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fcvtsl_frd_rs1() {
    let mut emu = Emulator::new();

    emu.cpu.xregs.write(31, -1i64 as u64);

    let data = vec![
        0xd3, 0xff, 0x2f, 0xd0, // fcvt.s.l f31,x31 (rm: 000)
    ];
    let expected_xregs = helper::create_xregs(vec![(31, -1i64 as u64)]);
    let expected_fregs = helper::create_fregs(vec![(31, -1.0)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fmvxw_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(31, 4.0);

    let data = vec![
        0xd3, 0x8f, 0x0f, 0xe0, // fmv.x.w x31, f31
    ];
    let expected_xregs = helper::create_xregs(vec![(31, 4.0f64.to_bits() & 0xffffffff)]);
    let expected_fregs = helper::create_fregs(vec![(31, 4.0)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fclasss_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(31, std::f64::INFINITY);

    let data = vec![
        0xd3, 0x9f, 0x0f, 0xe0, // fclass.s x31, f31
    ];
    let expected_xregs = helper::create_xregs(vec![(31, 7)]);
    let expected_fregs = helper::create_fregs(vec![(31, f64::INFINITY)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fmvwx_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.xregs.write(31, 4);

    let data = vec![
        0xd3, 0x8f, 0x0f, 0xf0, // fmv.w.x x31, f31
    ];
    let expected_xregs = helper::create_xregs(vec![(31, 4)]);
    let expected_fregs = helper::create_fregs(vec![(31, f64::from_bits(4))]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}
