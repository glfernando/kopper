/// return number of ticks since power on
pub fn ticks() -> u64 {
    let r: u64;
    unsafe {
        asm!("mrs   {}, cntpct_el0", out(reg) r);
    }
    r
}

/// return frequency of the tick timer
pub fn freq() -> u64 {
    let f: u64;
    unsafe {
        asm!("mrs   {}, cntfrq_el0", out(reg) f);
    }
    f
}
