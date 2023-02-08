use std::cmp::min;
use crate::color::Color;
use crate::vec2::Vec2;
use crate::vram::Vram;

pub struct FontGlyph {
    bitmap: [u16; 11],
    width: u8,
    height: u8,
    offset_y: u8,
}

impl FontGlyph {
    pub fn grapheme_to_index(grapheme: char) -> Option<usize> {
        todo!()
    }
}

pub enum Primitive {
    Particles {
        positions: Vec<Vec2>,
        color: Color,
    },
    Rect {
        position: Vec2,
        dimensions: Vec2,
        color: Color,
    },
    RectBorder {
        position: Vec2,
        dimensions: Vec2,
        thickness: u8,
        color: Color,
    },
    HorizontalLine {
        position: Vec2,
        length: u16,
        color: Color,
    },
    VerticalLine {
        position: Vec2,
        length: u16,
        color: Color,
    },
    Line {
        start: Vec2,
        end: Vec2,
        color: Color,
    },
    Grapheme {
        position: Vec2,
        grapheme: char,
        font: &'static [FontGlyph],
        scale: u8,
        color: Color,
    },
    Text {
        position: Vec2,
        text: String,
        font: &'static [FontGlyph],
        scale: u8,
        color: Color,
    },
    Bitmap {
        position: Vec2,
        dimensions: Vec2,
        bitmap: &'static [u8],
        color: Color,
    },
    Circle {
        center: Vec2,
        radius: u16,
        color: Color,
    },
    CircleBorder {
        center: Vec2,
        radius: u16,
        thickness: u8,
        color: Color,
    },
}

impl Primitive {
    pub fn particles(positions: Vec<Vec2>, color: Color) -> Self {
        Primitive::Particles { positions, color }
    }

    pub fn rect(position: Vec2, dimensions: Vec2, color: Color) -> Self {
        Primitive::Rect { position, dimensions, color }
    }

    pub fn rect_border(position: Vec2, dimensions: Vec2, thickness: u8, color: Color) -> Self {
        Primitive::RectBorder { position, dimensions, thickness, color }
    }

    pub fn horizontal_line(position: Vec2, length: u16, color: Color) -> Self {
        Primitive::HorizontalLine { position, length, color }
    }

    pub fn vertical_line(position: Vec2, length: u16, color: Color) -> Self {
        Primitive::VerticalLine { position, length, color }
    }

    pub fn line(start: Vec2, end: Vec2, color: Color) -> Self {
        Primitive::Line { start, end, color }
    }

    pub fn grapheme(position: Vec2, grapheme: char, font: &'static [FontGlyph], scale: u8, color: Color) -> Self {
        Primitive::Grapheme { position, grapheme, font, scale, color }
    }

    pub fn text(position: Vec2, text: &str, scale: u8, font: &'static [FontGlyph], color: Color) -> Self {
        Primitive::Text { position, text: text.to_string(), font, scale, color }
    }

    pub fn bitmap(position: Vec2, dimensions: Vec2, bitmap: &'static [u8], color: Color) -> Self {
        Primitive::Bitmap { position, dimensions, bitmap, color }
    }

    pub fn circle(center: Vec2, radius: u16, color: Color) -> Self {
        Primitive::Circle { center, radius, color }
    }

    pub fn circle_border(center: Vec2, radius: u16, thickness: u8, color: Color) -> Self {
        Primitive::CircleBorder { center, radius, thickness, color }
    }

    pub fn draw<const W: usize, const H: usize>(&self, vram: &mut Vram<W, H>) {
        match self {
            Primitive::Particles { positions, color } => {
                let color = color.raw();
                for position in positions {
                    let pos = position.clamp_display::<W, H>();

                    vram[pos.into()] = color;
                }
            }
            Primitive::Rect { position, dimensions, color } => {
                let color = color.raw();

                let position = position.clamp_display::<W, H>();
                let end = (position + *dimensions).clamp_display::<W, H>();

                for x in position.x()..end.x() {
                    for y in position.y()..end.y() {
                        vram[(x as usize, y as usize)] = color;
                    }
                }
            }
            Primitive::RectBorder { position, dimensions, thickness, color } => {
                let color = color.raw();

                todo!()
            }
            Primitive::HorizontalLine { position, length, color } => {
                let color = color.raw();

                let end = *position + Vec2::new(*length as i16, 0);
                let end = end.clamp_display::<W, H>();

                for x in position.x()..end.x() {
                    vram[(x as usize, position.y() as usize)] = color;
                }
            }
            Primitive::VerticalLine { position, length, color } => {
                let color = color.raw();

                let end = *position + Vec2::new(0, *length as i16);
                let end = end.clamp_display::<W, H>();

                for y in position.y()..end.y() {
                    vram[(position.x() as usize, y as usize)] = color;
                }
            }
            Primitive::Line { .. } => {
                todo!()
            }
            Primitive::Grapheme { position, grapheme, font, scale, color } => {
                let color = color.raw();

                if let Some(idx) = FontGlyph::grapheme_to_index(*grapheme) {
                    let font_glyph = &font[idx];

                    for y in 0..font_glyph.height {
                        let mut mask = 0x80_00_u16;
                        for x in 0..font_glyph.width {
                            if (font_glyph.bitmap[y as usize] & mask) != 0 {
                                let pos = (*position + Vec2::new(
                                    (font_glyph.offset_y + y) as i16,
                                    x as i16,
                                )).clamp_display::<W, H>();
                                vram[pos.into()] = color;
                            }

                            mask = mask >> 1;
                        }
                    }
                }
            }
            Primitive::Text { position, text, font, scale, color } => {
                let mut offset = Vec2::origin();

                for grapheme in text.chars() {
                    let pos = (*position + offset).clamp_display::<W, H>();

                    Primitive::grapheme(
                        pos,
                        grapheme,
                        font,
                        *scale,
                        *color,
                    ).draw(vram);

                    offset += Vec2::new(if grapheme == ' ' {
                        3
                    } else {
                        if let Some(index) = FontGlyph::grapheme_to_index(grapheme) {
                            font[index].width as i16
                        } else {
                            0
                        }
                    }, 0);
                }
            }
            Primitive::Bitmap { position, dimensions, bitmap, color } => {
                todo!()
            }
            Primitive::Circle { .. } => {
                todo!()
            }
            Primitive::CircleBorder { .. } => {
                todo!()
            }
        }
    }
}
