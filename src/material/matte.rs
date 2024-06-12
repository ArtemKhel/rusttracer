use std::sync::Arc;

use image::{Pixel, Rgb};

use crate::{
    bxdf::{BxDFEnum, DiffuseBxDF, BSDF},
    core::SurfaceInteraction,
    material::Material,
    textures::SpectrumTexture,
    SampledSpectrum, SampledWavelengths,
};

#[derive(Debug)]
pub struct Matte {
    // todo: dyn
    pub reflectance: Arc<dyn SpectrumTexture>,
}

// TODO: rgb for now, will need refactoring for spectrum
impl Material for Matte {
    type BxDF = BxDFEnum;

    fn get_bxdf(&self, surf_int: &SurfaceInteraction, lambda: &SampledWavelengths) -> <Matte as Material>::BxDF {
        let reflectance = self.reflectance.evaluate(surf_int, lambda).clamp(0., 1.);
        BxDFEnum::Diffuse(DiffuseBxDF::new(reflectance))
    }

    fn get_bsdf(&self, surf_int: &SurfaceInteraction, lambda: &SampledWavelengths) -> BSDF {
        let bxdf = self.get_bxdf(surf_int, lambda);
        // todo: box
        BSDF::new(**surf_int.hit.normal, surf_int.dp_du, Box::new(bxdf))
    }
}
