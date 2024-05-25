#![allow(unused)]
#![allow(internal_features)]
#![feature(core_intrinsics)]
#![feature(unboxed_closures, fn_traits)]
#![feature(stmt_expr_attributes)]
#![feature(test)]
#![feature(const_trait_impl, effects)]

use std::sync::atomic::AtomicU32;

use image::Rgb;
pub mod aggregates;
pub mod bxdf;
pub mod core;
pub mod material;
pub mod math;
pub mod mediums;
pub mod rendering;
mod samplers;
pub mod scene;
pub mod shapes;
pub mod test_scenes;
pub mod utils;

pub type Point2f = math::Point2<f32>;
pub type Point3f = math::Point3<f32>;
pub type Vec3f = math::Vec3<f32>;
pub type Normal3f = math::Normal3<f32>;

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
