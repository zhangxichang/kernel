ENTRY(_start)

SECTIONS {
    . = 0x100000;
    .start : {
        *(.start)
    }
    .multiboot2 : {
        *(.multiboot2)
    }
}
