use image::Rgb;
use math::{utils::random_unit, Normed};

use crate::{
    material::{Material, Scatter},
    scene::Intersection,
    Ray,
};

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    pub albedo: Rgb<f32>,
}

impl Material for Lambertian {
    fn scattered(&self, ray: &Ray, intersection: &Intersection) -> Option<Scatter> {
        let scattered_direction = (**intersection.hit.normal + *random_unit()).to_unit();
        let ray = Ray::new(
            intersection.hit.point + **intersection.hit.normal * 0.01,
            scattered_direction,
        );
        Some(Scatter {
            ray,
            attenuation: self.albedo,
        })
    }
}
