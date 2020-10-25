use crate::lib::timestamp;

/// do a busy loop for @us microseconds
pub fn udelay(us: u64) {
    let t = timestamp::ticks() + timestamp::us_to_ticks(us);
    while t > timestamp::ticks() {}
}

/// do a busy loop for @ms miliseconds
pub fn mdelay(ms: u64) {
    let t = timestamp::ticks() + timestamp::ms_to_ticks(ms);
    while t > timestamp::ticks() {}
}
