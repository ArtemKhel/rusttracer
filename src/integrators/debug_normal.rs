use std::sync::Arc;

use image::codecs::avif::ColorSpace;
use num_traits::Zero;

use crate::{
    core::Ray,
    integrators::{
        ray::{RIState, RayIntegrator},
        tile::TIState,
        IState, Integrator,
    },
    samplers::{IndependentSampler, SamplerType},
    scene::Scene,
    spectra::{
        rgb::{sRGB, RGBColorSpace, RGB},
        RGBAlbedoSpectrum, Spectrum,
    },
    SampledSpectrum, SampledWavelengths,
};

pub struct DebugNormalIntegrator {
    state: RIState,
    color_space: Arc<RGBColorSpace>,
}

unsafe impl Sync for DebugNormalIntegrator {}

unsafe impl Send for DebugNormalIntegrator {}

impl RayIntegrator for DebugNormalIntegrator {
    fn light_incoming(&self, ray: &Ray, lambda: &SampledWavelengths, sampler: &mut SamplerType) -> SampledSpectrum {
        self.normal_as_rgb(ray, lambda)
    }

    fn get_ri_state(&self) -> &RIState { &self.state }
}

impl DebugNormalIntegrator {
    pub fn new(scene: Scene) -> Self {
        DebugNormalIntegrator {
            color_space: sRGB.clone(),
            state: RIState {
                max_depth: 1,
                tile: TIState {
                    base: IState { scene },
                    sampler: SamplerType::Independent(IndependentSampler::new(1, 42)),
                    save_intermediate: false,
                },
            },
        }
    }

    fn normal_as_rgb(&self, ray: &Ray, lambda: &SampledWavelengths) -> SampledSpectrum {
        let closest_hit = self.get_state().scene.cast_ray(ray);
        if let Some(mut interaction) = closest_hit {
            let rgb = RGB::new(
                interaction.hit.normal.x.abs(),
                interaction.hit.normal.y.abs(),
                interaction.hit.normal.z.abs(),
            );
            let rgb_to_spec = RGBAlbedoSpectrum::new(&self.color_space, rgb);
            rgb_to_spec.sample(&lambda)
        } else {
            SampledSpectrum::zero()
        }
    }
}
