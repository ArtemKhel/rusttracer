use crate::{
    core::{ray::RayDifferential, Ray},
    math::{Normed, Transform, Transformable},
    ray,
    samplers::utils::sample_uniform_disk_concentric,
    scene::cameras::{
        base::BaseCameraConfig,
        projective::{ProjectiveCamera, ProjectiveCameraConfig, ScreenWindow},
        Camera, CameraSample,
    },
    unit_vec3, vec3, Normal3f, Point2f, Point3f, Vec3f,
};

pub struct OrthographicCamera {
    projective: ProjectiveCamera,
    dx_camera: Vec3f,
    dy_camera: Vec3f,
}

pub struct OrthographicCameraConfig {
    pub base_config: BaseCameraConfig,
    pub screen_window: ScreenWindow,
    pub lens_radius: f32,
    pub focal_distance: f32,
}

impl OrthographicCamera {
    fn adjust_for_dof(&self, ray: &mut Ray, p_lens: Point2f) {
        if self.projective.lens_radius > 0. {
            let point_on_lens = sample_uniform_disk_concentric(p_lens) * self.projective.lens_radius;
            let t_focal_intersect = self.projective.focal_distance / ray.dir.z;
            let focus_point = ray.at(t_focal_intersect);
            ray.origin = Point3f::from(point_on_lens);
            ray.dir = (focus_point - ray.origin).to_unit();
        }
    }

    fn generate_camera_space_ray(&self, sample: CameraSample) -> Ray {
        let point_raster: Point3f = sample.p_film.into();
        let point_camera = point_raster.transform(&self.projective.raster_to_camera);
        let mut ray = ray!(point_camera, unit_vec3!(0., 0., 1.));
        self.adjust_for_dof(&mut ray, sample.p_lens);
        ray
    }
}

impl Camera for OrthographicCamera {
    // TODO: camera ray wrapper?
    fn generate_ray(&self, sample: CameraSample) -> Ray {
        let ray = self.generate_camera_space_ray(sample);
        ray.transform(&self.projective.base.camera_to_world)
        // ray.transform(&self.projective.raster_to_camera)
    }

    fn generate_differential_ray(&self, sample: CameraSample) -> Ray {
        let mut ray = self.generate_camera_space_ray(sample);
        if self.projective.lens_radius > 0. {
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
}

impl From<OrthographicCameraConfig> for OrthographicCamera {
    fn from(config: OrthographicCameraConfig) -> Self {
        OrthographicCamera {
            projective: ProjectiveCamera::from(ProjectiveCameraConfig::from(config)),
            dx_camera: vec3!(1., 0., 0.),
            dy_camera: vec3!(0., 1., 0.),
        }
    }
}
