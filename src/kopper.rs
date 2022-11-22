//! kopper is a simple baremetal kernel

#![no_std]
#![feature(format_args_nl)]
#![feature(alloc_error_handler)]
#![feature(naked_functions)]
#![feature(const_mut_refs)]
#![allow(special_module_name)]

extern crate kopper_macros as macros;

/// kopper libraries
pub mod lib;

/// devices
pub mod device;

/// boards
pub mod board;

mod cpu;
mod panic;
mod rrt;

pub use macros::shell_cmd;
