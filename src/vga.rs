mod writer;

use core::fmt::{Arguments, Write};

use x86_64::instructions::port::Port;

use crate::{types::Vector2D, vga::writer::VGAWriter};

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
    const CELL_SIZE: usize = 2;

    pub fn new() -> Self {
        let vgascreen = Self {
            buffer_address: 0xb8000 as _,
            width: 80,
            height: 25,
            cursor_index_port: Port::new(0x3d4),
            cursor_data_port: Port::new(0x3d5),
        };
        vgascreen.reset();
        vgascreen
    }
    pub fn reset(&self) {
        self.move_cursor(Vector2D::ZERO);
        self.clear();
    }
    pub fn move_cursor(&self, position: Vector2D) {
        let position = self.clamp_position(position);
        let offset = position.y * self.width + position.x;
        let mut cursor_index_port = self.cursor_index_port.clone();
        let mut cursor_data_port = self.cursor_data_port.clone();
        unsafe {
            cursor_index_port.write(0x0f);
            cursor_data_port.write((offset & 0xff) as u8);
            cursor_index_port.write(0x0e);
            cursor_data_port.write(((offset >> 8) & 0xff) as u8);
        }
    }
    pub fn clear(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                self.write_byte(Vector2D { x, y }, b' ', Color::Black);
            }
        }
    }
    pub fn write_string(&self, position: Vector2D, string: &str, color: Color) {
        let mut writer = VGAWriter::new(self, position, color);
        let _ = writer.write_str(string);
        writer.flush_cursor();
    }
    pub fn write_fmt(&self, position: Vector2D, color: Color, args: Arguments<'_>) {
        let mut writer = VGAWriter::new(self, position, color);
        let _ = writer.write_fmt(args);
        writer.flush_cursor();
    }
    fn clamp_position(&self, position: Vector2D) -> Vector2D {
        Vector2D {
            x: position.x.min(self.width.saturating_sub(1)),
            y: position.y.min(self.height.saturating_sub(1)),
        }
    }
    fn write_byte(&self, position: Vector2D, byte: u8, color: Color) {
        let offset = (position.y * self.width + position.x) * Self::CELL_SIZE;
        unsafe {
            self.buffer_address.add(offset).write_volatile(byte);
            self.buffer_address
                .add(offset + 1)
                .write_volatile(color as u8);
        }
    }
}
