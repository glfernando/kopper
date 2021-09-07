/// QEMU boards
#[cfg(feature = "qemu")]
pub mod qemu;

#[cfg(feature = "qemu")]
pub use qemu::*;

/// Raspberry pi boards
#[cfg(feature = "raspberrypi")]
pub mod raspberrypi;

#[cfg(feature = "raspberrypi")]
pub use raspberrypi::*;
