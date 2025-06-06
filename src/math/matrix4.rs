use std::ops::{Index, IndexMut, Mul};

use derive_more::{Add, Sub};
use num_traits::Zero;
use strum::IntoEnumIterator;

use crate::{
    impl_axis_index,
    math::{
        axis::{
            Axis4,
            Axis4::{W, X, Y, Z},
        },
        dot, Matrix3, Number, Vec4,
    },
    vec4,
};

#[derive(Debug, Copy, Clone, PartialEq)]
#[derive(Add, Sub)]
pub struct Matrix4<T> {
    pub x: Vec4<T>,
    pub y: Vec4<T>,
    pub z: Vec4<T>,
    pub w: Vec4<T>,
}

impl<T: Number> Matrix4<T> {
    pub fn id() -> Self {
        Matrix4 {
            x: Vec4::from_axis(Axis4::X, T::one()),
            y: Vec4::from_axis(Axis4::Y, T::one()),
            z: Vec4::from_axis(Axis4::Z, T::one()),
            w: Vec4::from_axis(Axis4::W, T::one()),
        }
    }

    #[rustfmt::skip]
    #[allow(clippy::too_many_arguments)]
    pub fn from_elements(
        m00: T, m01: T, m02: T, m03: T,
        m10: T, m11: T, m12: T, m13: T,
        m20: T, m21: T, m22: T, m23: T,
        m30: T, m31: T, m32: T, m33: T,
    ) -> Self {
        Matrix4 {
            x: vec4!(m00, m01, m02, m03),
            y: vec4!(m10, m11, m12, m13),
            z: vec4!(m20, m21, m22, m23),
            w: vec4!(m30, m31, m32, m33),
        }
    }

    pub fn col(&self, axis: Axis4) -> Vec4<T> { vec4!(self.x[axis], self.y[axis], self.z[axis], self.w[axis]) }

    pub fn transpose(&self) -> Matrix4<T> {
        Matrix4 {
            x: vec4!(self.x.x, self.y.x, self.z.x, self.w.x),
            y: vec4!(self.x.y, self.y.y, self.z.y, self.w.y),
            z: vec4!(self.x.z, self.y.z, self.z.z, self.w.z),
            w: vec4!(self.x.w, self.y.w, self.z.w, self.w.w),
        }
    }

    pub fn determinant(&self) -> T {
        let m00 = self.x.x;
        let m01 = self.y.x;
        let m02 = self.z.x;
        let m03 = self.w.x;

        let m10 = self.x.y;
        let m11 = self.y.y;
        let m12 = self.z.y;
        let m13 = self.w.y;

        let m20 = self.x.z;
        let m21 = self.y.z;
        let m22 = self.z.z;
        let m23 = self.w.z;

        let m30 = self.x.w;
        let m31 = self.y.w;
        let m32 = self.z.w;
        let m33 = self.w.w;

        m00 * (m11 * (m22 * m33 - m23 * m32) - m12 * (m21 * m33 - m23 * m31) + m13 * (m21 * m32 - m22 * m31))
            - m01 * (m10 * (m22 * m33 - m23 * m32) - m12 * (m20 * m33 - m23 * m30) + m13 * (m20 * m32 - m22 * m30))
            + m02 * (m10 * (m21 * m33 - m23 * m31) - m11 * (m20 * m33 - m23 * m30) + m13 * (m20 * m31 - m21 * m30))
            - m03 * (m10 * (m21 * m32 - m22 * m31) - m11 * (m20 * m32 - m22 * m30) + m12 * (m20 * m31 - m21 * m30))
    }

    pub fn minor(&self, row: Axis4, col: Axis4) -> Matrix3<T> {
        match row {
            X => Matrix3 {
                x: self.y.drop(col),
                y: self.z.drop(col),
                z: self.w.drop(col),
            },
            Y => Matrix3 {
                x: self.x.drop(col),
                y: self.z.drop(col),
                z: self.w.drop(col),
            },
            Z => Matrix3 {
                x: self.x.drop(col),
                y: self.y.drop(col),
                z: self.w.drop(col),
            },
            W => Matrix3 {
                x: self.x.drop(col),
                y: self.y.drop(col),
                z: self.z.drop(col),
            },
        }
    }

    pub fn invert(&self) -> Option<Matrix4<T>> {
        let det = self.determinant();
        if det == T::zero() {
            return None;
        }
        let inv_det = det.recip();

        let t = self.transpose();
        let cf = |j, i| {
            #[rustfmt::skip]
            let mat = self.minor(i, j);
            let sign = if (i as usize + j as usize) & 1 == 1 {
                -T::one()
            } else {
                T::one()
            };
            mat.determinant() * sign * inv_det
        };

        use Axis4::*;
        #[rustfmt::skip]
        Some(Matrix4::from_elements(
            cf(X, X), cf(X, Y), cf(X, Z), cf(X, W),
            cf(Y, X), cf(Y, Y), cf(Y, Z), cf(Y, W),
            cf(Z, X), cf(Z, Y), cf(Z, Z), cf(Z, W),
            cf(W, X), cf(W, Y), cf(W, Z), cf(W, W),
        ))
    }
}

impl<T: Number> Mul<T> for Matrix4<T> {
    type Output = Matrix4<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Matrix4 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
            w: self.w * rhs,
        }
    }
}

impl<T: Number> Mul for Matrix4<T> {
    type Output = Self;

    #[rustfmt::skip]
    fn mul(self, rhs: Self) -> Self::Output {
        use Axis4::*;
        Matrix4::from_elements(
            dot(&self.x, &rhs.col(X)), dot(&self.x, &rhs.col(Y)), dot(&self.x, &rhs.col(Z)), dot(&self.x, &rhs.col(W)),
            dot(&self.y, &rhs.col(X)), dot(&self.y, &rhs.col(Y)), dot(&self.y, &rhs.col(Z)), dot(&self.y, &rhs.col(W)),
            dot(&self.z, &rhs.col(X)), dot(&self.z, &rhs.col(Y)), dot(&self.z, &rhs.col(Z)), dot(&self.z, &rhs.col(W)),
            dot(&self.w, &rhs.col(X)), dot(&self.w, &rhs.col(Y)), dot(&self.w, &rhs.col(Z)), dot(&self.w, &rhs.col(W)),
        )
    }
}

impl<T: Number> Zero for Matrix4<T> {
    fn zero() -> Self {
        Matrix4 {
            x: Vec4::zero(),
            y: Vec4::zero(),
            z: Vec4::zero(),
            w: Vec4::zero(),
        }
    }

    fn is_zero(&self) -> bool { self.x.is_zero() && self.y.is_zero() && self.z.is_zero() && self.w.is_zero() }
}

impl_axis_index!(Matrix4, Axis4, Vec4<T>, (X, x), (Y, y), (Z, z), (W, w));

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_inverse() {
        let input = Matrix4::from_elements(1., 2., 3., 4., 4., 3., 2., 1., 2., 1., 3., 4., 4., 3., 1., 2.);
        let expected = Matrix4::from_elements(
            -18., 2., 15., 5., 22., 2., -25., 5., 2., 22., 5., -25., 2., -18., 5., 15.,
        ) * (1. / 40.);

        assert_eq!(input.invert().unwrap(), expected)
    }
}
