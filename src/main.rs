//
// SPDX-License-Identifier: BSD-3-Clause
//
// Copyright (c) 2020 Fernando Lugo <lugo.fernando@gmail.com>
//

//! kopper is a simple baremetal kernel

#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(format_args_nl)]

mod cpu;
mod panic;

/// board specific code
pub mod board;
/// devices
pub mod device;
/// all kopper libraries
#[path = "klib.rs"]
pub mod lib;

static VERSION: &str = "0.0.1";

/// kernel main function
pub fn kmain() -> ! {
    println!("Welcome to kopper version {}", VERSION);
    loop {}
}
