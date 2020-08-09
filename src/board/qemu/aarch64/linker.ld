
SECTIONS
{
    . = 0x40080000;

    .text : {
        KEEP(*(.loader))
        KEEP(*(.text*))
    }

    .rodata ALIGN(4096) : {
        *(.rodata*)
    }

    .data ALIGN(4096): {
        *(.data)
    }

    .rela.dyn ALIGN(8): {
        __rela_dyn_start = .;
        *(.rela_dyn)
    }
    __rela_dyn_end = .;

    .bss ALIGN(4096) : {
        __bss_start = .;
        *(.bss*);
        . = ALIGN(8);
        __bss_end = .;
    }

    . = ALIGN(16);
    __stack_start = .;
    . = . + 0x10000; /* TODO: make it configurable */
    __stack_end = .;

    /DISCARD/ :
    {
        *(.eh_frame*)
        *(.comment)
        *(.note*)
	*(.dynsym)
	*(.dynstr*)
	*(.gnu*)
	*(.hash*)
    }
}
