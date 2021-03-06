use core::panic::PanicInfo;
use crate::sbi;

// panic的处理
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    match info.location() {
        Some(location) => {
            println!("[kernel] Panicked at {}:{} {}", 
                location.file(), 
                location.line(), 
                info.message().unwrap()
            );
        }
        None => println!("[kernel] Panicked: {}", info.message().unwrap())
    }
    sbi::shutdown()
}