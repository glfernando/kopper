//! kopper is a simple baremetal kernel

#![no_std]
#![feature(global_asm)]
#![feature(format_args_nl)]
#![feature(asm)]
#![feature(alloc_error_handler)]
//#![feature(const_raw_ptr_to_usize_cast)]
#![feature(naked_functions)]

/// kopper libraries
#[path = "lib/lib.rs"]
pub mod lib;

/// devices
pub mod device;

/// boards
pub mod board;

mod cpu;
mod panic;
