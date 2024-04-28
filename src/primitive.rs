use crate::material::Material;
use geometry::Object;

pub struct Primitive {
    pub object: Box<dyn Object>,
    pub material: Material,
}
