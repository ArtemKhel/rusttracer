use std::sync::Arc;

use derive_new::new;

use crate::{
    aggregates::{Aabb, Bounded},
    core::{Ray, SurfaceInteraction},
    material::MaterialsEnum,
    scene::primitives::Primitive,
    shapes::{BoundedIntersectable, Intersectable},
};

#[derive(Debug)]
#[derive(new)]
pub struct SimplePrimitive {
    pub shape: Arc<dyn BoundedIntersectable>,
    pub material: Arc<MaterialsEnum>,
}

impl Bounded<f32> for SimplePrimitive {
    fn bound(&self) -> Aabb<f32> { self.shape.bound() }
}

impl Intersectable for SimplePrimitive {
    fn intersect(&self, ray: &Ray, t_max: f32) -> Option<SurfaceInteraction> {
        if let Some(mut interaction) = self.shape.intersect(ray, t_max) {
            interaction.set_material_properties(&self.material, None);
            Some(interaction)
        } else {
            None
        }
    }

    fn check_intersect(&self, ray: &Ray, t_max: f32) -> bool { self.shape.check_intersect(ray, t_max) }
}

impl Primitive for SimplePrimitive {}
