#![allow(unused)]
mod camera;
mod material;
mod scene;

use geometry::Object;
use image::buffer::ConvertBuffer;
use image::{ImageBuffer, Rgb, RgbImage};

use crate::camera::Camera;
use geometry::point::Point;
use geometry::ray::Ray;
use geometry::sphere::Sphere;
use geometry::vec::Vec3;

fn main() {
    // TODO: dyn
    let world: Vec<Box<dyn Object>> = vec![
        Box::new(Sphere::new(Point::new(2., 0., -3.), 1.0)),
        Box::new(Sphere::new(Point::new(-2., 0.5, -4.), 2.0)),
        Box::new(Sphere::new(Point::new(0., -101., 0.), 100.0)),
    ];

    let camera = Camera::new(Point::default(), Point::new(0., 0., -1.), &world);
    let image = camera.render();

    let image: RgbImage = image.convert();
    image.save("./images/image.png").or_else(|err| {
        eprintln!("{err}");
        Err(err)
    });
}
