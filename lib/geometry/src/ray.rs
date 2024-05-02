use crate::point::Point;
use crate::unit_vec::UnitVec;
use crate::vec::Vec3;
use std::arch::x86_64::__m128;
use std::path::Path;

pub struct Ray {
    pub origin: Point,
    pub dir: UnitVec,
}

impl Ray {
    pub fn new(origin: Point, dir: UnitVec) -> Ray {
        Ray { origin, dir }
    }

    pub fn from_to(origin: Point, end: Point) -> Ray {
        Ray::new(origin, (end - origin).to_unit())
    }

    pub fn at(&self, t: f32) -> Point {
        self.origin + self.dir * t
    }
}
