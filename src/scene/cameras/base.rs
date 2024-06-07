use std::{ops::Deref, sync::Arc};

use crate::{
    core::{ray::RayDifferential, Ray},
    math::{dot, Normed, Transform, Transformable},
    point3,
    scene::{
        cameras::{Camera, CameraSample},
        film::RGBFilm,
    },
    vec3, Normal3f, Point2u, Point3f, Vec3f,
};

#[derive(Debug)]
pub(super) struct BaseCamera {
    pub(super) camera_to_world: Transform<f32>,
    pub(super) film: Arc<RGBFilm>,
    // pub(super) medium: ???
    min_pos_differential_x: Vec3f,
    min_pos_differential_y: Vec3f,
    min_dir_differential_x: Vec3f,
    min_dir_differential_y: Vec3f,
}

pub struct BaseCameraConfig {
    pub transform: Transform<f32>,
    pub film: RGBFilm, // pub medium: ???
}

impl BaseCamera {
    pub(super) fn generate_differential_ray<C: Camera>(camera: C, sample: CameraSample) -> Ray {
        // TODO: camera knows nothing about resolution

        let rx = camera.generate_ray({
            let mut s: CameraSample = sample;
            // TODO: what?
            s.p_film.x += 0.05;
            s
        });
        let ry = camera.generate_ray({
            let mut s: CameraSample = sample;
            s.p_film.y += 0.05;
            s
        });

        let mut ray = camera.generate_ray(sample);
        ray.diff = Some(RayDifferential {
            rx_origin: rx.origin,
            rx_direction: rx.dir,
            ry_origin: ry.origin,
            ry_direction: ry.dir,
        });
        ray
    }

    pub(super) fn approximate_dp_dxy(&self, p: Point3f, n: Normal3f, samples_per_pixel: u32) -> (Vec3f, Vec3f) {
        // Shamelessly --stolen-- commandeered from https://github.com/jalberse/shimmer/blob/main/src/camera.rs
        // TODO:
        let z = vec3!(0., 0., 1.);
        let p_camera = p.inv_transform(&self.camera_to_world);
        let down_z_from_camera = Transform::rotate_from_to(&p_camera.to_unit(), &z);
        let p_down_z = p_camera.transform(&down_z_from_camera);
        let n_down_z = n.inv_transform(&self.camera_to_world).transform(&down_z_from_camera);
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
        let spp_scale = (samples_per_pixel as f32).sqrt().recip().max(0.125);

        let dp_dx = spp_scale
            * (px - p_down_z)
                .inv_transform(&down_z_from_camera)
                .transform(&self.camera_to_world);
        let dp_dy = spp_scale
            * (py - p_down_z)
                .inv_transform(&down_z_from_camera)
                .transform(&self.camera_to_world);
        (dp_dx, dp_dy)
    }
}

impl From<BaseCameraConfig> for BaseCamera {
    fn from(config: BaseCameraConfig) -> Self {
        Self {
            camera_to_world: config.transform,
            film: Arc::new(config.film),
            min_pos_differential_x: Default::default(),
            min_pos_differential_y: Default::default(),
            min_dir_differential_x: Default::default(),
            min_dir_differential_y: Default::default(),
        }
    }
}
