use super::call::sbicall;
const EXTENSION_HSM: usize = 0x48534D;

pub fn sbi_hart_start(
    hartid: usize,
    start_addr: usize,
    opaque: usize
) -> super::ret::Ret {
    sbicall(
        EXTENSION_HSM,
        0,
        [hartid, start_addr, opaque, 0, 0]
    )
}