#![allow(unused)]
#![allow(internal_features)]
#![feature(core_intrinsics)]

pub mod material;
pub mod mediums;
pub mod scene;
pub mod utils;

pub mod rendering;

pub mod aggregates;

type F = f32;
pub type Point = math::Point3<F>;
pub type Vec3 = math::Vec3<F>;
pub type UnitVec = math::UnitVec3<F>;
pub type Ray = math::Ray<F>;
pub type Hit = math::Hit<F>;
pub type Aabb = math::Aabb<F>;
pub type Triangle = math::Triangle<F>;
pub type Quad = math::Quad<F>;
