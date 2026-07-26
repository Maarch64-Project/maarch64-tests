use maarch64_core::{
    cpu::CpuContext,
    interp::Interpreter,
    loader::ElfLoader,
    memory::MemoryManager,
};
use std::path::PathBuf;

fn run_busybox_applet(binary_name: &str, applet: &str) -> Result<u64, String> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let bin_path = manifest_dir.join("bin").join(binary_name);

    if !bin_path.exists() {
        return Ok(0); // Skip if precompiled binary is not present in build environment
    }

    let mut mem = MemoryManager::new();
    let target_args = vec![binary_name, applet];
    let loaded = ElfLoader::load_file_with_args(&bin_path, &target_args, &mut mem)
        .map_err(|e| e.to_string())?;

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
        if inst_count >= 10_000_000 {
            return Err("Execution exceeded instruction limit".to_string());
        }

        if let Some(thunk) = thunk_manager.get_thunk_by_address(ctx.pc) {
            let entry_pc = ctx.pc;
            thunk(&mut ctx, &mut mem)?;
            if ctx.pc == entry_pc {
                ctx.pc = ctx.get_x(30);
            }
            continue;
        }

        match Interpreter::step(&mut ctx, &mut mem) {
            Ok(true) => {},
            Ok(false) => break,
            Err(e) => return Err(format!("{:?}", e)),
        }
    }

    Ok(inst_count)
}

#[test]
fn test_static_busybox_whoami() {
    let res = run_busybox_applet("busybox", "whoami");
    assert!(res.is_ok(), "Static BusyBox whoami failed: {:?}", res);
}

#[test]
fn test_dynamic_busybox_whoami() {
    let res = run_busybox_applet("busybox_dynamic", "whoami");
    assert!(res.is_ok(), "Dynamic BusyBox whoami failed: {:?}", res);
}

#[test]
fn test_static_busybox_echo() {
    let res = run_busybox_applet("busybox", "echo");
    assert!(res.is_ok(), "Static BusyBox echo failed: {:?}", res);
}

#[test]
fn test_dynamic_busybox_echo() {
    let res = run_busybox_applet("busybox_dynamic", "echo");
    assert!(res.is_ok(), "Dynamic BusyBox echo failed: {:?}", res);
}
