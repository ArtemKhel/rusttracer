use std::f32::consts::PI;

use image::{buffer::ConvertBuffer, ImageBuffer, Rgb, RgbImage};
use rand::Rng;

use crate::{
    breakpoint, colors,
    core::Ray,
    integrators::{
        random_walk::RandomWalkIntegrator,
        ray::{RIState, RayIntegrator},
        tile::TIState,
        IState, Integrator,
    },
    math::dot,
    point2, ray,
    samplers::{utils::sample_uniform_sphere, IndependentSampler, SamplerType, StratifiedSampler},
    scene::{
        cameras::{Camera, CameraSample},
        Scene,
    },
    utils::{lerp, linear_to_gamma},
    Point2f,
};

pub struct DebugNormalIntegrator {
    state: RIState,
}

unsafe impl Sync for DebugNormalIntegrator {}

unsafe impl Send for DebugNormalIntegrator {}

impl RayIntegrator for DebugNormalIntegrator {
    fn light_incoming(&self, ray: &Ray, sampler: &mut SamplerType) -> Rgb<f32> { self.normal_as_rgb(ray) }

    fn get_ri_state(&self) -> &RIState { &self.state }

    fn get_ri_state_mut(&mut self) -> &mut RIState { &mut self.state }
}

impl DebugNormalIntegrator {
    pub fn new(scene: Scene) -> Self {
        DebugNormalIntegrator {
            state: RIState {
                max_depth: 1,
                tile: TIState {
                    base: IState { scene },
                    sampler: SamplerType::Independent(IndependentSampler::new(1, 42)),
                },
            },
        }
    }

    fn normal_as_rgb(&self, ray: &Ray) -> Rgb<f32> {
        let closest_hit = self.get_state().scene.cast_ray(ray);
        if let Some(mut interaction) = closest_hit {
            return Rgb([
                (interaction.hit.normal.x + 1.) / 2.,
                (interaction.hit.normal.y + 1.) / 2.,
                (interaction.hit.normal.z + 1.) / 2.,
            ]);
        } else {
            colors::BLACK
        }
    }
}
