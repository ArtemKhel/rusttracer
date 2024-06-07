use image::{Pixel, Rgb};

use crate::{
    bxdf::bxdf::BxDFFlags,
    colors,
    core::Ray,
    integrators::{
        ray::{RIState, RayIntegrator},
        tile::TIState,
        IState,
    },
    math::{dot, Unit},
    ray,
    samplers::{Sampler, SamplerType, StratifiedSampler},
    scene::Scene,
};

pub struct SimplePathIntegrator {
    state: RIState,
    sample_lights: bool,
    sample_bsdf: bool,
}

unsafe impl Send for SimplePathIntegrator {}

unsafe impl Sync for SimplePathIntegrator {}

impl SimplePathIntegrator {
    pub fn new(scene: Scene, max_depth: u32, samples_per_pixel: u32) -> Self {
        SimplePathIntegrator {
            sample_lights: true,
            sample_bsdf: true,
            state: RIState {
                max_depth,
                tile: TIState {
                    base: IState { scene },
                    // sampler: SamplerType::Independent(IndependentSampler::new(samples_per_pixel, 42)),
                    sampler: SamplerType::Stratified(StratifiedSampler::new(16, 16, true, 42)),
                },
            },
        }
    }
}

impl RayIntegrator for SimplePathIntegrator {
    // TODO: this is awful, refactor
    fn light_incoming(&self, ray: &Ray, sampler: &mut SamplerType) -> Rgb<f32> {
        let mut ray = ray.clone();
        let mut depth = 0;
        let mut radiance = colors::BLACK;
        // Fraction of radiance that arrives at the camera
        let mut throughput = colors::WHITE;
        let mut specular_bounce = true;

        while throughput != colors::BLACK {
            if let Some(mut interaction) = self.state.scene.cast_ray(&ray) {
                // TODO: medias

                if (!self.sample_lights || specular_bounce) {
                    let emitted = interaction.emitted_light().map2(&throughput, |e, t| e * t);
                    radiance.apply2(&emitted, |r, e| r + e)
                }

                if depth == self.state.max_depth {
                    break;
                } else {
                    depth += 1;
                }

                let Some(bsdf) = interaction.get_bsdf(&ray, &self.state.scene.camera, sampler) else {
                    continue;
                };

                if self.sample_lights {
                    // TODO: light sampler
                    let light = self.state.scene.lights.first().unwrap();
                    if let Some(light_sample) = light.sample_light(&interaction, sampler.get_2d()) {
                        if light_sample.pdf == 0. && light_sample.radiance == colors::BLACK {
                            break;
                        }
                        let mut reflected = bsdf.eval(*light_sample.incoming, *interaction.hit.outgoing);
                        // TODO: scene.cast_ray -> bool
                        if reflected != colors::BLACK
                        /* && self.state.scene.cast_ray(&ray!(interaction.hit.point + light_sample.incoming *1e-3,
                         * light_sample.incoming)).is_none() */
                        {
                            reflected.apply2(&throughput, |e, t| e * t);
                            reflected.apply2(&light_sample.radiance, |e, t| e * t /* / light_sample.pdf */);
                            radiance.apply2(&reflected, |r, e| r + e);
                        }
                    }
                }

                if self.sample_bsdf {
                    let Some(sample) = bsdf.sample(*interaction.hit.outgoing, sampler.get_2d(), sampler.get_1d())
                    else {
                        break;
                    };
                    // TODO: shading normal
                    let coef = dot(&interaction.hit.outgoing, &interaction.hit.normal).abs() / sample.pdf;
                    throughput.apply2(&sample.color, |t, s| t * s * coef);
                    specular_bounce = sample.flags.contains(BxDFFlags::Specular);
                    // todo: si.spawn_ray
                    ray = ray!(
                        interaction.hit.point + sample.incoming * 1e-3,
                        Unit::from_unchecked(sample.incoming)
                    );
                } else {
                    // todo uniformly sample
                }
            } else {
                // if (!self.sample_lights || specular_bounce) {
                //     // TODO: lights
                //     let background = throughput.map2(
                //         &lerp(colors::DARK_BLUE, colors::LIGHT_BLUE, (ray.dir.y + 1.) / 2.),
                //         |t, b| t * b,
                //     );
                //     radiance.apply2(&background, |r, b| r + b);
                // }
                break;
            }
        }
        radiance
    }

    fn get_ri_state(&self) -> &RIState { &self.state }

    fn get_ri_state_mut(&mut self) -> &mut RIState { &mut self.state }
}
