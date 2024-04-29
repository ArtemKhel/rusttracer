#![allow(unused)]

use crate::hit::Hit;
use crate::ray::Ray;
use crate::vec::Vec3;
use std::ops::{Add, Mul, Neg};

pub mod hit;
pub mod point;
pub mod ray;
pub mod sphere;
pub mod unit_vec;
pub mod utils;
pub mod vec;

pub trait Dot<T> {
    fn dot(&self, rhs: T) -> f32;
}

pub trait Cross<T> {
    fn cross(&self, rhs: T) -> Vec3;
}

pub trait Intersect {
    fn hit(&self, ray: &Ray) -> Option<Hit>;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}
