use std::sync::Arc;

use derive_new::new;

use crate::{
    aggregates::{Aabb, Bounded, BVH},
    core::{Ray, SurfaceInteraction},
    material::MaterialsEnum,
    shapes::{BoundedIntersectable, Intersectable},
};

pub trait Primitive: Bounded<f32> + Intersectable {}

impl Primitive for BVH<f32> {}

#[derive(Debug)]
pub enum PrimitiveEnum {
    Simple(SimplePrimitive),
    BVH(BVH<f32>),
}

#[derive(Debug)]
#[derive(new)]
pub struct SimplePrimitive {
    pub shape: Arc<dyn BoundedIntersectable>,
    pub material: Arc<MaterialsEnum>,
    // light
    // medium interface
    // alpha
}

impl Primitive for SimplePrimitive {}

impl Intersectable for PrimitiveEnum {
    fn intersect(&self, ray: &Ray, t_max: f32) -> Option<SurfaceInteraction> {
        match self {
            PrimitiveEnum::Simple(prim) => prim.intersect(ray, t_max),
            PrimitiveEnum::BVH(prim) => prim.intersect(ray, t_max),
        }
    }
}

impl Bounded<f32> for PrimitiveEnum {
    fn bound(&self) -> Aabb<f32> {
        match self {
            PrimitiveEnum::Simple(prim) => prim.bound(),
            PrimitiveEnum::BVH(prim) => prim.bound(),
        }
    }
}

// TODO: GeometricPrimitive

impl Intersectable for SimplePrimitive {
    fn intersect(&self, ray: &Ray, t_max: f32) -> Option<SurfaceInteraction> {
        if let Some(mut interaction) = self.shape.intersect(ray, t_max) {
            interaction.set_material_properties(self.material.clone());
            Some(interaction)
        } else {
            None
        }
    }
}

impl Bounded<f32> for SimplePrimitive {
    fn bound(&self) -> Aabb<f32> { self.shape.bound() }
}

// #[derive(Debug)]
// pub struct Primitive {
//     pub shape: Box<dyn BoundedIntersectable<f32>>,
//     pub material: MaterialsEnum,
// }

// impl Bounded<f32> for Primitive {
//     fn bound(&self) -> Aabb<f32> { self.shape.bound() }
// }
//
// impl Intersectable for Primitive {
//     fn intersect(&self, ray: &Ray, t_max: f32) -> Option<SurfaceInteraction>
// { self.shape.intersect(ray, t_max) } }

// #[derive(Debug)]
// pub struct Composite {
//     pub objects: Vec<Box<dyn BoundedIntersectable<f32>>>,
// }
//
// impl Bounded<f32> for Composite {
//     fn bound(&self) -> Aabb<f32> { self.objects.iter().fold(Aabb::default(),
// |acc, x| acc + x.bound()) } }
//
// impl Intersectable for Composite {
//     fn hit(&self, ray: &Ray) -> Option<Hit> {
// self.objects.iter().filter_map(|x| x.hit(ray)).min() } }
