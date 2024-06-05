use std::f32::consts::FRAC_1_PI;

use derive_new::new;
use image::{Pixel, Rgb};

use crate::{
    bxdf::{
        bsdf::BSDFSample,
        bxdf::{BxDF, BxDFType, Shading},
        utils::{abs_cos_theta, cosine_hemisphere_pdf, same_hemisphere, sample_cosine_hemisphere},
    },
    colors, Point2f, Vec3f,
};

#[derive(Debug, Copy, Clone)]
#[derive(new)]
pub struct DiffuseBxDF {
    reflectance: Rgb<f32>,
}

impl BxDF for DiffuseBxDF {
    fn bxdf_type(&self) -> BxDFType { BxDFType::Diffuse | BxDFType::Reflection }

    fn eval(&self, incoming: Shading<Vec3f>, outgoing: Shading<Vec3f>) -> Rgb<f32> {
        if same_hemisphere(incoming, outgoing) {
            self.reflectance.map(|x| x * FRAC_1_PI)
        } else {
            colors::BLACK
        }
    }

    fn sample(&self, sample_p: Point2f, sample_c: f32, outgoing: Shading<Vec3f>) -> Option<BSDFSample<Shading<Vec3f>>> {
        // TODO: flags
        let mut incoming = sample_cosine_hemisphere(sample_p);
        incoming.z *= outgoing.z.signum();
        let pdf = cosine_hemisphere_pdf(abs_cos_theta(incoming));
        Some(BSDFSample::new(self.reflectance.map(|x| x * FRAC_1_PI), incoming, pdf))
    }

    fn pdf(&self, incoming: Shading<Vec3f>, outgoing: Shading<Vec3f>) -> f32 {
        if same_hemisphere(incoming, outgoing) {
            cosine_hemisphere_pdf(abs_cos_theta(incoming))
        } else {
            0.0
        }
    }
}
