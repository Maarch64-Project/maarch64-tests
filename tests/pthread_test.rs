use maarch64_core::{
    cpu::CpuContext, memory::MemoryManager, syscall::linux::LinuxSyscall,
};

#[test]
fn test_pthread_multithreading_syscalls() {
    let mut mem = MemoryManager::new();
    let mut ctx = CpuContext::new();

    let code_addr: u64 = 0x500000;
    mem.map_anonymous(code_addr, 0x1000).unwrap();

    // 1. Test Syscall set_robust_list (99)
    ctx.set_x(8, 99);
    ctx.set_x(0, 0x7f005000); // head_ptr
    ctx.set_x(1, 24);         // len
    let res99 = LinuxSyscall::handle(&mut ctx, &mut mem);
    assert_eq!(res99.unwrap(), 0);
    assert_eq!(ctx.get_x(0), 0);

    // 2. Test Syscall futex (98) FUTEX_WAKE (1)
    let futex_addr: u64 = 0x7f006000;
    mem.map_anonymous(futex_addr, 0x1000).unwrap();
    mem.write(futex_addr, &1u32.to_le_bytes()).unwrap();

    ctx.set_x(8, 98);
    ctx.set_x(0, futex_addr);
    ctx.set_x(1, 1); // FUTEX_WAKE
    ctx.set_x(2, 1); // wake 1 thread
    let res98 = LinuxSyscall::handle(&mut ctx, &mut mem);
    assert_eq!(res98.unwrap(), 1);

    // 3. Test Syscall clone (56) with CLONE_THREAD
    let stack_addr: u64 = 0x7f007000;
    let tls_addr: u64 = 0x7f008000;
    mem.map_anonymous(stack_addr, 0x1000).unwrap();
    mem.map_anonymous(tls_addr, 0x1000).unwrap();

    ctx.set_x(8, 56);
    ctx.set_x(0, 0x00010000 | 0x00000100); // CLONE_THREAD | CLONE_VM
    ctx.set_x(1, stack_addr + 0x1000);    // SP
    ctx.set_x(2, 0);                      // ptid
    ctx.set_x(3, tls_addr);               // TLS
    ctx.set_x(4, 0);                      // ctid

    let child_tid = LinuxSyscall::handle(&mut ctx, &mut mem).unwrap();
    assert!(child_tid > 0);
}
