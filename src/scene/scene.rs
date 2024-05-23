use image::Rgb;

use crate::{
    aggregates::BVH,
    material::Material,
    scene::{Intersection, SimpleCamera},
    Ray, F,
};

pub struct Scene {
    pub camera: SimpleCamera,
    // pub objects: Vec<Primitive>,
    pub objects: BVH<F>,
    pub materials: Vec<Box<dyn Material>>,
    // lights:
    pub background_color: Rgb<f32>,
}

impl Scene {
    pub fn cast_ray(&self, ray: &Ray) -> Option<Intersection> {
        self.objects.hit(ray)
        // self.objects
        //     .iter()
        //     .filter_map(|obj| obj.shape.hit(ray).map(|hit| Intersection { hit, object: obj }))
        //     .min()
    }
}
