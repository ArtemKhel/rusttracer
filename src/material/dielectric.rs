use image::Rgb;
use math::{Dot, *};
use rand::{random, Rng};

use crate::{
    material::{Material, Scatter},
    scene::Intersection,
    Ray,
};

#[derive(Debug, Clone, Copy)]
pub struct Dielectric {
    pub attenuation: Rgb<f32>,
    pub refraction_index: f32,
}

impl Dielectric {
    fn reflectance(cos: f32, refract_coef: f32) -> f32 {
        let r0 = ((1. - refract_coef) / (1. + refract_coef)).powi(2);
        r0 + (1. - r0) * (1. - cos).powi(5)
    }
}

impl Material for Dielectric {
    fn scattered(&self, ray: &Ray, intersection: &Intersection) -> Option<Scatter> {
        let on_front = intersection.hit.on_front_side(ray);
        let refract_coef = if on_front {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let cos_theta = ray.dir.dot(&intersection.hit.normal);
        let sin_theta = f32::sqrt(1.0 - cos_theta.powi(2));

        let refracted =
            refract_coef * sin_theta < 1.0 && Self::reflectance(f32::abs(cos_theta), refract_coef) < random::<f32>();

        let direction = if refracted {
            refract(&ray.dir, &intersection.hit.normal, refract_coef)
        } else {
            reflect(&ray.dir, &intersection.hit.normal)
        };

        Some(Scatter {
            ray: Ray::new(
                intersection.hit.point
                    + if on_front ^ refracted {
                        *intersection.hit.normal * 0.01
                    } else {
                        *intersection.hit.normal * -0.01
                    },
                direction,
            ),
            attenuation: self.attenuation,
        })
    }
}
