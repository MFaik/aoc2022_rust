use std::ops::Add;
use std::ops::AddAssign;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct Vector2 {
    pub x: i32,
    pub y: i32
}

impl Add for Vector2 {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Vector2 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y
        }
    }
}

impl AddAssign for Vector2 {
    fn add_assign(&mut self, _rhs: Self) {
        self.x += _rhs.x;
        self.y += _rhs.y;
    }
}
