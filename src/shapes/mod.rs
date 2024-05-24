use std::fmt::Debug;

use crate::{
    aggregates::Aabb,
    core::{Hit, Ray},
    math::{Number, Point3},
};
pub mod mesh;
pub mod quad;
pub mod sphere;

pub trait Intersectable<T: Number> {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
}

pub trait Bounded<T: Number> {
    fn bound(&self) -> Aabb<T>;
}

pub trait BoundedIntersectable<T: Number>: Bounded<T> + Intersectable<T> + Debug {}

impl<Shape, T: Number> BoundedIntersectable<T> for Shape where Shape: Bounded<T> + Intersectable<T> + Debug {}

// impl<T: Number> Bounded<T> for Point3<T> {
//     fn bound(&self) -> Aabb<T> { Aabb { min: *self, max: *self } }
// }
