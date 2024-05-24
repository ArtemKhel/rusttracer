use std::{
    f32::consts::{FRAC_PI_2, FRAC_PI_3, FRAC_PI_4, FRAC_PI_6, FRAC_PI_8, PI},
    rc::Rc,
};

use image::Rgb;

use crate::{
    aggregates::BVH,
    material::{
        dielectric::Dielectric, diffuse_light::DiffuseLight, isotropic::Isotropic, lambertian::Lambertian,
        metal::Metal, Material,
    },
    math::{utils::Axis3, Transform},
    mediums::Medium,
    point3,
    scene::{CameraConfig, Composite, Primitive, Scene, SimpleCamera},
    shapes::{mesh::Triangle, quad::Quad, sphere::Sphere},
    vec3, Point3f,
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
                Transform::id(),
            )),
            material: Box::new(Lambertian {
                albedo: Rgb([0.12, 0.45, 0.15]),
            }),
        },
        Primitive {
            shape: Box::new(Quad::new(
                point3!(0., 0., 0.),
                vec3!(0., 555., 0.),
                vec3!(0., 0., 555.),
                Transform::id(),
            )),
            material: Box::new(Lambertian {
                albedo: Rgb([0.65, 0.05, 0.05]),
            }),
        },
        Primitive {
            shape: Box::new(Quad::new(
                point3!(455., 535., 455.),
                vec3!(-355., 0., 0.),
                vec3!(0., 0., -355.),
                Transform::id(),
            )),
            material: Box::new(DiffuseLight {
                color: Rgb([2., 2., 2.]),
            }),
        },
        Primitive {
            shape: Box::new(Quad::new(
                point3!(0., 0., 0.),
                vec3!(555., 0., 0.),
                vec3!(0., 0., 555.),
                Transform::id(),
            )),
            material: Box::new(Lambertian {
                albedo: Rgb([0.73, 0.73, 0.73]),
            }),
        },
        Primitive {
            shape: Box::new(Quad::new(
                point3!(555., 555., 555.),
                vec3!(-555., 0., 0.),
                vec3!(0., 0., -555.),
                Transform::id(),
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
                Transform::id(),
            )),
            material: Box::new(Lambertian {
                albedo: Rgb([0.73, 0.73, 0.73]),
            }),
        },
        // Primitive {
        //     shape: Box::new(Sphere::new(75., Transform::translate(vec3!(212.5, 240., 147.5)))),
        //     material: Box::new(Dielectric {
        //         attenuation: Rgb([1., 1., 1.]),
        //         refraction_index: 1.5,
        //     }),
        // },
    ];

    for side in Quad::quad_box(
        165.0,
        165.0,
        165.0,
        Transform::compose(
            Transform::rotate(Axis3::Y, FRAC_PI_8),
            Transform::translate(vec3!(150.5, 82.5, 150.5)),
        ),
    )
    .into_iter()
    {
        world.push(Primitive {
            shape: Box::new(side),
            material: Box::new(Lambertian {
                albedo: Rgb([0.73, 0.73, 0.73]),
            }),
        })
    }
    let box2 = Quad::quad_box(
        165.0,
        330.0,
        165.0,
        Transform::compose(
            Transform::rotate(Axis3::Y, -FRAC_PI_8),
            Transform::translate(vec3!(347.5, 165.1, 377.5)),
        ),
    )
    .into_iter()
    .map(|x: Quad| Box::new(x) as _)
    .collect();
    world.push(Primitive {
        shape: Box::new(Medium::new(Box::new(Composite { objects: box2 }), 0.015)),
        material: Box::new(Isotropic {
            color: Rgb([0.2, 0.2, 0.2]),
        }),
    });

    let obj = obj::Obj::load("./data/buddha.obj").unwrap();
    let vertices: Vec<Point3f> = obj.data.position.iter().map(|x| point3!(x[0], x[1], x[2])).collect();
    let normals = obj.data.normal;
    let group = obj.data.objects.first().unwrap().groups.first().unwrap();

    let mut triangles: Vec<Triangle> = vec![];
    for x in group.polys.iter() {
        let a = x.0[0].0;
        let b = x.0[1].0;
        let c = x.0[2].0;
        // let d = x.0[3].0;

        triangles.push(Triangle::new(
            vertices[a],
            vertices[b] - vertices[a],
            vertices[c] - vertices[a],
            &Transform::compose_iter([
                Transform::scale_uniform(40.),
                Transform::rotate(Axis3::Y, PI),
                Transform::translate(vec3!(400.5, 0., 200.5)),
            ]),
            // &Transform::compose_iter([
            //     Transform::scale_uniform(150.),
            //     Transform::rotate(Axis3::Z, FRAC_PI_2),
            //     Transform::rotate(Axis3::Y, PI),
            //     Transform::rotate(Axis3::X, -FRAC_PI_3),
            //     Transform::translate(vec3!(400.5, 150., 200.5)),
            // ]),
        ));
        // triangles.push(Triangle::new(
        //     vertices[d],
        //     vertices[c] - vertices[d],
        //     vertices[a] - vertices[d],
        //     &Transform::compose_iter([
        //         Transform::scale_uniform(150.),
        //         Transform::rotate(Axis3::Z, FRAC_PI_2),
        //         Transform::rotate(Axis3::Y, PI),
        //         Transform::rotate(Axis3::X, -FRAC_PI_3),
        //         Transform::translate(vec3!(400.5, 150., 200.5)),
        //     ]
        //     ),
        // ));
    }
    for t in triangles {
        world.push(Primitive {
            shape: Box::new(t),
            material: Box::new(Metal {
                albedo: Rgb([0.8, 0.5, 0.2]),
                fuzz: 0.75,
                // attenuation: Rgb([0.95, 0.95, 0.95]),
                // refraction_index: 2.4,
            }),
        })
    }

    let world = BVH::new(world.into_iter().map(Rc::new).collect(), 4);

    let camera = SimpleCamera::from(CameraConfig {
        position: point3!(278., 278., -800.),
        look_at: point3!(278., 278., 0.),
        up: vec3!(0., 1., 0.),
        aspect_ratio: 1.0,
        vertical_fov: 40.0,
        defocus_angle: 0.01,
        focus_dist: 950.0,
    });

    Scene {
        camera,
        objects: world,
        materials,
        background_color: Rgb([0.0, 0.0, 0.0]),
    }
}
