pub mod matte;

use std::fmt::Debug;

use std::option::Option;
use image::Rgb;

use crate::{
    bxdf::BSDF,
    core::{Ray, SurfaceInteraction},
    material::matte::Matte,
};
use crate::bxdf::{BxDF, BxDFEnum};

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Rgb<f32>,
}

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
}
