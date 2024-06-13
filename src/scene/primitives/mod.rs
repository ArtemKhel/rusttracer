use simple::SimplePrimitive;

use crate::{
    aggregates::{Aabb, Bounded, BVH},
    core::{Ray, SurfaceInteraction},
    light::Light,
    scene::primitives::geometric::GeometricPrimitive,
    shapes::{BoundedIntersectable, Intersectable},
};

pub mod geometric;
pub mod simple;

// TODO:
pub trait Primitive: Bounded<f32> + Intersectable {}

impl Primitive for BVH<f32> {}

#[derive(Debug)]
pub enum PrimitiveEnum {
    Simple(SimplePrimitive),
    Geometric(GeometricPrimitive),
    BVH(BVH<f32>),
}

impl Intersectable for PrimitiveEnum {
    fn intersect(&self, ray: &Ray, t_max: f32) -> Option<SurfaceInteraction> {
        match self {
            PrimitiveEnum::Simple(prim) => prim.intersect(ray, t_max),
            PrimitiveEnum::Geometric(prim) => prim.intersect(ray, t_max),
            PrimitiveEnum::BVH(prim) => prim.intersect(ray, t_max),
        }
    }

    fn check_intersect(&self, ray: &Ray, t_max: f32) -> bool {
        match self {
            PrimitiveEnum::Simple(prim) => prim.check_intersect(ray, t_max),
            PrimitiveEnum::Geometric(prim) => prim.check_intersect(ray, t_max),
            PrimitiveEnum::BVH(prim) => prim.check_intersect(ray, t_max),
        }
    }
}

impl Bounded<f32> for PrimitiveEnum {
    fn bound(&self) -> Aabb<f32> {
        match self {
            PrimitiveEnum::Simple(prim) => prim.bound(),
            PrimitiveEnum::Geometric(prim) => prim.bound(),
            PrimitiveEnum::BVH(prim) => prim.bound(),
        }
    }
}
