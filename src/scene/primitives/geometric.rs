use std::sync::Arc;

use derive_new::new;

use crate::{
    aggregates::{Aabb, Bounded},
    core::{Ray, SurfaceInteraction},
    light::LightEnum,
    material::MaterialsEnum,
    scene::primitives::{simple::SimplePrimitive, Primitive},
    shapes::{BoundedIntersectable, Intersectable},
};

#[derive(Debug)]
#[derive(new)]
pub struct GeometricPrimitive {
    pub shape: Arc<dyn BoundedIntersectable>,
    pub material: Arc<MaterialsEnum>,
    pub light: Option<Arc<LightEnum>>,
    // medium_interface
    // alpha
}

unsafe impl Send for GeometricPrimitive {}
unsafe impl Sync for GeometricPrimitive {}

impl Bounded<f32> for GeometricPrimitive {
    fn bound(&self) -> Aabb<f32> { self.shape.bound() }
}

impl Intersectable for GeometricPrimitive {
    fn intersect(&self, ray: &Ray, t_max: f32) -> Option<SurfaceInteraction> {
        if let Some(mut interaction) = self.shape.intersect(ray, t_max) {
            interaction.set_material_properties(&self.material, self.light.as_ref());
            Some(interaction)
        } else {
            None
        }
    }

    fn check_intersect(&self, ray: &Ray, t_max: f32) -> bool { self.shape.check_intersect(ray, t_max) }
}

impl Primitive for GeometricPrimitive {}
