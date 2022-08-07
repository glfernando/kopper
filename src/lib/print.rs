use crate::{board, device};
use core::fmt;
use core::fmt::Write;
use device::console::Console;

struct ConsolePrinter;

impl core::fmt::Write for ConsolePrinter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        let console = board::debug::console();
        for c in s.chars() {
            console.putc(c).map_err(|_| fmt::Error {})?;
        }
        Ok(())
    }
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    let mut con = ConsolePrinter {};
    con.write_fmt(args).unwrap();
}

/// retunrs standard output console
pub fn get_stdio() -> impl core::fmt::Write {
    ConsolePrinter {}
}

/// Prints without a newline.
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::print::_print(format_args!($($arg)*)));
}

/// Prints with a newline.
#[macro_export]
macro_rules! println {
    () => ($crate::lib::print!("\n"));
    ($($arg:tt)*) => ({
        $crate::lib::print::_print(format_args_nl!($($arg)*));
    })
}
