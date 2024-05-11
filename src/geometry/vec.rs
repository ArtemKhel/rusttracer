use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

use rand::{
    self,
    distributions::{Distribution, Standard},
    Rng,
};
use serde::{Deserialize, Serialize};

use crate::geometry::{unit_vec::UnitVec, utils::Axis, Cross, Dot};

// TODO: macros?
#[derive(Debug, Clone, Copy, PartialOrd, PartialEq, Serialize, Deserialize)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self { Vec3 { x, y, z } }

    pub fn to_unit(self) -> UnitVec { self.into() }

    pub fn len(&self) -> f32 { f32::sqrt(self.dot(*self)) }
}

impl Distribution<Vec3> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3 {
        Vec3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
            rng.gen_range(-1.0..1.0),
        )
    }
}

impl Index<Axis> for Vec3 {
    type Output = f32;

    fn index(&self, index: Axis) -> &Self::Output {
        match index {
            Axis::X => &self.x,
            Axis::Y => &self.y,
            Axis::Z => &self.z,
        }
    }
}

impl IndexMut<Axis> for Vec3 {
    fn index_mut(&mut self, index: Axis) -> &mut Self::Output {
        match index {
            Axis::X => &mut self.x,
            Axis::Y => &mut self.y,
            Axis::Z => &mut self.z,
        }
    }
}

impl Div<f32> for Vec3 {
    type Output = Self;

    fn div(self, rhs: f32) -> Self::Output {
        let x = 1.0 / rhs;
        self * x
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output { Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z) }
}

impl Add<UnitVec> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: UnitVec) -> Self::Output { self + rhs.vec }
}

impl Neg for Vec3 {
    type Output = Self;

    fn neg(self) -> Self::Output { Vec3::new(-self.x, -self.y, -self.z) }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output { self + (-rhs) }
}

impl Mul<f32> for Vec3 {
    type Output = Self;

    fn mul(self, rhs: f32) -> Self::Output { Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs) }
}

impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output { Vec3::new(rhs.x * self, rhs.y * self, rhs.z * self) }
}

impl Cross<Vec3> for Vec3 {
    fn cross(&self, rhs: Vec3) -> Vec3 {
        Vec3::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }
}

impl Dot<Vec3> for Vec3 {
    fn dot(&self, rhs: Vec3) -> f32 { self.x * rhs.x + self.y * rhs.y + self.z * rhs.z }
}

impl Dot<UnitVec> for Vec3 {
    fn dot(&self, rhs: UnitVec) -> f32 { self.dot(rhs.vec) }
}

impl Default for Vec3 {
    fn default() -> Self { Vec3::new(0., 0., 0.) }
}
