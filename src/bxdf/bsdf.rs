use std::ops::Deref;

use image::Rgb;

use crate::{
    bxdf::bxdf::{BxDF, BxDFFlags, Shading},
    colors,
    math::Frame,
    Point2f, Vec3f,
};

pub struct BSDF {
    bxdf: Box<dyn BxDF>,
    shading_frame: Frame<f32>,
}

#[derive(Debug, Copy, Clone)]
pub struct BSDFSample<T> {
    pub color: Rgb<f32>,
    pub incoming: T,
    pub pdf: f32,
    pub eta: f32,
    pub flags: BxDFFlags,
}

impl<T> BSDFSample<T> {
    pub(crate) fn new(color: Rgb<f32>, incoming: T, pdf: f32, flags: BxDFFlags) -> Self {
        BSDFSample {
            color,
            incoming,
            pdf,
            eta: 1.0,
            flags,
        }
    }
}

impl BSDF {
    // TODO: shading
    pub fn new(shading_normal: Vec3f, shading_dp_du: Vec3f, bxdf: Box<dyn BxDF>) -> Self {
        // TODO: frame sometimes with NaNs
        Self {
            bxdf,
            shading_frame: Frame::from_x_z(shading_dp_du, shading_normal),
        }
    }

    pub fn eval(&self, incoming: Vec3f, outgoing: Vec3f) -> Rgb<f32> {
        // TODO: normalized?
        let s_in = self.render_to_shading(incoming);
        let s_out = self.render_to_shading(outgoing);
        if s_out.z == 0.0 {
            return colors::BLACK;
        }
        self.bxdf.eval(s_in, s_out)
    }

    pub fn sample(&self, outgoing: Vec3f, sample_p: Point2f, sample_c: f32) -> Option<BSDFSample<Vec3f>> {
        let s_out = self.render_to_shading(outgoing);
        if s_out.z == 0.0
        /* TODO flags here */
        {
            return None;
        }
        if let Some(mut sample) = self.bxdf.sample(sample_p, sample_c, s_out) {
            if sample.pdf == 0.0 || sample.incoming.z == 0.0
            /* || RGB==0 */
            {
                None
            } else {
                Some(BSDFSample::new(
                    sample.color,
                    self.shading_to_render(sample.incoming),
                    sample.pdf,
                    self.bxdf.flags(),
                ))
            }
        } else {
            None
        }
    }

    pub fn pdf(&self, incoming: Vec3f, outgoing: Vec3f) -> f32 {
        let s_in = self.render_to_shading(incoming);
        let s_out = self.render_to_shading(outgoing);
        if s_out.z == 0.0 {
            return 0.0;
        }
        self.bxdf.pdf(s_in, s_out)
    }

    fn render_to_shading(&self, vec3f: Vec3f) -> Shading<Vec3f> {
        self.shading_frame.to_local_wrap::<Shading<Vec3f>>(vec3f as _)
    }

    fn shading_to_render(&self, vec3f: Shading<Vec3f>) -> Vec3f { self.shading_frame.from_local_unwrap(vec3f) }
}
