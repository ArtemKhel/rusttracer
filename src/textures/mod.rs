use std::fmt::Debug;

use crate::{
    core::SurfaceInteraction, textures::constant::ConstantSpectrumTexture, SampledSpectrum, SampledWavelengths,
};

// pub mod checkerboard;
pub mod constant;
pub mod mappings;

#[enum_delegate::register]
pub trait SpectrumTexture {
    fn evaluate(&self, surf_int: &SurfaceInteraction, lambda: &SampledWavelengths) -> SampledSpectrum;
}

#[enum_delegate::implement(SpectrumTexture)]
#[derive(Debug)]
pub enum SpectrumTextureEnum {
    Constant(ConstantSpectrumTexture),
}
