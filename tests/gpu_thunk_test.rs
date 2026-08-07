use maarch64_core::{cpu::CpuContext, memory::MemoryManager};
use maarch64_thunks::ThunkManager;

#[test]
fn test_gpu_thunk_registration_and_execution() {
    let manager = ThunkManager::new();
    let mut ctx = CpuContext::new();
    let mut mem = MemoryManager::new();

    // Check EGL thunk symbol resolution
    let egl_get_display = manager.get_thunk("eglGetDisplay");
    assert!(egl_get_display.is_some(), "eglGetDisplay thunk should be registered");

    let handler = egl_get_display.unwrap();
    ctx.set_x(0, 0); // EGL_DEFAULT_DISPLAY
    let res = handler(&mut ctx, &mut mem);
    assert!(res.is_ok(), "eglGetDisplay execution failed");
    assert_ne!(ctx.get_x(0), 0, "eglGetDisplay should return non-zero handle");

    // Check OpenGL thunk symbol resolution
    let gl_get_string = manager.get_thunk("glGetString");
    assert!(gl_get_string.is_some(), "glGetString thunk should be registered");

    let gl_handler = gl_get_string.unwrap();
    ctx.set_x(0, 0x1F00); // GL_VENDOR
    let res = gl_handler(&mut ctx, &mut mem);
    assert!(res.is_ok(), "glGetString execution failed");

    let str_addr = ctx.get_x(0);
    assert_ne!(str_addr, 0, "glGetString should return valid memory address");
    let vendor_str = String::from_utf8_lossy(&mem.read_string(str_addr).unwrap()).into_owned();
    assert!(vendor_str.contains("Maarch64"), "glGetString vendor should contain Maarch64");
}
