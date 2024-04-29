#![allow(unused)]

use image::buffer::ConvertBuffer;
use image::{Rgb, RgbImage};

use geometry::point::Point;
use geometry::sphere::Sphere;
use geometry::vec::Vec3;
use rusttracer::material::dielectric::Dielectric;
use rusttracer::material::lambertian::Lambertian;
use rusttracer::material::metal::Metal;
use rusttracer::material::Material;
use rusttracer::object::Object;
use rusttracer::render::{RayTracer, Render, Resolution};
use rusttracer::scene::{Camera, Scene};

fn main() {
    let materials = vec![];
    let world: Vec<Object> = vec![
        Object {
            shape: Box::new(Sphere::new(Point::new(0., 0., -3.), 1.0)),
            material: Box::new(Lambertian {
                albedo: Rgb([0.2, 0.2, 0.7]),
            }),
        },
        Object {
            shape: Box::new(Sphere::new(Point::new(-3., 0., -3.), 1.5)),
            material: Box::new(Dielectric {
                refraction_index: 1.5,
                // refraction_index: 0.8,
                attenuation: Rgb([0.8, 0.8, 0.8]),
            }),
        },
        Object {
            shape: Box::new(Sphere::new(Point::new(3., 0., -3.), 1.5)),
            material: Box::new(Metal {
                albedo: Rgb([0.8, 0.4, 0.1]),
                fuzz: 0.05,
            }),
        },
        Object {
            shape: Box::new(Sphere::new(Point::new(0., -101., 0.), 100.0)),
            material: Box::new(Lambertian {
                albedo: Rgb([0.2, 0.7, 0.2]),
            }),
        },
    ];
    let camera = Camera {
        position: Point::default(),
        look_at: Point::new(0., 0., 1.),
        focal_length: 1.0,
    };

    let scene = Scene {
        camera,
        objects: world,
        materials,
    };

    let raytracer = RayTracer {
        scene,
        resolution: Resolution {
            width: 400,
            height: 255,
        },
        antialiasing: 1,
        max_reflections: 10,
    };

    let image = raytracer.render();

    let image: RgbImage = image.convert();
    image.save("./images/image.png").or_else(|err| {
        eprintln!("{err}");
        Err(err)
    });
}
