use crate::{
    types::Vector2D,
    vga::{Color, VGAScreen},
};

pub struct Kernel {}
impl Kernel {
    pub fn new(vgascreen: VGAScreen) -> Result<Self, ()> {
        return Err(());
        vgascreen.write_string(Vector2D::ZERO, "Hello, world!", Color::White);
        Ok(Self {})
    }
    pub fn tick(&mut self) -> Result<(), ()> {
        Ok(())
    }
}
