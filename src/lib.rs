#![allow(unused)]
#![allow(internal_features)]
#![feature(core_intrinsics)]
#![feature(unboxed_closures, fn_traits)]
#![feature(stmt_expr_attributes)]
#![feature(test)]
#![feature(const_trait_impl, effects)]

use image::Rgb;

pub mod bxdf;
pub mod core;
pub mod material;
pub mod math;
pub mod mediums;
pub mod rendering;
pub mod scene;
pub mod utils;

pub mod aggregates;
pub mod shapes;
pub mod test_scenes;

pub type F = f32;
pub type Point2f = math::Point2<F>;
pub type Point3f = math::Point3<F>;
pub type Vec3f = math::Vec3<F>;
pub type Normal3f = math::Normal3<F>;
pub type Ray = core::Ray<F>;
pub type Hit = core::Hit<F>;

pub mod colors {
    use image::Rgb;

    pub const BLACK: Rgb<f32> = Rgb([0., 0., 0.]);
}
