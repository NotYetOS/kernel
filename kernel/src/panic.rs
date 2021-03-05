use core::panic::PanicInfo;

// panic的处理函数
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    loop {}
}