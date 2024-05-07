use crate::{
    geometry::{Bounded, BoundedIntersectable, Intersectable, AABB},
    material::Material,
};
use crate::geometry::{Hit, Ray};

#[derive(Debug)]
pub struct Primitive {
    pub shape: Box<dyn BoundedIntersectable>,
    pub material: Box<dyn Material>,
}

impl Bounded for Primitive {
    fn bound(&self) -> AABB { self.shape.bound() }
}
impl Intersectable for Primitive{
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        self.shape.hit(ray)
    }
}
