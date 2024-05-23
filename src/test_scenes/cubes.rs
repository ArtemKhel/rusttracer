use std::rc::Rc;

use image::Rgb;
use math::{point3, vec3, Quad};

use crate::{
    aggregates::BVH,
    material::{diffuse_light::DiffuseLight, lambertian::Lambertian},
    scene::{SimpleCamera, CameraConfig, Primitive, Scene},
    utils::lerp,
    Point3, Vec3,
};

pub fn cubes() -> Scene {
    fn gen_cubes(
        world: &mut Vec<Primitive>,
        count: usize,
        start: Point3,
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

    let camera = SimpleCamera::from(CameraConfig {
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
