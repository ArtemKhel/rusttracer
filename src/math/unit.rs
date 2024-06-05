use std::ops::{Add, Div, Mul, Sub};

use derive_more::{Deref as DMDeref, DerefMut as DMDerefMut, Neg};

use crate::math::{Normal3, Normed, Number, Vec3};

#[derive(Debug, Default, Clone, Copy, PartialEq, DMDeref, DMDerefMut, Neg)]
pub struct Unit<Inner> {
    value: Inner,
}

impl<Inner: Normed> Unit<Inner> {
    pub fn from_unchecked(value: Inner) -> Self { Unit { value } }

    pub fn into_inner(self) -> Inner { self.value }

    pub fn cast<Outer>(self) -> Unit<Outer>
    where Outer: From<Inner> {
        Unit {
            value: self.value.into(),
        }
    }
}

#[macro_export]
macro_rules! unit3 {
    ($x:expr, $y:expr, $z:expr) => {
        $crate::math::Unit::from($crate::vec3!($x, $y, $z))
    };
}
#[macro_export]
macro_rules! unit3_unchecked {
    ($x:expr, $y:expr, $z:expr) => {
        $crate::math::Unit::from_unchecked($crate::vec3!($x, $y, $z))
    };
}

impl<Inner, T> From<Inner> for Unit<Inner>
where Inner: Normed<Output = T> + Div<T, Output = Inner> + Copy
{
    fn from(value: Inner) -> Self {
        Unit {
            value: value / value.len(),
        }
    }
}

// TODO: add marker trait for any Vec wrapper
macro_rules! impl_unwrapping_add_sub {
    ($(($Trait:ident, $func:ident)),*) => {$(
        // unit - unit
        impl<T, Out> $Trait for Unit<T>
        where T: $Trait<Output = Out> + Copy
        {
            type Output = Out;

            fn $func(self, rhs: Self) -> Out { self.deref().$func(*rhs.deref()) }
        }

        // unit - vec
        impl<T, Out> $Trait<T> for Unit<T>
        where T: $Trait<Output = Out> + Copy
        {
            type Output = Out;

            fn $func(self, rhs: T) -> Out { self.deref().$func(rhs) }
        }

        // vec - unit
        impl<Wrap, T> $Trait<Wrap> for Vec3<T>
        where
            Wrap: std::ops::Deref<Target = Self>,
            T: $Trait<Output = T> + Copy,
        {
            type Output = Self;

            fn $func(self, rhs: Wrap) -> Self::Output { self.$func(*rhs.deref()) }
        }
    )*};
}

macro_rules! impl_unwrapping_mul_div {
    ($(($wrap:tt, $trait_:ident, $func:ident, $other:ty)),*) => {$(
        impl<T,Out> $trait_<$wrap<T>> for $other
        where T: $trait_<$other, Output=Out> + Copy{
            type Output = Out;
            // todo: other way
            fn $func(self, rhs: $wrap<T>) -> Self::Output { rhs.deref().$func(self) }
        }

        impl<T,Out> $trait_<$other> for $wrap<T>
        where T: $trait_<$other, Output=Out> + Copy{
            type Output = Out;
            fn $func(self, rhs: $other) -> Self::Output { self.deref().$func(rhs) }
        }
    )*};
}

impl_unwrapping_add_sub!((Add, add), (Sub, sub));
impl_unwrapping_mul_div!((Unit, Mul, mul, f32), (Unit, Div, div, f32));

#[cfg(test)]
mod tests {
    use std::f32::consts::{FRAC_1_SQRT_2, FRAC_1_SQRT_3, SQRT_2};

    use approx::assert_abs_diff_eq;

    use super::*;
    use crate::{
        math::{cross, dot, Cross},
        vec3,
    };

    #[test]
    fn test_imlp_unwrapping() {
        let u = vec3!(1.).to_unit();
        let v = vec3!(1.);
        let res1 = u + v;
        let res2 = v + u;
        let res3 = u + u;
        let expected12 = vec3!(1. + FRAC_1_SQRT_3);
        let expected3 = vec3!(2. * FRAC_1_SQRT_3);
        assert_abs_diff_eq!(res1, expected12);
        assert_abs_diff_eq!(res2, expected12);
        assert_abs_diff_eq!(res3, expected3);
    }

    #[test]
    fn test_macro() {
        let v = unit3!(5., 10., 0.);
        let len = v.len();
        let delta = 1e-5;
        assert!(-delta < len && len < 1. + delta)
    }

    #[test]
    fn test_dot_cross() {
        let u = unit3!(10., 0., 0.);
        let v = unit3!(0., 20., 0.);
        let r = cross(u.deref(), v.deref());
        let r2 = dot(u.deref(), u.deref());

        assert_eq!(r, vec3!(0., 0., 1.));
        assert_eq!(r2, 1.0);
    }
}
