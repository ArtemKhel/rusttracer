use image::Rgb;
use math::utils::random_unit;

use crate::{
    material::{Material, Scatter},
    scene::Intersection,
    Ray,
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
