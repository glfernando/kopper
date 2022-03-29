/// console devices
pub mod console;

/// uart devices
pub mod uart;

use core::marker::PhantomData;

/// device classes
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Class {
    /// uart device
    Uart,
    /// console device
    Console,
}

/// Generic device interface
pub trait Device {
    /// get device class
    fn class(&self) -> Class;
    /// get device name
    fn name(&self) -> &'static str;

    /// init device
    fn init(&self) {}
    /// deinit device
    fn deinit(&self) {}
}

/// wraps a register block
pub struct RegistersWrapper<T> {
    start_addr: usize,
    phantom: PhantomData<fn() -> T>,
}

/// wraps register block
impl<T> RegistersWrapper<T> {
    /// Create an instance.
    pub const fn new(start_addr: usize) -> Self {
        Self {
            start_addr,
            phantom: PhantomData,
        }
    }
}

impl<T> core::ops::Deref for RegistersWrapper<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        unsafe { &*(self.start_addr as *const _) }
    }
}
