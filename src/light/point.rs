use std::f32::consts::PI;

use image::{Pixel, Rgb};

use crate::{
    core::SurfaceInteraction,
    light::{base::BaseLight, Light, LightSample, LightType},
    math::{Normed, Transform, Transformable},
    point3, Point2f,
};

#[derive(Debug)]
pub struct PointLight {
    base: BaseLight,
    spectrum: Rgb<f32>,
    scale: f32,
}

impl PointLight {
    pub fn new(spectrum: Rgb<f32>, scale: f32, light_to_render: Transform<f32>) -> Self {
        PointLight {
            spectrum,
            scale,
            base: BaseLight {
                light_to_render,
                light_type: LightType::DeltaPosition,
            },
        }
    }
}

impl Light for PointLight {
    fn flux(&self) -> Rgb<f32> { self.spectrum.map(|x| 4. * PI * x * self.scale) }

    fn light_type(&self) -> LightType { self.base.light_type }

    fn sample_light(&self, surf_int: &SurfaceInteraction, rnd_p: Point2f) -> Option<LightSample> {
        let point = point3!(0., 0., 0.).transform(&self.base.light_to_render);
        let vec = point - surf_int.hit.point;
        let incoming = vec.to_unit();
        let distance = vec.len_squared();
        let radiance = self.spectrum.map(|x| x * self.scale / distance);
        Some(LightSample {
            radiance,
            incoming,
            pdf: 1.0,
            point,
        })
    }
}
