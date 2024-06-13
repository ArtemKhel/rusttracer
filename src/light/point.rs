use std::{f32::consts::PI, sync::Arc};

use image::{Pixel, Rgb};

use crate::{
    core::SurfaceInteraction,
    light::{base::BaseLight, Light, LightSample, LightType},
    math::{Normed, Transform, Transformable},
    point3,
    spectra::{Spectrum, SpectrumEnum},
    Point2f, SampledSpectrum, SampledWavelengths,
};

#[derive(Debug)]
pub struct PointLight {
    base: BaseLight,
    spectrum: Arc<SpectrumEnum>,
    scale: f32,
}

impl PointLight {
    // todo: new from rgb?
    pub fn new(spectrum: Arc<SpectrumEnum>, scale: f32, light_to_render: Transform<f32>) -> Self {
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
    fn flux(&self, lambda: &SampledWavelengths) -> SampledSpectrum {
        4. * PI * self.scale * self.spectrum.sample(lambda)
    }

    fn light_type(&self) -> LightType { self.base.light_type }

    fn sample(
        &self,
        surf_int: &SurfaceInteraction,
        lambda: &SampledWavelengths,
        rnd_p: Point2f,
    ) -> Option<LightSample> {
        let point = point3!(0., 0., 0.).transform(&self.base.light_to_render);
        let vec = point - surf_int.hit.point;
        let incoming = vec.to_unit();
        let distance_sqr = vec.len_squared();
        // TODO: more gradual fall-off?
        let radiance = self.spectrum.sample(lambda) * self.scale / distance_sqr;
        Some(LightSample {
            radiance,
            incoming,
            pdf: 1.0,
            point,
        })
    }
}
