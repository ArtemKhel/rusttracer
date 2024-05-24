use std::{
    fmt::Debug,
    ops::{Add, Deref, Mul, Neg},
    os::fd::IntoRawFd,
};

use approx::AbsDiffEq;
pub use frame::Frame;
pub use matrix::*;
pub use normal::Normal3;
use num_traits::{Float, Num, NumAssignOps, One, Pow, Signed};
pub use point::Point3;
pub use point2::Point2;
pub use transform::{Transform, Transformable};
pub use unit::Unit;
pub use vec::Vec3;
pub use vec4::Vec4;

mod frame;
mod matrix;
mod matrix4;
mod normal;
mod point;
mod point2;
mod transform;
mod unit;
pub mod utils;
mod vec;
mod vec4;

pub trait Number: Debug + Default + Float + NumAssignOps + Pow<f32, Output = Self> {}

impl<T> Number for T where T: Debug + Default + Float + NumAssignOps + Pow<f32, Output = Self> {}

pub trait Two {
    fn two() -> Self;
}

impl<T> Two for T
where T: One + Add<Output = T>
{
    fn two() -> Self { Self::one() + Self::one() }
}

#[allow(clippy::len_without_is_empty)]
pub trait Normed {
    type Output;
    fn to_unit(self) -> Unit<Self>
    where Self: Sized;
    fn len(&self) -> Self::Output;
    fn len_squared(&self) -> Self::Output;
}

impl<Ref, Base, Out> Normed for Ref
where
    Ref: Deref<Target = Base> + From<Base>,
    Base: Normed<Output = Out> + Copy,
{
    type Output = Out;

    fn to_unit(self) -> Unit<Self> { self.deref().to_unit().cast() }

    fn len(&self) -> Self::Output { self.deref().len() }

    fn len_squared(&self) -> Self::Output { self.deref().len_squared() }
}

pub trait Dot<RHS> {
    type Output;
    fn dot(&self, rhs: &RHS) -> Self::Output;
}

impl<T, U, Out: Number> Dot<U> for T
where
    U: Deref<Target = T>,
    T: Dot<T, Output = Out>,
{
    type Output = Out;

    fn dot(&self, rhs: &U) -> Self::Output { self.dot(rhs) }
}

pub fn dot<T: Dot<U>, U>(lhs: &T, rhs: &U) -> T::Output { lhs.dot(rhs) }

pub trait Cross<RHS = Self> {
    type Output;
    fn cross(&self, rhs: RHS) -> Self::Output;
}

pub fn cross<T: Cross>(lhs: T, rhs: T) -> T::Output { lhs.cross(rhs) }
