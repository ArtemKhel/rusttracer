use std::{f32::consts::PI, sync::Arc};

use image::{Pixel, Rgb};

use crate::{
    core::SurfaceInteraction,
    light::{base::BaseLight, Light, LightSample, LightType},
    math::{Normed, Transform},
    shapes::{BoundedIntersectable, Samplable},
    spectra::{Spectrum, SpectrumEnum},
    Point2f, SampledSpectrum, SampledWavelengths,
};

// TODO: emit on one side only? fix flux()
//       emission from texture
//       alpha

#[derive(Debug)]
pub struct DiffuseAreaLight {
    base: BaseLight,
    spectrum: Arc<SpectrumEnum>,
    scale: f32,
    shape: Arc<dyn BoundedIntersectable>,
    area: f32,
}

impl DiffuseAreaLight {
    pub fn new(
        spectrum: Arc<SpectrumEnum>,
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
    fn flux(&self, lambda: &SampledWavelengths) -> SampledSpectrum {
        2. * PI * self.scale * self.spectrum.sample(lambda)
    }

    fn light_type(&self) -> LightType { LightType::Area }

    fn sample_light(
        &self,
        surf_int: &SurfaceInteraction,
        lambda: &SampledWavelengths,
        rnd_p: Point2f,
    ) -> Option<LightSample> {
        let shape_sample = self.shape.sample_from_point(surf_int.hit.point, rnd_p)?;

        // TODO: mediums
        if shape_sample.pdf == 0. {
            return None;
        };

        let incoming = (shape_sample.hit.point - surf_int.hit.point).to_unit();

        self.radiance(surf_int, lambda).map(|emitted| LightSample {
            radiance: emitted,
            incoming,
            pdf: shape_sample.pdf,
            point: shape_sample.hit.point,
        })
    }

    fn radiance(&self, surf_int: &SurfaceInteraction, lambda: &SampledWavelengths) -> Option<SampledSpectrum> {
        Some(self.spectrum.sample(lambda) * self.scale)
    }
}
