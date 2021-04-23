// SBI is a good thing

#[inline]
pub fn sbicall(eid: usize, fid: usize, args: [usize; 5]) -> super::ret::SbiRet {
    let error: isize;
    let value: isize;

    // ecall to sbi
    unsafe {
        asm! {
            "ecall",
            lateout("x10") error, 
            lateout("x11") value,
            in("x10") args[0], in("x11") args[1], in("x12") args[2], 
            in("x13") args[3], in("x14") args[4], 
            in("x17") eid,
            in("x16") fid,
            options(nostack)
        }
    }

    super::ret::SbiRet {
        error: error.into(),
        value,
    }
}
