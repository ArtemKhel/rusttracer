#![allow(unused)]

use std::{
    f32::consts::PI,
    iter::repeat,
    sync::{
        atomic::{AtomicU32, Ordering::Relaxed},
        Arc,
    },
};
use std::f32::consts::FRAC_PI_4;

use image::{buffer::ConvertBuffer, Rgb, RgbImage};
use itertools::Itertools;
use log::debug;
use rusttracer::{
    aggregates::BVH,
    colors,
    integrators::{debug_normal::DebugNormalIntegrator, Integrator},
    material::{matte::Matte, MaterialsEnum},
    math::{axis::Axis3, Transform},
    point2,
    scene::{
        cameras::{
            base::BaseCameraConfig,
            orthographic::{OrthographicCamera, OrthographicCameraConfig},
            projective::ScreenWindow,
            CameraType::Orthographic,
        },
        film::{BaseFilm, Resolution},
        PrimitiveEnum, Scene, SimplePrimitive,
    },
    shapes::sphere::Sphere,
    test_scenes::*,
    textures::constant::ConstantTexture,
    vec3, Point2u, CALLS, SKIP,
};
use rusttracer::integrators::random_walk::RandomWalkIntegrator;

fn main() {
    env_logger::init();

    let camera = Orthographic(OrthographicCamera::from(OrthographicCameraConfig {
        base_config: BaseCameraConfig {
            transform: Transform::id()
                // .then_rotate_degrees(Axis3::Y, 225.)
                // .then_rotate_arbitrary_axis(vec3!(-1.,0.,1.), FRAC_PI_4)
                // .then_translate(vec3!(1., 1., 1.)),
                .then_translate(vec3!(0., 0., -1.)),
            film: BaseFilm {
                resolution: point2!(300u32, 300u32),
            },
        },
        screen_window: ScreenWindow {
            min: point2!(-1., -1.),
            max: point2!(1., 1.),
        },
        lens_radius: 0.0,
        focal_distance: 0.0,
    }));

    let primitives = vec![PrimitiveEnum::Simple(SimplePrimitive {
        shape: Arc::new(Sphere {
            radius: 0.5,
            transform: Default::default(),
        }),
        material: Arc::new(
            (MaterialsEnum::Matte(Matte {
                reflectance: Arc::new(ConstantTexture { value: colors::GREEN }),
            })),
        ),
    })]
    .into_iter()
    .map(Arc::new)
    .collect();

    let objects = PrimitiveEnum::BVH(BVH::new(primitives, 1));

    let scene = Scene {
        camera,
        objects,
        background_color: Rgb([0.25, 0.25, 0.25]),
    };

    // let integrator = DebugNormalIntegrator { scene };
    let integrator = RandomWalkIntegrator { scene, max_depth: 3 };
    let image = integrator.render();
    let image: RgbImage = image.convert();
    image.save("./images/_image.png").unwrap();

    // // let scene = spheres();
    // let scene = cornell_box();
    // // let scene = cubes();
    // // let scene = teapot();
    //
    // let raytracer = RayTracer {
    //     scene,
    //     resolution: Resolution {
    //         width: 640,
    //         height: 640,
    //         //
    //         // width: 1280,
    //         // height: 1280,
    //         //
    //         // width: 1920,
    //         // height: 1920,
    //         //
    //         // width: 640,
    //         // height: 360,
    //         //
    //         // width: 1280,
    //         // height: 720,
    //         //
    //         // width: 1920,
    //         // height: 1080,
    //     },
    //     // antialiasing: AAType::None.into(),
    //     antialiasing: RegularGrid(5).into(),
    //     max_reflections: 5,
    // };
    //
    // let image = raytracer.render();
    //
    // let image: RgbImage = image.convert();
    // image.save("./images/_image.png").unwrap();

    let c: u32 = CALLS.swap(0, Relaxed);
    let s: u32 = SKIP.swap(0, Relaxed);
    debug!("calls {c}, skips {s}, ratio {}", s as f32 / c as f32);
}
