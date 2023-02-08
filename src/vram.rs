use std::ops::{Index, IndexMut, Range};
use std::slice::SliceIndex;
use crate::vec2::Vec2;

pub struct Vram<const W: usize, const H: usize> {
    buffer: [[u16; W]; H],
}

impl<const W: usize, const H: usize> Vram<W, H> {
    pub fn new() -> Self {
        Self {
            buffer: [[0u16; W]; H],
        }
    }

    pub fn submatrix<const X: usize, const Y: usize>(&self, offset: Vec2) -> [[u16; X]; Y] {
        let mut output = [[0u16; X]; Y];

        for x in 0..X {
            for y in 0..Y {
                output[y][x] = self.buffer[offset.y() as usize + y][offset.x() as usize + x];
            }
        }

        output
    }
}

impl<const W: usize, const H: usize> Index<(usize, usize)> for Vram<W, H> {
    type Output = u16;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        if index.0 >= H {
            panic!("Row out of range: {}/{}", index.0, H);
        }

        if index.1 >= W {
            panic!("Column out of range: {}/{}", index.1, W)
        }

        &self.buffer[index.1][index.0]
    }
}

impl<const W: usize, const H: usize> IndexMut<(usize, usize)> for Vram<W, H> {
    fn index_mut(&mut self, index: (usize, usize)) -> &mut Self::Output {
        if index.0 >= H {
            panic!("Row out of range: {}/{}", index.0, H);
        }

        if index.1 >= W {
            panic!("Column out of range: {}/{}", index.1, W)
        }

        &mut self.buffer[index.1][index.0]
    }
}
