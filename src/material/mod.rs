pub mod matte;

use std::fmt::Debug;

use image::Rgb;

use crate::{
    bxdf::BSDF,
    core::{Ray, SurfaceInteraction},
    material::matte::Matte,
};

// pub mod dielectric;
// pub mod diffuse_light;
// pub mod isotropic;
// pub mod lambertian;
// pub mod metal;

pub struct Scatter {
    pub ray: Ray,
    pub attenuation: Rgb<f32>,
}

pub trait Material {
    type BxDF;
    fn get_bsdf(&self, surf_int: &SurfaceInteraction) -> BSDF;

    // fn scattered(&self, ray: &Ray, intersection: &Intersection) -> Option<Scatter> { None }
    // fn emitted(&self) -> Option<Rgb<f32>> { None }
}

#[derive(Debug)]
pub enum MaterialsEnum {
    Matte(Matte<Rgb<f32>>),
}
