use std::sync::Arc;

pub use base::BaseCameraConfig;
pub use orthographic::{OrthographicCamera, OrthographicCameraConfig};
pub use perspective::{PerspectiveCamera, PerspectiveCameraConfig};

use crate::{
    core::Ray,
    samplers::{Sampler, SamplerType},
    scene::film::RGBFilm,
    Normal3f, Point2f, Point2us, Point3f, Vec3f,
};

mod base;
mod orthographic;
mod perspective;
mod projective;

#[enum_delegate::register]
pub trait Camera {
    /// Cast ray corresponding to a given [CameraSample]
    fn generate_ray(&self, sample: CameraSample) -> Ray;
    /// Same as [Self::generate_ray], but also fills [Ray].diff option with 2
    /// rays with one pixel offset
    fn generate_differential_ray(&self, sample: CameraSample) -> Ray;

    /// Returns an approximation for dp_dx, dp_dy for a point in the scene
    fn approximate_dp_dxy(&self, point: Point3f, normal: Normal3f, samples_per_pixel: u32) -> (Vec3f, Vec3f);

    fn get_film(&self) -> Arc<RGBFilm>;
}

#[enum_delegate::implement(Camera)]
pub enum CameraType {
    Orthographic(OrthographicCamera),
    Perspective(PerspectiveCamera),
}

#[derive(Debug, Copy, Clone)]
pub struct CameraSample {
    pub p_film: Point2f,
    pub p_lens: Point2f,
}

impl CameraSample {
    pub fn new(pixel: Point2us, sampler: &mut SamplerType) -> Self {
        // TODO: filters.
        // Offset from discrete pixels to continuous one
        // Disc. |---0---|---1---|---2---|
        // Cont. 0-------1-------2-------3
        let p_film = pixel.map(|x| x as f32) + *sampler.get_2d();
        let p_lens = sampler.get_2d();
        CameraSample { p_film, p_lens }
    }
}
