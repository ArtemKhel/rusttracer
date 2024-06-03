#![allow(unused)]

use std::sync::Arc;

use image::{buffer::ConvertBuffer, Rgb};
use itertools::Itertools;
use rusttracer::{
    aggregates::BVH,
    colors,
    integrators::{random_walk::RandomWalkIntegrator, Integrator},
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
        film::RGBFilm,
        PrimitiveEnum, Scene, SimplePrimitive,
    },
    shapes::sphere::Sphere,
    textures::checkerboard::CheckerboardTexture,
    vec3,
};

fn main() {
    env_logger::init();

    let camera = Orthographic(OrthographicCamera::from(OrthographicCameraConfig {
        base_config: BaseCameraConfig {
            transform: Transform::id()
                // .then_rotate_degrees(Axis3::Y, 225.)
                // .then_rotate_arbitrary_axis(vec3!(-1.,0.,1.), FRAC_PI_4)
                // .then_translate(vec3!(1., 1., 1.)),
                .then_rotate_degrees(Axis3::X, 45.)
                .then_translate(vec3!(0., 1., -1.)),
            film: RGBFilm::new(point2!(300u32, 300u32)),
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
            MaterialsEnum::Matte(Matte {
                reflectance: Arc::new(
                    // ConstantTexture { value: colors::GREEN }
                    CheckerboardTexture {
                        light: colors::GREEN,
                        dark: colors::RED,
                        size: 0.1,
                    },
                ),
            }),
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
    let mut integrator = RandomWalkIntegrator::new(scene, 5, 1);
    integrator.render();
}

#[cfg(test)]
mod tests {
    use std::cmp::min;

    use ndarray::{Array2, ArrayViewMut2, Axis};
    use rusttracer::math::Bounds2;

    // keep it for now, may be useful for filters or something
    fn split_into_chunks<T>(array: &mut Array2<T>, chunk_rows: usize, chunk_cols: usize) -> Vec<ArrayViewMut2<T>> {
        let n_rows = array.nrows();
        let n_cols = array.ncols();
        let mut chunks = Vec::new();

        unsafe {
            let raw = array.raw_view_mut();
            let mut row_rest = raw;
            for row_start in (0..n_rows).step_by(chunk_rows) {
                let row_end = min(row_start + chunk_rows, n_rows);
                let (row_head, row_tail) = row_rest.split_at(Axis(0), row_end - row_start);

                let mut col_rest = row_head;
                for col_start in (0..n_cols).step_by(chunk_cols) {
                    let col_end = min(col_start + chunk_cols, n_cols);
                    let (mut col_head, col_tail) = col_rest.split_at(Axis(1), col_end - col_start);
                    chunks.push(col_head.deref_into_view_mut());
                    col_rest = col_tail;
                }
                row_rest = row_tail
            }
        }

        chunks
    }

    #[test]
    fn test_() {
        let mut array = Array2::from_elem((10, 10), [0.]);

        let chunks = unsafe { split_into_chunks(&mut array, 3, 6) };

        for chunk in chunks {
            println!("{:?}\n", chunk);
        }
    }
}
