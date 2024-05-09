use crate::{
    aggregates::BVH,
    geometry::{Hit, Intersectable, Ray},
    material::Material,
    scene::{camera::Camera, intersection::Intersection, primitive::Primitive},
};

pub struct Scene {
    pub camera: Camera,
    // pub objects: Vec<Primitive>,
    pub objects: BVH,
    pub materials: Vec<Box<dyn Material>>,
    // lights:
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
