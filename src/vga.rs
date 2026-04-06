mod writer;

use glam::UVec2;

use crate::vga::writer::VGAWriter;

#[repr(u8)]
#[derive(Clone, Copy)]
pub enum VGAColor {
    Black = 0x0,
    Red = 0x4,
    White = 0xf,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct VGAChar {
    ascii: u8,
    color: u8,
}
impl VGAChar {
    pub fn new(ascii: u8, color: VGAColor) -> Self {
        Self {
            ascii,
            color: color as u8,
        }
    }
}

pub struct VGA {
    buffer_address: *mut VGAChar,
    size: UVec2,
}
impl VGA {
    pub fn new() -> Self {
        let vga = Self {
            buffer_address: 0xb8000 as *mut VGAChar,
            size: UVec2::new(80, 25),
        };
        vga.clear();
        vga
    }
    fn cell_offset(&self, position: UVec2) -> Option<usize> {
        (position.x < self.size.x && position.y < self.size.y)
            .then(|| (position.y * self.size.x + position.x) as usize)
    }
    pub fn write_char(&self, position: UVec2, char: VGAChar) {
        unsafe {
            self.buffer_address
                .add(
                    self.cell_offset(position)
                        .expect("write position is out of bounds"),
                )
                .write_volatile(char);
        }
    }
    pub fn clear(&self) {
        let char = VGAChar::new(b' ', VGAColor::Black);
        for offset in 0..(self.size.x * self.size.y) as usize {
            unsafe {
                self.buffer_address.add(offset).write_volatile(char);
            }
        }
    }
    pub fn writer(&self, position: UVec2, color: VGAColor) -> VGAWriter<'_> {
        VGAWriter::new(self, position, color)
    }
}
