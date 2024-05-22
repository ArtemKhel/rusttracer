#![allow(unused)]
#![allow(internal_features)]
#![feature(core_intrinsics)]

pub mod material;
pub mod mediums;
pub mod scene;
pub mod utils;

pub mod rendering;

pub mod aggregates;
pub mod test_scenes;

type F = f32;
pub type Point3 = math::Point3<F>;
pub type Vec3 = math::Vec3<F>;
pub type Normal3 = math::Normal3<F>;
pub type Ray = math::Ray<F>;
pub type Hit = math::Hit<F>;
pub type Aabb = math::Aabb<F>;
