use std::fmt::Debug;

use crate::{core::SurfaceInteraction, textures::Texture};

#[derive(Debug)]
pub struct ConstantTexture<T> {
    pub value: T,
}
impl<T: Copy + Debug> Texture<T> for ConstantTexture<T> {
    fn evaluate(&self, surf_int: &SurfaceInteraction) -> T { self.value }
}
