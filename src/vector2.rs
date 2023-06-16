use std::ops::Add;
use std::ops::AddAssign;

#[derive(Debug, Eq, PartialEq, Copy, Clone, Ord, PartialOrd)]
pub struct Vector2<T: Add + AddAssign> {
    pub x: T,
    pub y: T
}

impl<T: Add<Output = T> + AddAssign> Add for Vector2<T> {
    type Output = Self;

    fn add(self, _rhs: Self) -> Self {
        Vector2 {
            x: self.x + _rhs.x,
            y: self.y + _rhs.y
        }
    }
}

impl<T: Add + AddAssign> AddAssign for Vector2<T> {
    fn add_assign(&mut self, _rhs: Self) {
        self.x += _rhs.x;
        self.y += _rhs.y;
    }
}
