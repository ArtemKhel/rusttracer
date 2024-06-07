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
        let ss = self.shape.sample_from_point(surf_int.hit.point, sample_p);
        if let Some(shape_sample) = ss {
            // TODO: mediums
            if shape_sample.pdf == 0. {
                return None;
            }
            let incoming = (shape_sample.hit.point - surf_int.hit.point).to_unit();
            let emitted = self.radiance(&surf_int);
            if emitted == colors::BLACK {
                None
            } else {
                Some(LightSample {
                    radiance: emitted,
                    incoming,
                    pdf: shape_sample.pdf,
                    point: shape_sample.hit.point,
                })
            }
        } else {
            None
        }
    }

    fn radiance(&self, surf_int: &SurfaceInteraction) -> Rgb<f32> { self.spectrum.map(|x| x * self.scale) }
}
