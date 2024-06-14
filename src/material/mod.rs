use std::fmt::Debug;

use bumpalo::Bump;
use image::Rgb;

use crate::{
    bxdf,
    bxdf::{BxDF, BSDF},
    core::{Ray, SurfaceInteraction},
    material::{glass::Glass, matte::Matte, metal::Metal},
    SampledSpectrum, SampledWavelengths,
};

pub mod glass;
pub mod matte;
pub mod metal;

#[enum_delegate::register]
pub trait Material {
    // TODO: useless in current implementation, remove?
    type BxDF;
    fn get_bsdf<'a>(
        &self,
        surf_int: &SurfaceInteraction,
        lambda: &mut SampledWavelengths,
        alloc: &'a mut Bump,
    ) -> BSDF<'a>;
}

#[derive(Debug)]
#[enum_delegate::implement(Material)]
pub enum MaterialsEnum {
    Matte(Matte),
    Metal(Metal),
    Glass(Glass),
}
