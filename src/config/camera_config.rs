pub struct CameraConfig {
    // aspect_ratio: f32,
    // width: u32,
    // height: u32,
    focal_length: f32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        // let aspect_ratio = 16. / 9.;
        // let width = 480;
        CameraConfig {
            // aspect_ratio,
            // width,
            // height: (width as f32 / aspect_ratio) as u32,
            focal_length: 1.,
        }
    }
}
