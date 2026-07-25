use maarch64_core::{
    cpu::CpuContext,
    interp::Interpreter,
    memory::MemoryManager,
};

#[test]
fn test_stack_stp_ldp_execution() {
    let mut ctx = CpuContext::new();
    let mut mem = MemoryManager::new();

    // Map 4KB code & stack memory
    let code_addr = 0x400000;
    let stack_addr = 0x7ffff0000000;
    mem.map_anonymous(code_addr, 0x1000).unwrap();
    mem.map_anonymous(stack_addr - 0x1000, 0x1000).unwrap();

    ctx.pc = code_addr;
    ctx.sp = stack_addr;
    ctx.set_x(0, 0x1122334455667788);
    ctx.set_x(1, 0xaabbccddeeff0011);

    // Assembly opcodes:
    // 1. STP X0, X1, [SP, #-16]!   (0xa9bf07e0)
    // 2. LDP X2, X3, [SP], #16     (0xa8c10fe2)
    let code: [u8; 8] = [
        0xe0, 0x07, 0xbf, 0xa9,
        0xe2, 0x0f, 0xc1, 0xa8,
    ];
    mem.write(code_addr, &code).unwrap();

    // Step 1: STP
    let inst1 = maarch64_core::decoder::Decoder::decode(u32::from_le_bytes(code[0..4].try_into().unwrap()), ctx.pc).unwrap();
    println!("Inst 1: {:?}", inst1);
    Interpreter::step(&mut ctx, &mut mem).unwrap();
    assert_eq!(ctx.sp, stack_addr - 16);

    // Step 2: LDP
    let inst2 = maarch64_core::decoder::Decoder::decode(u32::from_le_bytes(code[4..8].try_into().unwrap()), ctx.pc).unwrap();
    println!("Inst 2: {:?}", inst2);
    Interpreter::step(&mut ctx, &mut mem).unwrap();
    assert_eq!(ctx.sp, stack_addr);
    assert_eq!(ctx.get_x(2), 0x1122334455667788);
    assert_eq!(ctx.get_x(3), 0xaabbccddeeff0011);
}

#[test]
fn test_cmp_and_conditional_branch() {
    let mut ctx = CpuContext::new();
    let mut mem = MemoryManager::new();

    let code_addr = 0x400000;
    mem.map_anonymous(code_addr, 0x1000).unwrap();

    ctx.pc = code_addr;
    ctx.set_x(0, 42);
    ctx.set_x(1, 42);

    // Assembly opcodes:
    // 1. CMP X0, X1           (0xeb01001f)
    // 2. B.EQ #0x40000c       (0x54000040)
    let code: [u8; 8] = [
        0x1f, 0x00, 0x01, 0xeb,
        0x40, 0x00, 0x00, 0x54,
    ];
    mem.write(code_addr, &code).unwrap();

    // Step 1: CMP
    Interpreter::step(&mut ctx, &mut mem).unwrap();
    assert!(ctx.get_z());

    // Step 2: B.EQ (should branch to 0x40000c)
    Interpreter::step(&mut ctx, &mut mem).unwrap();
    assert_eq!(ctx.pc, code_addr + 0xc);
}

#[test]
fn test_ldrb_post_index() {
    let inst = maarch64_core::decoder::Decoder::decode(0x38401402, 0x400dd8).unwrap();
    println!("LDRB Inst: {:?}", inst);
    let inst2 = maarch64_core::decoder::Decoder::decode(0xf94047f8, 0x4049d4).unwrap();
    println!("0xf94047f8 Inst: {:?}", inst2);
}
