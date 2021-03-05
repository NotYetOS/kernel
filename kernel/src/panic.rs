use core::panic::PanicInfo;

// panic的处理
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}