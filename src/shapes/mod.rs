use std::fmt::Debug;

use crate::{aggregates::Bounded, core::{Interaction, Ray, SurfaceInteraction}, math::Number, Point2f, Point3f};

pub mod mesh;
// pub mod quad;
pub mod sphere;
// mod mesh_new;

pub trait Intersectable {
    fn intersect(&self, ray: &Ray, t_max: f32) -> Option<SurfaceInteraction>;
}

pub struct ShapeSample {
    interaction: Interaction,
    pdf: f32,
}

pub trait Samplable {
    /// Uniformly samples a point on the surface
    fn sample(&self, sample: Point2f) -> Option<ShapeSample>;
    fn sample_from_point(&self, point: Point3f, sample: Point2f) -> Option<ShapeSample>;
    // TODO: pdf_from_point?
    fn pdf(&self, interaction: &Interaction) -> f32;
    /// Surface area of the object
    fn area(&self) -> f32;
}

pub trait BoundedIntersectable: Bounded<f32> + Intersectable + Debug{}

impl<Shape> BoundedIntersectable for Shape where Shape: Bounded<f32> + Intersectable + Debug{}
