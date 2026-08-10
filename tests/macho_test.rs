use maarch64_core::{loader::{AutoLoader, TargetOs}, memory::MemoryManager};
use std::io::Write;

fn create_dummy_macho_arm64_binary() -> Vec<u8> {
    let mut buf = Vec::new();

    // 1. Mach-O Header 64-bit (32 bytes)
    buf.extend_from_slice(&0xfeedfacfu32.to_le_bytes()); // magic MH_MAGIC_64
    buf.extend_from_slice(&0x0100000cu32.to_le_bytes()); // cputype CPU_TYPE_ARM64
    buf.extend_from_slice(&0x00000000u32.to_le_bytes()); // cpusubtype
    buf.extend_from_slice(&0x02u32.to_le_bytes());        // filetype MH_EXECUTE
    buf.extend_from_slice(&1u32.to_le_bytes());         // ncmds
    buf.extend_from_slice(&72u32.to_le_bytes());        // sizeofcmds
    buf.extend_from_slice(&0x00200085u32.to_le_bytes()); // flags
    buf.extend_from_slice(&0u32.to_le_bytes());          // reserved

    // 2. LC_SEGMENT_64 Command (72 bytes)
    buf.extend_from_slice(&0x19u32.to_le_bytes());       // cmd LC_SEGMENT_64
    buf.extend_from_slice(&72u32.to_le_bytes());        // cmdsize
    let mut segname = [0u8; 16];
    segname[0..6].copy_from_slice(b"__TEXT");
    buf.extend_from_slice(&segname);                     // segname
    buf.extend_from_slice(&0x00000000u64.to_le_bytes()); // vmaddr
    buf.extend_from_slice(&0x00001000u64.to_le_bytes()); // vmsize
    buf.extend_from_slice(&0u64.to_le_bytes());          // fileoff
    buf.extend_from_slice(&104u64.to_le_bytes());        // filesize
    buf.extend_from_slice(&7i32.to_le_bytes());          // maxprot (rwx)
    buf.extend_from_slice(&5i32.to_le_bytes());          // initprot (rx)
    buf.extend_from_slice(&0u32.to_le_bytes());          // nsects
    buf.extend_from_slice(&0u32.to_le_bytes());          // flags

    // 3. Segment Data (AArch64 Code: MOV X0, #42; RET)
    buf.extend_from_slice(&0xd2800540u32.to_le_bytes()); // mov x0, #42
    buf.extend_from_slice(&0xd65f03c0u32.to_le_bytes()); // ret

    buf
}

#[test]
fn test_macho_arm64_loader_parse_and_stack() {
    let dummy_macho = create_dummy_macho_arm64_binary();
    let temp_dir = std::env::temp_dir();
    let file_path = temp_dir.join("test_macho_arm64");
    {
        let mut f = std::fs::File::create(&file_path).unwrap();
        f.write_all(&dummy_macho).unwrap();
    }

    let mut mem = MemoryManager::new();
    let loaded = AutoLoader::load_file_with_args(&file_path, &["test_macho_arm64", "arg1"], &mut mem).unwrap();

    assert_eq!(loaded.target_os, TargetOs::Darwin);
    assert!(loaded.entry_point >= 0x100000000);
    assert!(loaded.stack_pointer != 0);

    // Read argc from Darwin stack
    let argc_bytes = mem.read(loaded.stack_pointer, 8).unwrap();
    let argc = u64::from_le_bytes(argc_bytes.try_into().unwrap());
    assert_eq!(argc, 2);

    let _ = std::fs::remove_file(file_path);
}
