use crate::point::Point;
use crate::unit_vec::UnitVec;

pub struct Hit {
    pub point: Point,
    pub normal: UnitVec,
    pub t: f32,
}

impl Hit {
    pub fn new(point: Point, normal: UnitVec, t: f32) -> Hit {
        Hit { point, normal, t }
    }
}
