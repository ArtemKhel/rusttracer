#![allow(unused)]

use std::{
    fmt::Debug,
    ops::{Add, Mul, Neg},
    os::fd::IntoRawFd,
};

pub use aabb::Aabb;
pub use hit::Hit;
use log::trace;
pub use mesh::Triangle;
use num_traits::{Float, Num, NumAssignOps, Pow, Signed};
pub use point::{Point3, Point3f};
pub use quad::Quad;
pub use ray::Ray;
pub use sphere::Sphere;
pub use unit_vec::{local_normal, reflect, refract, UnitVec3, UnitVec3f};
pub use vec::{Vec3, Vec3f};

mod aabb;
mod hit;
mod mesh;
mod point;
mod quad;
mod ray;
mod sphere;
mod unit_vec;
pub mod utils;

mod vec;

pub trait Dot<RHS = Self> {
    type Output;
    fn dot(&self, rhs: &RHS) -> Self::Output;
}

pub fn dot<T: Dot>(lhs: &T, rhs: &T) -> T::Output { lhs.dot(rhs) }

pub trait Cross<RHS = Self> {
    type Output;
    fn cross(&self, rhs: RHS) -> Self::Output;
}

pub fn cross<T: Cross>(lhs: T, rhs: T) -> T::Output { lhs.cross(rhs) }

pub trait Number: Debug + Copy + Float + NumAssignOps + Neg + Pow<f32, Output = Self> {}

impl<T> Number for T where T: Debug + Copy + Float + NumAssignOps + Neg + Pow<f32, Output = Self> {}

pub trait Shape {}
pub trait Intersectable<T: Number> {
    fn hit(&self, ray: &Ray<T>) -> Option<Hit<T>>;
}

pub trait Bounded<T: Number> {
    fn bound(&self) -> Aabb<T>;
}

pub trait BoundedIntersectable<T: Number>: Bounded<T> + Intersectable<T> + Debug {}
impl<Shape, T: Number> BoundedIntersectable<T> for Shape where Shape: Bounded<T> + Intersectable<T> + Debug {}
