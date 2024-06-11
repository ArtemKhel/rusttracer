use std::sync::Arc;

use crate::{
    core::SurfaceInteraction,
    light::{
        light_sampler::{LightSampler, SampledLight},
        LightEnum, LightSample,
    },
};

pub struct UniformLightSampler<'a> {
    pub(crate) lights: &'a Vec<Arc<LightEnum>>,
}

impl LightSampler for UniformLightSampler<'_> {
    fn sample(&self, surf_int: &SurfaceInteraction, rnd_c: f32) -> Option<SampledLight> {
        if self.lights.is_empty() {
            None
        } else {
            let idx = (self.lights.len() as f32 * rnd_c) as usize;
            let light = self.lights.get(idx)?.clone();
            Some(SampledLight {
                light,
                prob: (self.lights.len() as f32).recip(),
            })
        }
    }

    fn pmf(&self, surf_int: &SurfaceInteraction, light: &LightEnum) -> f32 {
        if self.lights.is_empty() {
            0.
        } else {
            (self.lights.len() as f32).recip()
        }
    }
}
