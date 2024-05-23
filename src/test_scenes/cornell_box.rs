use std::rc::Rc;

use image::Rgb;
use math::{point3, vec3, Quad, Sphere};

use crate::{
    aggregates::BVH,
    material::{
        dielectric::Dielectric, diffuse_light::DiffuseLight, isotropic::Isotropic, lambertian::Lambertian, Material,
    },
    mediums::Medium,
    scene::{SimpleCamera, CameraConfig, Composite, Primitive, Scene},
};

pub fn cornell_box() -> Scene {
    let materials: Vec<Box<dyn Material>> = vec![Box::new(Lambertian {
        albedo: Rgb([1.0, 1.0, 1.0]),
    })];

    let mut world = vec![
        Primitive {
            shape: Box::new(Quad::new(
                point3!(555., 0., 0.),
                vec3!(0., 555., 0.),
                vec3!(0., 0., 555.),
            )),
            material: Box::new(Lambertian {
                albedo: Rgb([0.12, 0.45, 0.15]),
            }),
        },
        Primitive {
            shape: Box::new(Quad::new(point3!(0., 0., 0.), vec3!(0., 555., 0.), vec3!(0., 0., 555.))),
            material: Box::new(Lambertian {
                albedo: Rgb([0.65, 0.05, 0.05]),
            }),
        },
        Primitive {
            shape: Box::new(Quad::new(
                point3!(405., 554., 405.),
                vec3!(-255., 0., 0.),
                vec3!(0., 0., -255.),
            )),
            material: Box::new(DiffuseLight {
                color: Rgb([5., 5., 5.]),
            }),
        },
        Primitive {
            shape: Box::new(Quad::new(point3!(0., 0., 0.), vec3!(555., 0., 0.), vec3!(0., 0., 555.))),
            material: Box::new(Lambertian {
                albedo: Rgb([0.73, 0.73, 0.73]),
            }),
        },
        Primitive {
            shape: Box::new(Quad::new(
                point3!(555., 555., 555.),
                vec3!(-555., 0., 0.),
                vec3!(0., 0., -555.),
            )),
            material: Box::new(Lambertian {
                albedo: Rgb([0.73, 0.73, 0.73]),
            }),
        },
        Primitive {
            shape: Box::new(Quad::new(
                point3!(0., 0., 555.),
                vec3!(555., 0., 0.),
                vec3!(0., 555., 0.),
            )),
            material: Box::new(Lambertian {
                albedo: Rgb([0.73, 0.73, 0.73]),
            }),
        },
        Primitive {
            shape: Box::new(Sphere::new(point3!(212.5, 240., 147.5), 75.)),
            material: Box::new(Dielectric {
                attenuation: Rgb([1., 1., 1.]),
                refraction_index: 1.5,
            }),
        },
    ];

    for side in Quad::quad_box(point3!(130., 0., 65.), point3!(295., 165., 230.)).into_iter() {
        world.push(Primitive {
            shape: Box::new(side),
            material: Box::new(Lambertian {
                albedo: Rgb([0.73, 0.73, 0.73]),
            }),
        })
    }
    let box2 = Quad::quad_box(point3!(265.0, 0.1, 295.), point3!(430., 330., 460.))
        .into_iter()
        .map(|x: Quad<f32>| Box::new(x) as _)
        .collect();
    world.push(Primitive {
        shape: Box::new(Medium::new(Box::new(Composite { objects: box2 }), 0.01)),
        material: Box::new(Isotropic {
            color: Rgb([0.3, 0.3, 0.3]),
        }),
    });

    let world = BVH::new(world.into_iter().map(Rc::new).collect(), 4);

    let camera = SimpleCamera::from(CameraConfig {
        position: point3!(278., 278., -800.),
        look_at: point3!(278., 278., 0.),
        up: vec3!(0., 1., 0.),
        aspect_ratio: 1.0,
        vertical_fov: 40.0,
        defocus_angle: 0.0,
        focus_dist: 50.0,
    });

    Scene {
        camera,
        objects: world,
        materials,
        background_color: Rgb([0.0, 0.0, 0.0]),
    }
}
