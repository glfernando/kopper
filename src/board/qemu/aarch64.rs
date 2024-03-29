/// code for debugging
pub mod debug;

/// peripherals definition
pub mod peripherals;

/// power functions
pub mod power;

/// early qemu-aarch64 initialization
pub fn early_init() {}

/// qemu-aarch64 initialization
pub fn init() {
    peripherals::init();
}
