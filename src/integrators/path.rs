use image::Pixel;
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
    light::{Light, LightSampler, UniformLightSampler},
    math::{dot, Normed},
    samplers::{Sampler, SamplerType, StratifiedSampler},
    scene::Scene,
    SampledSpectrum, SampledWavelengths,
};

// TODO: or just copy lights for the light sampler?
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
        let rnd_c = sampler.get_1d();
        let rnd_p = sampler.get_2d();
        if let Some(sampled_light) = self.borrow_light_sampler().sample(interaction, rnd_c) {
            if let Some(sample) = sampled_light.light.sample(interaction, lambda, rnd_p)
                && sample.pdf != 0.
                && !sample.radiance.is_zero()
            {
                // TODO: shading_normal
                let cos = dot(&sample.incoming, &interaction.hit.normal).abs();
                let reflected = bsdf.eval(*sample.incoming, *interaction.hit.outgoing);
                let is_unoccluded =
                    self.borrow_state()
                        .scene
                        .unoccluded(interaction.hit.point, sample.incoming, sample.point);
                if reflected.is_zero() || !is_unoccluded {
                    return None;
                }
                todo!()
            }
            todo!()
        } else {
            None
        }
    }
}

impl RayIntegrator for PathIntegrator {
    fn light_incoming(&self, ray: &Ray, lambda: &mut SampledWavelengths, sampler: &mut SamplerType) -> SampledSpectrum {
        let mut depth = 0;
        // Total radiance from the current path
        let mut radiance = SampledSpectrum::zero();
        // Fraction of radiance that arrives at the camera
        let mut throughput = SampledSpectrum::one();

        loop {
            // Trace ray and find the closest path vertex and its BSDF
            let Some(mut interaction) = self.get_state().scene.cast_ray(&ray) else {
                // TODO: [infinite lights]
                break;
            };

            // TODO: emitted light

            let Some(bsdf) = interaction.get_bsdf(&ray, lambda, &self.borrow_state().scene.camera, sampler) else {
                // TODO: medias
                continue;
            };

            // TODO: geometry-aware film

            // Sample direct illumination
            let flags = bsdf.flags();
            if !flags.contains(BxDFFlags::Specular) {
                let direct = self.sample_direct_light(&interaction, &bsdf, lambda, sampler);
                // radiance += throughput * direct;
            }
        }

        radiance
    }

    fn get_ri_state(&self) -> &RIState { self.borrow_state() }
}
