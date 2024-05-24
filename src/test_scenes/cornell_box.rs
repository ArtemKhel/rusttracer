use std::{f32::consts::FRAC_PI_8, rc::Rc};

use image::Rgb;

use crate::{
    aggregates::BVH,
    material::{
        dielectric::Dielectric, diffuse_light::DiffuseLight, isotropic::Isotropic, lambertian::Lambertian, Material,
    },
    math::{utils::Axis3, Transform},
    mediums::Medium,
    point3,
    scene::{CameraConfig, Composite, Primitive, Scene, SimpleCamera},
    shapes::{mesh::Triangle, quad::Quad, sphere::Sphere},
    vec3, Point3, F,
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
                color: Rgb([1., 1., 1.]),
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
        // point3!(130., 0., 65.),
        // point3!(295., 165., 230.),
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
        // point3!(265., 0.1, 295.),
        // point3!(430., 330., 460.),
        165.0,
        330.0,
        165.0,
        Transform::compose(
            Transform::rotate(Axis3::Y, -FRAC_PI_8),
            Transform::translate(vec3!(347.5, 165.1, 377.5)),
        ),
    )
    .into_iter()
    .map(|x: Quad<f32>| Box::new(x) as _)
    .collect();
    world.push(Primitive {
        shape: Box::new(Medium::new(Box::new(Composite { objects: box2 }), 0.015)),
        material: Box::new(Isotropic {
            color: Rgb([0.2, 0.2, 0.2]),
        }),
    });

    let obj = obj::Obj::load("./data/brilliant_diamond.obj").unwrap();
    let vertices: Vec<Point3> = obj.data.position.iter().map(|x| point3!(x[0], x[1], x[2])).collect();
    let normals = obj.data.normal;
    let group = obj.data.objects.first().unwrap().groups.first().unwrap();

    let mut triangles: Vec<Triangle<F>> = vec![];
    for x in group.polys.iter() {
        let a = x.0[0].0;
        let b = x.0[1].0;
        let c = x.0[2].0;

        triangles.push(Triangle::new(
            vertices[a],
            vertices[b] - vertices[a],
            vertices[c] - vertices[a],
            &Transform::compose(
                Transform::scale_uniform(125.),
                Transform::translate(vec3!(400.5, 0., 200.5)),
            ),
        ));
    }
    for t in triangles {
        world.push(Primitive {
            shape: Box::new(t),
            material: Box::new(Dielectric {
                attenuation: Rgb([0.95, 0.95, 0.95]),
                refraction_index: 2.4,
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
