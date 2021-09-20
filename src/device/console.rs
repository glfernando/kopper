/// trait for a Console device
pub trait Console {
    /// send a char to the console
    fn putc(&self, c: char) -> Result<(), &'static str>;

    /// get a char from the console
    fn getc(&self) -> Result<char, &'static str>;
}
