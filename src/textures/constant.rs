use std::{fmt::Debug, sync::Arc};

use crate::{
    core::SurfaceInteraction,
    spectra::{Spectrum, SpectrumEnum},
    textures::SpectrumTexture,
    SampledSpectrum, SampledWavelengths,
};

#[derive(Debug)]
pub struct ConstantSpectrumTexture {
    pub value: Arc<SpectrumEnum>,
}

impl SpectrumTexture for ConstantSpectrumTexture {
    fn evaluate(&self, surf_int: &SurfaceInteraction, lambda: &SampledWavelengths) -> SampledSpectrum {
        self.value.sample(lambda)
    }
}
