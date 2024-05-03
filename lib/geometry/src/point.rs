use std::ops::{Add, Sub};

use crate::{unit_vec::UnitVec, vec::Vec3};

#[derive(Default, Debug, Clone, Copy, PartialOrd, PartialEq)]
pub struct Point {
    pub radius_vector: Vec3,
}

impl Point {
    pub fn new(x: f32, y: f32, z: f32) -> Point {
        Point {
            radius_vector: Vec3::new(x, y, z),
        }
    }

    pub fn unit_vector_to(self, target: Point) -> UnitVec { self.vector_to(target).to_unit() }

    pub fn vector_to(self, target: Point) -> Vec3 { target - self }

    pub fn distance_to(self, other: Point) -> f32 { (self - other).len() }
}

impl Sub for Point {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output { self.radius_vector - rhs.radius_vector }
}

impl Add<Vec3> for Point {
    type Output = Point;

    fn add(self, rhs: Vec3) -> Self::Output {
        Point {
            radius_vector: self.radius_vector + rhs,
        }
    }
}
impl Sub<Vec3> for Point {
    type Output = Point;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Point {
            radius_vector: self.radius_vector - rhs,
        }
    }
}
