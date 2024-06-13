use std::mem::offset_of;

use derive_new::new;
use image::Rgb;
use log::debug;
use num_traits::{Signed, Zero};

use crate::{
    bxdf::{
        bsdf::BSDFSample,
        bxdf::{BxDFFlags, Shading},
        utils::{abs_cos_theta, cos_theta},
        BxDF,
    },
    math::{utils::refract, Unit},
    unit_normal3, unit_normal3_unchecked, vec3, Point2f, SampledSpectrum, Vec3f,
};

#[derive(Debug)]
#[derive(new)]
pub struct DielectricBxDF {
    eta: f32,
}

impl BxDF for DielectricBxDF {
    fn flags(&self) -> BxDFFlags { BxDFFlags::SpecularTransmission | BxDFFlags::SpecularReflection }

    fn eval(&self, incoming: Shading<Vec3f>, outgoing: Shading<Vec3f>) -> SampledSpectrum {
        // TODO: microfacet
        SampledSpectrum::zero()
    }

    fn sample(&self, rnd_p: Point2f, rnd_c: f32, outgoing: Shading<Vec3f>) -> Option<BSDFSample<Shading<Vec3f>>> {
        // TODO: flags, microfacet
        // TODO: or use Schlick's approximation
        let prob_reflected = fresnel_dielectric(cos_theta(outgoing), self.eta);

        if rnd_c < prob_reflected {
            // Sample reflected light
            let incoming: Shading<Vec3f> = vec3!(-outgoing.x, -outgoing.y, outgoing.z).into();
            let c = prob_reflected / abs_cos_theta(incoming);
            let color = SampledSpectrum::from(c);
            Some(BSDFSample {
                spectrum: color,
                incoming,
                pdf: prob_reflected,
                eta: self.eta,
                flags: self.flags(),
            })
        } else {
            // TODO: outgoing should be unit as a Ray.dir. Need to change BSDF.sample param to Unit<> and make Shading
            //       work with other wrappers. Marker trait for Vector wrappers may be useful. Same for BSDFSample

            let prob_transmitted = 1. - prob_reflected;
            // Sample transmitted light
            // Should always be Some(), but float rounding errors exist
            if let Some((incoming, rel_eta)) = refract(
                Unit::from_unchecked(*outgoing),
                unit_normal3_unchecked!(0., 0., 1.),
                self.eta,
            ) {
                let incoming = Shading::from(incoming);
                let c = prob_transmitted / abs_cos_theta(incoming);
                let spectrum = SampledSpectrum::from(c);
                // TODO: [PBRT] Account for non-symmetry with transmission to different medium
                Some(BSDFSample {
                    spectrum,
                    incoming,
                    pdf: prob_transmitted,
                    eta: rel_eta,
                    flags: self.flags(),
                })
            } else {
                None
            }
        }
    }

    fn pdf(&self, incoming: Shading<Vec3f>, outgoing: Shading<Vec3f>) -> f32 {
        // TODO: microfacet
        0.
    }
}

/// Computes the unpolarized Fresnel reflection of a dielectric interface
/// https://graphics.stanford.edu/courses/cs148-10-summer/docs/2006--degreve--reflection_refraction.pdf
fn fresnel_dielectric(mut cos_theta_in: f32, mut eta: f32) -> f32 {
    // Flip in case of leaving the object
    if cos_theta_in.is_negative() {
        eta = eta.recip();
        cos_theta_in = -cos_theta_in
    }

    // Compute cos of transmitted ray
    // In case of total internal reflection return 1 to indicate that all rays are reflected
    let sin_2_theta_in = 1. - cos_theta_in.powi(2);
    let sin_2_theta_tr = sin_2_theta_in / eta.powi(2);
    if sin_2_theta_tr >= 1. {
        return 1.;
    }
    let cos_theta_tr = (1. - sin_2_theta_tr).sqrt();

    let reflection_parallel = (eta * cos_theta_in - cos_theta_tr) / (eta * cos_theta_in + cos_theta_tr);
    let reflection_perpend = (cos_theta_in - eta * cos_theta_tr) / (cos_theta_in + eta * cos_theta_tr);

    (reflection_parallel.powi(2) + reflection_perpend.powi(2)) / 2.
}
