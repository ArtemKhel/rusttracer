use crate::intersection::Intersection;
use geometry::ray::Ray;
use image::Rgb;
pub mod dielectric;
pub mod lambertian;
pub mod metal;

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Rgb<f32>,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<Scatter>;
}
