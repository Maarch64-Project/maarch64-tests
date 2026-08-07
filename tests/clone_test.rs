use maarch64_core::{
    cpu::CpuContext,
    interp::Interpreter,
    memory::MemoryManager,
};

#[test]
fn test_sys_clone_handling() {
    let mut mem = MemoryManager::new();
    let mut ctx = CpuContext::new();

    // Setup memory for parent_tidptr and child_tidptr
    let tid_ptr_area = mem.map_anonymous(0x7f00_0000, 4096).unwrap();
    let parent_tidptr = tid_ptr_area;
    let child_tidptr = tid_ptr_area + 4;
    let tls_val = 0x7f02_0000u64;

    // AArch64 sys_clone: SVC #0 (0xd4000001)
    // Flags = CLONE_SETTLS (0x8000) | CLONE_PARENT_SETTID (0x00100000) | CLONE_CHILD_CLEARTID (0x00200000)
    let flags: u64 = 0x8000 | 0x0010_0000 | 0x0020_0000;

    ctx.set_x(0, flags);
    ctx.set_x(1, 0x7fff_ef00); // newsp
    ctx.set_x(2, parent_tidptr);
    ctx.set_x(3, tls_val);
    ctx.set_x(4, child_tidptr);
    ctx.set_x(8, 220); // sys_clone

    let code_page = mem.map_anonymous(0x400000, 4096).unwrap();
    // SVC #0 instruction: 0xd4000001
    mem.write(code_page, &[0x01, 0x00, 0x00, 0xd4]).unwrap();
    ctx.pc = code_page;

    let res = Interpreter::step(&mut ctx, &mut mem);
    assert!(res.is_ok());

    let child_tid = ctx.get_x(0);
    assert!(child_tid >= 1001, "child_tid should be >= 1001");

    // Verify TLS set
    assert_eq!(ctx.tpidr_el0, tls_val, "TPIDR_EL0 should match passed TLS address");

    // Verify memory writes
    let parent_tid_written = u32::from_le_bytes(mem.read(parent_tidptr, 4).unwrap().try_into().unwrap());
    let child_tid_written = u32::from_le_bytes(mem.read(child_tidptr, 4).unwrap().try_into().unwrap());

    assert_eq!(parent_tid_written as u64, child_tid, "parent_tidptr should match return TID");
    assert_eq!(child_tid_written as u64, child_tid, "child_tidptr should match return TID");
}
