use std::{
    cmp::min,
    ops::{Index, Mul, Neg},
};

use crate::{vec::Vec3, Dot};

#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub struct UnitVec {
    pub vec: Vec3,
}

impl UnitVec {
    pub fn new(x: f32, y: f32, z: f32) -> Self { Vec3 { x, y, z }.to_unit() }

    pub fn reflect(&self, normal: UnitVec) -> UnitVec { (self.vec - (normal * self.dot(normal) * 2.0)).to_unit() }

    pub fn refract(&self, normal: UnitVec, refraction_coef_ratio: f32) -> UnitVec {
        let mut cos_theta = self.dot(normal);
        let sign = cos_theta.signum();
        // cos_theta *= sign;
        let perpend = refraction_coef_ratio * (self.vec + normal * cos_theta * sign);
        let parallel = normal * -f32::sqrt(f32::abs(1. - perpend.dot(perpend)));
        (perpend + parallel).to_unit()
    }
}

impl Index<usize> for UnitVec {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output { &self.vec[index] }
}

impl Neg for UnitVec {
    type Output = Self;

    fn neg(self) -> Self::Output { UnitVec { vec: -self.vec } }
}

impl Mul<f32> for UnitVec {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output { self.vec * rhs }
}

impl Mul<UnitVec> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: UnitVec) -> Self::Output { rhs.vec * self }
}

impl Dot<UnitVec> for UnitVec {
    fn dot(&self, rhs: UnitVec) -> f32 { self.vec.dot(rhs.vec) }
}

impl Dot<Vec3> for UnitVec {
    fn dot(&self, rhs: Vec3) -> f32 { self.vec.dot(rhs) }
}

impl From<Vec3> for UnitVec {
    fn from(value: Vec3) -> Self {
        UnitVec {
            vec: value / value.len(),
        }
    }
}
