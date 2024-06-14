use std::sync::Arc;

use image::{Pixel, Rgb};
use num_traits::{One, Zero};
use ouroboros::self_referencing;
use rand::Rng;

use crate::{
    bxdf::BxDFFlags,
    core::Ray,
    integrators::{
        ray::{RIState, RayIntegrator},
        tile::TIState,
        IState, Integrator,
    },
    light::{Light, LightSampler, UniformLightSampler},
    math::{dot, Normed, Unit},
    ray,
    samplers::{
        utils::{sample_uniform_hemisphere, sample_uniform_sphere, uniform_hemisphere_pdf, uniform_sphere_pdf},
        IndependentSampler, Sampler, SamplerType, StratifiedSampler,
    },
    scene::Scene,
    SampledSpectrum, SampledWavelengths,
};

// TODO: or just copy lights for the light sampler?
#[self_referencing]
pub struct SimplePathIntegrator {
    state: RIState,
    #[borrows(state)]
    #[covariant]
    light_sampler: UniformLightSampler<'this>,
    sample_lights: bool,
    sample_bsdf: bool,
}

unsafe impl Send for SimplePathIntegrator {}

unsafe impl Sync for SimplePathIntegrator {}

impl SimplePathIntegrator {
    pub fn create(scene: Scene, max_depth: u32, samples_per_pixel: u32) -> Self {
        let sqrt_spp = samples_per_pixel.isqrt();
        let state = RIState {
            max_depth,
            tile: TIState {
                base: IState { scene },
                // sampler: IndependentSampler::new(samples_per_pixel, 42).into(),
                sampler: StratifiedSampler::new(sqrt_spp, sqrt_spp, true, 42).into(),
                save_intermediate: false,
            },
        };
        SimplePathIntegrator::new(
            state,
            |state: &RIState| UniformLightSampler {
                lights: &state.scene.lights,
            },
            true,
            true,
        )
    }
}

impl RayIntegrator for SimplePathIntegrator {
    fn light_incoming(&self, ray: &Ray, lambda: &mut SampledWavelengths, sampler: &mut SamplerType) -> SampledSpectrum {
        let mut ray = *ray;
        let mut depth = 0;
        // Total radiance from the current path
        let mut radiance = SampledSpectrum::zero();
        // Fraction of radiance that arrives at the camera
        let mut throughput = SampledSpectrum::one();
        let mut specular_bounce = true;

        while !throughput.is_zero() {
            // Check for intersections with the scene
            let Some(mut interaction) = self.get_state().scene.cast_ray(&ray) else {
                // TODO: [infinite lights]
                break;
            };

            // Account for emissive materials
            if !self.borrow_sample_lights() || specular_bounce {
                if let Some(mut emitted) = interaction.emitted_light(lambda) {
                    emitted *= throughput;
                    radiance += emitted;
                }
            }

            if depth == self.borrow_state().max_depth {
                break;
            }
            depth += 1;

            let Some(bsdf) = interaction.get_bsdf(&ray, lambda, &self.borrow_state().scene.camera, sampler) else {
                // TODO: medias
                continue;
            };

            if *self.borrow_sample_lights()
                && let Some(sampled_light) = self.borrow_light_sampler().sample(&interaction, sampler.get_1d())
                && let Some(sample) = sampled_light.light.sample(&interaction, lambda, sampler.get_2d())
                && sample.pdf != 0.
                && !sample.radiance.is_zero()
            {
                // TODO: check occlusion before this?
                let mut reflected = bsdf.eval(*sample.incoming, *interaction.hit.outgoing);
                // TODO: [shading_normal]
                let cos = dot(&sample.incoming, &interaction.hit.normal).abs();
                reflected *= cos;

                let is_unoccluded =
                    self.borrow_state()
                        .scene
                        .unoccluded(interaction.hit.point, sample.incoming, sample.point);
                if !reflected.is_zero() && is_unoccluded {
                    reflected *= throughput * sample.radiance / (sampled_light.prob * sample.pdf);
                    radiance += reflected;
                }
            }

            if *self.borrow_sample_bsdf()
                && let Some(bsdf_sample) = bsdf.sample(*interaction.hit.outgoing, sampler.get_2d(), sampler.get_1d())
            {
                // TODO: [shading_normal]
                let cos = dot(&bsdf_sample.incoming, &interaction.hit.normal).abs() / bsdf_sample.pdf;

                throughput *= bsdf_sample.spectrum * cos;
                specular_bounce = bsdf_sample.flags.contains(BxDFFlags::Specular);
                ray = interaction.spawn_ray(Unit::from_unchecked(bsdf_sample.incoming));
            } else {
                // Uniformly sample sphere or hemisphere to get new path direction
                let flags = bsdf.flags();

                let (incoming, pdf) = if flags.contains(BxDFFlags::Reflection & BxDFFlags::Transmission) {
                    (sample_uniform_sphere(sampler.get_2d()), uniform_sphere_pdf())
                } else {
                    let mut incoming = sample_uniform_hemisphere(sampler.get_2d());
                    let pdf = uniform_hemisphere_pdf();
                    let in_out_in_different_hemispheres = (dot(&interaction.hit.outgoing, &interaction.hit.normal)
                        * dot(&incoming, &interaction.hit.normal))
                        < 0.;
                    if flags.contains(BxDFFlags::Reflection) && in_out_in_different_hemispheres {
                        incoming = -incoming;
                    } else if flags.contains(BxDFFlags::Transmission) && !in_out_in_different_hemispheres {
                        incoming = -incoming;
                    };
                    (incoming, pdf)
                };
                let cos = dot(&incoming, &interaction.hit.normal /* TODO: shading */).abs();
                throughput *= bsdf.eval(*incoming, *interaction.hit.outgoing) * cos / pdf;
                specular_bounce = false;
                ray = interaction.spawn_ray(incoming)
            }
        }
        radiance
    }

    fn get_ri_state(&self) -> &RIState { self.borrow_state() }
}
