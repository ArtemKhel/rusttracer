use image::Rgb;

use crate::{
    core::Ray,
    material::{Material, Scatter},
    math::{
        utils::{random_unit, reflect},
        Normed,
    },
    scene::Intersection,
};

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    pub albedo: Rgb<f32>,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scattered(&self, ray: &Ray, intersection: &Intersection) -> Option<Scatter> {
        let reflected_direction = (*reflect(&ray.dir, &intersection.hit.normal) + *random_unit() * self.fuzz).to_unit();
        let ray = Ray::new(
            intersection.hit.point + **intersection.hit.normal * 0.01,
            reflected_direction,
        );
        Some(Scatter {
            ray,
            attenuation: self.albedo,
        })
    }
}
