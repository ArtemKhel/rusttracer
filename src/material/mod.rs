use geometry::Ray;
use image::Rgb;

use crate::scene::Intersection;
pub mod dielectric;
pub mod diffuse_light;
pub mod lambertian;
pub mod metal;

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Rgb<f32>,
}

pub trait Material {
    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<Scatter>;
}
