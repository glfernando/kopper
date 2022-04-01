//
// SPDX-License-Identifier: BSD-3-Clause
//
// Copyright (c) 2020 Fernando Lugo <lugo.fernando@gmail.com>
//

//! kopper main application

#![no_std]
#![no_main]
#![feature(format_args_nl)]

mod cmds;

use kopper::board;
use kopper::lib::console;
use kopper::println;

const VERSION: &str = env!("CARGO_PKG_VERSION");

/// kopper app entry point
#[no_mangle]
pub fn main() -> ! {
    println!("Welcome to kopper version {VERSION}");

    console::run_shell(&board::debug::console()).unwrap();

    board::power::shutdown();
}
