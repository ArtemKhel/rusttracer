use std::sync::Arc;
use bumpalo::Bump;

use image::Rgb;

use crate::{
    bxdf::{BxDFEnum, ConductorBxDF, DiffuseBxDF, BSDF},
    core::SurfaceInteraction,
    material::Material,
    textures::SpectrumTexture,
    SampledSpectrum, SampledWavelengths,
};

#[derive(Debug)]
pub struct Metal {
    // TODO: either reflectance or eta + k
    pub reflectance: Arc<dyn SpectrumTexture>,
    pub eta: Arc<dyn SpectrumTexture>,
    pub k: Arc<dyn SpectrumTexture>,
}

impl Material for Metal {
    type BxDF = BxDFEnum;

    fn get_bsdf<'a>(
        &self,
        surf_int: &SurfaceInteraction,
        lambda: &mut SampledWavelengths,
        alloc: &'a mut Bump,
    ) -> BSDF<'a> {
        let reflectance = self.reflectance.evaluate(surf_int, lambda);
        let eta = SampledSpectrum::from(1.);
        let k = 2. * reflectance.sqrt() / (SampledSpectrum::from(1.) - reflectance).sqrt();
        // let eta = self.eta.evaluate(surf_int, lambda);
        // let k = self.k.evaluate(surf_int, lambda);
        
        let bxdf = alloc.alloc(BxDFEnum::Conductor(ConductorBxDF::new(eta, k)));
        BSDF::new(**surf_int.hit.normal, surf_int.dp_du, bxdf)
    }
}
