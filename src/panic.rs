use crate::sbi;
use core::panic::PanicInfo;
use sbi::ResetReason;
use sbi::ResetType;

// the processing of panic, just shutdown...
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    match info.location() {
        Some(location) => {
            println!(
                "[kernel] panicked at '{}', {}:{}:{}",
                info.message().unwrap(),
                location.file(),
                location.line(),
                location.column()
            );
        }
        None => println!("[kernel] panicked at '{}'", info.message().unwrap()),
    };
    sbi::reboot(ResetType::WarmReboot, ResetReason::SystemFailure)
}
