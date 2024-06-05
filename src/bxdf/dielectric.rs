use std::mem::offset_of;

use image::Rgb;
use num_traits::Signed;

use crate::{bxdf::{
    bsdf::BSDFSample,
    bxdf::{BxDFType, Shading},
    utils::{abs_cos_theta, cos_theta},
    BxDF,
}, vec3, Point2f, Vec3f, unit_normal3, unit_normal3_unchecked, colors};
use crate::math::Unit;
use crate::math::utils::refract;

struct DielectricBxDF {
    eta: f32,
}

impl BxDF for DielectricBxDF {
    fn bxdf_type(&self) -> BxDFType { todo!() }

    fn eval(&self, incoming: Shading<Vec3f>, outgoing: Shading<Vec3f>) -> Rgb<f32> {
        // TODO: microfacet
        colors::BLACK
    }

    fn sample(&self, sample_p: Point2f, sample_c: f32, outgoing: Shading<Vec3f>) -> Option<BSDFSample<Shading<Vec3f>>> {
        // TODO: flags, microfacet
        // TODO: or use Schlick's approximation
        let reflected = fresnel_dielectric(cos_theta(outgoing), self.eta);
        let transmitted = 1. - reflected;

        let prob_reflected = reflected / (reflected + transmitted);
        if sample_c < prob_reflected {
            // Sample reflected light
            let incoming: Shading<Vec3f> = vec3!(-outgoing.x, -outgoing.y, outgoing.z).into();
            let c = reflected / abs_cos_theta(incoming);
            let color = Rgb([c, c, c]);
            Some(BSDFSample {
                color,
                incoming,
                pdf: prob_reflected,
                eta: self.eta
            })
        } else {
            // Sample transmitted light
            // TODO: outgoing should be unit as a Ray.dir. Need to change BSDF.sample param to Unit<> and make Shading work with other wrappers. Vector wrapper marker trait may be useful
            // Should always be Some(), but float rounding errors exist
            if let Some((incoming, rel_eta)) = refract(Unit::from_unchecked(*outgoing), unit_normal3_unchecked!(0.,0.,1.), self.eta){
                let incoming = Shading::from(incoming);
                let c = transmitted / abs_cos_theta(incoming);
                let color = Rgb([c, c, c]);
                // TODO: [PBRT] Account for non-symmetry with transmission to different medium
                Some(BSDFSample{
                    color,
                    incoming,
                    pdf: 1. - prob_reflected,
                    eta: rel_eta
                })
            }else{
                None
            }
        }
    }

    fn pdf(&self, incoming: Shading<Vec3f>, outgoing: Shading<Vec3f>) -> f32 {
        // TODO: microfacet
        0.
    }
}

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
