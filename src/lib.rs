#![warn(clippy::all, clippy::pedantic)]
#![allow(clippy::similar_names, clippy::many_single_char_names)]
#![allow(unused)]
#![allow(clippy::module_inception)]
// For breakpoint! macro
#![allow(internal_features)]
#![feature(core_intrinsics)]
#![feature(stmt_expr_attributes)]
#![feature(const_trait_impl, effects)]
#![feature(get_mut_unchecked)]
#![feature(duration_millis_float)]
#![feature(more_float_constants)]
#![feature(isqrt)]
#![feature(let_chains)]

use std::{ops::DerefMut, sync::atomic::AtomicU32};

pub mod aggregates;
pub mod bxdf;
pub mod core;
pub mod material;
pub mod math;
// pub mod mediums;
pub mod integrators;
// pub mod rendering;
pub mod light;
mod samplers;
pub mod scene;
pub mod shapes;
pub mod test_scenes;
pub mod textures;
pub mod utils;

type Int = i32;
type UInt = u32;
pub type Point2f = math::Point2<f32>;
pub type Point2i = math::Point2<Int>;
pub type Point2u = math::Point2<UInt>;
pub type Point2us = math::Point2<usize>;
pub type Point3f = math::Point3<f32>;
pub type Point3i = math::Point3<Int>;
pub type Point3u = math::Point3<UInt>;
pub type Vec3f = math::Vec3<f32>;
pub type Normal3f = math::Normal3<f32>;
pub type Bounds2f = math::Bounds2<f32>;
pub type Bounds2i = math::Bounds2<Int>;
pub type Bounds2u = math::Bounds2<UInt>;

pub static CALLS: AtomicU32 = AtomicU32::new(0);
pub static SKIP: AtomicU32 = AtomicU32::new(0);

pub mod colors {
    use image::Rgb;

    pub const BLACK: Rgb<f32> = Rgb([0., 0., 0.]);
    pub const WHITE: Rgb<f32> = Rgb([1., 1., 1.]);
    pub const GREEN: Rgb<f32> = Rgb([0.12, 0.45, 0.15]);
    pub const RED: Rgb<f32> = Rgb([0.65, 0.05, 0.05]);
    pub const LIGHT_GRAY: Rgb<f32> = Rgb([0.73, 0.73, 0.73]);
    pub const DARK_GRAY: Rgb<f32> = Rgb([0.4, 0.4, 0.4]);
    pub const LIGHT_BLUE: Rgb<f32> = Rgb([0.5, 0.8, 0.95]);
    pub const DARK_BLUE: Rgb<f32> = Rgb([0.2, 0.3, 0.5]);
}
