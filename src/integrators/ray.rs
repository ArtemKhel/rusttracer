use std::sync::Arc;

use derive_more::{Deref, DerefMut};
use image::Rgb;

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
    Point2us,
};

pub(super) trait RayIntegrator: TileIntegrator {
    fn light_incoming(&self, ray: &Ray, sampler: &mut SamplerType) -> Rgb<f32>;
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
        // 0.5 - offset from discrete pixels to continuous one
        // Disc. |---0---|---1---|---2---|
        // Cont. 0-------1-------2-------3
        let p_film = pixel.map(|x| x as f32 + 0.5);
        let sample = CameraSample {
            p_film,
            p_lens: sampler.get_2d(),
        };

        let ray = self.get_state().scene.camera.generate_ray(sample);
        let pixel_value = self.light_incoming(&ray, sampler);
        // todo: other stuff here

        let mut arc_film = self.get_state().scene.camera.get_film();
        unsafe {
            let film = Arc::get_mut_unchecked(&mut arc_film);
            film.add_sample(pixel, pixel_value, 1.)
        };
    }

    fn get_ti_state(&self) -> &TIState { &self.get_ri_state().tile }
}
