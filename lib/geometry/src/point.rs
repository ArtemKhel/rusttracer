use crate::unit_vec::UnitVec;
use std::ops::{Add, Sub};

use crate::vec::Vec3;

#[derive(Default, Debug, Clone, Copy)]
pub struct Point {
    pub pos: Vec3,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point {
            pos: Vec3::new(x, y, z),
        }
    }
    pub fn vector_to(self, target: Point) -> UnitVec {
        (target - self).to_unit()
    }
}

impl Add<Vec3> for Point {
    type Output = Point;

    fn add(self, rhs: Vec3) -> Self::Output {
        Point { pos: self.pos + rhs }
    }
}

impl Sub for Point {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        self.pos - rhs.pos
    }
}
impl Sub<Vec3> for Point {
    type Output = Point;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Point { pos: self.pos - rhs }
    }
}
