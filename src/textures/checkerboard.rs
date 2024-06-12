use std::fmt::Debug;

use crate::{core::SurfaceInteraction, SampledWavelengths, textures::SpectrumTexture};

#[derive(Debug)]
pub struct CheckerboardTexture<T> {
    pub dark: T,
    pub light: T,
    pub size: f32,
}

impl<T: Copy + Debug> SpectrumTexture<T> for CheckerboardTexture<T> {
    fn evaluate(&self, surf_int: &SurfaceInteraction, lambda: SampledWavelengths) -> T {
        let uv = surf_int.hit.uv;
        let u = uv.x % (self.size * 2.);
        let v = uv.y % (self.size * 2.);

        if (u > self.size) ^ (v > self.size) {
            self.dark
        } else {
            self.light
        }
    }
}
