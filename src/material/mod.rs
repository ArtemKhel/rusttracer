use std::fmt::Debug;

use image::Rgb;

use crate::{
    bxdf,
    bxdf::{BxDF, BSDF},
    core::{Ray, SurfaceInteraction},
    material::{matte::Matte, metal::Metal},
};

pub mod matte;
pub mod metal;

#[enum_delegate::register]
pub trait Material {
    // #[enum_delegate(unify = "enum_wrap")]
    type BxDF;
    fn get_bsdf(&self, surf_int: &SurfaceInteraction) -> BSDF;
}

#[derive(Debug)]
#[enum_delegate::implement(Material)]
pub enum MaterialsEnum {
    Matte(Matte<Rgb<f32>>),
    Metal(Metal<Rgb<f32>>),
}
