mod lcd_driver;
mod hardware;
mod color;
mod primitive;
mod vec2;
mod vram;

#[cfg(test)]
mod tests {
    use crate::color::Color;
    use crate::primitive::Primitive;
    use crate::vec2::Vec2;
    use crate::vram::Vram;
    use super::*;

    #[test]
    fn draw_rect() {
        let mut vram = Vram::<160, 128>::new();
        let rect = Primitive::rect(
            Vec2::new(20, 20),
            Vec2::new(5, 5),
            Color::RGB24(0, 255, 0),
        );

        rect.draw(&mut vram);

        let reference = [[0x1F_u16; 5]; 5];

        assert_eq!(reference, vram.submatrix::<5, 5>(Vec2::new(20, 20)));
    }
}
