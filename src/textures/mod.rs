use std::fmt::Debug;

use crate::core::SurfaceInteraction;

pub mod constant;
pub mod mappings;

pub trait Texture<T>: Debug {
    fn evaluate(&self, surf_int: &SurfaceInteraction) -> T;
}
