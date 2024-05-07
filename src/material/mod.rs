use std::fmt::Debug;
use image::Rgb;

use crate::{geometry::Ray, scene::Intersection};
pub mod dielectric;
pub mod diffuse_light;
pub mod lambertian;
pub mod metal;

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Rgb<f32>,
}

pub trait Material:Debug {
    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<Scatter>;
}
