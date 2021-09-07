/// rapsberrypi boards

#[cfg(feature = "board_pico")]
#[path = "raspberrypi/pico/board.rs"]
pub mod pico;

#[cfg(feature = "board_pico")]
pub use pico::*;
