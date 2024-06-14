use std::sync::Arc;

use bumpalo::Bump;
use either::Either;

use crate::{
    bxdf::{BxDFEnum, ConductorBxDF, BSDF},
    core::SurfaceInteraction,
    material::Material,
    textures::{SpectrumTexture, SpectrumTextureEnum},
    Pair, SampledSpectrum, SampledWavelengths,
};

#[derive(Debug)]
pub struct Metal {
    pub reflectance: Either<Arc<SpectrumTextureEnum>, Pair<Arc<SpectrumTextureEnum>>>,
}

impl Material for Metal {
    type BxDF = BxDFEnum;

    fn get_bsdf<'a>(
        &self,
        surf_int: &SurfaceInteraction,
        lambda: &mut SampledWavelengths,
        alloc: &'a mut Bump,
    ) -> BSDF<'a> {
        let (eta, k) = match &self.reflectance {
            Either::Left(reflectance) => {
                let reflectance = reflectance.evaluate(surf_int, lambda);
                let eta = SampledSpectrum::from(1.);
                let k = 2. * reflectance.sqrt() / (SampledSpectrum::from(1.) - reflectance).sqrt();
                (eta, k)
            }
            Either::Right((eta, k)) => {
                unimplemented!()
            }
        };

        let bxdf = alloc.alloc(BxDFEnum::Conductor(ConductorBxDF::new(eta, k)));
        BSDF::new(**surf_int.hit.normal, surf_int.dp_du, bxdf)
    }
}
