use x86_64::instructions::port::Port;

use crate::types::Vector2D;

#[derive(Clone, Copy)]
pub enum Color {
    Black = 0x0,
    Red = 0x4,
    White = 0xf,
}

#[derive(Clone)]
pub struct VGAScreen {
    buffer_address: *mut u8,
    width: usize,
    height: usize,
    cursor_index_port: Port<u8>,
    cursor_data_port: Port<u8>,
}
impl VGAScreen {
    pub fn new() -> Self {
        let mut vgascreen = Self {
            buffer_address: 0xb8000 as _,
            width: 80,
            height: 25,
            cursor_index_port: Port::new(0x3d4),
            cursor_data_port: Port::new(0x3d5),
        };
        vgascreen.reset();
        vgascreen
    }
    pub fn move_cursor(&mut self, position: Vector2D) {
        let offset = position.y * self.width + position.x;
        unsafe {
            self.cursor_index_port.write(0x0f);
            self.cursor_data_port.write((offset & 0xff) as _);
            self.cursor_index_port.write(0x0e);
            self.cursor_data_port.write(((offset >> 8) & 0xff) as _);
        }
    }
    pub fn clear(&self) {
        for i in 0..self.width * self.height {
            unsafe {
                self.buffer_address.add(i * 2).write_volatile(0x20);
                self.buffer_address
                    .add(i * 2 + 1)
                    .write_volatile(Color::Black as _);
            }
        }
    }
    pub fn reset(&mut self) {
        self.move_cursor(Vector2D::ZERO);
        self.clear();
    }
    pub fn write_string(&self, position: Vector2D, string: &str, color: Color) {
        let mut offset = (position.y * self.width + position.x) * 2;
        for byte in string.bytes() {
            unsafe {
                self.buffer_address.add(offset).write_volatile(byte);
                self.buffer_address
                    .add(offset + 1)
                    .write_volatile(color as _);
                offset += 2;
            }
        }
    }
}
