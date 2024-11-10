use crate::display::structs::ColorCode;

pub mod structs;

pub struct Display {
    pub writer: Writer,
}

pub struct Char {
    pub character: u8,
    pub color: u8,
}

pub struct Writer {
    color: u8,
    cursor: usize,
    vga_buffer: *mut u8,
}

impl Writer {
    pub fn new(color: u8, vga_buffer: *mut u8) -> Writer {
        Writer {
            color,
            cursor: 0,
            vga_buffer,
        }
    }

    pub fn print_byte(&mut self, char: Char) {
        unsafe {
            *(self.vga_buffer.offset(self.cursor as isize * 2) as *mut (u8, u8)) = (char.character, char.color);
        }
        self.cursor += 1;
    }

    pub fn print_string(&mut self, str: &str) {
        str.bytes().for_each(|char: u8| {
            self.print_byte(Char {
                character: char,
                color: self.color,
            })
        })
    }
}

impl Display {
    pub fn new(vga_buffer: *mut u8, color: ColorCode) -> Self {
        Self {
            writer: Writer {
                color: color.get_color(),
                cursor: 0,
                vga_buffer,
            },
        }
    }
}