use crate::device;
use core::fmt;
struct QemuDebugCon;

impl device::console::ConsoleOut for QemuDebugCon {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.chars() {
            unsafe {
                while (core::ptr::read_volatile(0x4003_4018 as *mut u32) & (1 << 5)) != 0 {}
                core::ptr::write_volatile(0x4003_4000 as *mut u32, c as u32);
            }
        }
        Ok(())
    }
}

/// return simple consout out object
pub fn console() -> impl device::console::ConsoleOut {
    QemuDebugCon {}
}
