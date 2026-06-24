ENTRY(_start)

SECTIONS {
    . = 0xffffffff80000000;

    .text : { *(.text .text.*) }
    . = ALIGN(0x1000);

    .rodata : { *(.rodata .rodata.*) }
    . = ALIGN(0x1000);

    .requests : {
        . = ALIGN(8);
        KEEP(*(.requests_start_marker))
        . = ALIGN(8);
        KEEP(*(.requests))
        . = ALIGN(8);
        KEEP(*(.requests_end_marker))
    }
    .got : { *(.got .got.*) *(.got.plt .got.plt.*) }
    .data : { *(.data .data.*) }
    .bss : { *(COMMON) *(.bss .bss.*) }

    .shstrtab 0 : { *(.shstrtab) }

    /DISCARD/ : { *(.symtab) *(.strtab) *(.eh_frame*) *(.note*) *(.comment) *(.debug*) }
}
