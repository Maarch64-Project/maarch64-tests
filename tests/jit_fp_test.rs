use maarch64_core::{
    cpu::CpuContext, interp::Interpreter, jit::JitCache, memory::MemoryManager,
};

#[test]
fn test_jit_fp_and_tpidr_el0() {
    let mut mem = MemoryManager::new();
    let mut ctx = CpuContext::new();
    let mut jit_cache = JitCache::new();

    let code_addr: u64 = 0x400000;
    mem.map_anonymous(code_addr, 0x1000).unwrap();

    // 0x00: FADD D2, D0, D1  (0x1e212802)
    // 0x04: FSUB D3, D0, D1  (0x1e213803)
    // 0x08: MSR TPIDR_EL0, X0 (0xd51bd040)
    // 0x0c: MRS X1, TPIDR_EL0 (0xd53bd041)
    // 0x10: RET              (0xd65f03c0)
    let code: &[u8] = &[
        0x02, 0x28, 0x21, 0x1e, // FADD D2, D0, D1
        0x03, 0x38, 0x21, 0x1e, // FSUB D3, D0, D1
        0x40, 0xd0, 0x1b, 0xd5, // MSR TPIDR_EL0, X0
        0x41, 0xd0, 0x3b, 0xd5, // MRS X1, TPIDR_EL0
        0xc0, 0x03, 0x5f, 0xd6, // RET
    ];
    mem.write(code_addr, code).unwrap();

    // Setup input context:
    // D0 = 3.5, D1 = 2.5
    ctx.set_v_u64(0, 3.5f64.to_bits());
    ctx.set_v_u64(1, 2.5f64.to_bits());
    ctx.set_x(0, 0x7f001234); // TLS address in X0
    ctx.pc = code_addr;
    ctx.set_x(30, 0x800000); // LR

    let res = Interpreter::step_with_jit(&mut ctx, &mut mem, &mut jit_cache);
    assert!(res.unwrap());

    let d2_val = f64::from_bits(ctx.get_v_u64(2));
    let d3_val = f64::from_bits(ctx.get_v_u64(3));
    assert_eq!(d2_val, 6.0f64); // 3.5 + 2.5 = 6.0
    assert_eq!(d3_val, 1.0f64); // 3.5 - 2.5 = 1.0

    // TPIDR_EL0 = 0x7f001234 and X1 = 0x7f001234
    assert_eq!(ctx.tpidr_el0, 0x7f001234);
    assert_eq!(ctx.get_x(1), 0x7f001234);
    assert_eq!(ctx.pc, 0x800000);
}
