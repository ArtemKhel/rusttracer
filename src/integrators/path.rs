use bumpalo::Bump;
use image::Pixel;
use num_complex::ComplexFloat;
use num_traits::{One, Zero};
use ouroboros::self_referencing;
use rand::Rng;

use crate::{
    bxdf::{BxDFFlags, BSDF},
    core::{Ray, SurfaceInteraction},
    integrators::{
        ray::{RIState, RayIntegrator},
        tile::TIState,
        IState, Integrator,
    },
    light::{Light, LightEnum, LightSampler, UniformLightSampler},
    math::{dot, utils::power_heuristic, Normed, Unit},
    samplers::{Sampler, SamplerType, StratifiedSampler},
    scene::Scene,
    SampledSpectrum, SampledWavelengths,
};

#[self_referencing]
pub struct PathIntegrator {
    state: RIState,
    #[borrows(state)]
    #[covariant]
    light_sampler: UniformLightSampler<'this>,
    regularize: bool,
}

unsafe impl Send for PathIntegrator {}

unsafe impl Sync for PathIntegrator {}

impl PathIntegrator {
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
        PathIntegrator::new(
            state,
            |state: &RIState| UniformLightSampler {
                lights: &state.scene.lights,
            },
            true,
        )
    }

    fn sample_direct_light(
        &self,
        interaction: &SurfaceInteraction,
        bsdf: &BSDF,
        lambda: &SampledWavelengths,
        sampler: &mut SamplerType,
    ) -> Option<SampledSpectrum> {
        // TODO: [light sources emitting only on one side]

        // Choose a light source for the direct lighting calculation
        let rnd_c = sampler.get_1d();
        let rnd_p = sampler.get_2d();
        let Some(sampled_light) = self.borrow_light_sampler().sample(interaction, rnd_c) else {
            return None;
        };
        // Sample a point on the light source for direct lighting
        if let Some(sample) = sampled_light.light.sample(interaction, lambda, rnd_p)
            && sample.pdf != 0.
            && !sample.radiance.is_zero()
        {
            // Evaluate BSDF for light sample and check light visibility
            let cos = dot(&sample.incoming, &interaction.shading.normal).abs();
            let reflected = bsdf.eval(*sample.incoming, *interaction.hit.outgoing);
            let is_unoccluded =
                self.borrow_state()
                    .scene
                    .unoccluded(interaction.hit.point, sample.incoming, sample.point);
            if reflected.is_zero() || !is_unoccluded {
                return None;
            }

            // Return light's contribution to reflected radiance
            let prob_light = sampled_light.prob * sample.pdf;
            match sampled_light.light.as_ref() {
                LightEnum::Point(l) => Some(sample.radiance * reflected / prob_light),
                _ => {
                    let prob_bsdf = bsdf.pdf(*sample.incoming, *interaction.hit.outgoing);
                    let weight_light = power_heuristic(1, prob_light, 1, prob_bsdf);
                    Some(weight_light * sample.radiance * reflected / prob_light)
                }
            }
        } else {
            None
        }
    }
}

impl RayIntegrator for PathIntegrator {
    fn light_incoming(
        &self,
        ray: &Ray,
        lambda: &mut SampledWavelengths,
        sampler: &mut SamplerType,
        alloc: &mut Bump,
    ) -> SampledSpectrum {
        let mut ray = *ray;
        let mut depth = 0;
        // Total radiance from the current path
        let mut radiance = SampledSpectrum::zero();
        // Fraction of radiance that arrives at the camera
        let mut throughput = SampledSpectrum::one();

        let mut specular_bounce = false;
        let mut any_non_specular_bounces = false;
        let mut eta_scale = 1.;
        let mut prob_bsdf = 1.;
        // todo: ctx
        let mut prev_surf_int: SurfaceInteraction = SurfaceInteraction::default();

        loop {
            // Trace ray and find the closest path vertex and its BSDF
            let Some(mut interaction) = self.get_state().scene.cast_ray(&ray) else {
                // Incorporate emission from infinite lights for escaped ray
                // TODO: [infinite lights]
                break;
            };

            // TODO: emitted light
            // Incorporate emission from surface hit by ray
            if let Some(emitted) = interaction.emitted_light(lambda) {
                if (depth == 0 || specular_bounce) {
                    radiance += throughput * emitted;
                } else {
                    // Compute MIS weight for area light
                    let light_source = interaction.area_light.as_ref().unwrap().as_ref();
                    let prob_light = self.borrow_light_sampler().pmf(&prev_surf_int, light_source)
                        * light_source.pdf_incoming(ray.dir, &prev_surf_int);
                    let weight_light = power_heuristic(1, prob_bsdf, 1, prob_light);
                    radiance += throughput * weight_light * emitted;
                }
            }

            let Some(bsdf) = interaction.get_bsdf(&ray, lambda, &self.borrow_state().scene.camera, sampler, alloc)
            else {
                // TODO: medias
                continue;
            };

            // TODO: geometry-aware film
            if depth == self.borrow_state().max_depth {
                break;
            }
            depth += 1;

            // Sample direct illumination
            let flags = bsdf.flags();
            if !flags.contains(BxDFFlags::Specular) {
                if let Some(direct) = self.sample_direct_light(&interaction, &bsdf, lambda, sampler) {
                    radiance += throughput * direct;
                }
            }

            let Some(bsdf_sample) = bsdf.sample(*interaction.hit.outgoing, sampler.get_2d(), sampler.get_1d()) else {
                break;
            };

            // Update path state variables after surface scattering
            let cos = dot(&bsdf_sample.incoming, &interaction.shading.normal).abs();
            throughput *= bsdf_sample.spectrum * cos;
            // TODO: [LayeredBxDF]
            prob_bsdf = bsdf_sample.pdf;
            specular_bounce = bsdf_sample.flags.contains(BxDFFlags::Specular);
            any_non_specular_bounces |= specular_bounce;
            if bsdf_sample.flags.contains(BxDFFlags::Transmission) {
                eta_scale *= bsdf_sample.eta.powi(2)
            }
            prev_surf_int = interaction.clone();

            // TODO:
            ray = interaction.spawn_ray(Unit::from_unchecked(bsdf_sample.incoming));

            // TODO: Possibly terminate the path with Russian roulette
        }

        radiance
    }

    fn get_ri_state(&self) -> &RIState { self.borrow_state() }
}
