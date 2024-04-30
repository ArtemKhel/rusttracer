use crate::intersection::Intersection;
use crate::material::Material;
use crate::object::Object;
use geometry::ray::Ray;
use crate::camera::Camera;

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Object>,
    pub materials: Vec<Box<dyn Material>>,
    // lights:
}
impl Scene {
    pub fn cast_ray(&self, ray: &Ray) -> Option<Intersection> {
        self.objects
            .iter()
            .filter_map(|obj| obj.shape.hit(ray).map(|hit| Intersection { hit, object: obj }))
            .min()
    }
}
