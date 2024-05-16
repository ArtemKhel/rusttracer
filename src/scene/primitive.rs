use log::debug;

use crate::{
    geometry::{Aabb, Bounded, BoundedIntersectable, Hit, Intersectable, Ray},
    material::Material,
};

#[derive(Debug)]
pub struct Primitive {
    pub shape: Box<dyn BoundedIntersectable>,
    pub material: Box<dyn Material>,
}

#[derive(Debug)]
pub struct Composite {
    pub objects: Vec<Box<dyn BoundedIntersectable>>,
}

impl Bounded for Composite {
    fn bound(&self) -> Aabb { self.objects.iter().fold(Aabb::default(), |acc, x| acc + x.bound()) }
}
impl Intersectable for Composite {
    fn hit(&self, ray: &Ray) -> Option<Hit> { self.objects.iter().filter_map(|x| x.hit(ray)).min() }
}

impl Bounded for Primitive {
    fn bound(&self) -> Aabb { self.shape.bound() }
}
impl Intersectable for Primitive {
    fn hit(&self, ray: &Ray) -> Option<Hit> { self.shape.hit(ray) }
}
