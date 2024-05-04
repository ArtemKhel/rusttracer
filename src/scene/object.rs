use crate::{geometry::Intersect, material::Material};
pub struct Object {
    pub shape: Box<dyn Intersect>,
    pub material: Box<dyn Material>,
}
