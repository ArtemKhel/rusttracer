use std::{f32::consts::PI, intrinsics::breakpoint};

use approx::assert_abs_diff_eq;
use image::{Pixel, Rgb};
use itertools::any;
use rand::random;

use crate::{
    breakpoint, colors,
    core::Ray,
    integrators::{
        ray_integrator::{RIState, RayIntegrator},
        tile_integrator::TIState,
        IState, Integrator,
    },
    math::{dot, Normed},
    point2, ray,
    samplers::{utils::sample_uniform_sphere, IndependentSampler, Sampler, SamplerType},
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
        RandomWalkIntegrator {
            state: RIState {
                max_depth,
                tile: TIState {
                    base: IState { scene },
                    sampler: SamplerType::Independent(IndependentSampler::new(samples_per_pixel, 42)),
                },
            },
        }
    }

    fn random_walk(&self, ray: &Ray, depth: u32, sampler: &mut SamplerType) -> Rgb<f32> {
        // TODO: emitted
        let closest_hit = self.get_state().scene.cast_ray(ray);
        if let Some(mut interaction) = closest_hit {
            if let Some(bsdf) = interaction.get_bsdf(ray, &self.get_state().scene.camera, sampler) {
                if depth > self.state.max_depth {
                    return colors::BLACK;
                }

                // TODO: sampler here
                // TODO: weird shadows near {0,0,-1} and {*,*,0} normals.
                let incoming = sample_uniform_sphere(sampler.get_2d());
                let cos_in_out = dot(&incoming, &interaction.hit.normal).abs();
                let result = bsdf.eval(*interaction.hit.outgoing, *incoming).map(|x| x * cos_in_out);
                if result == colors::BLACK {
                    return result;
                }

                // TODO: SI.spawn_ray
                let incoming_ray = ray!(interaction.hit.point + **interaction.hit.normal * 1e-3, incoming);
                let incoming_radiance = self.random_walk(&incoming_ray, depth + 1, sampler);

                // breakpoint!(any(res_cos.0, f32::is_nan) || any(incoming_radiance.0, f32::is_nan));

                result.map2(&incoming_radiance, |x, y| x * y * (4. * PI))
            } else {
                colors::BLACK
            }
        } else {
            lerp(colors::DARK_BLUE, colors::LIGHT_BLUE, (ray.dir.y + 1.) / 2.)
        }
    }
}
