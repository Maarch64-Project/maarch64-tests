use maarch64_core::{cpu::CpuContext, memory::MemoryManager};
use maarch64_thunks::ThunkManager;

#[test]
fn test_curl_thunk_registration_and_execution() {
    let mut thunk_manager = ThunkManager::new();
    let mut ctx = CpuContext::new();
    let mut mem = MemoryManager::new();

    // 1. Test curl_global_init(CURL_GLOBAL_ALL = 3)
    ctx.set_x(0, 3);
    thunk_manager.resolve_dynamic_symbol("curl_global_init", 0x7f005000);
    let global_init_fn = thunk_manager.get_thunk_by_address(0x7f005000).unwrap();
    let init_res = global_init_fn(&mut ctx, &mut mem);
    assert!(init_res.is_ok());

    // 2. Test curl_easy_init
    thunk_manager.resolve_dynamic_symbol("curl_easy_init", 0x7f005008);
    let easy_init_fn = thunk_manager.get_thunk_by_address(0x7f005008).unwrap();
    let easy_res = easy_init_fn(&mut ctx, &mut mem);
    assert!(easy_res.is_ok());
    let handle = ctx.get_x(0);
    assert!(handle != 0);

    // 3. Test curl_easy_setopt (CURLOPT_URL = 10002)
    let url_addr = 0x400000;
    mem.map_anonymous(url_addr, 0x1000).unwrap();
    mem.write(url_addr, b"https://httpbin.org/get\0").unwrap();

    ctx.set_x(0, handle);
    ctx.set_x(1, 10002);
    ctx.set_x(2, url_addr);
    thunk_manager.resolve_dynamic_symbol("curl_easy_setopt", 0x7f005010);
    let setopt_fn = thunk_manager.get_thunk_by_address(0x7f005010).unwrap();
    let setopt_res = setopt_fn(&mut ctx, &mut mem);
    assert!(setopt_res.is_ok());

    // 4. Test curl_easy_cleanup
    ctx.set_x(0, handle);
    thunk_manager.resolve_dynamic_symbol("curl_easy_cleanup", 0x7f005018);
    let cleanup_fn = thunk_manager.get_thunk_by_address(0x7f005018).unwrap();
    let cleanup_res = cleanup_fn(&mut ctx, &mut mem);
    assert!(cleanup_res.is_ok());

    // 5. Test OpenSSL stub OPENSSL_init_ssl
    thunk_manager.resolve_dynamic_symbol("OPENSSL_init_ssl", 0x7f005020);
    let ssl_init_fn = thunk_manager.get_thunk_by_address(0x7f005020).unwrap();
    let ssl_res = ssl_init_fn(&mut ctx, &mut mem);
    assert!(ssl_res.is_ok());
    assert_eq!(ctx.get_x(0), 1);
}
