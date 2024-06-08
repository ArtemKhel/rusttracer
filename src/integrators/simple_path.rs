use image::{Pixel, Rgb};

use crate::{
    breakpoint,
    bxdf::BxDFFlags,
    colors,
    core::Ray,
    integrators::{
        ray::{RIState, RayIntegrator},
        tile::TIState,
        IState,
    },
    math::{dot, Normed, Unit},
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
        let sqrt_spp = samples_per_pixel.isqrt();
        SimplePathIntegrator {
            sample_lights: true,
            sample_bsdf: true,
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
}

impl RayIntegrator for SimplePathIntegrator {
    // TODO: this is awful, refactor
    fn light_incoming(&self, ray: &Ray, sampler: &mut SamplerType) -> Rgb<f32> {
        let mut ray = *ray;
        let mut depth = 0;
        // Total radiance from the current path
        let mut radiance = colors::BLACK;
        // Fraction of radiance that arrives at the camera
        let mut throughput = colors::WHITE;
        let mut specular_bounce = true;

        while throughput != colors::BLACK {
            // Check for intersections with the scene
            let Some(mut interaction) = self.state.scene.cast_ray(&ray) else {
                // TODO: sample infinite lights
                break;
            };

            // Account for emissive materials
            if !self.sample_lights || specular_bounce {
                if let Some(mut emitted) = interaction.emitted_light() {
                    emitted.apply2(&throughput, |e, t| e * t);
                    radiance.apply2(&emitted, |r, e| r + e)
                }
            }

            if depth == self.state.max_depth {
                break;
            }
            depth += 1;

            let Some(bsdf) = interaction.get_bsdf(&ray, &self.state.scene.camera, sampler) else {
                // TODO: medias
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
                    let cos = dot(
                        &light_sample.incoming,
                        /* TODO: shading_normal */ &interaction.hit.normal,
                    )
                    .abs();
                    reflected.apply(|x| x * cos);
                    // TODO: scene.cast_ray -> bool
                    let ray = ray!(
                        interaction.hit.point + light_sample.incoming * 1e-3,
                        light_sample.incoming
                    );
                    let t_max = (interaction.hit.point - light_sample.point).len() * 0.99;
                    let unoccluded = self.state.scene.cast_bounded_ray(&ray, t_max).is_none();
                    if reflected != colors::BLACK && unoccluded {
                        reflected.apply2(&throughput, |r, t| r * t);
                        reflected.apply2(&light_sample.radiance, |r, l| r * l / light_sample.pdf);
                        radiance.apply2(&reflected, |r, e| r + e);
                    }
                }
            }

            if self.sample_bsdf {
                let Some(sample) = bsdf.sample(*interaction.hit.outgoing, sampler.get_2d(), sampler.get_1d()) else {
                    break;
                };
                let coef = dot(
                    &sample.incoming,
                    /* TODO: shading normal */ &interaction.hit.normal,
                )
                .abs()
                    / sample.pdf;
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
        }
        radiance
    }

    fn get_ri_state(&self) -> &RIState { &self.state }

    fn get_ri_state_mut(&mut self) -> &mut RIState { &mut self.state }
}
