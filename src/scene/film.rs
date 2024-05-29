use image::Rgb;

use crate::Point2u;

pub trait Film {
    fn add_sample(&mut self, coord: Point2u, color: Rgb<f32>);
    fn sample_bounds(&self);
}

pub type Resolution = Point2u;
#[derive(Debug)]
pub struct BaseFilm {
    pub resolution: Resolution,
}
