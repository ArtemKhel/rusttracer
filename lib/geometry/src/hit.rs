use crate::point::Point;
use crate::ray::Ray;
use crate::unit_vec::UnitVec;
use crate::vec::Vec3;
use crate::Dot;
use std::cmp::Ordering;

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Hit {
    pub point: Point,
    pub normal: UnitVec,
    pub t: f32,
}

impl Hit {
    pub fn new(point: Point, normal: UnitVec, t: f32) -> Hit {
        Hit { point, normal, t }
    }

    pub fn on_front_side(&self, ray: &Ray) -> bool {
        self.normal.dot(ray.dir) < 0.
    }
}

impl Eq for Hit {}
impl Ord for Hit {
    fn cmp(&self, other: &Self) -> Ordering {
        self.t.partial_cmp(&other.t).unwrap()
    }
}
