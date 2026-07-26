use maarch64_core::{
    cpu::CpuContext, interp::Interpreter, jit::JitCache, memory::MemoryManager,
};

#[test]
fn test_jit_basic_block_compilation_and_execution() {
    let mut mem = MemoryManager::new();
    let mut ctx = CpuContext::new();
    let mut jit_cache = JitCache::new();

    let code_addr: u64 = 0x400000;
    mem.map_anonymous(code_addr, 0x1000).unwrap();

    // 0x00: MOV X0, #10      (0xd2800140)
    // 0x04: MOV X1, #20      (0xd2800281)
    // 0x08: ADD X2, X0, X1   (0x8b010002)
    // 0x0c: RET             (0xd65f03c0)
    let code: &[u8] = &[
        0x40, 0x01, 0x80, 0xd2, // MOV X0, #10
        0x81, 0x02, 0x80, 0xd2, // MOV X1, #20
        0x02, 0x00, 0x01, 0x8b, // ADD X2, X0, X1
        0xc0, 0x03, 0x5f, 0xd6, // RET
    ];
    mem.write(code_addr, code).unwrap();

    ctx.pc = code_addr;
    ctx.set_x(30, 0x800000); // LR

    let res1 = Interpreter::step_with_jit(&mut ctx, &mut mem, &mut jit_cache);
    assert!(res1.unwrap());
    assert_eq!(ctx.get_x(0), 10);
    assert_eq!(ctx.get_x(1), 20);
    assert_eq!(ctx.get_x(2), 30);
    assert_eq!(ctx.pc, 0x800000); // Returned to LR

    // Second execution at same entry point: Cache Hit
    ctx.pc = code_addr;
    ctx.set_x(0, 0);
    ctx.set_x(1, 0);
    ctx.set_x(2, 0);
    ctx.set_x(30, 0x800004);

    let res2 = Interpreter::step_with_jit(&mut ctx, &mut mem, &mut jit_cache);
    assert!(res2.unwrap());
    assert_eq!(ctx.get_x(0), 10);
    assert_eq!(ctx.get_x(1), 20);
    assert_eq!(ctx.get_x(2), 30);
    assert_eq!(ctx.pc, 0x800004);
}

#[test]
fn test_jit_basic_block_chaining() {
    let mut mem = MemoryManager::new();
    let mut ctx = CpuContext::new();
    let mut jit_cache = JitCache::new();

    let block1_addr: u64 = 0x400000;
    let block2_addr: u64 = 0x400010;
    mem.map_anonymous(block1_addr, 0x1000).unwrap();

    // Block 1 (0x400000):
    // 0x00: MOV X0, #5
    // 0x04: B 0x400010 (jump to Block 2: +12 bytes = 3 words offset)
    let block1_code: &[u8] = &[
        0xa0, 0x00, 0x80, 0xd2, // MOV X0, #5
        0x03, 0x00, 0x00, 0x14, // B +0x0c -> 0x400010
    ];
    mem.write(block1_addr, block1_code).unwrap();

    // Block 2 (0x400010):
    // 0x10: ADD X0, X0, #15
    // 0x14: RET
    let block2_code: &[u8] = &[
        0x00, 0x3c, 0x00, 0x91, // ADD X0, X0, #15
        0xc0, 0x03, 0x5f, 0xd6, // RET
    ];
    mem.write(block2_addr, block2_code).unwrap();

    ctx.pc = block1_addr;
    ctx.set_x(30, 0x800000);

    // Warm up compilation for both blocks
    jit_cache.compile_block(block1_addr, &mem).unwrap();
    jit_cache.compile_block(block2_addr, &mem).unwrap();

    // Execute with block chaining (max_chain = 10)
    let chain_res = jit_cache.execute_block_chain(&mut ctx, &mut mem, 10);
    assert!(chain_res.unwrap());

    assert_eq!(ctx.get_x(0), 20); // 5 + 15 = 20
    assert_eq!(ctx.pc, 0x800000); // Exited block 2 via RET
    assert!(jit_cache.chained_jumps > 0, "Chained jumps counter should be > 0");
}
