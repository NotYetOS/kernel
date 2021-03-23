use core::panic::PanicInfo;
use crate::sbi;

// panic的处理
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    match info.location() {
        Some(location) => {
            println!("[kernel] panicked at '{}', {}:{}:{}", 
                info.message().unwrap(),
                location.file(), 
                location.line(),
                location.column()
            );
        }
        None => println!("[kernel] panicked at '{}'", info.message().unwrap())
    }
    sbi::shutdown()
}