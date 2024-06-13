use std::{marker::PhantomData, sync::Arc};

use image::Rgb;

use crate::{bxdf::{BxDFEnum, ConductorBxDF, DielectricBxDF, DiffuseBxDF, BSDF}, core::SurfaceInteraction, material::Material, SampledWavelengths};
use crate::textures::SpectrumTexture;

#[derive(Debug)]
pub struct Glass {
    // TODO: spectrum texture in pbrt
    pub ior: f32,
    // pub roughness: f32,
    pub spectrum: Arc<dyn SpectrumTexture>,
}

impl Material for Glass {
    type BxDF = BxDFEnum;

    fn get_bxdf(&self, surf_int: &SurfaceInteraction, lambda: &SampledWavelengths) -> Self::BxDF {
        BxDFEnum::Dielectric(DielectricBxDF::new(self.ior))
    }

    fn get_bsdf(&self, surf_int: &SurfaceInteraction, lambda: &SampledWavelengths) -> BSDF {
        // todo: non-constant IOR
        let bxdf = self.get_bxdf(surf_int,lambda);
        // todo: box
        BSDF::new(**surf_int.hit.normal, surf_int.dp_du, Box::new(bxdf))
    }
}
