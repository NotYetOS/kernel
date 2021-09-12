use super::call::sbicall;
use super::ret::SbiRet;

const EXTENSION_HSM: usize = 0x48534D;

#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq, Eq)]
pub enum HSMHartStates {
    STARTED,
    STOPPED,
    START_PENDING,
    STOP_PENDING,
    SUSPENDED,
    SUSPEND_PRNDING,
    RESUME_PENDING,
    UNKNOWN,
}

impl From<SbiRet> for HSMHartStates {
    fn from(ret: SbiRet) -> Self {
        match ret.value {
            0 => HSMHartStates::STARTED,
            1 => HSMHartStates::STOPPED,
            2 => HSMHartStates::START_PENDING,
            3 => HSMHartStates::STOP_PENDING,
            4 => HSMHartStates::SUSPENDED,
            5 => HSMHartStates::SUSPEND_PRNDING,
            6 => HSMHartStates::RESUME_PENDING,
            _ => HSMHartStates::UNKNOWN,
        }
    }
}

pub fn sbi_hart_start(hart_id: usize, start_addr: usize, opaque: usize) -> SbiRet {
    sbicall(EXTENSION_HSM, 0, [hart_id, start_addr, opaque, 0, 0])
}

pub fn sbi_hart_stop() -> SbiRet {
    sbicall(EXTENSION_HSM, 1, [0, 0, 0, 0, 0])
}

pub fn sbi_hart_get_status(hart_id: usize) -> SbiRet {
    sbicall(EXTENSION_HSM, 2, [hart_id, 0, 0, 0, 0])
}
