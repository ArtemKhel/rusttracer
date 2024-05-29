use std::ops::Mul;

use crate::{
    core::{ray::RayDifferential, Ray},
    math::{dot, utils::random_in_unit_disk, Frame, Normed, Transform, Transformable, Unit},
    point3,
    scene::cameras::{Camera, CameraSample},
    utils::degrees_to_radians,
    vec3, Normal3f, Point3f, Vec3f,
};

#[derive(Debug)]
pub struct Screen {
    center: Point3f,
    basis: [Vec3f; 2],
}

#[derive(Debug)]
pub struct CameraConfig {
    pub transform: Transform<f32>,
    pub aspect_ratio: f32,
    pub vertical_fov: f32,
    pub defocus_angle: f32,
    pub focus_dist: f32,
}

#[derive(Debug)]
pub struct SimpleCamera {
    pub position: Point3f,
    pub screen: Screen,
    pub defocus_radius: f32,
    transform: Transform<f32>,
    min_pos_differential_x: Vec3f,
    min_pos_differential_y: Vec3f,
    min_dir_differential_x: Vec3f,
    min_dir_differential_y: Vec3f,
}

impl Camera for SimpleCamera {
    fn generate_ray(&self, coord: CameraSample) -> Ray {
        let direction = self.screen.center + self.screen.basis[0] * coord[0] + self.screen.basis[1] * coord[1];
        Ray::from_to(self.defocus_disk_sample(), direction)
    }

    fn generate_differential_ray(&self, coord: CameraSample) -> Ray {
        // TODO: camera knows nothing about resolution
        let rx = self.generate_ray(if coord[0] > 0. {
            [coord[0] - 0.01, coord[1]]
        } else {
            [coord[0] + 0.01, coord[1]]
        });
        let ry = self.generate_ray(if coord[1] > 0. {
            [coord[0], coord[1] - 0.01]
        } else {
            [coord[0], coord[1] + 0.01]
        });

        let mut ray = self.generate_ray(coord);
        ray.diff = Some(RayDifferential {
            rx_origin: rx.origin,
            ry_origin: ry.origin,
            rx_direction: rx.dir,
            ry_direction: ry.dir,
        });
        ray
    }

    fn approximate_dp_dxy(&self, p: Point3f, n: Normal3f, samples_per_pixel: u32) -> (Vec3f, Vec3f) {
        // Shamelessly --stolen-- commandeered from https://github.com/jalberse/shimmer/blob/main/src/camera.rs
        // TODO: this
        let z = vec3!(0., 0., 1.);
        let p_camera = p.inv_transform(&self.transform);
        let down_z_from_camera = Transform::rotate_from_to(p_camera.to_unit().coords, z);
        let p_down_z = p_camera.transform(&down_z_from_camera);
        let n_down_z = n.inv_transform(&self.transform).transform(&down_z_from_camera);
        let d = n_down_z.z * p_down_z.z;

        // Find intersection points for approximated camera differential rays
        let x_ray = Ray::new(
            point3!() + self.min_pos_differential_x,
            (z + self.min_dir_differential_x).to_unit(),
            None,
        );
        let tx = -(dot(&n_down_z, &x_ray.origin) - d) / dot(&n_down_z, &x_ray.dir);
        let y_ray = Ray::new(
            point3!() + self.min_pos_differential_y,
            (z + self.min_dir_differential_y).to_unit(),
            None,
        );

        let ty = -(dot(&n_down_z, &x_ray.origin) - d) / dot(&n_down_z, &y_ray.dir);
        let px = x_ray.at(tx);
        let py = y_ray.at(ty);

        // let spp_scale = if options.disable_pixel_jitter {
        //     1.0
        // } else {
        //     Float::max(0.125, 1.0 / (samples_per_pixel as Float).sqrt())
        // };
        let spp_scale = 1.0 / (samples_per_pixel as f32).sqrt().max(0.125);

        let dp_dx = spp_scale
            * (px - p_down_z)
                .inv_transform(&down_z_from_camera)
                .transform(&self.transform);
        let dp_dy = spp_scale
            * (py - p_down_z)
                .inv_transform(&down_z_from_camera)
                .transform(&self.transform);
        (dp_dx, dp_dy)
    }
}

impl SimpleCamera {
    fn defocus_disk_sample(&self) -> Point3f {
        let rnd = random_in_unit_disk();
        self.position + rnd * self.defocus_radius
    }

    fn find_minimum_differentials(&mut self) {
        self.min_pos_differential_x = vec3!(f32::INFINITY);
        self.min_pos_differential_y = vec3!(f32::INFINITY);
        self.min_dir_differential_x = vec3!(f32::INFINITY);
        self.min_dir_differential_y = vec3!(f32::INFINITY);

        let n = 128;
        for i in 0..n {
            let ray =
                self.generate_differential_ray([i as f32 / (n as f32 / 2.) - 1., i as f32 / (n as f32 / 2.) - 1.]);

            if ray.diff.is_none() {
                continue;
            }

            let mut diff = ray.diff.unwrap();

            let d_ox = (diff.rx_origin - ray.origin).inv_transform(&self.transform);
            if d_ox.len() < self.min_pos_differential_x.len() {
                self.min_pos_differential_x = d_ox;
            }

            let d_oy = (diff.ry_origin - ray.origin).inv_transform(&self.transform);
            if d_oy.len() < self.min_pos_differential_y.len() {
                self.min_pos_differential_y = d_oy;
            }

            let f = Frame::from_z(*ray.dir);
            let df = f.to_local_wrap::<Unit<_>>(*ray.dir);
            let dxf = f.to_local_wrap::<Unit<_>>(*diff.rx_direction);
            let dyf = f.to_local_wrap::<Unit<_>>(*diff.ry_direction);

            if (*dxf - *df).len() < self.min_dir_differential_x.len() {
                self.min_dir_differential_x = *dxf - *df;
            }
            if (*dyf - *df).len() < self.min_dir_differential_y.len() {
                self.min_dir_differential_y = *dyf - *df;
            }
        }
    }
}

impl From<CameraConfig> for SimpleCamera {
    fn from(config: CameraConfig) -> Self {
        let theta = degrees_to_radians(config.vertical_fov);
        let half_height = (theta / 2.0).tan() * config.focus_dist;
        let half_width = half_height * config.aspect_ratio;

        // Basis
        let position = point3!().transform(&config.transform);
        let forward = vec3!(0., 0., 1.).transform(&config.transform);
        let up = vec3!(0., 1., 0.).transform(&config.transform);
        let right = vec3!(1., 0., 0.).transform(&config.transform);

        let center = position + forward * config.focus_dist;

        let defocus_radius = config.focus_dist * degrees_to_radians(config.defocus_angle / 2.).tan();

        let mut camera = SimpleCamera {
            position,
            defocus_radius,
            screen: Screen {
                center,
                basis: [right * half_width, -up * half_height], // TODO: types
            },
            transform: config.transform,
            min_pos_differential_x: Default::default(),
            min_pos_differential_y: Default::default(),
            min_dir_differential_x: Default::default(),
            min_dir_differential_y: Default::default(),
        };
        camera.find_minimum_differentials();
        // dbg!(
        //     camera.min_dir_differential_x,
        //     camera.min_dir_differential_y,
        //     camera.min_pos_differential_y,
        //     camera.min_pos_differential_y,
        // );
        camera
    }
}

impl Default for CameraConfig {
    fn default() -> Self {
        CameraConfig {
            transform: Transform::id(),
            aspect_ratio: 16.0 / 9.0,
            vertical_fov: 90.0,
            defocus_angle: 0.5,
            focus_dist: 1.,
        }
    }
}
