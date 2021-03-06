#![allow(unused)]
#![allow(non_camel_case_types)]

pub struct Ret {
    pub error: ErrorType,
    pub value: isize,
}

// 定义错误类型，RISCV标准
// Undefind不该出现，但是为了匹配_
pub enum ErrorType {
    SBI_SUCCESS = 0,
    SBI_ERR_FAILED = -1,
    SBI_ERR_NOT_SUPPORTED = -2,
    SBI_ERR_INVALID_PARAM = -3,
    SBI_ERR_DENIED = -4,
    SBI_ERR_INVALID_ADDRESS = -5,
    SBI_ERR_ALREADY_AVAILABLE = -6,
    Undefind = -7,
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
            _ => ErrorType::Undefind
        }
    }
}