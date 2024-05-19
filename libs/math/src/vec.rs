use std::{
    fmt::{Debug, Display},
    ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub},
};

use derive_new::new;
use gen_ops::gen_ops;
use num_traits::{Float, Num, One, Pow, Signed, Zero};
use rand::{
    self,
    distributions::{uniform::SampleUniform, Distribution, Standard},
    Rng,
};

use crate::{unit_vec::UnitVec3, utils::Axis, Cross, Dot, Number};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, new)]
pub struct Vec3<T: Number> {
    pub x: T,
    pub y: T,
    pub z: T,
}

pub type Vec3f = Vec3<f32>;

#[macro_export]
macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {
        Vec3 { x: $x, y: $y, z: $z }
    };
    ($x:expr) => {
        Vec3 { x: $x, y: $x, z: $x }
    };
}

impl<T: Number> Vec3<T> {
    pub fn to_unit(self) -> UnitVec3<T> { self.into() }

    pub fn len(&self) -> T { T::pow(self.dot(self), 0.5) }

    pub fn len_squared(&self) -> T { self.dot(self) }

    pub fn ones() -> Vec3<T> { vec3!(T::one()) }

    pub fn only(&self, axis: Axis) -> Vec3<T> {
        match axis {
            Axis::X => vec3!(self.x, T::zero(), T::zero()),
            Axis::Y => vec3!(T::zero(), self.y, T::zero()),
            Axis::Z => vec3!(T::zero(), T::zero(), self.z),
        }
    }
}

impl<T: Number + SampleUniform> Distribution<Vec3<T>> for Standard {
    /// Random vector with each coordinate varying from -1 to 1
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec3<T> { vec3!(rng.gen_range(-T::one()..=T::one())) }
}

impl<T: Number> Index<Axis> for Vec3<T> {
    type Output = T;

    fn index(&self, index: Axis) -> &Self::Output {
        match index {
            Axis::X => &self.x,
            Axis::Y => &self.y,
            Axis::Z => &self.z,
        }
    }
}

impl<T: Number> IndexMut<Axis> for Vec3<T> {
    fn index_mut(&mut self, index: Axis) -> &mut Self::Output {
        match index {
            Axis::X => &mut self.x,
            Axis::Y => &mut self.y,
            Axis::Z => &mut self.z,
        }
    }
}

gen_ops!(
    <T>;
    types Vec3<T>, Vec3<T> => Vec3<T>;

    for + call |a: &Vec3<T>, b: &Vec3<T>| {
        vec3!(a.x + b.x, a.y + b.y, a.z + b.z)
    };

    for - call |a: &Vec3<T>, b: &Vec3<T>| {
        vec3!(a.x - b.x, a.y - b.y, a.z - b.z)
    };

    where T:Number
);

gen_ops!(
    <T>;
    types Vec3<T>, T => Vec3<T>;

    for * call |a: &Vec3<T>, b: &T| {
        vec3!(a.x * *b, a.y * *b, a.z * *b)
    };

    for / call |a: &Vec3<T>, b: &T| {
        vec3!(a.x / *b, a.y / *b, a.z / *b)
    };

    where T:Number
);

gen_ops!(
    <T>;
    types Vec3<T>, Vec3<T>;

    for += call |a: &mut Vec3<T>, b: &Vec3<T>| {
        a.x = a.x + b.x;
        a.y = a.y + b.y;
        a.z = a.z + b.z;
    };

    for -= call |a: &mut Vec3<T>, b: &Vec3<T>| {
        a.x = a.x - b.x;
        a.y = a.y - b.y;
        a.z = a.z - b.z;
    };

    where T:Number
);

#[macro_export]
macro_rules! gen_mul {
    ($( $T:ty ),*) => {$(
        impl Mul<Vec3<$T>> for $T{
            type Output = Vec3<$T>;

            fn mul(self, rhs: Vec3<$T>) -> Self::Output { rhs * self }
        }
    )*};
}
gen_mul!(f32);

gen_ops!(
    <T>;
    types Vec3<T> => Vec3<T>;

    for - call |a: &Vec3<T>| {
        vec3!(-a.x, -a.y, -a.z)
    };
    where T:Number
);

impl<T: Number> Cross<Vec3<T>> for Vec3<T>
where T: Copy
{
    type Output = Vec3<T>;

    fn cross(&self, rhs: Vec3<T>) -> Self::Output {
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

impl<T: Number> Zero for Vec3<T>
where T: Copy
{
    fn zero() -> Self { vec3!(T::zero()) }

    fn is_zero(&self) -> bool { self.x == T::zero() && self.y == T::zero() && self.z == T::zero() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::dot;

    #[test]
    fn test() {
        let u = vec3!(1., 1., 1.);
        let v = vec3!(2., 2., 2.);
        let r = v + u;

        assert_eq!(r, Vec3f::new(3., 3., 3.))
    }

    #[test]
    fn test_cross() {
        let u = vec3!(1., 0., 0.);
        let v = vec3!(0., 1., 0.);
        let r = u.cross(v);
        let r2 = crate::cross(u, v);

        assert_eq!(r, Vec3f::new(0., 0., 1.));
        assert_eq!(r2, Vec3f::new(0., 0., 1.));
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

    #[test]
    fn test_axis_index() {
        let mut v = vec3!(1., 2., 3.);
        assert_eq!(v.x, 1.);
        assert_eq!(v.y, 2.);
        assert_eq!(v.z, 3.);

        v += v;
        assert_eq!(v.x, 2.);
        assert_eq!(v.y, 4.);
        assert_eq!(v.z, 6.);
    }
}
