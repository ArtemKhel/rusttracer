use crate::material::Material;
use crate::scene::camera::Camera;
use crate::scene::intersection::Intersection;
use crate::scene::object::Object;
use geometry::Ray;

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
