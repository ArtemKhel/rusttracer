use std::array;
use std::iter::zip;

use derive_new::new;
use ndarray::array;
use num_complex::Complex32;
use num_traits::Zero;

use crate::{bxdf::{
    bsdf::BSDFSample,
    bxdf::{BxDFFlags, Shading},
    BxDF,
    utils::abs_cos_theta,
}, Point2f, SampledSpectrum, vec3, Vec3f};

#[derive(Debug, Copy, Clone)]
#[derive(new)]
pub struct ConductorBxDF {
    eta: SampledSpectrum,
    k: SampledSpectrum,
}

impl BxDF for ConductorBxDF {
    fn flags(&self) -> BxDFFlags { BxDFFlags::SpecularReflection }

    fn eval(&self, incoming: Shading<Vec3f>, outgoing: Shading<Vec3f>) -> SampledSpectrum {
        // if same_hemisphere(incoming, outgoing) {
        //     // TODO: microfacet
        //     colors::BLACK
        // } else {
        //     colors::BLACK
        // }
        SampledSpectrum::zero()
    }

    fn sample(&self, rnd_p: Point2f, rnd_c: f32, outgoing: Shading<Vec3f>) -> Option<BSDFSample<Shading<Vec3f>>> {
        // TODO: flags, microfacet
        let incoming: Shading<Vec3f> = vec3!(-outgoing.x, -outgoing.y, outgoing.z).into();
        let cos_in = abs_cos_theta(incoming);
        let mut spectrum = fresnel_complex_im_re(cos_in, self.eta, self.k) / cos_in;
        Some(BSDFSample::new(spectrum, incoming, 1., self.flags()))
    }

    fn pdf(&self, incoming: Shading<Vec3f>, outgoing: Shading<Vec3f>) -> f32 {
        // TODO: microfacet
        0.
    }
}

/// https://graphics.stanford.edu/courses/cs148-10-summer/docs/2006--degreve--reflection_refraction.pdf
fn fresnel_complex(mut cos_theta_in: f32, eta: Complex32) -> f32 {
    cos_theta_in = cos_theta_in.clamp(0., 1.);
    let sin_2_theta_in = 1. - cos_theta_in.powi(2);
    let sin_2_theta_tr = sin_2_theta_in / eta.powi(2);
    let cos_theta_tr = (1. - sin_2_theta_tr).sqrt();

    let reflection_parallel = (eta * cos_theta_in - cos_theta_tr) / (eta * cos_theta_in + cos_theta_tr);
    let reflection_perpend = (cos_theta_in - eta * cos_theta_tr) / (cos_theta_in + eta * cos_theta_tr);

    (reflection_parallel.norm() + reflection_perpend.norm()) / 2.
}

fn fresnel_complex_im_re(mut cos_theta_in: f32, eta: SampledSpectrum, k: SampledSpectrum) -> SampledSpectrum {
    array::from_fn(|i| fresnel_complex(cos_theta_in, Complex32::new(eta[i], k[i]))).into()
    // eta.map2(&k, |eta_i, k_i| {
    //     fresnel_complex(cos_theta_in, Complex32::new(eta_i, k_i))
    // })
}
