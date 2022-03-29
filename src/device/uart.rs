#[cfg(feature = "pl011")]
/// Driver for PL011
pub mod pl011;

/// Uart interface
pub trait Uart {
    /// error return by Uart implementation
    type Error;

    /// write to uart
    fn write(&self, data: &[u8]) -> Result<(), Self::Error>;

    /// read from uart
    fn read(&self, data: &mut [u8]) -> Result<(), Self::Error>;
}
