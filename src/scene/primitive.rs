use math::{Bounded, BoundedIntersectable, Intersectable, Shape};

use crate::{material::Material, Aabb, Hit, Ray, F};

#[derive(Debug)]
pub struct Primitive {
    pub shape: Box<dyn BoundedIntersectable<F>>,
    pub material: Box<dyn Material>,
}

#[derive(Debug)]
pub struct Composite {
    pub objects: Vec<Box<dyn BoundedIntersectable<F>>>,
}

impl Bounded<F> for Composite {
    fn bound(&self) -> Aabb { self.objects.iter().fold(Aabb::default(), |acc, x| acc + x.bound()) }
}
impl Intersectable<F> for Composite {
    fn hit(&self, ray: &Ray) -> Option<Hit> { self.objects.iter().filter_map(|x| x.hit(ray)).min() }
}

impl Bounded<F> for Primitive {
    fn bound(&self) -> Aabb { self.shape.bound() }
}
impl Intersectable<F> for Primitive {
    fn hit(&self, ray: &Ray) -> Option<Hit> { self.shape.hit(ray) }
}
