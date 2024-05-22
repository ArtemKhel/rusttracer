use std::fmt::Debug;

use image::Rgb;

use crate::{scene::Intersection, Ray};

pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Rgb<f32>,
}

pub trait Material: Debug {
    fn scattered(&self, ray: &Ray, intersection: &Intersection) -> Option<Scatter> { None }

    fn emitted(&self) -> Option<Rgb<f32>> { None }
}
