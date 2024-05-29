mod helper;

use rvemu::emulator::Emulator;

#[test]
fn fld_rd_offset_rs1() {
    let mut emu = Emulator::new();

    // TODO: fix LoadAccessFault
    let data = vec![
        0x93, 0x0f, 0x20, 0x00, // addi x31, x0, 2
        0x13, 0x0f, 0x40, 0x00, // addi x30, x0, 4
        0x87, 0xbf, 0x0f, 0x00, // fld f31, 0(x31)
    ];
    let expected_xregs = helper::create_xregs(vec![(30, 4), (31, 2)]);
    let expected_fregs = helper::create_fregs(vec![]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fsd_rs2_offset_rs1() {
    let mut emu = Emulator::new();

    // TODO: fix StoreAMOAccessFault
    let data = vec![
        0x93, 0x0f, 0x20, 0x00, // addi x31, x0, 2
        0x13, 0x0f, 0x40, 0x00, // addi x30, x0, 4
        0x27, 0xb0, 0xff, 0x01, // fsd f31, 0(x31)
        0x87, 0xbf, 0x0f, 0x00, // fld f31, 0(x31)
    ];
    let expected_xregs = helper::create_xregs(vec![(30, 4), (31, 2)]);
    let expected_fregs = helper::create_fregs(vec![]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fmaddd_rd_rs1_rs2_rs3() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(28, -0.5);
    emu.cpu.fregs.write(29, 4.2);
    emu.cpu.fregs.write(30, 1.2);

    let data = vec![
        0xc3, 0x0f, 0xdf, 0xe3, // fmadd.d f31, f30, f29, f28
    ];
    let expected_xregs = helper::create_xregs(vec![]);
    let expected_fregs = helper::create_fregs(vec![(31, 4.54), (30, 1.2), (29, 4.2), (28, -0.5)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fmsubd_rd_rs1_rs2_rs3() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(28, -0.5);
    emu.cpu.fregs.write(29, 4.2);
    emu.cpu.fregs.write(30, 1.2);

    let data = vec![
        0xc7, 0x0f, 0xdf, 0xe3, // fmsub.d f31, f30, f29, f28
    ];
    let expected_xregs = helper::create_xregs(vec![]);
    let expected_fregs = helper::create_fregs(vec![(31, 5.54), (30, 1.2), (29, 4.2), (28, -0.5)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fnmaddd_rd_rs1_rs2_rs3() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(28, -0.5);
    emu.cpu.fregs.write(29, 4.2);
    emu.cpu.fregs.write(30, 1.2);

    let data = vec![
        0xcb, 0x0f, 0xdf, 0xe3, // fnmadd.d f31, f30, f29, f28
    ];
    let expected_xregs = helper::create_xregs(vec![]);
    let expected_fregs = helper::create_fregs(vec![(31, -5.54), (30, 1.2), (29, 4.2), (28, -0.5)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fnmsubd_rd_rs1_rs2_rs3() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(28, -0.5);
    emu.cpu.fregs.write(29, 4.2);
    emu.cpu.fregs.write(30, 1.2);

    let data = vec![
        0xcf, 0x0f, 0xdf, 0xe3, // fnmsub.d f31, f30, f29, f28
    ];
    let expected_xregs = helper::create_xregs(vec![]);
    let expected_fregs = helper::create_fregs(vec![(31, -4.54), (30, 1.2), (29, 4.2), (28, -0.5)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn faddd_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(29, 4.2);
    emu.cpu.fregs.write(30, 2.5);

    let data = vec![
        0xd3, 0x0f, 0xdf, 0x03, // fadd.d f31, f30, f29
    ];
    let expected_xregs = helper::create_xregs(vec![]);
    let expected_fregs = helper::create_fregs(vec![(31, 6.7), (30, 2.5), (29, 4.2)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fsubd_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(29, 4.2);
    emu.cpu.fregs.write(30, 2.8);

    let _data = vec![
        0xd3, 0x0f, 0xdf, 0x0b, // fsub.d f31, f30, f29
    ];
    let _expected_xregs = helper::create_xregs(vec![]);
    let _expected_fregs = helper::create_fregs(vec![(31, -1.4), (30, 2.8), (29, 4.2)]);

    // TODO: fix floating point precision problem
    //helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fmuld_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(29, 4.2);
    emu.cpu.fregs.write(30, -1.2);

    let data = vec![
        0xd3, 0x0f, 0xdf, 0x13, // fmul.d f31, f30, f29
    ];
    let expected_xregs = helper::create_xregs(vec![]);
    let expected_fregs = helper::create_fregs(vec![(31, -5.04), (30, -1.2), (29, 4.2)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fdivd_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(29, -1.2);
    emu.cpu.fregs.write(30, 4.2);

    let _data = vec![
        0xd3, 0x0f, 0xdf, 0x1b, // fdiv.d f31, f30, f29
    ];
    let _expected_xregs = helper::create_xregs(vec![]);
    let _expected_fregs = helper::create_fregs(vec![(31, -3.5), (30, 4.2), (29, -1.2)]);

    // TODO: fix floating point precision problem
    //helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fsgnjd_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(29, -1.2);
    emu.cpu.fregs.write(30, 4.2);

    let data = vec![
        0xd3, 0x0f, 0xdf, 0x23, // fsgnj.d f31, f30, f29
    ];
    let expected_xregs = helper::create_xregs(vec![]);
    let expected_fregs = helper::create_fregs(vec![(31, -4.2), (30, 4.2), (29, -1.2)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fsgnjnd_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(29, -1.2);
    emu.cpu.fregs.write(30, 4.2);

    let data = vec![
        0xd3, 0x1f, 0xdf, 0x23, // fsgnjn.d f31, f30, f29
    ];
    let expected_xregs = helper::create_xregs(vec![]);
    let expected_fregs = helper::create_fregs(vec![(31, 4.2), (30, 4.2), (29, -1.2)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fsgnjxd_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(29, -1.2);
    emu.cpu.fregs.write(30, 4.2);

    let data = vec![
        0xd3, 0x2f, 0xdf, 0x23, // fsgnjx.d f31, f30, f29
    ];
    let expected_xregs = helper::create_xregs(vec![]);
    let expected_fregs = helper::create_fregs(vec![(31, -4.2), (30, 4.2), (29, -1.2)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fmind_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(29, 4.2);
    emu.cpu.fregs.write(30, -1.2);

    let data = vec![
        0xd3, 0x0f, 0xdf, 0x2b, // fmin.d f31, f30, f29
    ];
    let expected_xregs = helper::create_xregs(vec![]);
    let expected_fregs = helper::create_fregs(vec![(31, -1.2), (30, -1.2), (29, 4.2)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fmaxd_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(29, 4.2);
    emu.cpu.fregs.write(30, -1.2);

    let data = vec![
        0xd3, 0x1f, 0xdf, 0x2b, // fmax.d f31, f30, f29
    ];
    let expected_xregs = helper::create_xregs(vec![]);
    let expected_fregs = helper::create_fregs(vec![(31, 4.2), (30, -1.2), (29, 4.2)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fcvtsd_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(30, -1.2);

    let data = vec![
        0xd3, 0x0f, 0x1f, 0x40, // fcvt.s.d f31, f30
    ];
    let expected_xregs = helper::create_xregs(vec![]);
    let expected_fregs = helper::create_fregs(vec![(31, -1.2), (30, -1.2)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fcvtds_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(30, -1.2);

    let _data = vec![
        0xd3, 0x0f, 0x0f, 0x42, // fcvt.d.s f31, f30
    ];
    let _expected_xregs = helper::create_xregs(vec![]);
    let _expected_fregs = helper::create_fregs(vec![(31, -1.2), (30, -1.2)]);

    // TODO: fix floating point precision problem
    //helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fsqrtd_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(30, 4.2);

    let data = vec![
        0xd3, 0x0f, 0x0f, 0x5a, // fmax.d f31, f30
    ];
    let expected_xregs = helper::create_xregs(vec![]);
    let expected_fregs = helper::create_fregs(vec![(31, 2.04939015319192), (30, 4.2)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fled_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(29, 4.2);
    emu.cpu.fregs.write(30, 4.2);

    let data = vec![
        0xd3, 0x0f, 0xdf, 0xa3, // fle.d f31, f30, f29
    ];
    let expected_xregs = helper::create_xregs(vec![(31, 1)]);
    let expected_fregs = helper::create_fregs(vec![(30, 4.2), (29, 4.2)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fltd_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(29, 4.2);
    emu.cpu.fregs.write(30, -1.2);

    let data = vec![
        0xd3, 0x1f, 0xdf, 0xa3, // flt.d f31, f30, f29
    ];
    let expected_xregs = helper::create_xregs(vec![(31, 1)]);
    let expected_fregs = helper::create_fregs(vec![(30, -1.2), (29, 4.2)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn feqd_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(29, 4.2);
    emu.cpu.fregs.write(30, 4.2);

    let data = vec![
        0xd3, 0x2f, 0xdf, 0xa3, // feq.d f31, f30, f29
    ];
    let expected_xregs = helper::create_xregs(vec![(31, 1)]);
    let expected_fregs = helper::create_fregs(vec![(30, 4.2), (29, 4.2)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fcvtwd_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(31, -4.2);

    let data = vec![
        0xd3, 0x8f, 0x0f, 0xc2, // fcvt.w.d x31, f31 (rm: 000)
    ];
    let expected_xregs = helper::create_xregs(vec![(31, -4 as i64 as u64)]);
    let expected_fregs = helper::create_fregs(vec![(31, -4.2)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fcvtwud_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(31, 4.2);

    let data = vec![
        0xd3, 0x8f, 0x1f, 0xc2, // fcvt.wu.d x31, f31 (rm: 000)
    ];
    let expected_xregs = helper::create_xregs(vec![(31, 4)]);
    let expected_fregs = helper::create_fregs(vec![(31, 4.2)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fcvtdw_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.xregs.write(31, -4 as i64 as u64);

    let data = vec![
        0xd3, 0x8f, 0x0f, 0xd2, // fcvt.d.w f31, x31 (rm: 000)
    ];
    let expected_xregs = helper::create_xregs(vec![(31, -4 as i64 as u64)]);
    let expected_fregs = helper::create_fregs(vec![(31, -4.0)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fcvtdwu_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.xregs.write(31, 4);

    let data = vec![
        0xd3, 0x8f, 0x1f, 0xd2, // fcvt.d.wu f31, x31 (rm: 000)
    ];
    let expected_xregs = helper::create_xregs(vec![(31, 4)]);
    let expected_fregs = helper::create_fregs(vec![(31, 4.0)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fcvtld_rd_fs1() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(31, -1.0);

    let data = vec![
        0xd3, 0xff, 0x2f, 0xc2, // fcvt.l.d x31,f31 (rm: 000)
    ];
    let expected_xregs = helper::create_xregs(vec![(31, -1i64 as u64)]);
    let expected_fregs = helper::create_fregs(vec![(31, -1.0)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fcvtdl_frd_rs1() {
    let mut emu = Emulator::new();

    emu.cpu.xregs.write(31, -1i64 as u64);

    let data = vec![
        0xd3, 0xff, 0x2f, 0xd2, // fcvt.d.l f31,x31 (rm: 000)
    ];
    let expected_xregs = helper::create_xregs(vec![(31, -1i64 as u64)]);
    let expected_fregs = helper::create_fregs(vec![(31, -1.0)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}

#[test]
fn fclassd_rd_rs1_rs2() {
    let mut emu = Emulator::new();

    emu.cpu.fregs.write(31, std::f64::INFINITY);

    let data = vec![
        0xd3, 0x9f, 0x0f, 0xe2, // fclass.d x31, f31
    ];
    let expected_xregs = helper::create_xregs(vec![(31, 7)]);
    let expected_fregs = helper::create_fregs(vec![(31, std::f64::INFINITY)]);

    helper::run(&mut emu, data, &expected_xregs, &expected_fregs);
}
