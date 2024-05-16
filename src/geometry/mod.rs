#![allow(unused)]

use std::{
    fmt::Debug,
    ops::{Add, Mul, Neg},
    os::fd::IntoRawFd,
};

pub use aabb::Aabb;
pub use hit::Hit;
pub use mesh::Triangle;
pub use point::Point;
pub use quad::Quad;
pub use ray::Ray;
pub use sphere::Sphere;
pub use unit_vec::UnitVec;
pub use vec::Vec3;

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
    fn bound(&self) -> Aabb;
}

pub trait BoundedIntersectable: Bounded + Intersectable + Debug {}

impl<T> BoundedIntersectable for T where T: Bounded + Intersectable + Debug {}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}
