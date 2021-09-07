#[cfg(target_arch = "aarch64")]
/// implement aarch64 low level functions
pub mod aarch64;
#[cfg(target_arch = "aarch64")]
pub use aarch64::*;

#[cfg(feature = "board_pico")]
/// implement raspberry pi pico low level functions
pub mod pico;
#[cfg(feature = "board_pico")]
pub use pico::*;

fn ticks_to_xs(ticks: u64, x: u32) -> u64 {
    ticks * x as u64 / freq()
}

fn xs_to_ticks(val: u64, x: u32) -> u64 {
    val * freq() / x as u64
}

/// convert ticks to miliseconds
pub fn ticks_to_ms(ticks: u64) -> u64 {
    ticks_to_xs(ticks, 1000)
}

/// convert miliseconds to ticks
pub fn ms_to_ticks(ms: u64) -> u64 {
    xs_to_ticks(ms, 1000)
}

/// convert ticks to microseconds
pub fn ticks_to_us(ticks: u64) -> u64 {
    ticks_to_xs(ticks, 1000000)
}

/// convert microseconds to ticks
pub fn us_to_ticks(us: u64) -> u64 {
    xs_to_ticks(us, 1000000)
}

/// convert ticks to nanooseconds
pub fn ticks_to_ns(ticks: u64) -> u64 {
    ticks_to_xs(ticks, 1000000000)
}

/// convert nanoseconds to ticks
pub fn ns_to_ticks(ns: u64) -> u64 {
    xs_to_ticks(ns, 1000000000)
}

/// get timestamp in miliseconds
pub fn ms() -> u64 {
    ticks_to_ms(ticks())
}

/// get timestamp in microseconds
pub fn us() -> u64 {
    ticks_to_us(ticks())
}

/// get timestamp in manoseconds
pub fn ns() -> u64 {
    ticks_to_ns(ticks())
}
