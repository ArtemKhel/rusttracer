use crate::intersection::Intersection;
use crate::material::Material;
use crate::object::Object;
use geometry::point::Point;
use geometry::ray::Ray;

pub struct Camera {
    pub position: Point,
    pub look_at: Point,
    pub focal_length: f32,
    // pub config: CameraConfig,
}
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
