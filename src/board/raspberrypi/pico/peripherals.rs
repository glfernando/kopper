use crate::device::uart::pl011::Pl011;
use crate::device::Device;

static UART0: Pl011 = Pl011::new("uart0", 0x4003_4000, 125_000_000, 230400);

/// initialize peripherals
pub fn init() {
    UART0.init();
}
