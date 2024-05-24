use crate::{
    aggregates::Aabb,
    core::{Hit, Ray},
    material::Material,
    shapes::{Bounded, BoundedIntersectable, Intersectable},
};

#[derive(Debug)]
pub struct Primitive {
    pub shape: Box<dyn BoundedIntersectable<f32>>,
    pub material: Box<dyn Material>,
}

#[derive(Debug)]
pub struct Composite {
    pub objects: Vec<Box<dyn BoundedIntersectable<f32>>>,
}

impl Bounded<f32> for Composite {
    fn bound(&self) -> Aabb<f32> { self.objects.iter().fold(Aabb::default(), |acc, x| acc + x.bound()) }
}

impl Intersectable<f32> for Composite {
    fn hit(&self, ray: &Ray) -> Option<Hit> { self.objects.iter().filter_map(|x| x.hit(ray)).min() }
}

impl Bounded<f32> for Primitive {
    fn bound(&self) -> Aabb<f32> { self.shape.bound() }
}

impl Intersectable<f32> for Primitive {
    fn hit(&self, ray: &Ray) -> Option<Hit> { self.shape.hit(ray) }
}
