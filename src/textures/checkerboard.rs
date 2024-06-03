use std::fmt::Debug;

use crate::{core::SurfaceInteraction, textures::Texture, Float};

#[derive(Debug)]
pub struct CheckerboardTexture<T> {
    pub dark: T,
    pub light: T,
    pub size: Float,
}

impl<T: Copy + Debug> Texture<T> for CheckerboardTexture<T> {
    fn evaluate(&self, surf_int: &SurfaceInteraction) -> T {
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
