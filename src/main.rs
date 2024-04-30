#![allow(unused)]

use image::buffer::ConvertBuffer;
use image::{Rgb, RgbImage};
use rand::random;

use geometry::point::Point;
use geometry::sphere::Sphere;
use geometry::vec::Vec3;
use rusttracer::camera::Camera;
use rusttracer::material::dielectric::Dielectric;
use rusttracer::material::lambertian::Lambertian;
use rusttracer::material::Material;
use rusttracer::material::metal::Metal;
use rusttracer::object::Object;
use rusttracer::render::{RayTracer, Render, Resolution};
use rusttracer::scene::Scene;

fn main() {
    let materials = vec![];

    let mut world: Vec<Object> = vec![
        Object {
            shape: Box::new(Sphere::new(Point::new(-4., 1.0, 0.), 1.0)),
            material: Box::new(Lambertian {
                albedo: Rgb([0.4, 0.2, 0.1]),
            }),
        },
        Object {
            shape: Box::new(Sphere::new(Point::new(0., 1.0, 0.), 0.9)),
            material: Box::new(Metal {
                albedo: Rgb([0.7, 0.6, 0.6]),
                fuzz: 0.1,
            }),
        },
        Object {
            shape: Box::new(Sphere::new(Point::new(4., 1.0, 0.), 0.8)),
            material: Box::new(Dielectric {
                refraction_index: 1.5,
                attenuation: Rgb([0.95, 0.95, 0.95]),
            }),
        },
        Object {
            shape: Box::new(Sphere::new(Point::new(0., -1000., 0.), 1000.)),
            material: Box::new(Lambertian {
                albedo: Rgb([0.2, 0.2, 0.2]),
            }),
        },
    ];
    for a in -7..7 {
        for b in -7..7 {
            let choose_mat = random::<f32>();
            let center = Point::new(a as f32 + 0.9*random::<f32>(), (random::<f32>() + 3.) / 16., b as f32 + 0.9*random::<f32>());

            if (center - Point::new(4., 0.2, 0.)).len() > 0.9 {
                let sphere_material:Box<dyn Material> = if (choose_mat < 0.5) {
                    Box::new(Lambertian{
                        albedo: Rgb(random()),
                    })
                } else if (choose_mat < 0.8) {
                    Box::new(Metal{
                        albedo: Rgb(random()),
                        fuzz: random(),
                    })
                } else {
                    Box::new(Dielectric{ attenuation: Rgb(random()), refraction_index: 1.5 })
                };
                world.push(Object{ shape: Box::new(Sphere::new(center, center.radius_vector.y)), material: sphere_material });
            }
        }
    }


    let camera = Camera {
        position: Point::new(13., 2., 4.),
        look_at: Point::new(0., 0., 0.),
        up: Vec3::new(0., 1., 0.),
        vertical_fov: 20.0,
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
            height: 225,
            // width: 1280,
            // height: 720,
        },
        antialiasing: 1,
        max_reflections: 5,
    };

    let image = raytracer.render();

    let image: RgbImage = image.convert();
    image.save("./images/image.png").or_else(|err| {
        eprintln!("{err}");
        Err(err)
    });
}
