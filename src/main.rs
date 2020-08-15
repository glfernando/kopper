//
// SPDX-License-Identifier: BSD-3-Clause
//
// Copyright (c) 2020 Fernando Lugo <lugo.fernando@gmail.com>
//

//! kopper is a simple baremetal kernel

#![no_std]
#![no_main]
#![feature(global_asm)]

mod cpu;
mod panic;

/// kernel main function
//
// SPDX-License-Identifier: BSD-3-Clause
//
// Copyright (c) 2020 Fernando Lugo <lugo.fernando@gmail.com>
//

pub fn kmain() -> ! {
    loop {}
}
