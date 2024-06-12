use std::fmt::Debug;

use bitflags::bitflags;
pub use diffuse_area::DiffuseAreaLight;
pub use light_sampler::LightSampler;
pub use point::PointLight;
pub use uniform_sampler::UniformLightSampler;

use crate::{core::SurfaceInteraction, math::Unit, Point2f, Point3f, SampledSpectrum, SampledWavelengths, Vec3f};

mod base;
mod diffuse_area;
mod light_sampler;
mod point;
mod uniform_sampler;

#[enum_delegate::register]
pub trait Light {
    // TODO: naming
    /// Total emitted power.
    /// Phi() in PBRT
    fn flux(&self, lambda: &SampledWavelengths) -> SampledSpectrum;

    fn light_type(&self) -> LightType;

    /// sampleLi() in PBRT
    fn sample_light(
        &self,
        surf_int: &SurfaceInteraction,
        lambda: &SampledWavelengths,
        rnd_p: Point2f,
    ) -> Option<LightSample>;

    /// Radiance emitted back along the intersecting ray. Only for area light.
    /// This method should never be called for any light that does not have geometry associated with it.
    /// L() in PBRT
    fn radiance(&self, surf_int: &SurfaceInteraction, lambda: &SampledWavelengths) -> Option<SampledSpectrum> { None }

    // todo: fn pdf_incoming(&self, incoming: Vec3f, surf_int: &SurfaceInteraction) -> f32 {}
    //       fn Le(&self, ...) -> ... {}
}

bitflags! {
    #[derive(Copy, Clone, Debug)]
    pub struct LightType: u32{
        /// Emits from single point
        const DeltaPosition = 1 << 0;
        /// Emits in single direction
        const DeltaDirection = 1 << 1;
        /// Emits from the surface of an object
        const Area = 1 << 2;
        /// For rays that escaped the scene
        const Infinite = 1 << 3;
    }
}

#[derive(Debug)]
pub struct LightSample {
    /// Amount of radiance leaving the light source toward the receiving point
    pub radiance: SampledSpectrum,
    /// Direction towards the light source from passed [SurfaceInteraction]
    pub incoming: Unit<Vec3f>,
    pub pdf: f32,
    // TODO: mediums
    /// Point from which light is being emitted
    pub point: Point3f,
}

#[derive(Debug)]
#[enum_delegate::implement(Light)]
pub enum LightEnum {
    Point(PointLight),
    DiffuseArea(DiffuseAreaLight),
}
