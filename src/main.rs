#![allow(unused)]

use image::{buffer::ConvertBuffer, RgbImage};
use rusttracer::{
    rendering::{AAType::RegularGrid, RayTracer, Renderer, Resolution},
    test_scenes::*,
};

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
