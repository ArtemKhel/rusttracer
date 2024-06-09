use std::sync::Arc;

use image::Rgb;

use crate::{
    aggregates::BVH,
    core::{Ray, SurfaceInteraction},
    light::{Light, LightEnum},
    material::Material,
    scene::{cameras::CameraType, primitives::PrimitiveEnum},
    shapes::Intersectable,
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
    // pub fn cast_ray_pred(&self, ray: &Ray) -> bool { self.objects.intersect(ray, f32::INFINITY) }
}
