use core::fmt::Write;

use glam::UVec2;

use crate::vga::{VGA, VGAChar, VGAColor};

pub struct VGAWriter<'a> {
    vga: &'a VGA,
    position: UVec2,
    color: VGAColor,
}
impl<'a> VGAWriter<'a> {
    pub fn new(vga: &'a VGA, position: UVec2, color: VGAColor) -> Self {
        Self {
            vga,
            position,
            color,
        }
    }
    fn next_line(&mut self) {
        self.position.x = 0;
        self.position.y += 1;
    }
    fn advance(&mut self) {
        self.position.x += 1;
        if self.position.x >= self.vga.size.x {
            self.next_line();
        }
    }
    fn write_ascii(&mut self, ascii: u8) {
        self.vga
            .write_char(self.position, VGAChar::new(ascii, self.color));
        self.advance();
    }
}
impl Write for VGAWriter<'_> {
    fn write_str(&mut self, string: &str) -> core::fmt::Result {
        for byte in string.bytes() {
            match byte {
                b'\n' => self.next_line(),
                b'\r' => self.position.x = 0,
                b'\t' => {
                    for _ in 0..4 {
                        self.write_ascii(b' ');
                    }
                }
                0x20..=0x7e => self.write_ascii(byte),
                _ => self.write_ascii(b'?'),
            }
        }
        Ok(())
    }
}
