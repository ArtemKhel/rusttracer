#![allow(unused)]

use std::{ops::Mul, rc::Rc};

use env_logger::fmt::style::AnsiColor::White;
use image::{buffer::ConvertBuffer, Rgb, RgbImage};
use math::{point3, vec3, Point3, Quad, Sphere};
use rand::random;
use rusttracer::{
    aggregates::BVH,
    material::{
        dielectric::Dielectric, diffuse_light::DiffuseLight, isotropic::Isotropic, lambertian::Lambertian,
        metal::Metal, Material,
    },
    mediums::Medium,
    rendering::{AAType::RegularGrid, RayTracer, Renderer, Resolution},
    scene::{Camera, CameraConfig, Composite, Primitive, Scene},
    utils::lerp,
    Point, Triangle, Vec3,
};
use serde::de::value::IsizeDeserializer;

fn spheres() -> Scene {
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

fn cornell_box() -> Scene {
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
    let box2 = Quad::quad_box(point3!(265., 0.1, 295.), point3!(430., 330., 460.))
        .into_iter()
        .map(|x| Box::new(x) as _)
        .collect();
    world.push(Primitive {
        shape: Box::new(Medium::new(Box::new(Composite { objects: box2 }), 0.01)),
        material: Box::new(Isotropic {
            color: Rgb([0.3, 0.3, 0.3]),
        }),
    });

    let world = BVH::new(world.into_iter().map(Rc::new).collect(), 1);

    let camera = Camera::from(CameraConfig {
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

fn cubes() -> Scene {
    fn gen_cubes(
        world: &mut Vec<Primitive>,
        count: usize,
        start: Point,
        size: f32,
        offset: f32,
        start_color: Rgb<f32>,
        end_color: Rgb<f32>,
    ) {
        let diag = Vec3::ones() * size;
        let offset = vec3!((size + offset) / 2., size + offset, (size + offset) / 2.);
        for i in 0..count {
            let start = start + offset * i as f32;
            let sides = Quad::quad_box(start, start + diag);

            let col = lerp(start_color, end_color, (i as f32 / count as f32));
            for side in sides {
                world.push(Primitive {
                    shape: Box::new(side),
                    material: Box::new(Lambertian { albedo: col }),
                })
            }
        }
    }

    let mut world = vec![];
    let size = 4.0;
    let offset = 0.5;
    let x = offset * 2.0 + 1.5 * size;
    let off = vec3!(x, 0., -x);
    let x2 = offset + size;
    let z2 = -size / 2. - offset;
    let off2 = vec3!(x2, z2, z2);
    let off22 = vec3!(-x2, z2, -z2);
    gen_cubes(
        &mut world,
        5,
        point3!(0., 0., 0.),
        4.,
        0.5,
        Rgb([0.06, 0.2, 0.07]),
        Rgb([0.2, 0.9, 0.3]),
    );
    gen_cubes(
        &mut world,
        5,
        point3!(0., 0., 0.) + off,
        4.,
        0.5,
        Rgb([0.2, 0.02, 0.02]),
        Rgb([0.9, 0.2, 0.2]),
    );
    gen_cubes(
        &mut world,
        5,
        point3!(0., 0., 0.) + -off,
        4.,
        0.5,
        Rgb([0.2, 0.02, 0.02]),
        Rgb([0.9, 0.2, 0.2]),
    );

    gen_cubes(
        &mut world,
        5,
        point3!(0., 0., 0.) + -off2,
        4.,
        0.5,
        Rgb([0.02, 0.02, 0.2]),
        Rgb([0.2, 0.2, 0.9]),
    );
    gen_cubes(
        &mut world,
        5,
        point3!(0., 0., 0.) + -off22,
        4.,
        0.5,
        Rgb([0.02, 0.02, 0.2]),
        Rgb([0.2, 0.2, 0.9]),
    );

    world.push(Primitive {
        shape: Box::new(Quad::new(
            point3!(-100., 200., -100.),
            vec3!(200., 0., 0.),
            vec3!(0., 0., 200.),
        )),
        material: Box::new(DiffuseLight {
            color: Rgb([10., 10., 10.]),
        }),
    });

    let world = BVH::new(world.into_iter().map(Rc::new).collect(), 4);

    let materials = vec![];

    let camera = Camera::from(CameraConfig {
        position: point3!(-20., 15., -5.),
        look_at: point3!(3., 10., 3.),
        up: vec3!(0., 1., 0.),
        aspect_ratio: 16.0 / 9.0,
        vertical_fov: 40.0,
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

fn teapot() -> Scene {
    let obj = obj::Obj::load("./data/teapot.obj").unwrap();
    let vertices: Vec<Point> = obj.data.position.iter().map(|x| point3!(x[0], x[1], x[2])).collect();
    let normals = obj.data.normal;
    let group = obj.data.objects.first().unwrap().groups.first().unwrap();

    let mut triangles: Vec<Triangle> = vec![];
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

    let world = BVH::new(world.into_iter().map(Rc::new).collect(), 1);

    let materials = vec![];

    let camera = Camera::from(CameraConfig {
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
fn main() {
    env_logger::init();
    // let scene = spheres();
    let scene = cornell_box();
    // let scene = cubes();
    // let scene = teapot();

    let raytracer = RayTracer {
        scene,
        resolution: Resolution {
            width: 640,
            height: 640,
            // width: 1280,
            // height: 1280,

            // width: 1920,
            // height: 1920,

            // width: 640,
            // height: 360,

            // width: 1280,
            // height: 720,

            // width: 1920,
            // height: 1080,
        },
        // antialiasing: AAType::None.into(),
        antialiasing: RegularGrid(3).into(),
        max_reflections: 5,
    };

    let image = raytracer.render();

    let image: RgbImage = image.convert();
    image.save("./images/_image.png").unwrap()
}
