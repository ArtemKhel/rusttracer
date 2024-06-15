use std::{f32::consts::PI, sync::Arc};

use crate::{
    core::SurfaceInteraction,
    light::{base::BaseLight, Light, LightSample, LightType},
    math::{utils::lerp, Normed, Transform, Transformable, Unit},
    point3,
    spectra::{Spectrum, SpectrumEnum},
    Point2f, SampledSpectrum, SampledWavelengths, Vec3f,
};

#[derive(Debug)]
pub struct Spotlight {
    base: BaseLight,
    spectrum: Arc<SpectrumEnum>,
    scale: f32,
    cos_falloff_start: f32,
    cos_falloff_end: f32,
}

impl Spotlight {
    pub fn new(
        spectrum: Arc<SpectrumEnum>,
        scale: f32,
        light_to_world: Transform<f32>,
        falloff_start: f32,
        falloff_end: f32,
    ) -> Self {
        assert!(falloff_start <= falloff_end);
        let base = BaseLight {
            light_type: LightType::DeltaPosition,
            light_to_render: light_to_world,
        };
        Spotlight {
            base,
            spectrum,
            scale,
            cos_falloff_start: falloff_start.to_radians().cos(),
            cos_falloff_end: falloff_end.to_radians().cos(),
        }
    }

    fn falloff(&self, to_object: Unit<Vec3f>) -> f32 {
        // TODO: make cos_theta public?
        //       smooth
        let cos = to_object.z;
        if cos > self.cos_falloff_start {
            1.
        } else if cos < self.cos_falloff_end {
            0.
        } else {
            let t = (cos - self.cos_falloff_end) / (self.cos_falloff_start - self.cos_falloff_end);
            lerp(0., 1., t.powi(2))
        }
    }
}

impl Light for Spotlight {
    fn flux(&self, lambda: &SampledWavelengths) -> SampledSpectrum {
        self.spectrum.sample(lambda)
            * 2.
            * PI
            * self.scale
            * ((1. - self.cos_falloff_start) + (self.cos_falloff_start - self.cos_falloff_end) / 2.)
    }

    fn light_type(&self) -> LightType { self.base.light_type }

    fn sample(
        &self,
        surf_int: &SurfaceInteraction,
        lambda: &SampledWavelengths,
        rnd_p: Point2f,
    ) -> Option<LightSample> {
        let point = point3!(0., 0., 0.).transform(&self.base.light_to_render);
        let to_light = point - surf_int.hit.point;
        let to_object = (-to_light).inv_transform(&self.base.light_to_render).to_unit();
        let falloff = self.falloff(to_object);
        let radiance = self.spectrum.sample(lambda) * self.scale * falloff;
        Some(LightSample {
            radiance,
            incoming: to_light.to_unit(),
            pdf: 1.0,
            point,
        })
    }

    fn pdf_incoming(&self, incoming: Unit<Vec3f>, surf_int: &SurfaceInteraction) -> f32 { 0. }
}
