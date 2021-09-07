//
// SPDX-License-Identifier: BSD-3-Clause
//
// Copyright (c) 2020 Fernando Lugo <lugo.fernando@gmail.com>
//

extern "C" {
    static __stack_end: u8;
}

#[no_mangle]
#[naked]
#[link_section = ".loader"]
extern "C" fn _loader() -> ! {
    unsafe {
        asm!(
        "adr x4, {}",
        "mov sp, x4",
        "bl {}",
        sym __stack_end,
        sym loader,
        options(noreturn)
        );
    }
}

#[repr(C)]
struct reloc_entry {
    offset: u64,
    info: u64,
    addend: u64,
}

extern "C" {
    static __rela_dyn_start: reloc_entry;
    static __rela_dyn_end: reloc_entry;
    static mut __bss_start: u32;
    static mut __bss_end: u32;
}

const R_AARCH64_RELATIVE: u64 = 1027;

use core::mem;
use core::ptr::write_volatile;

#[link_section = ".loader"]
unsafe fn loader() -> ! {
    // runtime relocation

    // calculate loading offset (assume linker start address is 0x0)
    let _offset: u64;
    asm!(
        "adrp {0}, {1}",
        "add  {0}, {0}, #:lo12:{1}",
        out(reg) _offset,
        sym _loader
    );

    // lets check if we have data in .rela_dyn, fi so then we fixup addresses in .got
    let rela_start: usize;
    let rela_end: usize;
    asm!(
        "adrp {0}, {2}",
        "add  {0}, {0}, #:lo12:{2}",
        "adrp {1}, {3}",
        "add  {1}, {1}, #:lo12:{3}",
        out(reg) rela_start,
        out(reg) rela_end,
        sym __rela_dyn_start,
        sym __rela_dyn_end,
    );

    let size = (rela_end - rela_start) / mem::size_of::<reloc_entry>();
    let relocs = core::slice::from_raw_parts(rela_start as *const reloc_entry, size);

    for r in relocs {
        let &reloc_entry {
            offset,
            info,
            addend,
        } = r;

        if info == R_AARCH64_RELATIVE {
            let addr = (offset + _offset) as *mut u64;
            let val = addend + _offset;
            write_volatile(addr, val);
        }
    }

    start();
}

fn start() -> ! {
    use crate::rrt;
    rrt::init();
}
