pub use independent::IndependentSampler;
use num_traits::One;

pub use crate::samplers::stratified::StratifiedSampler;
use crate::{math::Point2, Point2f, Point2u, Point2us};

mod independent;
mod stratified;
pub mod utils;

#[enum_delegate::register]
pub trait Sampler {
    // const EPS: Float = Float::EPSILON;
    // const ONE_MINUS_EPS: Float = Float::one() - Self::EPS;
    fn samples_per_pixel(&self) -> u32;
    fn start_pixel_sample(&mut self, pixel: Point2us, sample_index: u32);
    fn get_1d(&mut self) -> f32;
    fn get_2d(&mut self) -> Point2f;
    fn get_pixel(&mut self) -> Point2f;
}

#[derive(Clone, Debug)]
#[enum_delegate::implement(Sampler)]
pub enum SamplerType {
    Independent(IndependentSampler),
    Stratified(StratifiedSampler),
}
