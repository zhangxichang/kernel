use core::fmt::{Error, Result, Write};

use crate::{
    types::Vector2D,
    vga::{Color, VGAScreen},
};

pub struct VGAWriter<'a> {
    screen: &'a VGAScreen,
    position: Vector2D,
    color: Color,
}
impl<'a> VGAWriter<'a> {
    pub fn new(screen: &'a VGAScreen, position: Vector2D, color: Color) -> Self {
        Self {
            screen,
            position: screen.clamp_position(position),
            color,
        }
    }
    fn newline(&mut self) {
        self.position.x = 0;
        self.position.y += 1;
    }
    fn advance(&mut self) {
        self.position.x += 1;
        if self.position.x >= self.screen.width {
            self.newline();
        }
    }
    fn write_visible_byte(&mut self, byte: u8) -> Result {
        if self.position.y >= self.screen.height {
            return Err(Error);
        }
        self.screen.write_byte(self.position, byte, self.color);
        self.advance();
        Ok(())
    }
    pub fn flush_cursor(&self) {
        self.screen.move_cursor(self.cursor_position());
    }
    fn cursor_position(&self) -> Vector2D {
        if self.position.y >= self.screen.height {
            Vector2D {
                x: self.screen.width.saturating_sub(1),
                y: self.screen.height.saturating_sub(1),
            }
        } else {
            self.position
        }
    }
}
impl Write for VGAWriter<'_> {
    fn write_str(&mut self, string: &str) -> Result {
        for ch in string.chars() {
            match ch {
                '\n' => {
                    self.newline();
                    if self.position.y >= self.screen.height {
                        return Err(Error);
                    }
                }
                '\r' => self.position.x = 0,
                '\t' => {
                    let tab_width = 4;
                    let spaces = tab_width - (self.position.x % tab_width);
                    for _ in 0..spaces {
                        self.write_visible_byte(b' ')?;
                    }
                }
                _ => {
                    let byte = if ch.is_ascii_graphic() || ch == ' ' {
                        ch as u8
                    } else {
                        b'?'
                    };
                    self.write_visible_byte(byte)?;
                }
            }
        }
        Ok(())
    }
}
