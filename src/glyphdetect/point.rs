use std::ops::{Sub, Add};
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Point{pub x: i32, pub y: i32}

impl Point {
    pub fn zero() -> Self {
        return Point{x: 0, y: 0}
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
impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct BoundingBox {
    pub u_left: Point,
    pub l_right: Point,
}
impl Sub for BoundingBox {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            u_left: self.u_left - other.u_left,
            l_right: self.l_right - other.l_right,
        }
    }
}

/// Starts from the top left and goes down to the bottom right
/// Returns (x,y)
pub fn get_pos(index: i32, width: i32) -> Point {
    let x = index % width; 
    let y = index / width;
    return Point{x: x as i32, y: y as i32};
}

/// Opposite of get_pos
/// Takes in an x and y and returns the corresponding index
pub fn get_index(point: Point, width: i32) -> usize {
    return (point.y * width + point.x) as usize;
}


