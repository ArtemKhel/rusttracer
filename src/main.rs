mod camera;
mod scene;

use geometry::Object;
use image::{ImageBuffer, Rgb};

use geometry::point::Point;
use geometry::ray::Ray;
use geometry::sphere::Sphere;
use geometry::vec::Vec3;
use crate::camera::Camera;


fn main() {
    let world: Vec<Box<dyn Object>> = vec![
        Box::new(Sphere::new(Point::new(2., 0., -3.), 1.0)),
        Box::new(Sphere::new(Point::new(-2., 0., -3.), 2.0)),
        Box::new(Sphere::new(Point::new(0., -101., 0.), 100.0)),
    ];

    let camera = Camera::new(
        Point::default(),
        Point::new(0.,0.,-1.),
        &world,
    );
    let image = camera.render();

    image.save("./images/image.png").expect("TODO: panic message");
}
