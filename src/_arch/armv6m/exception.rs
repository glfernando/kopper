extern "C" {
    static __stack_end: u8;
}

use crate::cpu::arch_cpu::_start;

#[repr(C)]
#[repr(align(256))]
struct VectorTable {
    sp_main: *const u8,
    reset: fn() -> !,
    //nmi: fn(),
}

#[link_section = ".vector_table"]
#[used]
static mut VECTOR_TABLE: VectorTable = VectorTable {
    sp_main: unsafe { &__stack_end as *const u8 },
    reset: _start,
};
