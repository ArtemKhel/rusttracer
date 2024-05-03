use geometry::{utils::random_in_unit_disk, Cross, Point, Ray, Vec3};

use crate::{rendering::PixelCoord, utils::degrees_to_radians};

#[derive(Debug, Default)]
pub struct Screen {
    center: Point,
    basis: [Vec3; 2],
}

pub struct CameraConfig {
    pub position: Point,
    pub look_at: Point,
    pub up: Vec3,
    pub aspect_ratio: f32,
    pub vertical_fov: f32,
    pub defocus_angle: f32,
    pub focus_dist: f32,
}

#[derive(Debug, Default)]
pub struct Camera {
    pub position: Point,
    pub screen: Screen,
    pub defocus_radius: f32,
}

impl Camera {
    pub fn create_ray(&self, coord: PixelCoord) -> Ray {
        let direction = self.screen.center + self.screen.basis[0] * coord[0] + self.screen.basis[1] * coord[1];
        Ray::from_to(self.defocus_disk_sample(), direction)
    }

    fn defocus_disk_sample(&self) -> Point {
        let rnd = random_in_unit_disk();
        self.position + rnd * self.defocus_radius
    }
}

impl From<CameraConfig> for Camera {
    fn from(config: CameraConfig) -> Self {
        let theta = degrees_to_radians(config.vertical_fov);
        let half_height = (theta / 2.0).tan() * config.focus_dist;
        let half_width = half_height * config.aspect_ratio;

        // Basis
        let backward = (config.position - config.look_at).to_unit();
        let right = config.up.cross(backward.vec).to_unit();
        let up = backward.vec.cross(right.vec).to_unit();

        let center = config.position - backward * config.focus_dist;

        let defocus_radius = config.focus_dist * degrees_to_radians(config.defocus_angle / 2.).tan();

        Camera {
            position: config.position,
            defocus_radius,
            screen: Screen {
                center,
                basis: [right * half_width, -up * half_height], // TODO: types
            },
        }
    }
}

impl Default for CameraConfig {
    fn default() -> Self {
        CameraConfig {
            position: Point::default(),
            look_at: Point::new(0., 0., -1.),
            up: Vec3::new(0., 1., 0.),
            aspect_ratio: 16.0 / 9.0,
            vertical_fov: 90.0,
            defocus_angle: 0.5,
            focus_dist: 1.,
        }
    }
}
