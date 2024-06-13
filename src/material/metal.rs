use std::sync::Arc;

use image::Rgb;

use crate::{bxdf::{BxDFEnum, ConductorBxDF, DiffuseBxDF, BSDF}, core::SurfaceInteraction, material::Material, SampledSpectrum, SampledWavelengths};
use crate::textures::SpectrumTexture;

#[derive(Debug)]
pub struct Metal {
    // TODO: either reflectance or eta + k
    pub reflectance: Arc<dyn SpectrumTexture>,
    pub eta: Arc<dyn SpectrumTexture>,
    pub k: Arc<dyn SpectrumTexture>,
}

impl Material for Metal {
    type BxDF = BxDFEnum;

    fn get_bxdf(&self, surf_int: &SurfaceInteraction, lambda: &SampledWavelengths) -> Self::BxDF {
        let reflectance = self.reflectance.evaluate(surf_int,lambda);
        let eta = SampledSpectrum::from(1.);
        let k = 2. * reflectance.sqrt() / (SampledSpectrum::from(1.) - reflectance).sqrt();
        // let eta = self.eta.evaluate(surf_int, lambda);
        // let k = self.k.evaluate(surf_int, lambda);
        BxDFEnum::Conductor(ConductorBxDF::new(eta, k))
    }

    fn get_bsdf(&self, surf_int: &SurfaceInteraction, lambda: &SampledWavelengths) -> BSDF {
            let bxdf = self.get_bxdf(surf_int, lambda);
            // todo: box
            BSDF::new(**surf_int.hit.normal, surf_int.dp_du, Box::new(bxdf))
    }
}
