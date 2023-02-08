use std::cmp::{max, min, Ordering};
use std::ops::{Add, AddAssign, Mul, Neg};

#[derive(Copy, Clone)]
pub struct Vec2 {
    x: i16,
    y: i16,
}

impl Vec2 {
    pub fn new(x: i16, y: i16) -> Self {
        Vec2 { x, y }
    }

    pub fn origin() -> Self {
        Vec2 { x: 0, y: 0 }
    }

    pub fn up() -> Self {
        Vec2 { x: 0, y: -1 }
    }

    pub fn right() -> Self {
        Vec2 { x: 1, y: 0 }
    }

    pub fn x(&self) -> i16 {
        self.x
    }

    pub fn y(&self) -> i16 {
        self.y
    }

    pub fn dot(&self, rhs: Vec2) -> i16 {
        ((self.x as i32) * (rhs.x as i32) + (self.y as i32) * (rhs.y as i32)) as i16
    }

    pub fn maginitude(&self) -> f32 {
        let x = self.x as f32;
        let y = self.y as f32;

        f32::sqrt(x * x + y * y)
    }

    pub fn clamp(&self, bottom: Vec2, top: Vec2) -> Vec2 {
        Vec2 {
            x: max(min(self.x, top.x), bottom.x),
            y: max(min(self.y, top.y), bottom.y),
        }
    }

    pub fn clamp_display<const W: usize, const H: usize>(&self) -> Vec2 {
        Vec2 {
            x: self.x.clamp(0, W as i16),
            y: self.y.clamp(0, H as i16),
        }
    }
}

impl Into<(usize, usize)> for Vec2 {
    fn into(self) -> (usize, usize) {
        (self.x as usize, self.y as usize)
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, rhs: Self) -> Self::Output {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

impl AddAssign for Vec2 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Neg for Vec2 {
    type Output = Vec2;

    fn neg(self) -> Self::Output {
        Vec2 { x: -self.x, y: -self.y }
    }
}

impl Mul<i16> for Vec2 {
    type Output = Vec2;

    fn mul(self, rhs: i16) -> Self::Output {
        Vec2 { x: self.x * rhs, y: self.y * rhs }
    }
}
