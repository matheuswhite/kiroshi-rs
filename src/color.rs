#[derive(Copy, Clone)]
pub enum Color {
    RGB24(u8, u8, u8),
    RBG16(u16),
}

impl Color {
    pub fn rgb32(r: u8, g: u8, b: u8) -> Color {
        Color::RGB24(r, g, b)
    }

    pub fn rbg16(value: u16) -> Color {
        Color::RBG16(value)
    }

    pub fn red(&self) -> u8 {
        match self {
            Color::RGB24(r, _, _) => *r,
            Color::RBG16(value) => ((*value >> 10) as u8) & 0x1F,
        }
    }

    pub fn green(&self) -> u8 {
        match self {
            Color::RGB24(_, g, _) => *g,
            Color::RBG16(value) => (*value as u8) & 0x1F,
        }
    }

    pub fn blue(&self) -> u8 {
        match self {
            Color::RGB24(_, _, b) => *b,
            Color::RBG16(value) => ((*value >> 5) as u8) & 0x1F,
        }
    }

    pub fn to_rgb32(self) -> Color {
        match self {
            Color::RBG16(value) => Color::rgb32(
                ((value >> 10) as u8) & 0x1F,
                (value as u8) & 0x1F,
                ((value >> 5) as u8) & 0x1F,
            ),
            c => c,
        }
    }

    pub fn to_rbgu16(self) -> Color {
        match self {
            Color::RGB24(r, g, b) => Color::RBG16(
                (((r & 0x1F) as u16) << 10) |
                    (((b & 0x1F) as u16) << 5) |
                    ((g & 0x1F) as u16)
            ),
            c => c,
        }
    }

    pub fn raw(&self) -> u16 {
        match self {
            Color::RBG16(val) => *val,
            Color::RGB24(r, g, b) => (((*r & 0x1F) as u16) << 10) |
                (((*b & 0x1F) as u16) << 5) |
                ((*g & 0x1F) as u16),
        }
    }
}
