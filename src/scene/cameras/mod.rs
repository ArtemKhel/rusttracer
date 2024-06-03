pub mod base;
pub mod orthographic;
pub mod projective;

use std::{cmp::max, ops::Mul, sync::Arc};

use crate::{
    core::{ray::RayDifferential, Ray},
    math::{utils::random_in_unit_disk, *},
    point3,
    scene::{
        cameras::{orthographic::OrthographicCamera, projective::ProjectiveCamera},
        film::RGBFilm,
    },
    vec3, Normal3f, Point2f, Point3f, Vec3f,
};

pub type PixelCoord = [f32; 2];

#[enum_delegate::register]
pub trait Camera {
    /// Cast ray corresponding to a given [PixelCoord]
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
}

#[derive(Debug, Copy, Clone)]
pub struct CameraSample {
    pub p_film: Point2f,
    pub p_lens: Point2f,
}
