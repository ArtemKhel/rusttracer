use image::{ImageBuffer, Rgb};
use rayon::iter::ParallelIterator;

use crate::integrators::debug_normal::DebugNormalIntegrator;

pub mod random_walk;
pub mod debug_normal;

// #[enum_delegate::register]
pub trait Integrator {
    fn render(&self) -> ImageBuffer<Rgb<f32>, Vec<f32>>;
}

// #[enum_delegate::implement(Integrator)]
// pub enum Integrators{
//     Normal(NormalIntegrator)
// }
