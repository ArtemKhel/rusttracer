use crate::vec::Vec3;
use crate::Dot;
use std::ops::{Mul, Neg};

#[derive(Debug, Clone, Copy)]
pub struct UnitVec {
    pub vec: Vec3,
}

impl Neg for UnitVec {
    type Output = Self;

    fn neg(self) -> Self::Output {
        UnitVec { vec: -self.vec }
    }
}

impl Mul<f32> for UnitVec {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        self.vec * rhs
    }
}

impl Dot<UnitVec> for UnitVec {
    fn dot(&self, rhs: UnitVec) -> f32 {
        self.vec.dot(rhs.vec)
    }
}

impl Dot<Vec3> for UnitVec {
    fn dot(&self, rhs: Vec3) -> f32 {
        self.vec.dot(rhs)
    }
}

impl From<Vec3> for UnitVec {
    fn from(value: Vec3) -> Self {
        UnitVec {
            vec: value / value.len(),
        }
    }
}
