#![allow(clippy::just_underscores_and_digits)]

use std::{
    fmt::Debug,
    marker::PhantomData,
    ops::{Add, AddAssign, Deref, Div, DivAssign, Index, IndexMut, Mul, MulAssign, Neg, Sub, SubAssign},
    os::fd::IntoRawFd,
};

use approx::AbsDiffEq;
pub use bounds::Bounds2;
pub use frame::Frame;
pub use matrix3::*;
pub use normal::Normal3;
use num_traits::{Float, Num, NumAssignOps, One, Pow, Signed};
pub use point2::Point2;
pub use point3::Point3;
pub use transform::{Transform, Transformable};
pub use unit::Unit;
pub use vec2::Vec2;
pub use vec3::Vec3;
pub use vec4::Vec4;

use crate::{normal3, unit3, unit_normal3};

pub mod axis;
mod bounds;
mod frame;
mod matrix3;
mod matrix4;
mod normal;
mod point2;
mod point3;
mod transform;
mod unit;
pub mod utils;
mod vec2;
mod vec3;
mod vec4;

pub trait Number: Debug + Default + Float + NumAssignOps + Pow<f32, Output = Self> {}

impl<T> Number for T where T: Debug + Default + Float + NumAssignOps + Pow<f32, Output = Self> {}

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

// TODO: how it dereferences rhs???
impl<Lhs, Rhs, Ref, Out> Dot<Rhs> for Lhs
where
    Lhs: Deref<Target = Ref>,
    Ref: Dot<Rhs, Output = Out>,
{
    type Output = Out;

    fn dot(&self, rhs: &Rhs) -> Self::Output { self.deref().dot(rhs) }
}

// impl<T, U, Out: Number> Dot<U> for T
// where
//     U: Deref<Target = T>,
//     T: Dot<T, Output = Out>,
// {
//     type Output = Out;
//
//     fn dot(&self, rhs: &U) -> Self::Output { self.dot(rhs) }
// }

pub fn dot<LHS: Dot<RHS>, RHS>(lhs: &LHS, rhs: &RHS) -> LHS::Output { lhs.dot(rhs) }

pub trait Cross<RHS = Self> {
    type Output;
    fn cross(&self, rhs: &RHS) -> Self::Output;
}

pub fn cross<T: Cross>(lhs: &T, rhs: &T) -> T::Output { lhs.cross(rhs) }

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec3;

    #[test]
    fn test_deref() {
        let v = vec3!(1., 2., 3.);
        let nv = normal3!(1., 2., 3.);
        let uv = unit3!(1., 2., 3.);
        let unv = unit_normal3!(1., 2., 3.);

        dot(&v, &v);
        dot(&v, &nv);
        dot(&v, &uv);
        dot(&v, &unv);

        dot(&nv, &nv);
        dot(&nv, &uv);
        dot(&nv, &unv);

        dot(&uv, &uv);
        dot(&uv, &unv);

        dot(&unv, &unv);
    }
}
