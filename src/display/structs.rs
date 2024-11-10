#[derive(Copy, Clone)]
pub enum Colors {
    BLACK = 0x0,
    BLUE = 0x1,
    GREEN = 0x2,
    CYAN = 0x3,
    RED = 0x4,
    MAGENTA = 0x5,
    BROWN = 0x6,
    LIGHT_GRAY = 0x7,
    DARK_GRAY = 0x8,
    LIGHT_BLUE = 0x9,
    LIGHT_GREEN = 0xA,
    LIGHT_CYAN = 0xB,
    LIGHT_RED = 0xC,
    PINK = 0xD,
    YELLOW = 0xE,
    WHITE = 0xF,
}

#[derive(Copy, Clone)]
pub struct ColorCode {
    fg: Colors,
    bg: Colors,
}

impl ColorCode {
    pub(crate) fn get_color(&self) -> u8 {
        ((self.bg as u8) & 0x0F) << 4 | (self.fg as u8)
    }

    pub fn new(fg: Colors, bg: Colors) -> Self {
        Self { fg, bg }
    }
}
