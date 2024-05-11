use std::{
    fmt::{Debug, Formatter},
    ops::{Add, Index, IndexMut, Sub},
};

use serde::{Deserialize, Serialize};

use crate::geometry::{unit_vec::UnitVec, utils::Axis, vec::Vec3, Aabb};

#[derive(Default, Debug, Clone, Copy, PartialOrd, PartialEq, Serialize, Deserialize)]
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

    pub fn min_coords(lhs: Point, rhs: Point) -> Point {
        Point::new(
            lhs.radius_vector.x.min(rhs.radius_vector.x),
            lhs.radius_vector.y.min(rhs.radius_vector.y),
            lhs.radius_vector.z.min(rhs.radius_vector.z),
        )
    }

    pub fn max_coords(lhs: Point, rhs: Point) -> Point {
        Point::new(
            lhs.radius_vector.x.max(rhs.radius_vector.x),
            lhs.radius_vector.y.max(rhs.radius_vector.y),
            lhs.radius_vector.z.max(rhs.radius_vector.z),
        )
    }
}

impl Index<Axis> for Point {
    type Output = f32;

    fn index(&self, index: Axis) -> &Self::Output { &self.radius_vector[index] }
}

impl IndexMut<Axis> for Point {
    fn index_mut(&mut self, index: Axis) -> &mut Self::Output { &mut self.radius_vector[index] }
}

impl Sub for Point {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output { self.radius_vector - rhs.radius_vector }
}

impl Add<Vec3> for Point {
    type Output = Self;

    fn add(self, rhs: Vec3) -> Self::Output {
        Point {
            radius_vector: self.radius_vector + rhs,
        }
    }
}

impl Sub<Vec3> for Point {
    type Output = Self;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Point {
            radius_vector: self.radius_vector - rhs,
        }
    }
}
