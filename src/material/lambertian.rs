use image::Rgb;

use crate::{
    geometry::{utils::random_unit, Ray},
    material::{Material, Scatter},
    scene::Intersection,
};

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    pub albedo: Rgb<f32>,
}

impl Material for Lambertian {
    fn scattered(&self, ray: &Ray, intersection: &Intersection) -> Option<Scatter> {
        let scattered_direction = (intersection.hit.normal.vec + random_unit().vec).to_unit();
        let ray = Ray::new(
            intersection.hit.point + intersection.hit.normal * 0.01,
            scattered_direction,
        );
        Some(Scatter {
            ray,
            attenuation: self.albedo,
        })
    }
}
