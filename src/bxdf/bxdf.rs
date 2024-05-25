#![allow(non_upper_case_globals)]

use bitflags::bitflags;
use derive_more::{Deref, DerefMut, From};
use image::Rgb;
use num_traits::Zero;

use crate::{bxdf::bsdf::BSDFSample, Point2f, Vec3f};

bitflags! {
    pub struct BxDFType: u32 {
        const Reflection = 1 << 0;
        const Transmission = 1 << 1;
        const Diffuse = 1 << 2;
        const Glossy = 1 << 3;
        const Specular = 1 << 4;
    }
}

impl BxDFType {
    const All: BxDFType = Self::Diffuse | Self::Glossy | Self::Specular | Self::Reflection | Self::Transmission;
    const DiffuseTransmission: BxDFType = Self::Diffuse | Self::Transmission;
    const Diffuse_reflection: BxDFType = (Self::Diffuse | Self::Reflection);
    const GlossyReflection: BxDFType = Self::Glossy | Self::Reflection;
    const GlossyTransmission: BxDFType = Self::Glossy | Self::Transmission;
    const SpecularReflection: BxDFType = Self::Specular | Self::Reflection;
    const SpecularTransmission: BxDFType = Self::Specular | Self::Transmission;
}

bitflags! {
    pub struct BxDFSampleType: u32 {
        const Reflection = 1 << 0;
        const Transmission = 1 << 1;
    }
}
impl BxDFSampleType {
    const All: BxDFSampleType = Self::Reflection | Self::Transmission;
}

#[derive(Debug, Copy, Clone)]
#[derive(From, Deref, DerefMut)]
pub struct Shading<T> {
    vec: T,
}

pub trait BxDF {
    fn bxdf_type(&self) -> BxDFType;
    fn eval(&self, incoming: Shading<Vec3f>, outgoing: Shading<Vec3f>) -> Rgb<f32>;
    fn sample(&self, point: Point2f, outgoing: Shading<Vec3f>) -> Option<BSDFSample<Shading<Vec3f>>>;
    fn pdf(&self, incoming: Shading<Vec3f>, outgoing: Shading<Vec3f>) -> f32;
    // TODO:  fn rho()
}
