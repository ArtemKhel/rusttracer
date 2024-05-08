use crate::{
    geometry::{Aabb, Bounded, BoundedIntersectable, Hit, Intersectable, Ray},
    material::Material,
};

#[derive(Debug)]
pub struct Primitive {
    pub shape: Box<dyn BoundedIntersectable>,
    pub material: Box<dyn Material>,
}

impl Bounded for Primitive {
    fn bound(&self) -> Aabb { self.shape.bound() }
}
impl Intersectable for Primitive {
    fn hit(&self, ray: &Ray) -> Option<Hit> { self.shape.hit(ray) }
}
