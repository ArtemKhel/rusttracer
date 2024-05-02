use crate::intersection::Intersection;
use crate::material::{Material, Scatter};
use geometry::ray::Ray;
use geometry::utils::random_unit;
use image::Rgb;

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    pub albedo: Rgb<f32>,
}

impl Material for Lambertian {
    fn scatter(&self, ray: &Ray, intersection: &Intersection) -> Option<Scatter> {
        let scattered_direction = (intersection.hit.normal.vec + random_unit().vec).into();
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
