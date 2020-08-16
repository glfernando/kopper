use crate::{board, device};
use core::fmt;

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use device::console::ConsoleOut;

    board::debug::console().write_fmt(args).unwrap();
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
