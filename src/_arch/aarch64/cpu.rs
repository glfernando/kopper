//
// SPDX-License-Identifier: BSD-3-Clause
//
// Copyright (c) 2020 Fernando Lugo <lugo.fernando@gmail.com>
//

global_asm!(include_str!("loader.S"));
global_asm!(include_str!("start.S"));

#[no_mangle]
pub extern "C" fn cpu_start() -> ! {
    // call extern kmain function
    extern "Rust" {
        pub fn main() -> !;
    }
    unsafe {
        main();
    }
}
