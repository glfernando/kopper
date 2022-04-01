use core::arch::asm;

const SYS_EXIT: usize = 0x18;

#[allow(non_upper_case_globals)]
const ADP_Stopped_ApplicationExit: usize = 0x20026;

#[derive(Debug, Clone, Copy)]
#[repr(C)]
struct SysExitParams {
    excep_type: usize,
    subcode: usize,
}

impl SysExitParams {
    pub const fn new(excep_type: usize, subcode: usize) -> Self {
        Self {
            excep_type,
            subcode,
        }
    }
}

#[inline]
fn semihost(op: usize, params_addr: usize) -> usize {
    let mut ret = op;
    unsafe {
        asm!(
            "hlt 0xf000",
            inout("x0") ret,
            in("x1") params_addr,
        );
    }
    ret
}

/// shutdown board
pub fn shutdown() -> ! {
    let params = SysExitParams::new(ADP_Stopped_ApplicationExit, 0);
    semihost(SYS_EXIT, &params as *const _ as usize);

    loop {}
}
