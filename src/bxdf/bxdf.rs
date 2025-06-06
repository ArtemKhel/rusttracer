#![allow(non_upper_case_globals)]

use bitflags::bitflags;
use derive_more::{Deref, DerefMut, From};
use image::Rgb;
use num_traits::Zero;

use crate::{
    bxdf::{
        bsdf::BSDFSample, conductor::ConductorBxDF, dielectric::DielectricBxDF, diffuse::DiffuseBxDF,
        utils::abs_cos_theta,
    },
    math::dot,
    samplers::utils::{sample_uniform_hemisphere, uniform_hemisphere_pdf},
    Point2f, SampledSpectrum, Vec3f,
};

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct BxDFFlags: u32 {
        const None = 0;
        const Reflection = 1 << 0;
        const Transmission = 1 << 1;
        const Diffuse = 1 << 2;
        const Glossy = 1 << 3;
        const Specular = 1 << 4;

        const All = Self::Diffuse.bits() | Self::Glossy.bits() | Self::Specular.bits() | Self::Reflection.bits() | Self::Transmission.bits();
        const DiffuseReflection = Self::Diffuse.bits() | Self::Reflection.bits();
        const DiffuseTransmission = Self::Diffuse.bits() | Self::Transmission.bits();
        const GlossyReflection = Self::Glossy.bits() | Self::Reflection.bits();
        const GlossyTransmission = Self::Glossy.bits() | Self::Transmission.bits();
        const SpecularReflection = Self::Specular.bits() | Self::Reflection.bits();
        const SpecularTransmission = Self::Specular.bits() | Self::Transmission.bits();
    }
}

// TODO:
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
/// Vector in local coordinates for material evaluation. X and Y lie on the surface, Z is normal
pub struct Shading<T> {
    vec: T,
}

#[enum_delegate::register]
pub trait BxDF {
    /// Material properties
    fn flags(&self) -> BxDFFlags;

    /// Returns the value of the distribution function for the given pair of directions (in the local reflection
    /// coordinate system). f() in PBRT
    fn eval(&self, incoming: Shading<Vec3f>, outgoing: Shading<Vec3f>) -> SampledSpectrum;

    /// Determines the direction of the incident light and returns the value of BxDF for the pair of directions
    /// sample_f() in PBRT
    fn sample(&self, rnd_p: Point2f, rnd_c: f32, outgoing: Shading<Vec3f>) -> Option<BSDFSample<Shading<Vec3f>>>;

    fn pdf(&self, incoming: Shading<Vec3f>, outgoing: Shading<Vec3f>) -> f32;

    ///
    fn hd_reflectance<const N: usize>(
        &self,
        outgoing: Shading<Vec3f>,
        rnd_p: &[Point2f; N],
        rnd_c: &[f32; N],
    ) -> SampledSpectrum {
        // TODO: perfect reflectors/transmitters?
        let mut spectrum = SampledSpectrum::zero();
        for i in 0..N {
            if let Some(sample) = self.sample(rnd_p[i], rnd_c[i], outgoing) {
                let cos = abs_cos_theta(sample.incoming);
                spectrum += sample.spectrum * cos / sample.pdf
            }
        }
        spectrum / N as f32
    }

    ///
    fn hh_reflectance<const N: usize>(
        &self,
        rnd_p_out: &[Point2f; N],
        rnd_p: &[Point2f; N],
        rnd_c: &[f32; N],
    ) -> SampledSpectrum {
        let mut spectrum = SampledSpectrum::zero();
        for i in 0..N {
            let outgoing = Shading::from(*sample_uniform_hemisphere(rnd_p_out[i]));
            let out_pdf = uniform_hemisphere_pdf();
            if let Some(sample) = self.sample(rnd_p[i], rnd_c[i], outgoing) {
                let cos = abs_cos_theta(sample.incoming);
                spectrum += sample.spectrum * cos / (sample.pdf * out_pdf)
            }
        }
        spectrum / N as f32
    }
}

#[enum_delegate::implement(BxDF)]
pub enum BxDFEnum {
    Diffuse(DiffuseBxDF),
    Conductor(ConductorBxDF),
    Dielectric(DielectricBxDF),
}
