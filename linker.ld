ENTRY(_start32)

SECTIONS {
    . = 0x100000;
    .multiboot2 ALIGN(8) : {
        *(.multiboot2)
    }
    .note.Xen : {
        *(.note.Xen)
    }
    .text : {
        *(.text .text.*)
    }
    .rodata : {
        *(.rodata .rodata.*)
    }
    .data : {
        *(.data .data.*)
    }
    .bss : {
        *(.bss .bss.*)
    }
    .eh_frame_hdr : {
        *(.eh_frame_hdr)
    }
    .eh_frame : {
        *(.eh_frame)
    }
    .symtab : {
        *(.symtab)
    }
    .shstrtab : {
        *(.shstrtab)
    }
    .strtab : {
        *(.strtab)
    }
    .got : {
        *(.got)
    }
    /DISCARD/ : {
        *(.comment .comment.*)
    }
}
