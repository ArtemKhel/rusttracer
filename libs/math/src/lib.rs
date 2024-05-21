#![feature(unboxed_closures, fn_traits)]
#![allow(unused)]
#![feature(stmt_expr_attributes)]

use std::{
    fmt::Debug,
    ops::{Add, Deref, Mul, Neg},
    os::fd::IntoRawFd,
};

pub use aabb::Aabb;
use approx::AbsDiffEq;
pub use hit::Hit;
pub use matrix::*;
pub use mesh::Triangle;
pub use normal::Normal3;
use num_traits::{Float, Num, NumAssignOps, Pow, Signed};
pub use point::{Point3, Point3f};
pub use quad::Quad;
pub use ray::Ray;
pub use sphere::Sphere;
pub use unit::Unit;
pub use vec::{Vec3, Vec3f};
pub use vec4::Vec4;

mod aabb;
mod hit;
mod matrix;
mod matrix4;
mod mesh;
mod normal;
mod point;
mod quad;
mod ray;
mod sphere;
mod transform;
mod unit;
pub mod utils;
mod vec;
mod vec4;

pub trait Number: Debug + Float + NumAssignOps + Pow<f32, Output = Self> {}
impl<T> Number for T where T: Debug + Float + NumAssignOps + Pow<f32, Output = Self> {}

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

    fn to_unit(self) -> Unit<Self> { self.deref().to_unit().lift() }

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

pub trait Intersectable<T: Number> {
    fn hit(&self, ray: &Ray<T>) -> Option<Hit<T>>;
}

pub trait Bounded<T: Number> {
    fn bound(&self) -> Aabb<T>;
}

pub trait BoundedIntersectable<T: Number>: Bounded<T> + Intersectable<T> + Debug {}

impl<Shape, T: Number> BoundedIntersectable<T> for Shape where Shape: Bounded<T> + Intersectable<T> + Debug {}
