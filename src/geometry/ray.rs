use std::{arch::x86_64::__m128, path::Path};

use crate::geometry::{point::Point, unit_vec::UnitVec, vec::Vec3};

pub struct Ray {
    pub origin: Point,
    pub dir: UnitVec,
}

impl Ray {
    pub fn new(origin: Point, dir: UnitVec) -> Ray { Ray { origin, dir } }

    pub fn from_to(origin: Point, end: Point) -> Ray { Ray::new(origin, (end - origin).to_unit()) }

    pub fn at(&self, t: f32) -> Point { self.origin + self.dir * t }
}
