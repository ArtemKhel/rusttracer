use std::ops::{Index, Mul};

use arrayvec::ArrayVec;
use itertools::Itertools;
use num_traits::Zero;
use strum::IntoEnumIterator;

use crate::{
    impl_axis_index,
    math::{axis::Axis3, dot, Number, Vec3},
    vec3,
};

#[derive(Debug, Copy, Clone, PartialEq)]
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

    pub fn diag(x: T, y: T, z: T) -> Self {
        Matrix3 {
            x: Vec3::from_axis(Axis3::X, x),
            y: Vec3::from_axis(Axis3::Y, y),
            z: Vec3::from_axis(Axis3::Z, z),
        }
    }

    #[rustfmt::skip]
    #[allow(clippy::too_many_arguments)]
    pub fn from_elements(
        m00: T, m01: T, m02: T,
        m10: T, m11: T, m12: T,
        m20: T, m21: T, m22: T,
    ) -> Self {
        Matrix3 {
            x: vec3!(m00, m01, m02),
            y: vec3!(m10, m11, m12),
            z: vec3!(m20, m21, m22),
        }
    }

    pub fn from_array(array: &[T; 9]) -> Self {
        Self::from_elements(
            array[0], array[1], array[2], array[3], array[4], array[5], array[6], array[7], array[8],
        )
    }

    pub fn row(&self, axis: Axis3) -> Vec3<T> { self[axis] }

    pub fn col(&self, axis: Axis3) -> Vec3<T> {
        use Axis3::*;
        Vec3::new(self[X][axis], self[Y][axis], self[Z][axis])
    }

    pub fn determinant(&self) -> T {
        let m00 = self.x.x;
        let m01 = self.x.y;
        let m02 = self.x.z;

        let m10 = self.y.x;
        let m11 = self.y.y;
        let m12 = self.y.z;

        let m20 = self.z.x;
        let m21 = self.z.y;
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
        let inv_det = det.recip();

        #[rustfmt::skip]
            let adj = Matrix3 {
            x: vec3!(
                (self.y.y * self.z.z - self.y.z * self.z.y),
               -(self.y.x * self.z.z - self.y.z * self.z.x),
                (self.y.x * self.z.y - self.y.y * self.z.x)
            ),
            y: vec3!(
               -(self.x.y * self.z.z - self.x.z * self.z.y),
                (self.x.x * self.z.z - self.x.z * self.z.x),
               -(self.x.x * self.z.y - self.x.y * self.z.x)
            ),
            z: vec3!(
                (self.x.y * self.y.z - self.x.z * self.y.y),
               -(self.x.x * self.y.z - self.x.z * self.y.x),
                (self.x.x * self.y.y - self.x.y * self.y.x)
            ),
        }.transpose();
        Some(adj * inv_det)
    }
}

impl<T: Number> Mul for Matrix3<T> {
    type Output = Matrix3<T>;

    fn mul(self, rhs: Self) -> Self::Output {
        Matrix3::from_array(
            &Axis3::iter()
                .cartesian_product(Axis3::iter())
                .map(|(i, j)| self[i][j] * rhs[j][i])
                .collect::<ArrayVec<T, 9>>()
                .into_inner()
                .unwrap(),
        )
    }
}

impl<T: Number> Mul<Vec3<T>> for Matrix3<T> {
    type Output = Vec3<T>;

    fn mul(self, rhs: Vec3<T>) -> Self::Output {
        Vec3 {
            x: dot(&self.x, &rhs),
            y: dot(&self.y, &rhs),
            z: dot(&self.z, &rhs),
        }
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

impl_axis_index!(Matrix3, Axis3, Vec3<T>, (X, x), (Y, y), (Z, z));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inverse() {
        let input = Matrix3::from_elements(1., 2., 3., 3., 2., 1., 2., 1., 3.);
        let expected = Matrix3::from_elements(-5., 3., 4., 7., 3., -8., 1., -3., 4.) * (1. / 12.);

        assert_eq!(input.determinant(), -12.);
        assert_eq!(input.invert().unwrap(), expected)
    }
}
