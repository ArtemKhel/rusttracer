use std::sync::Arc;

use derive_more::Deref;

use crate::{
    core::Ray,
    integrators::{
        tile::{TIState, TileIntegrator},
        Integrator,
    },
    samplers::{Sampler, SamplerType},
    scene::{
        cameras::{Camera, CameraSample},
        film::Film,
    },
    Point2us, SampledSpectrum, SampledWavelengths,
};

pub(super) trait RayIntegrator: TileIntegrator {
    fn light_incoming(&self, ray: &Ray, lambda: &SampledWavelengths, sampler: &mut SamplerType) -> SampledSpectrum;
    fn get_ri_state(&self) -> &RIState;
}

#[derive(Deref)]
pub(super) struct RIState {
    #[deref]
    pub(super) tile: TIState,
    // TODO: move to more concrete structs?
    pub(super) max_depth: u32,
}

impl<T> TileIntegrator for T
where T: RayIntegrator
{
    fn evaluate_pixel(&self, pixel: Point2us, sampler: &mut SamplerType) {
        let state = self.get_state();
        let lambda = state.scene.camera.get_film().sample_wavelengths(sampler.get_1d());
        let sample = CameraSample::new(pixel, sampler);

        // TODO: [AA] should be diff ray.
        //       [AA] something about diff scaling
        //       [filters] should account for CameraRay weight
        //       [realistic camera] need to know about wavelengths
        let ray = state.scene.camera.generate_ray(sample);
        let spectrum = self.light_incoming(&ray, &lambda, sampler);

        let mut arc_film = state.scene.camera.get_film();
        unsafe {
            let film = Arc::get_mut_unchecked(&mut arc_film);
            film.add_sample(pixel, spectrum, lambda, 1.)
        };
    }

    fn get_ti_state(&self) -> &TIState { &self.get_ri_state().tile }
}
