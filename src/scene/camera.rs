use std::ops::Mul;

use crate::{
    math::{utils::random_in_unit_disk, Cross, *},
    point3,
    rendering::PixelCoord,
    utils::degrees_to_radians,
    vec3, Point3f, Ray, Vec3f,
};

pub trait Camera {
    fn create_ray(&self, coord: PixelCoord) -> Ray;
}

#[derive(Debug, Default)]
pub struct Screen {
    center: Point3f,
    basis: [Vec3f; 2],
}

#[derive(Debug)]
pub struct CameraConfig {
    pub position: Point3f,
    pub look_at: Point3f,
    pub up: Vec3f,
    pub aspect_ratio: f32,
    pub vertical_fov: f32,
    pub defocus_angle: f32,
    pub focus_dist: f32,
}

#[derive(Debug, Default)]
pub struct SimpleCamera {
    pub position: Point3f,
    pub screen: Screen,
    pub defocus_radius: f32,
}

impl Camera for SimpleCamera {
    fn create_ray(&self, coord: PixelCoord) -> Ray {
        let direction = self.screen.center + self.screen.basis[0] * coord[0] + self.screen.basis[1] * coord[1];
        Ray::from_to(self.defocus_disk_sample(), direction)
    }
}

impl SimpleCamera {
    fn defocus_disk_sample(&self) -> Point3f {
        let rnd = random_in_unit_disk();
        self.position + rnd * self.defocus_radius
    }
}

impl From<CameraConfig> for SimpleCamera {
    fn from(config: CameraConfig) -> Self {
        let theta = degrees_to_radians(config.vertical_fov);
        let half_height = (theta / 2.0).tan() * config.focus_dist;
        let half_width = half_height * config.aspect_ratio;

        // Basis
        let backward = (config.position - config.look_at).to_unit();
        let right = config.up.cross(*backward).to_unit();
        let up = backward.cross(*right).to_unit();

        let center = config.position + -backward.mul(config.focus_dist);

        let defocus_radius = config.focus_dist * degrees_to_radians(config.defocus_angle / 2.).tan();

        SimpleCamera {
            position: config.position,
            defocus_radius,
            screen: Screen {
                center,
                basis: [*right * half_width, -*up * half_height], // TODO: types
            },
        }
    }
}

impl Default for CameraConfig {
    fn default() -> Self {
        CameraConfig {
            position: Point3::default(),
            look_at: point3!(0., 0., -1.),
            up: vec3!(0., 1., 0.),
            aspect_ratio: 16.0 / 9.0,
            vertical_fov: 90.0,
            defocus_angle: 0.5,
            focus_dist: 1.,
        }
    }
}
