use maarch64_core::{
    cpu::CpuContext,
    interp::Interpreter,
    loader::ElfLoader,
    memory::MemoryManager,
};
use std::path::PathBuf;

#[test]
fn test_hello_rust_binary_execution() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let bin_path = manifest_dir.join("bin/hello_rust");
    
    if !bin_path.exists() {
        return; // Skip if binary is not precompiled in environment
    }

    let mut mem = MemoryManager::new();
    let loaded = ElfLoader::load_file(&bin_path, &mut mem).unwrap();

    let mut ctx = CpuContext::new();
    ctx.pc = loaded.entry_point;
    ctx.sp = loaded.stack_pointer;

    let res = Interpreter::run(&mut ctx, &mut mem);
    assert!(res.is_ok(), "hello_rust binary execution failed");
}

#[test]
fn test_hello_c_binary_execution() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let bin_path = manifest_dir.join("bin/hello_c");
    
    if !bin_path.exists() {
        return; // Skip if binary is not precompiled in environment
    }

    let mut mem = MemoryManager::new();
    let loaded = ElfLoader::load_file(&bin_path, &mut mem).unwrap();

    let mut ctx = CpuContext::new();
    ctx.pc = loaded.entry_point;
    ctx.sp = loaded.stack_pointer;

    let res = Interpreter::run(&mut ctx, &mut mem);
    assert!(res.is_ok(), "hello_c binary execution failed");
}
