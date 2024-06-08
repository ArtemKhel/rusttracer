use std::sync::Arc;

use crate::{
    breakpoint,
    core::{ray::RayDifferential, Ray},
    math::{Normed, Transform, Transformable},
    point3, ray,
    samplers::utils::sample_uniform_disk_concentric,
    scene::{
        cameras::{
            projective::{ProjectiveCamera, ProjectiveCameraConfig},
            BaseCameraConfig, Camera, CameraSample, CameraType, OrthographicCamera, OrthographicCameraConfig,
        },
        film::RGBFilm,
    },
    vec3, Bounds2f, Normal3f, Point2f, Point3f, Vec3f,
};

pub struct PerspectiveCamera {
    projective: ProjectiveCamera,
    dx_camera: Vec3f,
    dy_camera: Vec3f,
}

pub struct PerspectiveCameraConfig {
    pub base_config: BaseCameraConfig,
    pub fov: f32,
    pub screen_window: Bounds2f,
    pub lens_radius: f32,
    pub focal_distance: f32,
}

impl PerspectiveCamera {
    pub fn new(config: PerspectiveCameraConfig) -> Self { PerspectiveCamera::from(config) }

    fn generate_camera_space_ray(&self, sample: CameraSample) -> Ray {
        let ray_origin = point3!(0., 0., 0.);
        let point_raster: Point3f = sample.p_film.into();
        let point_camera = point_raster.transform(&self.projective.raster_to_camera);
        let ray_direction = point_camera.coords.to_unit();
        let mut ray = ray!(ray_origin, ray_direction);
        self.projective.adjust_for_dof(&mut ray, sample.p_lens);
        ray
    }
}

impl Camera for PerspectiveCamera {
    fn generate_ray(&self, sample: CameraSample) -> Ray {
        let ray = self.generate_camera_space_ray(sample);
        ray.transform(&self.projective.base.camera_to_world)
    }

    fn generate_differential_ray(&self, sample: CameraSample) -> Ray {
        let mut ray = self.generate_camera_space_ray(sample);
        let point_raster: Point3f = sample.p_film.into();
        let point_camera = point_raster.transform(&self.projective.raster_to_camera);

        if self.projective.lens_radius > 0. {
            // TODO
        } else {
            let diff = RayDifferential {
                rx_origin: ray.origin,
                ry_origin: ray.origin,
                rx_direction: (*point_camera + self.dx_camera).to_unit(),
                ry_direction: (*point_camera + self.dy_camera).to_unit(),
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

impl From<PerspectiveCameraConfig> for PerspectiveCamera {
    fn from(config: PerspectiveCameraConfig) -> Self {
        let projective = ProjectiveCamera::from(config);
        let dx_camera = vec3!(1., 0., 0.).transform(&projective.raster_to_camera)
            - vec3!(0., 0., 0.).transform(&projective.raster_to_camera);
        let dy_camera = vec3!(0., 1., 0.).transform(&projective.raster_to_camera)
            - vec3!(0., 0., 0.).transform(&projective.raster_to_camera);
        PerspectiveCamera {
            projective,
            dx_camera,
            dy_camera,
        }
    }
}

impl From<PerspectiveCameraConfig> for ProjectiveCameraConfig {
    fn from(config: PerspectiveCameraConfig) -> Self {
        ProjectiveCameraConfig {
            base_config: config.base_config,
            camera_to_screen: Transform::perspective(config.fov, 1., 100.),
            screen_window: config.screen_window,
            lens_radius: config.lens_radius,
            focal_distance: config.focal_distance,
        }
    }
}
