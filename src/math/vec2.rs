use std::{
    fmt::{Debug, Display},
    ops::{Deref, Index, IndexMut},
};

use approx::AbsDiffEq;
use derive_more::{Add, AddAssign, Div, DivAssign, From, Mul, MulAssign, Neg, Sub, SubAssign};
use derive_new::new;
use num_traits::{Float, Num, One, Pow, Signed, Zero};
use rand::{
    self,
    distributions::{uniform::SampleUniform, Distribution, Standard},
    Rng,
};

use crate::{
    impl_axis_index,
    math::{axis::Axis2, transform::Transformable, unit::Unit, Cross, Dot, Normed, Number},
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Hash)]
#[derive(new, Neg, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T,
}

#[macro_export]
macro_rules! vec2 {
    ($x:expr, $y:expr) => {
        $crate::math::Vec2 { x: $x, y: $y }
    };
    ($x:expr) => {
        $crate::math::Vec2 { x: $x, y: $x }
    };
}

impl<T: Number> Vec2<T> {
    pub fn ones() -> Vec2<T> { vec2!(T::one()) }

    pub fn from_axis(axis: Axis2, value: T) -> Vec2<T> {
        match axis {
            Axis2::X => vec2!(value, T::zero()),
            Axis2::Y => vec2!(T::zero(), value),
        }
    }

    pub fn only(&self, axis: Axis2) -> Vec2<T> {
        match axis {
            Axis2::X => vec2!(self.x, T::zero()),
            Axis2::Y => vec2!(T::zero(), self.y),
        }
    }
}
impl<T: Copy> Vec2<T> {
    pub fn map<F, Out>(&self, mut f: F) -> Vec2<Out>
    where F: FnMut(T) -> Out {
        vec2!(f(self.x), f(self.y))
    }
}

impl<T: Number> Normed for Vec2<T> {
    type Output = T;

    fn to_unit(self) -> Unit<Self> { self.into() }

    fn len(&self) -> T { T::pow(self.dot(self), 0.5) }

    fn len_squared(&self) -> T { self.dot(self) }
}

impl<T: Number + SampleUniform> Distribution<Vec2<T>> for Standard
where Standard: Distribution<T>
{
    /// Random vector with each coordinate varying from 0 to 1
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Vec2<T> {
        // vec2!(rng.gen_range(-T::one()..=T::one()), rng.gen_range(-T::one()..=T::one()))
        Vec2::new(rng.gen(), rng.gen())
    }
}

impl_axis_index!(Vec2, Axis2, T, (X, x), (Y, y));

macro_rules! gen_mul {
    ($( $T:ty ),*) => {$(
        impl Mul<Vec2<$T>> for $T{
            type Output = Vec2<$T>;

            fn mul(self, rhs: Vec2<$T>) -> Self::Output { rhs * self }
        }
    )*};
}
gen_mul!(f32 /* , f64 */);

impl<T: Number> Dot<Vec2<T>> for Vec2<T> {
    type Output = T;

    fn dot(&self, rhs: &Vec2<T>) -> Self::Output { self.x * rhs.x + self.y * rhs.y }
}

impl<T: Number> Zero for Vec2<T> {
    fn zero() -> Self { vec2!(T::zero()) }

    fn is_zero(&self) -> bool { self.x == T::zero() && self.y == T::zero() }
}

impl<T: Float + AbsDiffEq<Epsilon = T>> AbsDiffEq for Vec2<T> {
    type Epsilon = T;

    fn default_epsilon() -> Self::Epsilon { T::epsilon() }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.x.abs_diff_eq(&other.x, epsilon) && self.y.abs_diff_eq(&other.y, epsilon)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::math::dot;

    #[test]
    fn test() {
        let u = vec2!(1., 1.);
        let v = vec2!(2., 3.);
        let r = v + u;

        assert_eq!(r, vec2!(3., 4.));
    }

    #[test]
    fn test_dot() {
        let u = vec2!(2., 3.);
        let v = vec2!(1., 2.);
        let r = v.dot(&u);
        let r2 = dot(&u, &v);

        assert_eq!(r, 8.0);
        assert_eq!(r2, 8.0);
    }
}
