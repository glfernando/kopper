//
// SPDX-License-Identifier: BSD-3-Clause
//
// Copyright (c) 2020 Fernando Lugo <lugo.fernando@gmail.com>
//

//! kopper main application

#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(format_args_nl)]
#![feature(asm)]

use kopper::println;

static VERSION: &str = "0.0.1";

/// kopper app entry point
#[no_mangle]
pub fn main() -> ! {
    println!("Welcome to kopper version {}", VERSION);

    loop {}
}
