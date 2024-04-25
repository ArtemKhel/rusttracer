use std::time::Duration;
use crate::point::Point;

#[derive(Default, Debug, Clone, Copy)]
struct Sphere {
    center: Point,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Point, radius: f32) -> Sphere {
        Sphere { center, radius }
    }
}
