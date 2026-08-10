use maarch64_core::{cpu::CpuContext, memory::MemoryManager};
use maarch64_thunks::ThunkManager;

#[test]
fn test_sdl2_thunk_registration_and_execution() {
    let mut thunk_manager = ThunkManager::new();
    let mut ctx = CpuContext::new();
    let mut mem = MemoryManager::new();

    // 1. Test SDL_Init(SDL_INIT_VIDEO = 0x00000020)
    ctx.set_x(0, 0x00000020);
    thunk_manager.resolve_dynamic_symbol("SDL_Init", 0x7f002000);
    let handler = thunk_manager.get_thunk_by_address(0x7f002000).unwrap();
    let res = handler(&mut ctx, &mut mem);
    assert!(res.is_ok());

    // 2. Test SDL_CreateWindow
    let title_addr = 0x400000;
    mem.map_anonymous(title_addr, 0x1000).unwrap();
    mem.write(title_addr, b"Maarch64 Test Window\0").unwrap();

    ctx.set_x(0, title_addr); // title
    ctx.set_x(1, 100);        // x
    ctx.set_x(2, 100);        // y
    ctx.set_x(3, 640);        // w
    ctx.set_x(4, 480);        // h
    ctx.set_x(5, 0);          // flags

    thunk_manager.resolve_dynamic_symbol("SDL_CreateWindow", 0x7f002008);
    let create_win_fn = thunk_manager.get_thunk_by_address(0x7f002008).unwrap();
    let win_res = create_win_fn(&mut ctx, &mut mem);
    assert!(win_res.is_ok());
    let window_ptr = ctx.get_x(0);
    assert!(window_ptr != 0);

    // 3. Test SDL_DestroyWindow
    ctx.set_x(0, window_ptr);
    thunk_manager.resolve_dynamic_symbol("SDL_DestroyWindow", 0x7f002010);
    let destroy_win_fn = thunk_manager.get_thunk_by_address(0x7f002010).unwrap();
    let destroy_res = destroy_win_fn(&mut ctx, &mut mem);
    assert!(destroy_res.is_ok());

    // 4. Test SDL_Quit
    thunk_manager.resolve_dynamic_symbol("SDL_Quit", 0x7f002018);
    let quit_fn = thunk_manager.get_thunk_by_address(0x7f002018).unwrap();
    let quit_res = quit_fn(&mut ctx, &mut mem);
    assert!(quit_res.is_ok());
}
