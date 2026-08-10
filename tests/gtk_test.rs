use maarch64_core::{cpu::CpuContext, memory::MemoryManager};
use maarch64_thunks::ThunkManager;

#[test]
fn test_gtk_thunk_registration_and_execution() {
    let mut thunk_manager = ThunkManager::new();
    let mut ctx = CpuContext::new();
    let mut mem = MemoryManager::new();

    // 1. Test gtk_init_check
    ctx.set_x(0, 0); // argc = NULL
    ctx.set_x(1, 0); // argv = NULL
    thunk_manager.resolve_dynamic_symbol("gtk_init_check", 0x7f003000);
    let init_check_fn = thunk_manager.get_thunk_by_address(0x7f003000).unwrap();
    let res = init_check_fn(&mut ctx, &mut mem);
    assert!(res.is_ok());

    // 2. Test gtk_window_new(GTK_WINDOW_TOPLEVEL = 0)
    ctx.set_x(0, 0);
    thunk_manager.resolve_dynamic_symbol("gtk_window_new", 0x7f003008);
    let win_new_fn = thunk_manager.get_thunk_by_address(0x7f003008).unwrap();
    let win_res = win_new_fn(&mut ctx, &mut mem);
    assert!(win_res.is_ok());
    let window_ptr = ctx.get_x(0);
    assert!(window_ptr != 0);

    // 3. Test gtk_window_set_title
    let title_addr = 0x400000;
    mem.map_anonymous(title_addr, 0x1000).unwrap();
    mem.write(title_addr, b"Maarch64 GTK Test\0").unwrap();

    ctx.set_x(0, window_ptr);
    ctx.set_x(1, title_addr);
    thunk_manager.resolve_dynamic_symbol("gtk_window_set_title", 0x7f003010);
    let set_title_fn = thunk_manager.get_thunk_by_address(0x7f003010).unwrap();
    let title_res = set_title_fn(&mut ctx, &mut mem);
    assert!(title_res.is_ok());

    // 4. Test gtk_widget_destroy
    ctx.set_x(0, window_ptr);
    thunk_manager.resolve_dynamic_symbol("gtk_widget_destroy", 0x7f003018);
    let destroy_fn = thunk_manager.get_thunk_by_address(0x7f003018).unwrap();
    let destroy_res = destroy_fn(&mut ctx, &mut mem);
    assert!(destroy_res.is_ok());
}
