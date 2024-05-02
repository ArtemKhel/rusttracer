use geometry::Vec3;
use image::Pixel;
use image::Rgb;
use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::f32::consts::PI;

pub(crate) fn lerp(/*a: Rgb<u8>, b: Rgb<u8>,*/ t: f32) -> Rgb<f32> {
    let a = Rgb([1.0, 1.0, 1.0]);
    let b = Rgb([0.5, 0.7, 1.0]);
    Rgb([
        (1. - t) * a.0[0] + t * b.0[0],
        (1. - t) * a.0[1] + t * b.0[1],
        (1. - t) * a.0[2] + t * b.0[2],
    ])
}
pub(crate) fn linear_to_gamma(linear: Rgb<f32>) -> Rgb<f32> {
    linear.map(|x| if x > 0. { x.sqrt() } else { x })
}

pub fn degrees_to_radians(degrees: f32) -> f32 {
    PI * degrees / 180.0
}
