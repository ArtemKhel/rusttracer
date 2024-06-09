use crate::{light::LightType, math::Transform};

#[derive(Debug)]
pub(super) struct BaseLight {
    pub(super) light_type: LightType,
    // TODO: keep them in world space?
    pub(super) light_to_render: Transform<f32>,
}
