#![allow(unused)]

use std::sync::Arc;

use image::buffer::ConvertBuffer;
use itertools::Itertools;
use num_traits::Pow;
use rusttracer::{
    aggregates::BVH,
    integrators::{DebugNormalIntegrator, Integrator, PathIntegrator, RandomWalkIntegrator, SimplePathIntegrator},
    light::{DiffuseAreaLight, Light, PointLight},
    material::{matte::Matte, MaterialsEnum},
    math::Transform,
    point2, point3,
    scene::{
        cameras::{BaseCameraConfig, CameraType, OrthographicCamera, OrthographicCameraConfig},
        film::RGBFilm,
        primitives::{geometric::GeometricPrimitive, simple::SimplePrimitive, PrimitiveEnum},
        Scene,
    },
    shapes::{mesh::Triangle, sphere::Sphere},
    test_scenes::cornell_box,
    textures::constant::ConstantSpectrumTexture,
    vec3, Bounds2f, Point3f,
};

fn main() {
    env_logger::init();

    let scene = cornell_box();
    // let mut integrator = DebugNormalIntegrator::new(scene);
    // let mut integrator = RandomWalkIntegrator::new(scene, 5, 2u32.pow(4));
    // let mut integrator = SimplePathIntegrator::create(scene, 6, 2u32.pow(4));
    let mut integrator = PathIntegrator::create(scene, 6, 2u32.pow(4));
    integrator.render();
}

// #[cfg(test)]
// mod tests {
//     use std::cmp::min;
//
//     use ndarray::{Array2, ArrayViewMut2, Axis};
//
//     // keep it for now, may be useful for filters or something
//     fn split_into_chunks<T>(array: &mut Array2<T>, chunk_rows: usize, chunk_cols: usize) -> Vec<ArrayViewMut2<T>> {
//         let n_rows = array.nrows();
//         let n_cols = array.ncols();
//         let mut chunks = Vec::new();
//
//         unsafe {
//             let raw = array.raw_view_mut();
//             let mut row_rest = raw;
//             for row_start in (0..n_rows).step_by(chunk_rows) {
//                 let row_end = min(row_start + chunk_rows, n_rows);
//                 let (row_head, row_tail) = row_rest.split_at(Axis(0), row_end - row_start);
//
//                 let mut col_rest = row_head;
//                 for col_start in (0..n_cols).step_by(chunk_cols) {
//                     let col_end = min(col_start + chunk_cols, n_cols);
//                     let (mut col_head, col_tail) = col_rest.split_at(Axis(1), col_end - col_start);
//                     chunks.push(col_head.deref_into_view_mut());
//                     col_rest = col_tail;
//                 }
//                 row_rest = row_tail
//             }
//         }
//
//         chunks
//     }
//
//     #[test]
//     fn test_() {
//         let mut array = Array2::from_elem((10, 10), [0.]);
//
//         let chunks = unsafe { split_into_chunks(&mut array, 3, 6) };
//
//         for chunk in chunks {
//             println!("{:?}\n", chunk);
//         }
//     }
// }
