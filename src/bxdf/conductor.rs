use derive_new::new;
use image::{Pixel, Rgb};
use num_complex::Complex32;

use crate::{
    bxdf::{
        bsdf::BSDFSample,
        bxdf::{BxDFType, Shading},
        BxDF,
        utils::abs_cos_theta,
    },
    colors, Point2f, vec3, Vec3f,
};

#[derive(Debug, Copy, Clone)]
#[derive(new)]
pub struct ConductorBxDF {
    eta: Rgb<f32>,
    k: Rgb<f32>,
}

impl BxDF for ConductorBxDF {
    fn bxdf_type(&self) -> BxDFType { BxDFType::SpecularReflection }

    fn eval(&self, incoming: Shading<Vec3f>, outgoing: Shading<Vec3f>) -> Rgb<f32> {
        // if same_hemisphere(incoming, outgoing) {
        //     // TODO: microfacet
        //     colors::BLACK
        // } else {
        //     colors::BLACK
        // }
        colors::BLACK
    }

    fn sample(&self, sample_p: Point2f, sample_c: f32, outgoing: Shading<Vec3f>) -> Option<BSDFSample<Shading<Vec3f>>> {
        // TODO: flags, microfacet
        let incoming: Shading<Vec3f> = vec3!(-outgoing.x, -outgoing.y, outgoing.z).into();
        let mut color = fresnel_complex_im_re(abs_cos_theta(incoming), self.eta, self.k);
        color.apply(|x| x / abs_cos_theta(incoming));
        Some(BSDFSample::new(color, incoming, 1.))
    }

    fn pdf(&self, incoming: Shading<Vec3f>, outgoing: Shading<Vec3f>) -> f32 {
        // TODO: microfacet
        0.
    }
}

fn fresnel_complex(mut cos_theta_in: f32, eta: Complex32) -> f32 {
    cos_theta_in = cos_theta_in.clamp(0., 1.);
    let sin_2_theta_in = 1. - cos_theta_in.powi(2);
    let sin_2_theta_tr = sin_2_theta_in / eta.powi(2);
    let cos_theta_tr = (1. - sin_2_theta_tr).sqrt();

    let reflection_parallel = (eta * cos_theta_in - cos_theta_tr) / (eta * cos_theta_in + cos_theta_tr);
    let reflection_perpend = (cos_theta_in - eta * cos_theta_tr) / (cos_theta_in + eta * cos_theta_tr);

    (reflection_parallel.norm() + reflection_perpend.norm()) / 2.
}

fn fresnel_complex_im_re(mut cos_theta_in: f32, eta: Rgb<f32>, k: Rgb<f32>) -> Rgb<f32> {
    eta.map2(&k, |eta_i, k_i| {
        fresnel_complex(cos_theta_in, Complex32::new(eta_i, k_i))
    })
}
