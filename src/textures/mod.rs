use std::fmt::Debug;

use crate::{core::SurfaceInteraction, SampledSpectrum, SampledWavelengths};

// pub mod checkerboard;
pub mod constant;
pub mod mappings;

pub trait SpectrumTexture: Debug {
    fn evaluate(&self, surf_int: &SurfaceInteraction, lambda: &SampledWavelengths) -> SampledSpectrum;
}
