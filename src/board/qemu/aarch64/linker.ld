
ENTRY(_loader)

SECTIONS
{
    . = 0x0;

    .text : {
        KEEP(*(.loader))
        *(.text .text.*)
    }

    .rodata ALIGN(16) : {
        *(.rodata .rodata.*)
    }

    .shell_cmds ALIGN(8) : {
        __shell_cmds_start = .;
        KEEP(*(SORT(.shell_cmds.*)))
        KEEP(*(.shell_cmds))
        __shell_cmds_end = .;
    }

    .data ALIGN(16): {
        *(.data .data.*)
    }

    .rela.dyn ALIGN(8): {
        __rela_dyn_start = .;
        *(.rela_dyn)
    }
    __rela_dyn_end = .;

    .bss ALIGN(4096) : {
        __bss_start = .;
        *(.bss .bss.*);
        . = ALIGN(8);
        __bss_end = .;
    }

    . = ALIGN(16);
    __stack_start = .;
    . = . + 0x10000; /* TODO: make it configurable */
    __stack_end = .;

    . = ALIGN(16);
    __heap_start = .;
    . = . + 0x100000; /* TODO: make it configurable */
    __heap_end = .;

    /DISCARD/ :
    {
        *(.eh_frame*)
        *(.comment)
        *(.note*)
	*(.dynsym)
	*(.dynstr*)
	*(.gnu*)
	*(.hash*)
	*(.dynamic)
	*(.plt*)
    }
}
