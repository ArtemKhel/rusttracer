#![allow(unused)]
#![allow(internal_features)]
#![feature(core_intrinsics)]
#![feature(unboxed_closures, fn_traits)]
#![feature(stmt_expr_attributes)]
#![feature(test)]
#![feature(const_trait_impl, effects)]
#![feature(trait_alias)]

use std::{ops::DerefMut, sync::atomic::AtomicU32};

pub mod aggregates;
pub mod bxdf;
pub mod core;
pub mod material;
pub mod math;
// pub mod mediums;
pub mod integrators;
// pub mod rendering;
mod samplers;
pub mod scene;
pub mod shapes;
pub mod test_scenes;
pub mod textures;
pub mod utils;

type F = f32;
type I = i32;
type U = u32;
pub type Point2f = math::Point2<F>;
pub type Point2i = math::Point2<I>;
pub type Point2u = math::Point2<U>;
pub type Point3f = math::Point3<F>;
pub type Point3i = math::Point3<I>;
pub type Point3u = math::Point3<U>;
pub type Vec3f = math::Vec3<F>;
pub type Normal3f = math::Normal3<F>;

pub static CALLS: AtomicU32 = AtomicU32::new(0);
pub static SKIP: AtomicU32 = AtomicU32::new(0);

pub mod colors {
    use image::Rgb;

    pub const BLACK: Rgb<f32> = Rgb([0., 0., 0.]);
    pub const WHITE: Rgb<f32> = Rgb([1., 1., 1.]);
    pub const GREEN: Rgb<f32> = Rgb([0.12, 0.45, 0.15]);
    pub const RED: Rgb<f32> = Rgb([0.65, 0.05, 0.05]);
    pub const LIGHT_GRAY: Rgb<f32> = Rgb([0.73, 0.73, 0.73]);
}
