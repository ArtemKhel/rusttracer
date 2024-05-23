#![allow(unused)]
#![allow(internal_features)]
#![feature(core_intrinsics)]
#![feature(unboxed_closures, fn_traits)]
#![feature(stmt_expr_attributes)]

pub mod material;
pub mod mediums;
pub mod scene;
pub mod utils;

pub mod core;
pub mod math;
pub mod rendering;

pub mod aggregates;
pub mod shapes;
pub mod test_scenes;

type F = f32;
pub type Point3 = math::Point3<F>;
pub type Vec3 = math::Vec3<F>;
pub type Normal3 = math::Normal3<F>;
pub type Ray = core::Ray<F>;
pub type Hit = core::Hit<F>;
