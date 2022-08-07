use crate::device::uart::pl011::Pl011;
use crate::device::Device;

static UART0: Pl011 = Pl011::new("uart0", 0x0900_0000, 24_000_000, 115200);

/// initialize qemu-aarch64 peripherals
pub fn init() {
    UART0.init();
}
