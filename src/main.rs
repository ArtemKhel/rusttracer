#![allow(unused)]

use std::rc::Rc;

use image::{buffer::ConvertBuffer, Rgb, RgbImage};
use rand::random;
use rusttracer::{
    aggregates::BVH,
    geometry::{Point, Quad, Sphere, Vec3},
    material::{dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal, Material},
    rendering::{AAType::RegularGrid, RayTracer, Renderer, Resolution},
    scene::{Camera, CameraConfig, Primitive, Scene},
};

fn spheres() -> Scene {
    let mut world = vec![
        Primitive {
            shape: Box::new(Sphere::new(Point::new(-4., 1.0, 0.), 1.0)),
            material: Box::new(Lambertian {
                albedo: Rgb([0.4, 0.2, 0.1]),
            }),
        },
        Primitive {
            shape: Box::new(Sphere::new(Point::new(0., 1.0, 0.), 0.9)),
            material: Box::new(Metal {
                albedo: Rgb([0.7, 0.6, 0.6]),
                fuzz: 0.1,
            }),
        },
        Primitive {
            shape: Box::new(Sphere::new(Point::new(4., 1.0, 0.), 0.8)),
            material: Box::new(Dielectric {
                refraction_index: 1.5,
                attenuation: Rgb([0.95, 0.95, 0.95]),
            }),
        },
        Primitive {
            shape: Box::new(Sphere::new(Point::new(0., -1000., 0.), 1000.)),
            material: Box::new(Lambertian {
                albedo: Rgb([0.2, 0.2, 0.2]),
            }),
        },
        Primitive {
            shape: Box::new(Sphere::new(Point::new(10., 20.0, 10.), 10.0)),
            material: Box::new(DiffuseLight {
                color: Rgb([1., 1., 1.]),
            }),
        },
    ];

    for a in -7..7 {
        for b in -7..7 {
            let choose_mat = random::<f32>();
            let center = Point::new(
                a as f32 + 0.9 * random::<f32>(),
                (random::<f32>() + 3.) / 16.,
                b as f32 + 0.9 * random::<f32>(),
            );

            if (center - Point::new(4., 0.2, 0.)).len() > 0.9 {
                let sphere_material: Box<dyn Material> = if (choose_mat < 0.5) {
                    Box::new(Lambertian { albedo: Rgb(random()) })
                } else if (choose_mat < 0.8) {
                    Box::new(Metal {
                        albedo: Rgb(random()),
                        fuzz: random(),
                    })
                } else {
                    Box::new(Dielectric {
                        attenuation: Rgb(random()),
                        refraction_index: 1.5,
                    })
                };
                world.push(Primitive {
                    shape: Box::new(Sphere::new(center, center.radius_vector.y)),
                    material: sphere_material,
                });
            }
        }
    }

    let world = BVH::new(world.into_iter().map(Rc::new).collect(), 1);

    let materials = vec![];

    let camera = Camera::from(CameraConfig {
        position: Point::new(13., 2., 4.),
        look_at: Point::new(0., 0., 0.),
        up: Vec3::new(0., 1., 0.),
        aspect_ratio: 16.0 / 9.0,
        vertical_fov: 20.0,
        defocus_angle: 0.5,
        focus_dist: 10.0,
    });

    let scene = Scene {
        camera,
        objects: world,
        materials,
    };

    scene
}

fn cornell_box() -> Scene {
    let materials: Vec<Box<dyn Material>> = vec![Box::new(Lambertian {
        albedo: Rgb([1.0, 1.0, 1.0]),
    })];

    let world = vec![
        Primitive {
            shape: Box::new(Quad::new(
                Point::new(555., 0., 0.),
                Vec3::new(0., 555., 0.),
                Vec3::new(0., 0., 555.),
            )),
            material: Box::new(Lambertian {
                albedo: Rgb([0.12, 0.45, 0.15]),
            }),
        },
        Primitive {
            shape: Box::new(Quad::new(
                Point::new(0., 0., 0.),
                Vec3::new(0., 555., 0.),
                Vec3::new(0., 0., 555.),
            )),
            material: Box::new(Lambertian {
                albedo: Rgb([0.65, 0.05, 0.05]),
            }),
        },
        Primitive {
            shape: Box::new(Quad::new(
                Point::new(405., 554., 405.),
                Vec3::new(-255., 0., 0.),
                Vec3::new(0., 0., -255.),
            )),
            material: Box::new(DiffuseLight {
                color: Rgb([15., 15., 15.]),
            }),
        },
        Primitive {
            shape: Box::new(Quad::new(
                Point::new(0., 0., 0.),
                Vec3::new(555., 0., 0.),
                Vec3::new(0., 0., 555.),
            )),
            material: Box::new(Lambertian {
                albedo: Rgb([0.73, 0.73, 0.73]),
            }),
        },
        Primitive {
            shape: Box::new(Quad::new(
                Point::new(555., 555., 555.),
                Vec3::new(-555., 0., 0.),
                Vec3::new(0., 0., -555.),
            )),
            material: Box::new(Lambertian {
                albedo: Rgb([0.73, 0.73, 0.73]),
            }),
        },
        Primitive {
            shape: Box::new(Quad::new(
                Point::new(0., 0., 555.),
                Vec3::new(555., 0., 0.),
                Vec3::new(0., 555., 0.),
            )),
            material: Box::new(Lambertian {
                albedo: Rgb([0.73, 0.73, 0.73]),
            }),
        },
    ];

    for x in world.iter() {
        dbg!(x);
    }

    let world = BVH::new(world.into_iter().map(Rc::new).collect(), 1);

    let camera = Camera::from(CameraConfig {
        position: Point::new(278., 278., -800.),
        look_at: Point::new(278., 278., 0.),
        up: Vec3::new(0., 1., 0.),
        aspect_ratio: 1.0,
        vertical_fov: 40.0,
        defocus_angle: 0.0,
        focus_dist: 50.0,
    });

    let scene = Scene {
        camera,
        objects: world,
        materials,
    };

    scene
}

fn main() {
    // TODO:

    // let scene = spheres();
    let scene = cornell_box();

    let raytracer = RayTracer {
        scene,
        resolution: Resolution {
            width: 640,
            height: 640,
            // height: 360,

            // width: 1280,
            // height: 720,
        },
        // antialiasing: AAType::None.into(),
        antialiasing: RegularGrid(7).into(),
        max_reflections: 5,
    };

    let image = raytracer.render();

    let image: RgbImage = image.convert();
    image.save("./images/_image.png").unwrap()
}
