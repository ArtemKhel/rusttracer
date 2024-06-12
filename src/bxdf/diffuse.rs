use std::f32::consts::FRAC_1_PI;

use derive_new::new;
use image::Rgb;
use num_traits::Zero;

use crate::{
    bxdf::{
        bsdf::BSDFSample,
        bxdf::{BxDF, BxDFFlags, Shading},
        utils::{abs_cos_theta, cosine_hemisphere_pdf, same_hemisphere, sample_cosine_hemisphere},
    },
    Point2f, SampledSpectrum, Vec3f,
};

#[derive(Debug, Clone)]
#[derive(new)]
pub struct DiffuseBxDF {
    reflectance: SampledSpectrum,
}

impl BxDF for DiffuseBxDF {
    fn flags(&self) -> BxDFFlags { BxDFFlags::Diffuse | BxDFFlags::Reflection }

    fn eval(&self, incoming: Shading<Vec3f>, outgoing: Shading<Vec3f>) -> SampledSpectrum {
        if same_hemisphere(incoming, outgoing) {
            self.reflectance * FRAC_1_PI
        } else {
            SampledSpectrum::zero()
        }
    }

    fn sample(&self, rnd_p: Point2f, rnd_c: f32, outgoing: Shading<Vec3f>) -> Option<BSDFSample<Shading<Vec3f>>> {
        // TODO: flags
        let mut incoming = sample_cosine_hemisphere(rnd_p);
        incoming.z *= outgoing.z.signum();
        let pdf = cosine_hemisphere_pdf(abs_cos_theta(incoming));
        Some(BSDFSample::new(
            self.reflectance * FRAC_1_PI,
            incoming,
            pdf,
            self.flags(),
        ))
    }

    fn pdf(&self, incoming: Shading<Vec3f>, outgoing: Shading<Vec3f>) -> f32 {
        if same_hemisphere(incoming, outgoing) {
            cosine_hemisphere_pdf(abs_cos_theta(incoming))
        } else {
            0.0
        }
    }
}
