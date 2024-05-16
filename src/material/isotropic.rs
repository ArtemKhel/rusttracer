use image::Rgb;

use crate::{
    geometry::{utils::random_unit, Ray},
    material::{Material, Scatter},
    scene::Intersection,
};

#[derive(Debug, Clone, Copy)]
pub struct Isotropic {
    pub color: Rgb<f32>,
}

impl Material for Isotropic {
    fn scattered(&self, ray: &Ray, intersection: &Intersection) -> Option<Scatter> {
        Some(Scatter {
            ray: Ray::new(intersection.hit.point, random_unit()),
            attenuation: self.color,
        })
    }
}
