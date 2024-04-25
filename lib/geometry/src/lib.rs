#![allow(unused)]

use crate::vec::Vec3;
use std::ops::{Add, Mul, Neg};

pub mod point;
pub mod ray;
pub mod unit_vec;
pub mod vec;
mod sphere;

trait Dot {
    fn cross(&self, rhs: Vec3) -> f32;
}

trait Cross {
    fn cross(&self, rhs: Vec3) -> Vec3;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(true, true);
    }
}
