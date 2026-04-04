#![no_std]
#![no_main]

mod kernel;
mod types;
mod vga;

use core::{arch::global_asm, panic::PanicInfo};

use crate::{
    kernel::Kernel,
    types::Vector2D,
    vga::{Color, VGAScreen},
};

global_asm!(include_str!("asm/start.s"));

const MAGIC: u32 = 0xe85250d6;
const ARCH: u32 = 0;
const HEADER_LEN: u32 = 24;

#[repr(C, align(8))]
struct Multiboot2Header {
    magic: u32,
    arch: u32,
    length: u32,
    checksum: u32,
    t_type: u16,
    t_flags: u16,
    t_size: u32,
}

#[repr(C)]
struct PVHNote {
    n_name_len: u32,
    n_desc_len: u32,
    n_type: u32,
    name: [u8; 4],
    entry: u32,
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
    t_type: 0,
    t_flags: 0,
    t_size: 8,
};

#[used]
#[unsafe(link_section = ".note.Xen")]
static PVH_NOTE: PVHNote = PVHNote {
    n_name_len: 4,
    n_desc_len: 4,
    n_type: 0x12,
    name: *b"Xen\0",
    entry: 0x100000,
};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let vgascreen = VGAScreen::new();
    vgascreen.reset();
    vgascreen.write_fmt(Vector2D::ZERO, Color::Red, format_args!("{info}"));
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn main() -> ! {
    let mut kernel = Kernel::new(VGAScreen::new()).expect("Kernel init error");
    loop {
        kernel.tick().expect("Kernel tick error")
    }
}
