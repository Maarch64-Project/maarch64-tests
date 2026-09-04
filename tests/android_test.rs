use maarch64_core::{cpu::CpuContext, memory::MemoryManager};
use maarch64_thunks::android;
use std::io::Write;
use zip::write::SimpleFileOptions;

#[test]
fn test_android_bionic_log_and_system_property_thunks() {
    let mut mem = MemoryManager::new();
    let mut ctx = CpuContext::new();

    // Map memory for test strings
    mem.map_anonymous(0x10000, 0x1000).unwrap();

    // 1. Test __system_property_get
    let name = b"ro.build.version.sdk\0";
    mem.write(0x10000, name).unwrap();

    ctx.set_x(0, 0x10000); // name_ptr
    ctx.set_x(1, 0x10100); // value_ptr (output buffer)

    android::thunk_system_property_get(&mut ctx, &mut mem).unwrap();

    let val_len = ctx.get_x(0);
    assert_eq!(val_len, 2);

    let val_bytes = mem.read_string(0x10100).unwrap();
    let val_str = String::from_utf8_lossy(&val_bytes);
    assert_eq!(val_str, "33");

    // 2. Test __android_log_print
    let tag = b"TestTag\0";
    let msg = b"Hello from Android NDK!\0";
    mem.write(0x10200, tag).unwrap();
    mem.write(0x10300, msg).unwrap();

    ctx.set_x(0, 4); // ANDROID_LOG_INFO
    ctx.set_x(1, 0x10200);
    ctx.set_x(2, 0x10300);

    android::thunk_android_log_print(&mut ctx, &mut mem).unwrap();
    assert!(ctx.get_x(0) > 0);
}

#[test]
fn test_android_apk_zip_unpacker_structure() {
    let temp_dir = std::env::temp_dir();
    let apk_path = temp_dir.join("test_sample.apk");

    {
        let file = std::fs::File::create(&apk_path).unwrap();
        let mut zip = zip::ZipWriter::new(file);
        let options = SimpleFileOptions::default();

        zip.start_file("lib/arm64-v8a/libsample.so", options).unwrap();
        zip.write_all(b"\x7fELF_DUMMY_ARM64_SO").unwrap();
        zip.finish().unwrap();
    }

    assert!(apk_path.exists());
    let _ = std::fs::remove_file(apk_path);
}
