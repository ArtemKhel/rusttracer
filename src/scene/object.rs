use crate::material::Material;
use geometry::Intersect;
pub struct Object {
    pub shape: Box<dyn Intersect>,
    pub material: Box<dyn Material>,
}
