use std::ops::{Add, AddAssign};
use std::hash::Hash;
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
impl Hash for Vector2 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let x_bytes = self.x.to_ne_bytes();
        let y_bytes = self.y.to_ne_bytes();
        state.write(&x_bytes);
        state.write(&y_bytes);
    }
}
impl Eq for Vector2 {}//RIP float equality
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
