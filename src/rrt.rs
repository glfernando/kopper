use crate::board;

extern "C" {
    static mut __bss_start: u32;
    static mut __bss_end: u32;
}

pub fn init() -> ! {
    board::early_init();

    let bss = unsafe {
        let bss_start = &mut __bss_start as *mut _;
        let bss_end = &mut __bss_end as *mut u32;
        let size = bss_end.offset_from(bss_start) as usize;

        core::slice::from_raw_parts_mut(bss_start, size)
    };

    for x in bss {
        *x = 0;
    }

    board::init();

    // call extern kmain function
    extern "Rust" {
        pub fn main() -> !;
    }

    unsafe {
        main();
    }
}
