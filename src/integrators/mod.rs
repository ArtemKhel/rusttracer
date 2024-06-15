pub use debug_normal::DebugNormalIntegrator;
use image::{ImageBuffer, Rgb};
pub use random_walk::RandomWalkIntegrator;
use rayon::iter::ParallelIterator;
pub use simple_path::SimplePathIntegrator;

use crate::{
    math::Point2,
    scene::{cameras::Camera, film::Film, Scene},
    Int, Point2u,
};
pub use path::PathIntegrator;

mod debug_normal;
mod path;
mod random_walk;
mod ray;
mod simple_path;
mod tile;

// #[enum_delegate::implement(Integrator)]
// pub enum Integrators{
//     Normal(NormalIntegrator)
// }
// #[enum_delegate::register]
pub trait Integrator {
    fn render(&mut self);
    fn get_state(&self) -> &IState;

    fn save_image(&self) {
        self.get_state()
            .scene
            .camera
            .get_film()
            .write_image("./images/_image.png");
    }
}
pub struct IState {
    pub scene: Scene,
}

// pub struct BaseIntegrator {
//     pub scene: Scene,
// }
