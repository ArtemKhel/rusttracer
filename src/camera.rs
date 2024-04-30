use geometry::point::Point;
use geometry::vec::Vec3;

pub struct Camera {
    pub position: Point,
    pub look_at: Point,
    pub up: Vec3,
    pub vertical_fov: f32,
    // pub config: CameraConfig,
}

impl Camera {
    // fn new(position: Point, look_at: Point, up: Vec3, vertical_fov: f32) -> Self{
    //     Camera{
    //         position,look_at, up, vertical_fov
    //     }
    // }
}

impl Default for Camera{
    fn default() -> Self {
        Camera {
            position: Point::default(),
            look_at: Point::new(0., 0., -1.),
            up: Vec3::new(0., 1., 0.),
            vertical_fov: 90.0,
        }
    }
}
