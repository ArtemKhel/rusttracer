#![warn(clippy::all)]
#![allow(clippy::similar_names, clippy::many_single_char_names)]
#![allow(unused)]
#![allow(clippy::module_inception)]
#![allow(internal_features)]
#![feature(test)]
#![feature(core_intrinsics)]
#![feature(stmt_expr_attributes)]
#![feature(const_trait_impl, effects)]
#![feature(get_mut_unchecked)]
#![feature(duration_millis_float)]
#![feature(more_float_constants)]
#![feature(isqrt)]
#![feature(let_chains)]
#![feature(lazy_cell)]

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
pub mod spectra;
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

pub const N_WAVELENGTHS: usize = 4;
pub type SampledSpectrum = spectra::SampledSpectrum<N_WAVELENGTHS>;
pub type SampledWavelengths = spectra::SampledWavelengths<N_WAVELENGTHS>;

pub static CALLS: AtomicU32 = AtomicU32::new(0);
pub static SKIP: AtomicU32 = AtomicU32::new(0);
