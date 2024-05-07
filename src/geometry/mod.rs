#![allow(unused)]

use std::{
    ops::{Add, Mul, Neg},
    os::fd::IntoRawFd,
};
use std::fmt::Debug;

pub use aabb::AABB;
pub use bvh::BVH;
pub use hit::Hit;
pub use point::Point;
pub use ray::Ray;
pub use sphere::Sphere;
pub use unit_vec::UnitVec;
pub use vec::Vec3;

mod hit;

mod point;

mod ray;

mod sphere;

mod unit_vec;

mod aabb;
mod bvh;
pub mod utils;
mod vec;

pub trait Dot<T> {
    fn dot(&self, rhs: T) -> f32;
}

pub trait Cross<T> {
    fn cross(&self, rhs: T) -> Vec3;
}

pub trait Intersectable {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
}

pub trait Bounded {
    fn bound(&self) -> AABB;
}

pub trait BoundedIntersectable: Bounded + Intersectable + Debug{}
impl<T> BoundedIntersectable for T where T: Bounded + Intersectable + Debug {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}
