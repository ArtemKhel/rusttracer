use std::{f32::consts::PI, sync::Arc};

use image::{Pixel, Rgb};

use crate::{
    colors,
    core::SurfaceInteraction,
    light::{base::BaseLight, Light, LightSample, LightType},
    math::{Normed, Transform},
    shapes::{BoundedIntersectable, Samplable},
    Point2f,
};

// TODO: emit on one side only? fix flux()
//       emission from texture
//       alpha

#[derive(Debug)]
pub struct DiffuseAreaLight {
    base: BaseLight,
    spectrum: Rgb<f32>,
    scale: f32,
    shape: Arc<dyn BoundedIntersectable>,
    area: f32,
}

impl DiffuseAreaLight {
    pub fn new(
        spectrum: Rgb<f32>,
        scale: f32,
        light_to_render: Transform<f32>,
        shape: Arc<dyn BoundedIntersectable>,
    ) -> Self {
        DiffuseAreaLight {
            base: BaseLight {
                light_type: LightType::Area,
                light_to_render,
            },
            spectrum,
            scale,
            area: shape.area(),
            shape,
        }
    }
}

impl Light for DiffuseAreaLight {
    fn flux(&self) -> Rgb<f32> { self.spectrum.map(|x| x * self.scale * PI * 2.) }

    fn light_type(&self) -> LightType { LightType::Area }

    fn sample_light(&self, surf_int: &SurfaceInteraction, sample_p: Point2f) -> Option<LightSample> {
        let shape_sample = self.shape.sample_from_point(surf_int.hit.point, sample_p)?;

        // TODO: mediums
        if shape_sample.pdf == 0. {
            return None;
        };

        let incoming = (shape_sample.hit.point - surf_int.hit.point).to_unit();

        self.radiance(surf_int).and_then(|emitted| {
            Some(LightSample {
                radiance: emitted,
                incoming,
                pdf: shape_sample.pdf,
                point: shape_sample.hit.point,
            })
        })
    }

    fn radiance(&self, surf_int: &SurfaceInteraction) -> Option<Rgb<f32>> {
        Some(self.spectrum.map(|x| x * self.scale))
    }
}
