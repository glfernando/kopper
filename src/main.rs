/*
 * SPDX-License-Identifier: BSD-3-Clause
 *
 * Copyright (c) 2020 Fernando Lugo <lugo.fernando@gmail.com>
 */

#![no_std]
#![no_main]
#![feature(global_asm)]

mod panic;
mod cpu;

pub fn kmain() -> ! {
    loop{}
}
