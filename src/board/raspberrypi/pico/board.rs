/// code for debugging
pub mod debug;

// pico bootloader
global_asm!(include_str!("bs2_default_padded_checksummed.S"));

#[allow(dead_code)]

const RESETS_BASE: u32 = 0x4000_c000;
const RESET: u32 = RESETS_BASE + 0x00;
const RESET_DONE: u32 = RESETS_BASE + 0x08;

/// do early init
pub fn early_init() {
    // TODO: init bootrom funtions
}

fn write(addr: u32, val: u32) {
    unsafe {
        core::ptr::write_volatile(addr as *mut u32, val);
    }
}

fn write_clr(addr: u32, val: u32) {
    unsafe {
        core::ptr::write_volatile((addr + 0x3000) as *mut u32, val);
    }
}

fn write_set(addr: u32, val: u32) {
    unsafe {
        core::ptr::write_volatile((addr + 0x2000) as *mut u32, val);
    }
}

fn read(addr: u32) -> u32 {
    unsafe { core::ptr::read_volatile(addr as *const u32) }
}

/// do init
pub fn init() {
    // TODO: move it to a SoC module
    write_set(RESET, !0x3240);
    write_clr(RESET, 0x01ffffff & !0x01c38001);
    read(0x4005_4008);

    while (!read(RESET_DONE) & (0x01ffffff & !0x01c38001)) != 0 {}

    // WDT
    write(0x4005802c, (1 << 9) | 12);
    write(0x40008078, 0);

    // XOSC
    write(0x40024000, 0xd1eaa0);
    write(0x4002400c, 0x2f);
    write(0x40024000, 0xfab << 12);

    while (read(0x40024004) & (1u32 << 31)) == 0 {}

    write(0x40008030, 0x2);

    for _ in 0..10000 {
        unsafe {
            asm!("nop", "yield",);
        }
    }

    write(0x4000803c, 0);

    write(0x40008080, 12000);
    write(0x40008090, 10);
    write(0x40008094, 9);

    while (read(0x40008098) & (1 << 4)) == 0 {}

    // SYS PLL
    write_set(0x4000c000, 1 << 12);
    write_clr(0x4000c000, 1 << 12);
    while (read(0x4000c008) & (1 << 12)) == 0 {}

    write(0x40028004, 0xffffffff);
    write(0x40028008, 125);
    write_clr(0x40028004, (1 << 0) | (1 << 5));
    while (read(0x40028000) & (1u32 << 31)) == 0 {}
    write(0x4002800c, (6 << 16) | (2 << 12));
    write_clr(0x40028004, 1 << 3);

    // sys clk to pll
    write(0x4000803c, 1);

    unsafe {
        asm!("isb");
    }

    for _ in 0..100 {
        unsafe { asm!("nop", "yield",) }
    }

    // peri clk
    write(0x40008048, 1 << 11);

    write_clr(0x4000c000, (1 << 22) | (1 << 8) | (1 << 5));
    while (read(0x4000c008) & ((1 << 22) | (1 << 8) | (1 << 5))) == 0 {}
    // gpio pads

    // gpio tx/rx func
    write(0x40014004, 2);
    write(0x4001400c, 2);

    // initialize uart
    const UART0_BASE: u32 = 0x4003_4000;
    const UART_IBRD: u32 = UART0_BASE + 0x24;
    const UART_FBRD: u32 = UART0_BASE + 0x28;
    const UART_LCR_H: u32 = UART0_BASE + 0x2c;
    const UART_CR: u32 = UART0_BASE + 0x30;
    // set baudrate
    write(UART_IBRD, 0x21);
    write(UART_FBRD, 0x3a);
    // enable fifo
    write(UART_LCR_H, 0x70);

    // enable uart
    write(UART_CR, 0x301);
}
