#![no_std]
#![no_main]

use core::panic::PanicInfo;

const MAGIC: u32 = 0xe85250d6;
const ARCH: u32 = 0;
const HEADER_LEN: u32 = 24;

#[repr(C, align(8))]
struct Multiboot2Header {
    magic: u32,
    arch: u32,
    length: u32,
    checksum: u32,
    tag_type: u16,
    tag_flags: u16,
    tag_size: u32,
}

#[used]
#[unsafe(link_section = ".multiboot2")]
static MULTIBOOT2: Multiboot2Header = Multiboot2Header {
    magic: MAGIC,
    arch: ARCH,
    length: HEADER_LEN,
    checksum: MAGIC
        .wrapping_add(ARCH)
        .wrapping_add(HEADER_LEN)
        .wrapping_neg(),
    tag_type: 0,
    tag_flags: 0,
    tag_size: 8,
};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_ptr = 0xb8000 as *mut u8;
    for (i, byte) in b"Hello World!".into_iter().enumerate() {
        let offset = i as isize * 2;
        unsafe {
            *vga_ptr.offset(offset) = *byte;
            *vga_ptr.offset(offset + 1) = 0xb;
        }
    }
    loop {}
}
