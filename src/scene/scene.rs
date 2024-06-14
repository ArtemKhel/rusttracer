use std::sync::Arc;

use image::Rgb;

use crate::{
    aggregates::BVH,
    core::{Ray, SurfaceInteraction},
    light::{Light, LightEnum},
    material::Material,
    math::{Normed, Unit},
    ray,
    scene::{cameras::CameraType, primitives::PrimitiveEnum},
    shapes::Intersectable,
    Point3f, Vec3f,
};

pub struct Scene {
    pub camera: CameraType,
    pub objects: PrimitiveEnum,
    // pub materials: ???
    pub lights: Vec<Arc<LightEnum>>,
    // pub background_color: Rgb<f32>,
}

impl Scene {
    pub fn cast_ray(&self, ray: &Ray) -> Option<SurfaceInteraction> { self.objects.intersect(ray, f32::INFINITY) }

    pub fn cast_bounded_ray(&self, ray: &Ray, t_max: f32) -> Option<SurfaceInteraction> {
        self.objects.intersect(ray, t_max)
    }

    pub fn unoccluded(&self, from: Point3f, dir: Unit<Vec3f>, to: Point3f) -> bool {
        // Technically, `dir` is redundant, but it is already available from LightSample.
        // TODO: add id to objects and check intersection with it?
        let ray = ray!(from + dir * 1e-3, dir);
        let t_max = (to - from).len() * 0.99;
        !self.objects.check_intersect(&ray, t_max)
    }
}
