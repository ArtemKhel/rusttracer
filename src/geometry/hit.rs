use std::cmp::Ordering;

use derive_new::new;

use crate::geometry::{Dot, Point, Ray, UnitVec};

#[derive(Debug, Clone, Copy, PartialEq, new)]
pub struct Hit {
    pub point: Point,
    pub normal: UnitVec,
    pub t: f32,
}

impl Hit {
    pub fn on_front_side(&self, ray: &Ray) -> bool { self.normal.dot(ray.dir) < 0. }
}

impl Eq for Hit {}

impl PartialOrd for Hit {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}
impl Ord for Hit {
    fn cmp(&self, other: &Self) -> Ordering { self.t.partial_cmp(&other.t).unwrap() }
}
