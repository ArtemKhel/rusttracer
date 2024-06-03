use image::Rgb;

use crate::{
    aggregates::BVH,
    core::{Ray, SurfaceInteraction},
    material::Material,
    scene::{cameras::CameraType, PrimitiveEnum},
    shapes::Intersectable,
};

pub struct Scene {
    pub camera: CameraType,
    pub objects: PrimitiveEnum,
    // pub materials: ???
    // pub lights: ???
    pub background_color: Rgb<f32>,
}

impl Scene {
    pub fn cast_ray(&self, ray: &Ray) -> Option<SurfaceInteraction> { self.objects.intersect(ray, f32::INFINITY) }
}
