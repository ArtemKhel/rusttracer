use crate::{
    geometry::Ray,
    material::Material,
    scene::{camera::Camera, intersection::Intersection, object::Object},
};

pub struct Scene {
    pub camera: Camera,
    pub objects: Vec<Object>,
    // pub objects: ObjectList
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
