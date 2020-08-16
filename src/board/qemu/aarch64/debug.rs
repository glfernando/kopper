use crate::device;
use core::fmt;
struct QemuDebugCon;

impl device::console::ConsoleOut for QemuDebugCon {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        unsafe {
            core::ptr::write_volatile(0x0900_002c as *mut u32, 1 << 4);
            core::ptr::write_volatile(0x0900_0030 as *mut u32, 0x301);
        }
        for c in s.chars() {
            unsafe {
                core::ptr::write_volatile(0x0900_0000 as *mut u32, c as u32);
            }
        }
        Ok(())
    }
}

/// return simple consout out object
pub fn console() -> impl device::console::ConsoleOut {
    QemuDebugCon {}
}
