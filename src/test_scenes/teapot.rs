use std::rc::Rc;

use image::Rgb;
use math::{point3, vec3, Sphere, Triangle};

use crate::{
    aggregates::BVH,
    material::{diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal},
    scene::{SimpleCamera, CameraConfig, Primitive, Scene},
    Point3, F,
};

pub fn teapot() -> Scene {
    let obj = obj::Obj::load("./data/teapot.obj").unwrap();
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
        ));
        // let na = normals[x.0[0].2.unwrap()];
        // let nb = normals[x.0[1].2.unwrap()];
        // let nc = normals[x.0[2].2.unwrap()];
        // triangles.push(Triangle::new_with_normals(  vertices[a],  vertices[b] - vertices[a],  vertices[c] - vertices[a],
        //     [
        //         UnitVec::new(na[0], na[1], na[2]),
        //         UnitVec::new(nb[0], nb[1], nb[2]),
        //         UnitVec::new(nc[0], nc[1], nc[2]),
        //     ]
        // ))
    }

    let mut world = vec![
        Primitive {
            shape: Box::new(Sphere::new(point3!(5., 20.0, 10.), 10.0)),
            material: Box::new(DiffuseLight {
                color: Rgb([5., 5., 5.]),
            }),
        },
        Primitive {
            shape: Box::new(Sphere::new(point3!(0., -1001., 0.), 1000.)),
            material: Box::new(Lambertian {
                albedo: Rgb([0.5, 0.5, 0.5]),
            }),
        },
    ];
    let material = Box::new(Metal {
        albedo: Rgb([0.8, 0.5, 0.2]),
        fuzz: 0.75,
    });
    for t in triangles {
        world.push(Primitive {
            shape: Box::new(t),
            material: material.clone(),
        })
    }

    let world = BVH::new(world.into_iter().map(Rc::new).collect(), 8);

    let materials = vec![];

    let camera = SimpleCamera::from(CameraConfig {
        position: point3!(2., 5., 5.),
        look_at: point3!(0.5, 1., 0.),
        up: vec3!(0., 1., 0.),
        aspect_ratio: 16.0 / 9.0,
        vertical_fov: 40.0,
        defocus_angle: 0.05,
        focus_dist: 10.0,
    });

    Scene {
        camera,
        objects: world,
        materials,
        background_color: Rgb([0.1, 0.1, 0.1]),
    }
}
