use geometry::Intersect;

use crate::material::Material;
pub struct Object {
    pub shape: Box<dyn Intersect>,
    pub material: Box<dyn Material>,
}
