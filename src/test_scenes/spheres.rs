use std::rc::Rc;

use image::Rgb;
use math::{point3, vec3, Normed, Sphere};
use rand::random;

use crate::{
    aggregates::BVH,
    material::{dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal, Material},
    scene::{Camera, CameraConfig, Primitive, Scene},
};

pub fn spheres() -> Scene {
    let mut world = vec![
        Primitive {
            shape: Box::new(Sphere::new(point3!(-4., 1.0, 0.), 1.0)),
            material: Box::new(Lambertian {
                albedo: Rgb([0.4, 0.2, 0.1]),
            }),
        },
        Primitive {
            shape: Box::new(Sphere::new(point3!(0., 1.0, 0.), 0.9)),
            material: Box::new(Metal {
                albedo: Rgb([0.7, 0.6, 0.6]),
                fuzz: 0.1,
            }),
        },
        Primitive {
            shape: Box::new(Sphere::new(point3!(4., 1.0, 0.), 0.8)),
            material: Box::new(Dielectric {
                refraction_index: 1.5,
                attenuation: Rgb([0.95, 0.95, 0.95]),
            }),
        },
        Primitive {
            shape: Box::new(Sphere::new(point3!(0., -1000., 0.), 1000.)),
            material: Box::new(Lambertian {
                albedo: Rgb([0.2, 0.2, 0.2]),
            }),
        },
        Primitive {
            shape: Box::new(Sphere::new(point3!(10., 20.0, 10.), 10.0)),
            material: Box::new(DiffuseLight {
                color: Rgb([3., 3., 3.]),
            }),
        },
    ];

    for a in -7..7 {
        for b in -7..7 {
            let choose_mat = random::<f32>();
            let center = point3!(
                a as f32 + 0.9 * random::<f32>(),
                (random::<f32>() + 3.) / 16.,
                b as f32 + 0.9 * random::<f32>()
            );

            if (center - point3!(4., 0.2, 0.)).len() > 0.9 {
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
                    shape: Box::new(Sphere::new(center, center.coords.y)),
                    material: sphere_material,
                });
            }
        }
    }

    let world = BVH::new(world.into_iter().map(Rc::new).collect(), 4);

    let materials = vec![];

    let camera = Camera::from(CameraConfig {
        position: point3!(13., 2., 4.),
        look_at: point3!(0., 0., 0.),
        up: vec3!(0., 1., 0.),
        aspect_ratio: 16.0 / 9.0,
        vertical_fov: 20.0,
        defocus_angle: 0.5,
        focus_dist: 10.0,
    });

    Scene {
        camera,
        objects: world,
        materials,
        background_color: Rgb([0.1, 0.1, 0.1]),
    }
}
