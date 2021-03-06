use core::panic::PanicInfo;
use crate::sbi;

// panic的处理
#[panic_handler]
fn panic(_: &PanicInfo) -> ! {
    sbi::shutdown()
}