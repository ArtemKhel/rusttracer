use std::sync::Arc;

use crate::{
    core::SurfaceInteraction,
    light::{Light, LightEnum, LightSample},
};

// TODO: power and bvh light sampler

pub struct SampledLight {
    pub light: Arc<LightEnum>,
    pub prob: f32,
}

pub trait LightSampler {
    fn sample(&self, surf_int: &SurfaceInteraction, sample_c: f32) -> Option<SampledLight>;
    fn pmf(&self, surf_int: &SurfaceInteraction, light: &LightEnum) -> f32;
    // todo: no-ctx variants
}
