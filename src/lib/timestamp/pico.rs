/// return number of ticks since power on
pub fn ticks() -> u64 {
    let hi = unsafe { core::ptr::read_volatile(0x4005_4008 as *mut u32) };
    let lo = unsafe { core::ptr::read_volatile(0x4005_400c as *mut u32) };
    (hi as u64) << 32 | lo as u64
}

/// return frequency of the tick timer
pub fn freq() -> u64 {
    1_000_000
}
