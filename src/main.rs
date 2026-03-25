#![no_std]
#![no_main]

use core::panic::PanicInfo;

#[repr(C, align(8))]
struct Multiboot2Header {
    magic: u32,
    arch: u32,
    length: u32,
    checksum: u32,
}

#[unsafe(link_section = ".multiboot2")]
#[used]
static MULTIBOOT2: Multiboot2Header = Multiboot2Header {
    magic: 0xe85250d6,
    arch: 0,
    length: 16,
    checksum: (0u32
        .wrapping_sub(0xe85250d6)
        .wrapping_sub(0)
        .wrapping_sub(16)),
};

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}

const HELLO: &[u8] = b"Hello World!";
