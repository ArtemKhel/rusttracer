use std::ops::Mul;

use num_traits::Zero;

use crate::{
    math::{utils::Axis3, Number, Vec3},
    vec3,
};

#[derive(Debug, Copy, Clone)]
pub struct Matrix3<T> {
    pub x: Vec3<T>,
    pub y: Vec3<T>,
    pub z: Vec3<T>,
}

impl<T: Number> Matrix3<T> {
    pub fn id() -> Self {
        Matrix3 {
            x: Vec3::from_axis(Axis3::X, T::one()),
            y: Vec3::from_axis(Axis3::Y, T::one()),
            z: Vec3::from_axis(Axis3::Z, T::one()),
        }
    }

    pub fn zero() -> Self {
        Matrix3 {
            x: Vec3::zero(),
            y: Vec3::zero(),
            z: Vec3::zero(),
        }
    }

    pub fn determinant(&self) -> T {
        let m00 = self.x.x;
        let m01 = self.y.x;
        let m02 = self.z.x;

        let m10 = self.x.y;
        let m11 = self.y.y;
        let m12 = self.z.y;

        let m20 = self.x.z;
        let m21 = self.y.z;
        let m22 = self.z.z;

        m00 * (m11 * m22 - m12 * m21) - m01 * (m10 * m22 - m12 * m20) + m02 * (m10 * m21 - m11 * m20)
    }

    pub fn transpose(&self) -> Matrix3<T> {
        Matrix3 {
            x: vec3!(self.x.x, self.y.x, self.z.x),
            y: vec3!(self.x.y, self.y.y, self.z.y),
            z: vec3!(self.x.z, self.y.z, self.z.z),
        }
    }

    pub fn invert(&self) -> Option<Matrix3<T>> {
        let det = self.determinant();

        if det == T::zero() {
            return None;
        }

        let inv_det = T::one() / det;
        let inv = Matrix3 {
            x: vec3!(
                (self.y.y * self.z.z - self.y.z * self.z.y),
                (self.z.y * self.x.z - self.z.z * self.x.y),
                (self.x.y * self.y.z - self.x.z * self.y.y)
            ),
            y: vec3!(
                (self.y.z * self.z.x - self.y.x * self.z.z),
                (self.z.z * self.x.x - self.z.x * self.x.z),
                (self.x.z * self.y.x - self.x.x * self.y.z)
            ),
            z: vec3!(
                (self.y.x * self.z.y - self.y.y * self.z.x),
                (self.z.x * self.x.y - self.z.y * self.x.x),
                (self.x.x * self.y.y - self.x.y * self.y.x)
            ),
        };

        Some(inv * inv_det)
    }
}

impl<T: Number> Mul<T> for Matrix3<T> {
    type Output = Matrix3<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Matrix3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}
