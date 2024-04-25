use crate::vec::Vec3;
use std::ops::Mul;

#[derive(Debug, Clone, Copy)]
pub struct UnitVec {
    pub vec: Vec3,
}

impl Mul<f32> for UnitVec {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        self.vec * rhs
    }
}

impl From<Vec3> for UnitVec {
    fn from(value: Vec3) -> Self {
        UnitVec { vec: value / value.len() }
    }
}
