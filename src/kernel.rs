use core::fmt::Write;

use glam::UVec2;

use crate::vga::{VGA, VGAColor};

pub struct Kernel {}
impl Kernel {
    pub fn new(vga: VGA) -> Self {
        let mut writer = vga.writer(UVec2::ZERO, VGAColor::White);
        writer
            .write_str("Hello, world!")
            .expect("failed to write text");
        Self {}
    }
    pub fn tick(&mut self) {}
}
