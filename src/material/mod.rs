use std::fmt::Debug;

use image::Rgb;

// use crate::material::glass::Glass;
use crate::material::matte::Matte;
use crate::{
    bxdf,
    bxdf::{BxDF, BSDF},
    core::{Ray, SurfaceInteraction},
    SampledSpectrum, SampledWavelengths,
};
// use crate::material::metal::Metal;

pub mod matte;
// pub mod glass;
// pub mod metal;

#[enum_delegate::register]
pub trait Material {
    // #[enum_delegate(unify = "enum_wrap")]
    // todo: remove?
    type BxDF;
    fn get_bxdf(&self, surf_int: &SurfaceInteraction, lambda: &SampledWavelengths) -> Self::BxDF;
    fn get_bsdf(&self, surf_int: &SurfaceInteraction, lambda: &SampledWavelengths) -> BSDF;
}

#[derive(Debug)]
#[enum_delegate::implement(Material)]
pub enum MaterialsEnum {
    Matte(Matte),
    // Metal(Metal),
    // Glass(Glass),
}
