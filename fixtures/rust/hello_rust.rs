#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let msg = b"Hello from Rust on ARM64!\n";
    
    unsafe {
        // sys_write(1, msg.as_ptr(), msg.len())
        core::arch::asm!(
            "mov x0, #1",
            "mov x1, {0}",
            "mov x2, {1}",
            "mov x8, #64",
            "svc #0",
            in(reg) msg.as_ptr(),
            in(reg) msg.len(),
            out("x0") _,
            out("x1") _,
            out("x2") _,
            out("x8") _,
        );

        // sys_exit(0)
        core::arch::asm!(
            "mov x0, #0",
            "mov x8, #93",
            "svc #0",
            out("x0") _,
            out("x8") _,
        );
    }

    loop {}
}
