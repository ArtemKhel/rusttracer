use std::{
    fmt::{Debug, Display},
    mem::transmute,
    ops::{Deref, Index, IndexMut},
};

use approx::AbsDiffEq;
use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use derive_new::new;
use num_traits::{Float, Num, One, Pow, Signed, Zero};
use rand::{
    self,
    distributions::{uniform::SampleUniform, Distribution, Standard},
    Rng,
};

use crate::{
    impl_axis_index,
    math::{
        axis::Axis3,
        transform::{Transform, Transformable},
        unit::Unit,
        Cross, Dot, Normal3, Normed, Number, Vec2, Vec4,
    },
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[derive(new, Neg, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign)]
pub struct Vec3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}

#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {
        $crate::math::Vec3 { x: $x, y: $y, z: $z }
    };
    ($x:expr) => {
        $crate::math::Vec3 { x: $x, y: $x, z: $x }
    };
}
#[macro_export]
macro_rules! unit_vec3 {
    ($x:expr, $y:expr, $z:expr) => {
        $crate::math::Unit::from($crate::vec3!($x, $y, $z))
    };
}

impl<T: Number> Vec3<T> {
    pub fn ones() -> Vec3<T> { vec3!(T::one()) }

    pub fn to_normal(self) -> Normal3<T> { Normal3 { value: self } }

    pub fn from_axis(axis: Axis3, value: T) -> Vec3<T> {
        match axis {
            Axis3::X => vec3!(value, T::zero(), T::zero()),
            Axis3::Y => vec3!(T::zero(), value, T::zero()),
            Axis3::Z => vec3!(T::zero(), T::zero(), value),
        }
    }

    pub fn only(&self, axis: Axis3) -> Vec3<T> {
        match axis {
            Axis3::X => vec3!(self.x, T::zero(), T::zero()),
            Axis3::Y => vec3!(T::zero(), self.y, T::zero()),
            Axis3::Z => vec3!(T::zero(), T::zero(), self.z),
        }
    }

    pub fn map<F, Out>(&self, mut f: F) -> Vec3<Out>
    where F: FnMut(T) -> Out {
        vec3!(f(self.x), f(self.y), f(self.z))
    }
}

impl<T: Number> Normed for Vec3<T> {
    type Output = T;

    fn to_unit(self) -> Unit<Self> { self.into() }

    fn len(&self) -> T { self.dot(self).sqrt() }

    fn len_squared(&self) -> T { self.dot(self) }
}

impl<T: Number + SampleUniform> Distribution<Vec3<T>> for Standard
where Standard: Distribution<T>
{
    /// Random vector with each coordinate varying from 0 to 1
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3<T> {
        Vec3::new(rng.gen(), rng.gen(), rng.gen())
        // vec3!(
        //     rng.gen_range(-T::one()..=T::one()),
        //     rng.gen_range(-T::one()..=T::one()),
        //     rng.gen_range(-T::one()..=T::one())
        // )
    }
}

impl_axis_index!(Vec3, Axis3, T, (X, x), (Y, y), (Z, z));

macro_rules! gen_mul {
    ($( $T:ty ),*) => {$(
        impl Mul<Vec3<$T>> for $T{
            type Output = Vec3<$T>;

            fn mul(self, rhs: Vec3<$T>) -> Self::Output { rhs * self }
        }
    )*};
}
gen_mul!(f32 /* , f64 */);

impl<T: Number> Cross<Vec3<T>> for Vec3<T>
where T: Copy
{
    type Output = Vec3<T>;

    fn cross(&self, rhs: &Vec3<T>) -> Self::Output {
        vec3!(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x
        )
    }
}

impl<T: Number> Dot<Vec3<T>> for Vec3<T> {
    type Output = T;

    fn dot(&self, rhs: &Vec3<T>) -> Self::Output { self.x * rhs.x + self.y * rhs.y + self.z * rhs.z }
}

impl<T: Number> Zero for Vec3<T> {
    fn zero() -> Self { vec3!(T::zero()) }

    fn is_zero(&self) -> bool { self.x == T::zero() && self.y == T::zero() && self.z == T::zero() }
}

impl<T: Float + AbsDiffEq<Epsilon = T>> AbsDiffEq for Vec3<T> {
    type Epsilon = T;

    fn default_epsilon() -> Self::Epsilon { T::epsilon() }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.x.abs_diff_eq(&other.x, epsilon)
            && self.y.abs_diff_eq(&other.y, epsilon)
            && self.z.abs_diff_eq(&other.z, epsilon)
    }
}

impl<T: Number> Transformable<T> for Vec3<T> {
    fn transform(&self, trans: &Transform<T>) -> Self {
        let vec = Vec4::from(*self);
        let px = vec.dot(&trans.mat.x);
        let py = vec.dot(&trans.mat.y);
        let pz = vec.dot(&trans.mat.z);
        vec3!(px, py, pz)
    }

    fn inv_transform(&self, trans: &Transform<T>) -> Self {
        let vec = Vec4::from(*self);
        let px = vec.dot(&trans.inv.x);
        let py = vec.dot(&trans.inv.y);
        let pz = vec.dot(&trans.inv.z);
        vec3!(px, py, pz)
    }
}

impl<T: Zero> From<Vec2<T>> for Vec3<T> {
    fn from(vec: Vec2<T>) -> Self { vec3!(vec.x, vec.y, T::zero()) }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::{cross, dot};

    #[test]
    fn test() {
        let u = vec3!(1., 1., 1.);
        let v = vec3!(2., 2., 2.);
        let r = v + u;

        assert_eq!(r, vec3!(3., 3., 3.));
    }

    #[test]
    fn test_cross() {
        let u = vec3!(1., 0., 0.);
        let v = vec3!(0., 1., 0.);
        let r = u.cross(&v);
        let r2 = cross(&u, &v);

        assert_eq!(r, vec3!(0., 0., 1.));
        assert_eq!(r2, vec3!(0., 0., 1.));
    }

    #[test]
    fn test_dot() {
        let u = vec3!(2., 3., 4.);
        let v = vec3!(1., 2., 3.);
        let r = v.dot(&u);
        let r2 = dot(&u, &v);

        assert_eq!(r, 20.0);
        assert_eq!(r2, 20.0);
    }
}
