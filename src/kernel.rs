use x86_64::instructions::port::Port;

pub struct Kernel {}
impl Kernel {
    pub fn new() -> Self {
        let mut index_port = Port::<u8>::new(0x3D4);
        let mut data_port = Port::<u8>::new(0x3D5);
        unsafe {
            index_port.write(0x0E);
            data_port.write(0x00);
            index_port.write(0x0F);
            data_port.write(0x00);
        }
        let vga_ptr = 0xb8000 as *mut u8;
        for i in 0..2000 {
            unsafe {
                vga_ptr.add(i * 2).write_volatile(0x20);
                vga_ptr.add(i * 2 + 1).write_volatile(0x07);
            }
        }
        for (i, byte) in b"Hello World!".iter().enumerate() {
            unsafe {
                vga_ptr.add(i * 2).write(*byte);
            }
        }
        Self {}
    }
    pub fn tick(&mut self) {}
}
