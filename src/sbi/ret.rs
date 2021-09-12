#![allow(unused)]
#![allow(non_camel_case_types)]

#[derive(Debug)]
pub struct SbiRet {
    pub error: ErrorType,
    pub value: isize,
}

// define error type，RISCV SBI standard
// 'Other' to match console_getchar
#[repr(isize)]
#[derive(Debug)]
pub enum ErrorType {
    SBI_SUCCESS = 0,
    SBI_ERR_FAILED = -1,
    SBI_ERR_NOT_SUPPORTED = -2,
    SBI_ERR_INVALID_PARAM = -3,
    SBI_ERR_DENIED = -4,
    SBI_ERR_INVALID_ADDRESS = -5,
    SBI_ERR_ALREADY_AVAILABLE = -6,
    Other(isize) = -7,
}

impl From<isize> for ErrorType {
    fn from(e: isize) -> Self {
        match e {
            0 => ErrorType::SBI_SUCCESS,
            -1 => ErrorType::SBI_ERR_FAILED,
            -2 => ErrorType::SBI_ERR_NOT_SUPPORTED,
            -3 => ErrorType::SBI_ERR_INVALID_PARAM,
            -4 => ErrorType::SBI_ERR_DENIED,
            -5 => ErrorType::SBI_ERR_INVALID_ADDRESS,
            -6 => ErrorType::SBI_ERR_ALREADY_AVAILABLE,
            v => ErrorType::Other(v),
        }
    }
}

impl From<ErrorType> for isize {
    fn from(e: ErrorType) -> Self {
        match e {
            ErrorType::SBI_SUCCESS => 0,
            ErrorType::SBI_ERR_FAILED => -1,
            ErrorType::SBI_ERR_NOT_SUPPORTED => -2,
            ErrorType::SBI_ERR_INVALID_PARAM => -3,
            ErrorType::SBI_ERR_DENIED => -4,
            ErrorType::SBI_ERR_INVALID_ADDRESS => -5,
            ErrorType::SBI_ERR_ALREADY_AVAILABLE => -6,
            ErrorType::Other(v) => v,
        }
    }
}
