pub use antialiasing::{AAType, AntiAliasing};
use image::{ImageBuffer, Pixel, Rgb};
pub use raytracer::RayTracer;

mod antialiasing;
mod raytracer;

pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

pub trait Renderer {
    fn render(&self) -> ImageBuffer<Rgb<f32>, Vec<f32>>;
}

pub type PixelCoord = [f32; 2];
