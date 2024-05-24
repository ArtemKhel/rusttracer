use image::Rgb;
use num_traits::Zero;

use crate::{
    bxdf::bxdf::{BxDF, Shading},
    colors,
    math::Frame,
    Point2f, Vec3f,
};

pub struct BSDF {
    bxdf: Box<dyn BxDF>,
    shading_frame: Frame<f32>,
}

#[derive(Debug, Copy, Clone)]
pub struct BSDFSample {
    color: Rgb<f32>,
    incoming: Vec3f,
    pdf: f32,
}

impl BSDF {
    pub fn eval(&self, incoming: Vec3f, outgoing: Vec3f) -> Rgb<f32> {
        // TODO: normalized?
        let s_in = self.render_to_shading(incoming);
        let s_out = self.render_to_shading(outgoing);
        if s_out.z == 0.0 {
            return colors::BLACK;
        }
        self.bxdf.eval(s_in, s_out)
    }

    // TODO: zero-valued instead of Option?
    pub fn sample(&self, point: Point2f, outgoing: Vec3f) -> Option<BSDFSample> {
        let s_out = self.render_to_shading(outgoing);
        if s_out.z == 0.0
        /*TODO flags here*/
        {
            return None;
        }
        if let Some(mut sample) = self.bxdf.sample(point, s_out) {
            if sample.pdf == 0.0 || sample.incoming.z == 0.0
            /* || RGB==0?! */
            {
                None
            } else {
                // TODO: how to convert efficiently and type-safe?
                Some(BSDFSample {
                    color: sample.color,
                    incoming: self.shading_to_render(sample.incoming),
                    pdf: sample.pdf,
                })
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
        self.shading_frame.to_local::<Shading<Vec3f>>(vec3f as _)
    }

    fn shading_to_render(&self, vec3f: Shading<Vec3f>) -> Vec3f { self.shading_frame.from_local(vec3f) }
}
