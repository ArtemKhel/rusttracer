use image::Rgb;

use crate::{
    geometry::{utils::random_unit, Ray},
    material::{Material, Scatter},
    scene::Intersection,
};

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    pub albedo: Rgb<f32>,
    pub fuzz: f32,
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<Scatter> {
        let reflected_direction = (ray.dir.reflect(intersection.hit.normal).vec + random_unit() * self.fuzz).to_unit();
        let ray = Ray::new(
            intersection.hit.point + intersection.hit.normal * 0.01,
            reflected_direction,
        );
        Some(Scatter {
            ray,
            attenuation: self.albedo,
        })
    }
}
