use std::fmt::Debug;

use crate::{
    aggregates::Bounded,
    core::{Interaction, Ray, SurfaceInteraction},
    math::{Number, Unit},
    Point2f, Point3f, Vec3f,
};

pub mod mesh;
pub mod quad;
pub mod sphere;
// mod mesh_new;

pub trait Intersectable {
    fn intersect(&self, ray: &Ray, t_max: f32) -> Option<SurfaceInteraction>;
    fn check_intersect(&self, ray: &Ray, t_max: f32) -> bool;
}

pub struct ShapeSample {
    pub hit: Interaction,
    pub pdf: f32,
}

pub trait Samplable {
    /// Uniformly samples a point on the surface
    fn sample(&self, rnd_p: Point2f) -> Option<ShapeSample>;
    
    /// Same as [sample], but only samples surface visible from point
    fn sample_from_point(&self, point: Point3f, rnd_p: Point2f) -> Option<ShapeSample>;
    
    fn pdf(&self, interaction: &Interaction) -> f32;
    
    // Returns the shape’s probability of sampling a point on the light such that the incident direction at the
    // reference point is `incoming`
    fn pdf_incoming(&self, interaction: &SurfaceInteraction, incoming: Unit<Vec3f>) -> f32;
    
    /// Surface area of the object
    fn area(&self) -> f32;
}

pub trait BoundedIntersectable: Bounded<f32> + Intersectable + Samplable + Debug {}

impl<Shape> BoundedIntersectable for Shape where Shape: Bounded<f32> + Intersectable + Samplable + Debug {}
