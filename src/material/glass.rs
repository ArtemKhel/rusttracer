use std::{cmp::PartialEq, marker::PhantomData, sync::Arc};

use image::Rgb;

use crate::{
    bxdf::{BxDFEnum, ConductorBxDF, DielectricBxDF, DiffuseBxDF, BSDF},
    core::SurfaceInteraction,
    material::Material,
    spectra::{Spectrum, SpectrumEnum},
    textures::SpectrumTexture,
    SampledWavelengths,
};

#[derive(Debug)]
pub struct Glass {
    // TODO: spectrum texture in pbrt
    pub ior: SpectrumEnum,
    // pub roughness: f32,
    pub spectrum: Arc<dyn SpectrumTexture>,
}

impl Material for Glass {
    type BxDF = BxDFEnum;

    fn get_bxdf(&self, surf_int: &SurfaceInteraction, lambda: &mut SampledWavelengths) -> Self::BxDF {
        let first_wavelength_ior = self.ior.value(lambda[0]);
        match self.ior {
            SpectrumEnum::Constant(_) => {}
            _ => lambda.terminate_secondary(),
        }
        BxDFEnum::Dielectric(DielectricBxDF::new(first_wavelength_ior))
    }

    fn get_bsdf(&self, surf_int: &SurfaceInteraction, lambda: &mut SampledWavelengths) -> BSDF {
        // todo: non-constant IOR
        let bxdf = self.get_bxdf(surf_int, lambda);
        BSDF::new(**surf_int.hit.normal, surf_int.dp_du, bxdf)
    }
}
