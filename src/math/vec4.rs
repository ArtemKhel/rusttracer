use std::{
    fmt::{Debug, Display},
    ops::{Deref, Index, IndexMut},
};

use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use derive_new::new;
use gen_ops::gen_ops;
use num_traits::{One, Pow, Zero};
use rand::{
    self,
    distributions::{uniform::SampleUniform, Distribution, Standard},
    Rng,
};

use crate::{
    impl_axis_index,
    math::{axis::Axis4, Cross, Dot, Number, Point3, Vec3},
    vec3,
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[derive(new, Neg, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign)]
pub struct Vec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

#[macro_export]
macro_rules! vec4 {
    ($x:expr, $y:expr, $z:expr, $w:expr) => {
        $crate::math::Vec4 {
            x: $x,
            y: $y,
            z: $z,
            w: $w,
        }
    };
    ($x:expr) => {
        $crate::math::Vec4 {
            x: $x,
            y: $x,
            z: $x,
            w: $x,
        }
    };
}

impl<T: Number> Vec4<T> {
    pub fn ones() -> Vec4<T> { vec4!(T::one()) }

    pub fn from_axis(axis: Axis4, value: T) -> Vec4<T> {
        match axis {
            Axis4::X => vec4!(value, T::zero(), T::zero(), T::zero()),
            Axis4::Y => vec4!(T::zero(), value, T::zero(), T::zero()),
            Axis4::Z => vec4!(T::zero(), T::zero(), value, T::zero()),
            Axis4::W => vec4!(T::zero(), T::zero(), T::zero(), value),
        }
    }

    pub fn only(&self, axis: Axis4) -> Vec4<T> {
        match axis {
            Axis4::X => vec4!(self.x, T::zero(), T::zero(), T::zero()),
            Axis4::Y => vec4!(T::zero(), self.y, T::zero(), T::zero()),
            Axis4::Z => vec4!(T::zero(), T::zero(), self.z, T::zero()),
            Axis4::W => vec4!(T::zero(), T::zero(), T::zero(), self.w),
        }
    }

    pub fn drop(&self, axis: Axis4) -> Vec3<T> {
        match axis {
            Axis4::X => vec3!(self.y, self.z, self.w),
            Axis4::Y => vec3!(self.x, self.z, self.w),
            Axis4::Z => vec3!(self.x, self.y, self.w),
            Axis4::W => vec3!(self.x, self.y, self.z),
        }
    }
}

impl<T: Number + SampleUniform> Distribution<Vec4<T>> for Standard {
    /// Random vector with each coordinate varying from -1 to 1
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec4<T> {
        vec4!(
            rng.gen_range(-T::one()..=T::one()),
            rng.gen_range(-T::one()..=T::one()),
            rng.gen_range(-T::one()..=T::one()),
            rng.gen_range(-T::one()..=T::one())
        )
    }
}

impl_axis_index!(Vec4, Axis4, T, (X, x), (Y, y), (Z, z), (W, w));

macro_rules! gen_mul {
    ($( $T:ty ),*) => {$(
        impl Mul<Vec4<$T>> for $T{
            type Output = Vec4<$T>;

            fn mul(self, rhs: Vec4<$T>) -> Self::Output { rhs * self }
        }
    )*};
}
gen_mul!(f32 /* , f64 */);

impl<T: Number> Dot<Vec4<T>> for Vec4<T> {
    type Output = T;

    fn dot(&self, rhs: &Vec4<T>) -> Self::Output { self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w }
}

impl<T: Number> Zero for Vec4<T>
where T: Copy
{
    fn zero() -> Self { vec4!(T::zero()) }

    fn is_zero(&self) -> bool {
        self.x == T::zero() && self.y == T::zero() && self.z == T::zero() && self.w == T::zero()
    }
}

impl<T: Number> From<Vec3<T>> for Vec4<T> {
    fn from(v: Vec3<T>) -> Self { vec4!(v.x, v.y, v.z, T::zero()) }
}

impl<T: Number> From<Point3<T>> for Vec4<T> {
    fn from(v: Point3<T>) -> Self { vec4!(v.x, v.y, v.z, T::one()) }
}

// impl<Ref: Deref<Target = Vec3<T>>, T: Number> From<Ref> for Vec4<T> {
//     fn from(value: Ref) -> Self { Self::from(*value) }
// }
