use std::f32::consts::PI;

use image::Pixel;
use num_traits::Zero;

use crate::{
    core::Ray,
    integrators::{
        ray::{RIState, RayIntegrator},
        tile::TIState,
        IState,
    },
    math::dot,
    ray,
    samplers::{utils::sample_uniform_sphere, Sampler, SamplerType, StratifiedSampler},
    scene::Scene,
    SampledSpectrum, SampledWavelengths,
};

pub struct RandomWalkIntegrator {
    state: RIState,
}

unsafe impl Sync for RandomWalkIntegrator {}

unsafe impl Send for RandomWalkIntegrator {}

impl RayIntegrator for RandomWalkIntegrator {
    fn light_incoming(&self, ray: &Ray, lambda: &mut SampledWavelengths, sampler: &mut SamplerType) -> SampledSpectrum {
        self.random_walk(ray, lambda, 0, sampler)
    }

    fn get_ri_state(&self) -> &RIState { &self.state }
}

impl RandomWalkIntegrator {
    pub fn new(scene: Scene, max_depth: u32, samples_per_pixel: u32) -> Self {
        let sqrt_spp = samples_per_pixel.isqrt();
        RandomWalkIntegrator {
            state: RIState {
                max_depth,
                tile: TIState {
                    base: IState { scene },
                    // sampler: IndependentSampler::new(samples_per_pixel, 42).into(),
                    sampler: StratifiedSampler::new(sqrt_spp, sqrt_spp, true, 42).into(),
                    save_intermediate: false,
                },
            },
        }
    }

    fn random_walk(
        &self,
        ray: &Ray,
        lambda: &mut SampledWavelengths,
        depth: u32,
        sampler: &mut SamplerType,
    ) -> SampledSpectrum {
        let closest_hit = self.state.scene.cast_ray(ray);
        if let Some(mut interaction) = closest_hit {
            let emitted = interaction.emitted_light(lambda).unwrap_or(SampledSpectrum::zero());

            if depth > self.state.max_depth {
                return emitted;
            }

            if let Some(bsdf) = interaction.get_bsdf(ray, lambda, &self.state.scene.camera, sampler) {
                // todo: [infinite lights]
                let incoming = sample_uniform_sphere(sampler.get_2d());
                let cos_in_out = dot(&incoming, &interaction.hit.normal).abs();
                let radiance = bsdf.eval(*incoming, *interaction.hit.outgoing) * cos_in_out;
                if radiance.is_zero() {
                    return emitted;
                }

                // TODO: SI.spawn_ray
                // TODO: ray offset
                let incoming_ray = ray!(interaction.hit.point + **interaction.hit.normal * 1e-3, incoming);
                let incoming_radiance = self.random_walk(&incoming_ray, lambda, depth + 1, sampler);

                radiance * incoming_radiance * 4. * PI + emitted
            } else {
                emitted
            }
        } else {
            SampledSpectrum::zero()
        }
    }
}
