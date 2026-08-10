use maarch64_core::{cpu::CpuContext, memory::MemoryManager};
use maarch64_thunks::ThunkManager;

#[test]
fn test_qt_thunk_registration_and_execution() {
    let mut thunk_manager = ThunkManager::new();
    let mut ctx = CpuContext::new();
    let mut mem = MemoryManager::new();

    // 1. Test QApplication_create
    ctx.set_x(0, 0); // argc = 0
    ctx.set_x(1, 0); // argv = NULL
    thunk_manager.resolve_dynamic_symbol("QApplication_create", 0x7f004000);
    let app_create_fn = thunk_manager.get_thunk_by_address(0x7f004000).unwrap();
    let app_res = app_create_fn(&mut ctx, &mut mem);
    assert!(app_res.is_ok());
    let app_ptr = ctx.get_x(0);
    assert!(app_ptr != 0);

    // 2. Test QWidget_create
    ctx.set_x(0, 0); // parent = NULL
    ctx.set_x(1, 0); // flags = 0
    thunk_manager.resolve_dynamic_symbol("QWidget_create", 0x7f004008);
    let win_create_fn = thunk_manager.get_thunk_by_address(0x7f004008).unwrap();
    let win_res = win_create_fn(&mut ctx, &mut mem);
    assert!(win_res.is_ok());
    let widget_ptr = ctx.get_x(0);
    assert!(widget_ptr != 0);

    // 3. Test QWidget_setWindowTitle
    let title_addr = 0x400000;
    mem.map_anonymous(title_addr, 0x1000).unwrap();
    mem.write(title_addr, b"Maarch64 Qt Test Window\0").unwrap();

    ctx.set_x(0, widget_ptr);
    ctx.set_x(1, title_addr);
    thunk_manager.resolve_dynamic_symbol("QWidget_setWindowTitle", 0x7f004010);
    let set_title_fn = thunk_manager.get_thunk_by_address(0x7f004010).unwrap();
    let title_res = set_title_fn(&mut ctx, &mut mem);
    assert!(title_res.is_ok());

    // 4. Test QWidget_show
    ctx.set_x(0, widget_ptr);
    thunk_manager.resolve_dynamic_symbol("QWidget_show", 0x7f004018);
    let show_fn = thunk_manager.get_thunk_by_address(0x7f004018).unwrap();
    let show_res = show_fn(&mut ctx, &mut mem);
    assert!(show_res.is_ok());

    // 5. Test C++ Mangled Symbol Resolution _ZN7QWidget4showEv
    thunk_manager.resolve_dynamic_symbol("_ZN7QWidget4showEv", 0x7f004020);
    let mangled_show_fn = thunk_manager.get_thunk_by_address(0x7f004020).unwrap();
    let mangled_res = mangled_show_fn(&mut ctx, &mut mem);
    assert!(mangled_res.is_ok());
}
