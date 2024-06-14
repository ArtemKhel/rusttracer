use std::{cmp::PartialEq, marker::PhantomData, sync::Arc};

use bumpalo::Bump;
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

    fn get_bsdf<'a>(
        &self,
        surf_int: &SurfaceInteraction,
        lambda: &mut SampledWavelengths,
        alloc: &'a mut Bump,
    ) -> BSDF<'a> {
        // If IOR depend on wavelength, trace only the first one
        let first_wavelength_ior = self.ior.value(lambda[0]);
        match self.ior {
            SpectrumEnum::Constant(_) => {}
            _ => lambda.terminate_secondary(),
        }

        let bxdf = alloc.alloc(BxDFEnum::Dielectric(DielectricBxDF::new(first_wavelength_ior)));
        BSDF::new(**surf_int.hit.normal, surf_int.dp_du, bxdf)
    }
}
