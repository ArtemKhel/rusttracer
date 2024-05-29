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
    // pub objects: Vec<Primitive>,
    pub objects: PrimitiveEnum,
    // pub materials: Vec<Box<dyn Material>>,
    // lights:
    pub background_color: Rgb<f32>,
}

impl Scene {
    pub fn cast_ray(&self, ray: &Ray) -> Option<SurfaceInteraction> {
        self.objects.intersect(ray, f32::INFINITY)
        // self.objects
        //     .iter()
        //     .filter_map(|obj| obj.shape.hit(ray).map(|hit| Intersection { hit, object: obj }))
        //     .min()
    }
}
