use std::sync::Arc;

use crate::{
    core::{ray::RayDifferential, Ray},
    math::{Normed, Transform, Transformable},
    ray,
    samplers::utils::sample_uniform_disk_concentric,
    scene::{
        cameras::{
            base::BaseCameraConfig,
            projective::{ProjectiveCamera, ProjectiveCameraConfig},
            Camera, CameraSample, CameraType,
        },
        film::RGBFilm,
    },
    unit_vec3, vec3, Bounds2f, Normal3f, Point2f, Point3f, Vec3f,
};

pub struct OrthographicCamera {
    projective: ProjectiveCamera,
    dx_camera: Vec3f,
    dy_camera: Vec3f,
}

pub struct OrthographicCameraConfig {
    pub base_config: BaseCameraConfig,
    /// Screen bounds in film plane. (0,0) is the camera position
    pub screen_window: Bounds2f,
    pub lens_radius: f32,
    pub focal_distance: f32,
}

impl OrthographicCamera {
    pub fn new(config: OrthographicCameraConfig) -> Self { OrthographicCamera::from(config) }

    fn generate_camera_space_ray(&self, sample: CameraSample) -> Ray {
        let point_raster: Point3f = sample.p_film.into();
        let point_camera = point_raster.transform(&self.projective.raster_to_camera);
        let mut ray = ray!(point_camera, unit_vec3!(0., 0., 1.));
        self.projective.adjust_for_dof(&mut ray, sample.p_lens);
        ray
    }
}

impl Camera for OrthographicCamera {
    // TODO: camera ray wrapper?
    fn generate_ray(&self, sample: CameraSample) -> Ray {
        let ray = self.generate_camera_space_ray(sample);
        ray.transform(&self.projective.base.camera_to_world)
    }

    fn generate_differential_ray(&self, sample: CameraSample) -> Ray {
        let mut ray = self.generate_camera_space_ray(sample);

        if self.projective.lens_radius > 0. {
            // TODO
        } else {
            let diff = RayDifferential {
                rx_origin: ray.origin + self.dx_camera,
                ry_origin: ray.origin + self.dy_camera,
                rx_direction: ray.dir,
                ry_direction: ray.dir,
            };
            ray.diff = Some(diff)
        }
        ray.transform(&self.projective.base.camera_to_world)
    }

    fn approximate_dp_dxy(&self, point: Point3f, normal: Normal3f, samples_per_pixel: u32) -> (Vec3f, Vec3f) {
        self.projective
            .base
            .approximate_dp_dxy(point, normal, samples_per_pixel)
    }

    fn get_film(&self) -> Arc<RGBFilm> { self.projective.base.film.clone() }
}

impl From<OrthographicCameraConfig> for OrthographicCamera {
    fn from(config: OrthographicCameraConfig) -> Self {
        let projective = ProjectiveCamera::from(config);
        OrthographicCamera {
            dx_camera: vec3!(1., 0., 0.).transform(&projective.raster_to_camera),
            dy_camera: vec3!(0., 1., 0.).transform(&projective.raster_to_camera),
            projective,
        }
    }
}

impl From<OrthographicCameraConfig> for ProjectiveCameraConfig {
    fn from(config: OrthographicCameraConfig) -> Self {
        ProjectiveCameraConfig {
            base_config: config.base_config,
            camera_to_screen: Transform::orthographic(0., 1.),
            screen_window: config.screen_window,
            lens_radius: config.lens_radius,
            focal_distance: config.focal_distance,
        }
    }
}
