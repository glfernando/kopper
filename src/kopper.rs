//! kopper is a simple baremetal kernel

#![no_std]
#![feature(global_asm)]
#![feature(format_args_nl)]
#![feature(asm)]

/// kopper libraries
pub mod lib;

/// devices
pub mod device;

/// boards
pub mod board;

mod cpu;
mod panic;
