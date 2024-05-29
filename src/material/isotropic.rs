use image::Rgb;

use crate::{
    core::Ray,
    material::{Material, Scatter},
    math::utils::random_unit,
    scene::Intersection,
};

#[derive(Debug, Clone, Copy)]
pub struct Isotropic {
    pub color: Rgb<f32>,
}

impl Material for Isotropic {
    fn scattered(&self, ray: &Ray, intersection: &Intersection) -> Option<Scatter> {
        Some(Scatter {
            ray: Ray::new(intersection.hit.point, random_unit(), None),
            attenuation: self.color,
        })
    }
}
