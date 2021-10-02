use crate::device;

struct QemuDebugCon;

impl device::console::Console for QemuDebugCon {
    fn putc(&self, c: char) -> Result<(), &'static str> {
        unsafe {
            while (core::ptr::read_volatile(0x0900_0018 as *mut u32) & (1 << 5)) != 0 {}
            core::ptr::write_volatile(0x0900_0000 as *mut u32, c as u32);
        }
        Ok(())
    }

    fn getc(&self) -> Result<char, &'static str> {
        let c = unsafe {
            while (core::ptr::read_volatile(0x0900_0018 as *mut u32) & (1 << 4)) != 0 {}
            core::ptr::read_volatile(0x0900_0000 as *mut u32) as u8
        };
        Ok(c as char)
    }
}

/// return simple consout out object
pub fn console() -> impl device::console::Console {
    QemuDebugCon {}
}
