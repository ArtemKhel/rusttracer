use geometry::hit::Hit;

use crate::material::Material;
use crate::primitive::Primitive;

pub struct Intersection {
    pub hit: Hit,
    // pub primitive: Box<Primitive>,
    pub material: Material,
}
