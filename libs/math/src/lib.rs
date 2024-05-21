#![feature(unboxed_closures, fn_traits)]
#![allow(unused)]
#![feature(stmt_expr_attributes)]

use std::{
    fmt::Debug,
    iter::zip,
    ops::{Add, Deref, Mul, Neg},
    os::fd::IntoRawFd,
};

pub use aabb::Aabb;
use approx::AbsDiffEq;
use derive_where::derive_where;
pub use hit::Hit;
pub use matrix::*;
pub use mesh::Triangle;
use num_traits::{Float, Num, NumAssignOps, Pow, Signed};
pub use point::{Point3, Point3f};
pub use quad::Quad;
pub use ray::Ray;
pub use sphere::Sphere;
pub use unit_vec::{local_normal, reflect, refract, UnitVec3, UnitVec3f};
pub use unit_vec4::*;
pub use vec::{Vec3, Vec3f};
pub use vec4::Vec4;

mod aabb;
mod hit;
mod mesh;
mod point;
mod quad;
mod ray;
mod sphere;
mod unit_vec;
pub mod utils;

mod matrix;
mod matrix4;
mod transform;
mod unit_vec4;
mod vec;
mod vec4;

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
// impl<T:Number> Dot<[T;3]> for [T;3]{
//     type Output = T;
//     fn dot(&self, rhs: &[T;3]) -> Self::Output {zip(self, rhs).reduce(|acc:T,(&x,&y)| acc + x*y)}
// }

pub fn dot<T: Dot<U>, U>(lhs: &T, rhs: &U) -> T::Output { lhs.dot(rhs) }

pub trait Cross<RHS = Self> {
    type Output;
    fn cross(&self, rhs: RHS) -> Self::Output;
}

pub fn cross<T: Cross>(lhs: T, rhs: T) -> T::Output { lhs.cross(rhs) }

pub trait Number: Debug + Float + NumAssignOps + Pow<f32, Output = Self> {}

impl<T> Number for T where T: Debug + Float + NumAssignOps + Pow<f32, Output = Self> {}

pub trait Intersectable<T: Number> {
    fn hit(&self, ray: &Ray<T>) -> Option<Hit<T>>;
}

pub trait Bounded<T: Number> {
    fn bound(&self) -> Aabb<T>;
}

pub trait BoundedIntersectable<T: Number>: Bounded<T> + Intersectable<T> + Debug {}
impl<Shape, T: Number> BoundedIntersectable<T> for Shape where Shape: Bounded<T> + Intersectable<T> + Debug {}
