use maarch64_core::{cpu::CpuContext, interp::Interpreter, memory::MemoryManager};
use maarch64_thunks::ThunkManager;

#[test]
fn test_thunk_malloc_and_memset() {
    let mut mem = MemoryManager::new();
    let mut ctx = CpuContext::new();
    let mut thunks = ThunkManager::new();

    let malloc_addr: u64 = 0x900000;
    let memset_addr: u64 = 0x900010;

    let malloc_fn = thunks.get_thunk("malloc").unwrap();
    let memset_fn = thunks.get_thunk("memset").unwrap();

    thunks.register_thunk_address(malloc_addr, malloc_fn);
    thunks.register_thunk_address(memset_addr, memset_fn);

    // 1. Call malloc(100)
    ctx.pc = malloc_addr;
    ctx.set_x(0, 100);
    ctx.set_x(30, 0x400004); // LR

    let res = Interpreter::step_with_thunk_lookup(&mut ctx, &mut mem, |addr| {
        thunks.get_thunk_by_address(addr)
    });
    assert!(res.unwrap());
    assert_eq!(ctx.pc, 0x400004);
    let allocated_vaddr = ctx.get_x(0);
    assert!(allocated_vaddr > 0);

    // 2. Call memset(allocated_vaddr, 0xAA, 50)
    ctx.pc = memset_addr;
    ctx.set_x(0, allocated_vaddr);
    ctx.set_x(1, 0xAA);
    ctx.set_x(2, 50);
    ctx.set_x(30, 0x400008); // LR

    let res2 = Interpreter::step_with_thunk_lookup(&mut ctx, &mut mem, |addr| {
        thunks.get_thunk_by_address(addr)
    });
    assert!(res2.unwrap());
    assert_eq!(ctx.pc, 0x400008);

    // Verify memory contents
    let read_buf = mem.read(allocated_vaddr, 50).unwrap();
    assert_eq!(read_buf, vec![0xAA; 50]);
}

#[test]
fn test_thunk_strlen_and_memcpy() {
    let mut mem = MemoryManager::new();
    let mut ctx = CpuContext::new();
    let mut thunks = ThunkManager::new();

    let src_addr: u64 = 0x500000;
    let dest_addr: u64 = 0x501000;
    mem.map_anonymous(src_addr, 0x1000).unwrap();
    mem.map_anonymous(dest_addr, 0x1000).unwrap();

    let text = "Hello Maarch64 Thunks!\0";
    mem.write(src_addr, text.as_bytes()).unwrap();

    let strlen_addr: u64 = 0x900020;
    let memcpy_addr: u64 = 0x900030;

    let strlen_fn = thunks.get_thunk("strlen").unwrap();
    let memcpy_fn = thunks.get_thunk("memcpy").unwrap();

    thunks.register_thunk_address(strlen_addr, strlen_fn);
    thunks.register_thunk_address(memcpy_addr, memcpy_fn);

    // 1. Call strlen(src_addr)
    ctx.pc = strlen_addr;
    ctx.set_x(0, src_addr);
    ctx.set_x(30, 0x400004);

    Interpreter::step_with_thunk_lookup(&mut ctx, &mut mem, |addr| thunks.get_thunk_by_address(addr)).unwrap();
    assert_eq!(ctx.get_x(0), 22);

    // 2. Call memcpy(dest_addr, src_addr, 23)
    ctx.pc = memcpy_addr;
    ctx.set_x(0, dest_addr);
    ctx.set_x(1, src_addr);
    ctx.set_x(2, 23);
    ctx.set_x(30, 0x400008);

    Interpreter::step_with_thunk_lookup(&mut ctx, &mut mem, |addr| thunks.get_thunk_by_address(addr)).unwrap();
    let dest_bytes = mem.read_string(dest_addr).unwrap();
    let dest_text = String::from_utf8_lossy(&dest_bytes);
    assert_eq!(dest_text, "Hello Maarch64 Thunks!");
}

#[test]
fn test_thunk_exit() {
    let mut mem = MemoryManager::new();
    let mut ctx = CpuContext::new();
    let mut thunks = ThunkManager::new();

    let exit_addr: u64 = 0x900040;
    let exit_fn = thunks.get_thunk("exit").unwrap();
    thunks.register_thunk_address(exit_addr, exit_fn);

    ctx.pc = exit_addr;
    ctx.set_x(0, 42); // exit code

    let res = Interpreter::step_with_thunk_lookup(&mut ctx, &mut mem, |addr| thunks.get_thunk_by_address(addr)).unwrap();
    assert!(!res); // Exited cleanly
    assert!(ctx.exited);
    assert_eq!(ctx.exit_code, 42);
}

#[test]
fn test_thunk_strcmp() {
    let mut mem = MemoryManager::new();
    let mut ctx = CpuContext::new();
    let mut thunks = ThunkManager::new();

    let s1_addr: u64 = 0x500000;
    let s2_addr: u64 = 0x501000;
    mem.map_anonymous(s1_addr, 0x1000).unwrap();
    mem.map_anonymous(s2_addr, 0x1000).unwrap();

    mem.write(s1_addr, b"apple\0").unwrap();
    mem.write(s2_addr, b"apple\0").unwrap();

    let strcmp_addr: u64 = 0x900050;
    let strcmp_fn = thunks.get_thunk("strcmp").unwrap();
    thunks.register_thunk_address(strcmp_addr, strcmp_fn);

    ctx.pc = strcmp_addr;
    ctx.set_x(0, s1_addr);
    ctx.set_x(1, s2_addr);
    ctx.set_x(30, 0x400004);

    Interpreter::step_with_thunk_lookup(&mut ctx, &mut mem, |addr| thunks.get_thunk_by_address(addr)).unwrap();
    assert_eq!(ctx.get_x(0), 0);
}
