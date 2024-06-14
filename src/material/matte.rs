use std::sync::Arc;

use bumpalo::Bump;
use image::{Pixel, Rgb};

use crate::{
    bxdf::{BxDFEnum, DiffuseBxDF, BSDF},
    core::SurfaceInteraction,
    material::Material,
    textures::{SpectrumTexture, SpectrumTextureEnum},
    SampledSpectrum, SampledWavelengths,
};

#[derive(Debug)]
pub struct Matte {
    pub reflectance: Arc<SpectrumTextureEnum>,
}

impl Material for Matte {
    type BxDF = BxDFEnum;

    fn get_bsdf<'a>(
        &self,
        surf_int: &SurfaceInteraction,
        lambda: &mut SampledWavelengths,
        alloc: &'a mut Bump,
    ) -> BSDF<'a> {
        let bxdf: &mut BxDFEnum =
            alloc.alloc(DiffuseBxDF::new(self.reflectance.evaluate(surf_int, lambda).clamp(0., 1.)).into());
        BSDF::new(**surf_int.hit.normal, surf_int.dp_du, bxdf)
    }
}
