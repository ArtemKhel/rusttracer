use derive_new::new;
use image::Rgb;
use num_traits::Zero;

use crate::{
    bxdf::{
        bxdf::{BxDF, BxDFSample, BxDFType, Shading},
        utils::{abs_cos_theta, cosine_hemisphere_pdf, same_hemisphere, sample_cosine_hemisphere},
    },
    colors, Point2f, Vec3f,
};

#[derive(Debug, Copy, Clone)]
#[derive(new)]
struct DiffuseBxDF {
    color: Rgb<f32>,
}

impl BxDF for DiffuseBxDF {
    fn bxdf_type(&self) -> BxDFType { BxDFType::Diffuse | BxDFType::Reflection }

    fn eval(&self, incoming: Shading<Vec3f>, outgoing: Shading<Vec3f>) -> Rgb<f32> {
        if same_hemisphere(incoming, outgoing) {
            self.color //TODO: * FRAC_1_PI
        } else {
            colors::BLACK
        }
    }

    fn sample(&self, point: Point2f, outgoing: Shading<Vec3f>) -> Option<BxDFSample> {
        // TODO: flags
        let mut incoming = sample_cosine_hemisphere(point);
        incoming.z *= outgoing.z.signum();
        let pdf = cosine_hemisphere_pdf(abs_cos_theta(incoming));
        Some(BxDFSample {
            color: self.color, //TODO: * FRAC_1_PI
            incoming,
            pdf,
        })
    }

    fn pdf(&self, incoming: Shading<Vec3f>, outgoing: Shading<Vec3f>) -> f32 {
        if same_hemisphere(incoming, outgoing) {
            cosine_hemisphere_pdf(abs_cos_theta(incoming))
        } else {
            0.0
        }
    }
}
