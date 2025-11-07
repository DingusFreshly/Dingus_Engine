use std::ops::{Add, AddAssign};
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector2 {
    pub x: f64,
    pub y: f64,
}
impl Vector2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vector2 { x, y }
    }
    pub fn to_tuple(&self) -> (f64, f64) {
        (self.x, self.y)
    }
}
impl Add for Vector2 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Vector2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl AddAssign for Vector2 {
    fn add_assign(&mut self, other: Self) {
        self.x += other.x;
        self.y += other.y;
    }
}
