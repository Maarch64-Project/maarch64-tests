use maarch64_core::{
    cpu::CpuContext,
    interp::Interpreter,
    loader::ElfLoader,
    memory::MemoryManager,
};
use std::path::PathBuf;

#[test]
fn test_rust_std_app_execution() {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let bin_path = manifest_dir.join("bin").join("rust_std_app");

    if !bin_path.exists() {
        return; // Skip if precompiled binary is not present in environment
    }

    let mut mem = MemoryManager::new();
    let target_args = vec!["rust_std_app"];
    let loaded = ElfLoader::load_file_with_args(&bin_path, &target_args, &mut mem)
        .expect("Failed to load rust_std_app ELF");

    let mut ctx = CpuContext::new();
    ctx.pc = loaded.entry_point;
    ctx.sp = loaded.stack_pointer;

    let mut thunk_manager = maarch64_thunks::ThunkManager::new();
    for (addr, name) in &loaded.dynamic_thunks {
        thunk_manager.resolve_dynamic_symbol(name, *addr);
    }

    let mut inst_count: u64 = 0;
    loop {
        inst_count += 1;
        assert!(inst_count < 10_000_000, "Rust std execution exceeded instruction limit");

        if let Some(thunk) = thunk_manager.get_thunk_by_address(ctx.pc) {
            let entry_pc = ctx.pc;
            thunk(&mut ctx, &mut mem).expect("Thunk execution failed");
            if ctx.exited {
                break;
            }
            if ctx.pc == entry_pc {
                ctx.pc = ctx.get_x(30);
            }
            continue;
        }

        match Interpreter::step(&mut ctx, &mut mem) {
            Ok(true) => {},
            Ok(false) => break,
            Err(e) => panic!("Interpreter error in rust_std_app: {:?}", e),
        }

        if ctx.exited {
            break;
        }
    }

    assert_eq!(ctx.exit_code, 0, "Rust std app exited with non-zero status");
}
