use crate::{
    math::Transform,
    scene::cameras::{
        base::{BaseCamera, BaseCameraConfig},
        orthographic::OrthographicCameraConfig,
    },
    vec3, Point2f, Point2u,
};

pub struct ProjectiveCamera {
    pub base: BaseCamera,
    pub lens_radius: f32,
    pub focal_distance: f32,
    pub camera_to_screen: Transform<f32>,
    pub raster_to_camera: Transform<f32>,
    pub screen_to_raster: Transform<f32>,
}

pub struct ScreenWindow {
    pub min: Point2f,
    pub max: Point2f,
}

pub struct ProjectiveCameraConfig {
    pub base_config: BaseCameraConfig,
    pub camera_to_screen: Transform<f32>,
    pub screen_window: ScreenWindow,
    pub lens_radius: f32,
    pub focal_distance: f32,
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

impl From<ProjectiveCameraConfig> for ProjectiveCamera {
    fn from(config: ProjectiveCameraConfig) -> Self {
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
