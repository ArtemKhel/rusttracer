use std::fmt::Debug;

use image::Rgb;

use crate::{
    bxdf,
    bxdf::{BxDF, BSDF},
    core::{Ray, SurfaceInteraction},
    material::{glass::Glass, matte::Matte, metal::Metal},
};

pub mod glass;
pub mod matte;
pub mod metal;

#[enum_delegate::register]
pub trait Material {
    // #[enum_delegate(unify = "enum_wrap")]
    // todo: remove?
    type BxDF;
    fn get_bxdf(&self, surf_int: &SurfaceInteraction) -> Self::BxDF;
    fn get_bsdf(&self, surf_int: &SurfaceInteraction) -> BSDF;
}

#[derive(Debug)]
#[enum_delegate::implement(Material)]
pub enum MaterialsEnum {
    Matte(Matte<Rgb<f32>>),
    Metal(Metal<Rgb<f32>>),
    Glass(Glass<Rgb<f32>>),
}
