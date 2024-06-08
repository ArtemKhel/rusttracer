use std::f32::consts::PI;

use image::{Pixel, Rgb};
use itertools::any;

use crate::{
    breakpoint, colors,
    core::Ray,
    integrators::{
        ray::{RIState, RayIntegrator},
        tile::TIState,
        IState, Integrator,
    },
    math::dot,
    ray,
    samplers::{utils::sample_uniform_sphere, IndependentSampler, Sampler, SamplerType, StratifiedSampler},
    scene::Scene,
    utils::lerp,
};

pub struct RandomWalkIntegrator {
    state: RIState,
}

unsafe impl Sync for RandomWalkIntegrator {}

unsafe impl Send for RandomWalkIntegrator {}

impl RayIntegrator for RandomWalkIntegrator {
    fn light_incoming(&self, ray: &Ray, sampler: &mut SamplerType) -> Rgb<f32> { self.random_walk(ray, 0, sampler) }

    fn get_ri_state(&self) -> &RIState { &self.state }

    fn get_ri_state_mut(&mut self) -> &mut RIState { &mut self.state }
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
                },
            },
        }
    }

    fn random_walk(&self, ray: &Ray, depth: u32, sampler: &mut SamplerType) -> Rgb<f32> {
        let closest_hit = self.state.scene.cast_ray(ray);
        if let Some(mut interaction) = closest_hit {
            let emitted = interaction.emitted_light().unwrap_or(colors::BLACK);

            if depth > self.state.max_depth {
                return emitted;
            }

            if let Some(bsdf) = interaction.get_bsdf(ray, &self.state.scene.camera, sampler) {
                // todo: infinite lights

                let incoming = sample_uniform_sphere(sampler.get_2d());
                let cos_in_out = dot(&incoming, &interaction.hit.normal).abs();
                // let radiance = bsdf.eval(*interaction.hit.outgoing, *incoming).map(|x| x * cos_in_out);
                let radiance = bsdf.eval(*incoming, *interaction.hit.outgoing).map(|x| x * cos_in_out);
                if radiance == colors::BLACK {
                    return emitted;
                }

                // TODO: SI.spawn_ray
                // TODO: ray offset
                let incoming_ray = ray!(interaction.hit.point + **interaction.hit.normal * 1e-3, incoming);
                let incoming_radiance = self.random_walk(&incoming_ray, depth + 1, sampler);

                radiance
                    .map2(&incoming_radiance, |x, y| x * y * (4. * PI))
                    .map2(&emitted, |r, e| r + e)
            } else {
                emitted
            }
        } else {
            colors::BLACK
            // lerp(colors::DARK_BLUE, colors::LIGHT_BLUE, (ray.dir.y + 1.) / 2.)
        }
    }
}
