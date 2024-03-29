#![allow(dead_code)]
use std::ops::{Sub, Add, AddAssign, Mul};
/// Just a basic point representation
/// i32, i32
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point{pub x: i32, pub y: i32}

impl Point {
    pub fn zero() -> Self {
        Point{x: 0, y: 0}
    }

    pub fn from(x: i32, y: i32) -> Self {
        Point{x, y}
    }
}
impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self{
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        *self = Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl Mul for Point {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Mul<i32> for Point {
    type Output = Self;
    fn mul(self, other: i32) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

pub const SURROUNDING: [Point; 8] = [
    Point{x: 1, y: 0},
    Point{x:0, y: -1},
    Point{x: -1, y: 0},
    Point{x: 0, y: 1},
    Point{x: 1, y: 1},
    Point{x: 1, y: -1},
    Point{x: -1, y: -1},
    Point{x: -1, y: 1},
];
