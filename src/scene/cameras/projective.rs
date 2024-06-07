use crate::{
    core::Ray,
    math::{Normed, Transform},
    samplers::utils::sample_uniform_disk_concentric,
    scene::cameras::base::{BaseCamera, BaseCameraConfig},
    vec3, Bounds2f, Point2f, Point3f,
};

pub struct ProjectiveCamera {
    pub base: BaseCamera,
    pub lens_radius: f32,
    pub focal_distance: f32,
    pub camera_to_screen: Transform<f32>,
    pub raster_to_camera: Transform<f32>,
    pub screen_to_raster: Transform<f32>,
}

pub struct ProjectiveCameraConfig {
    pub base_config: BaseCameraConfig,
    pub camera_to_screen: Transform<f32>,
    pub screen_window: Bounds2f,
    pub lens_radius: f32,
    pub focal_distance: f32,
}

impl ProjectiveCamera {
    pub(super) fn adjust_for_dof(&self, ray: &mut Ray, p_lens: Point2f) {
        if self.lens_radius > 0. {
            let point_on_lens = sample_uniform_disk_concentric(p_lens) * self.lens_radius;
            let t_focal_intersect = self.focal_distance / ray.dir.z;
            let focus_point = ray.at(t_focal_intersect);
            ray.origin = Point3f::from(point_on_lens);
            ray.dir = (focus_point - ray.origin).to_unit();
        }
    }
}

impl<T> From<T> for ProjectiveCamera
where T: Into<ProjectiveCameraConfig>
{
    fn from(config: T) -> Self {
        let config = config.into();
        let screen_to_ndc = Transform::compose(
            Transform::translate(vec3!(-config.screen_window.min.x, -config.screen_window.max.y, 0.)),
            Transform::scale(
                (config.screen_window.max.x - config.screen_window.min.x).recip(),
                (config.screen_window.max.y - config.screen_window.min.y).recip(),
                1.,
            ),
        );
        let ndc_to_raster = Transform::scale(
            config.base_config.film.resolution.x as f32,
            -(config.base_config.film.resolution.y as f32),
            1.,
        );
        let screen_to_raster = Transform::compose(screen_to_ndc, ndc_to_raster);
        let raster_to_camera = Transform::compose(screen_to_raster.invert(), config.camera_to_screen.invert());

        // TODO:
        Self {
            base: BaseCamera::from(config.base_config),
            lens_radius: config.lens_radius,
            focal_distance: config.focal_distance,
            camera_to_screen: config.camera_to_screen,
            raster_to_camera,
            screen_to_raster,
        }
    }
}
