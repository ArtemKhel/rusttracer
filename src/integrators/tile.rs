use std::{
    cell::{Cell, RefCell},
    cmp::{max, min},
    intrinsics::breakpoint,
    sync::Arc,
};

use derive_more::Deref;
use image::Rgb;
use itertools::{iproduct, Itertools};
use log::{debug, info};
use ndarray::iter::LanesMut;
use rayon::{current_thread_index, prelude::*};
use thread_local::ThreadLocal;

use crate::{
    breakpoint,
    integrators::{IState, Integrator},
    math::{Bounds2, Point2},
    point2,
    samplers::{Sampler, SamplerType},
    scene::{cameras::Camera, film::Film, Scene},
    utils::time_it,
    Point2us,
};

#[derive(Deref)]
pub(super) struct TIState {
    #[deref]
    pub(crate) base: IState,
    pub(crate) sampler: SamplerType,
}

pub(super) trait TileIntegrator: Integrator {
    fn evaluate_pixel(&self, pixel: Point2us, sampler: &mut SamplerType);
    fn get_ti_state(&self) -> &TIState;
    fn get_ti_state_mut(&mut self) -> &mut TIState;
}

impl<T> Integrator for T
where T: TileIntegrator + Sync + Send
{
    fn render(&mut self) {
        // TODO: scratch buffer
        let spp = self.get_ti_state().sampler.samples_per_pixel();
        let mut start = 0;
        let mut till = 1;
        let tiles = self.get_state().scene.camera.get_film().tiles(10, 10);
        // NOTE: thread_local's doc says
        //
        // Note that since thread IDs are recycled when a thread exits, it is possible for one
        // thread to retrieve the object of another thread.
        //
        // That may be a problem. Though rayon uses thread pool and tests show that threads keep
        // their id between waves, it may not always be the case, so who knows?
        let mut thread_local_sampler = ThreadLocal::<RefCell<SamplerType>>::new();

        let rendering_time = time_it(|| {
            while start < spp {
                info!("Starting wave {}-{}", start, till);

                tiles.par_iter().for_each(|&tile_bounds| {
                    iproduct!(
                        (tile_bounds.min.y..tile_bounds.max.y),
                        (tile_bounds.min.x..tile_bounds.max.x),
                        (start..=till)
                    )
                    .for_each(|(y, x, sample_index)| {
                        let pixel_coords = point2!(x, y);
                        let mut thread_sampler = thread_local_sampler
                            .get_or(|| RefCell::new(self.get_ti_state().sampler.clone()))
                            .borrow_mut();

                        // breakpoint!(x == 145 && y == 155);

                        thread_sampler.start_pixel_sample(pixel_coords, sample_index);
                        self.evaluate_pixel(pixel_coords, &mut thread_sampler);
                    });
                });
                start = till;
                till = min(till * 2, spp)
            }
        });
        info!(
            "Rendering time: {rendering_time:.3}s, {:.3}s per sample",
            rendering_time / spp as f32
        );
        self.get_state()
            .scene
            .camera
            .get_film()
            .write_image("./images/_image.png");
    }

    fn get_state(&self) -> &IState { &self.get_ti_state().base }

    fn get_state_mut(&mut self) -> &mut IState { &mut self.get_ti_state_mut().base }
}
