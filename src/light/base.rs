use crate::{light::LightType, math::Transform};

#[derive(Debug)]
pub(super) struct BaseLight {
    pub(super) light_type: LightType,
    // TODO:
    pub(super) light_to_render: Transform<f32>,
}
