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

    // First execution: Cache Miss & Compilation
    assert_eq!(jit_cache.cache_hits, 0);
    assert_eq!(jit_cache.cache_misses, 0);

    let res1 = Interpreter::step_with_jit(&mut ctx, &mut mem, &mut jit_cache);
    assert!(res1.unwrap());
    assert_eq!(ctx.get_x(0), 10);
    assert_eq!(ctx.get_x(1), 20);
    assert_eq!(ctx.get_x(2), 30);
    assert_eq!(ctx.pc, 0x800000); // Returned to LR
    assert_eq!(jit_cache.cache_misses, 0);
    assert_eq!(jit_cache.cache_hits, 1);

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
    assert_eq!(jit_cache.cache_hits, 2);
}
