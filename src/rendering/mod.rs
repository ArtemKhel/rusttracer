mod antialiasing;
mod raytracer;

pub use antialiasing::{AAType, AntiAliasing};
use image::{ImageBuffer, Pixel, Rgb};
use rayon::iter::ParallelIterator;
pub use raytracer::RayTracer;

pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

pub trait Render {
    fn render(&self) -> ImageBuffer<Rgb<f32>, Vec<f32>>;
}

pub type PixelCoord = [f32; 2];
