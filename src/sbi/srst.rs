#![allow(unused)]

use super::call::sbicall;

const EXTENSION_SRST: usize = 0x53525354;

pub enum ResetType {
    Shutdown,
    ColdReboot,
    WarmReboot,
}

pub enum ResetReason {
    NoReason,
    SystemFailure,
}

pub fn shutdown(reset_reason: ResetReason) -> ! {
    sbicall(
        EXTENSION_SRST, 
        0, 
        [0, reset_reason as usize, 0, 0, 0]
    );
    unreachable!()
}

pub fn reboot(reset_type: ResetType, reset_reason: ResetReason) -> ! {
    sbicall(
        EXTENSION_SRST, 
        0, 
        [reset_type as usize, reset_reason as usize, 0, 0, 0]
    );
    unreachable!()
}
