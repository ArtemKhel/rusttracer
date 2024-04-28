#![allow(unused)]

use image::buffer::ConvertBuffer;
use image::{Rgb, RgbImage};

use geometry::point::Point;
use geometry::sphere::Sphere;
use rusttracer::camera::Camera;
use rusttracer::material::Material;
use rusttracer::primitive::Primitive;

fn main() {
    let world: Vec<Box<Primitive>> = vec![
        Box::new(Primitive {
            object: Box::new(Sphere::new(Point::new(0., 0., -3.), 1.0)),
            material: Material::new(Rgb([0.2, 0.2, 0.7])),
        }),
        Box::new(Primitive {
            object: Box::new(Sphere::new(Point::new(-3., 0., -3.), 1.5)),
            material: Material {
                color: Rgb([0.7, 0.2, 0.2]),
                metal:true,
                fuzz: 0.3
            },
        }),
        Box::new(Primitive {
            object: Box::new(Sphere::new(Point::new(3., 0., -3.), 1.5)),
            material: Material {
                color: Rgb([0.7, 0.7, 0.7]),
                metal:true,
                fuzz: 0.3
            },
        }),
        Box::new(Primitive {
            object: Box::new(Sphere::new(Point::new(0., -101., 0.), 100.0)),
            material: Material::new( Rgb([0.2, 0.5, 0.2])),
        }),
    ];

    let camera = Camera::new(Point::default(), Point::new(0., 0., -1.), &world);
    let image = camera.render();

    let image: RgbImage = image.convert();
    image.save("./images/image.png").or_else(|err| {
        eprintln!("{err}");
        Err(err)
    });
}
