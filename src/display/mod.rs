pub struct Display {
    vga_buffer: *mut u8,
}

impl Display {
    pub fn print(&self, text: &[u8]) {
        for (i, &byte) in text.iter().enumerate() {
            unsafe {
                *self.vga_buffer.offset(i as isize * 2) = byte;
                *self.vga_buffer.offset(i as isize * 2 + 1) = 0x0F;
            }
        }
    }

    pub fn new(vga_buffer: *mut u8) -> Self {
        Self { vga_buffer }
    }
}