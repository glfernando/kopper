//
// SPDX-License-Identifier: BSD-3-Clause
//
// Copyright (c) 2021 Fernando Lugo <lugo.fernando@gmail.com>
//

pub mod exception;

extern "C" {
    static mut __data_start: u32;
    static mut __data_end: u32;
    static mut __fdata: u32;
}

use core::arch::asm;

#[no_mangle]
#[naked]
#[link_section = ".reset"]
extern "C" fn _reset() -> ! {
    unsafe {
        asm!(
        "b {}",
        "nop",
        sym _start,
        options(noreturn)
        );
    }
}

#[no_mangle]
#[naked]
#[link_section = ".reset"]
extern "C" fn _board_entry() -> ! {
    unsafe {
        asm!(
            "movs    r0, #0",
            "ldr     r1, =(0xe0000000 + 0x0000ed00)",
            "str     r0, [r1, #8]",
            "ldmia   r0!, {{r1, r2}}",
            "bx      r2",
            options(noreturn)
        );
    }
}

#[link_section = ".reset"]
fn _start() -> ! {
    // TODO: support relocation

    // move data from flash to RAM
    unsafe {
        let data_start = &mut __data_start as *mut _;
        let data_end = &mut __data_end as *mut u32;
        let size = data_end.offset_from(data_start) as usize;
        let fdata_start = &mut __fdata as *mut _;

        core::ptr::copy_nonoverlapping(fdata_start, data_start, size);
    }

    use crate::rrt;
    rrt::init();
}

#[no_mangle]
fn __aeabi_unwind_cpp_pr0() {}
