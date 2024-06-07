use image::{ImageBuffer, Rgb};
use rayon::iter::ParallelIterator;

use crate::{math::Point2, scene::Scene, Int, Point2u};

pub mod debug_normal;
pub mod random_walk;
mod ray;
pub mod simple_path;
mod tile;

// #[enum_delegate::implement(Integrator)]
// pub enum Integrators{
//     Normal(NormalIntegrator)
// }
// #[enum_delegate::register]
pub trait Integrator {
    fn render(&mut self);
    fn get_state(&self) -> &IState;
    fn get_state_mut(&mut self) -> &mut IState;
}
pub struct IState {
    pub scene: Scene,
}

// pub struct BaseIntegrator {
//     pub scene: Scene,
// }
