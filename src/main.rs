#![allow(unused)]

use std::sync::Arc;

use image::buffer::ConvertBuffer;
use itertools::Itertools;
use num_traits::Pow;
use rusttracer::{
    aggregates::BVH,
    colors,
    integrators::{random_walk::RandomWalkIntegrator, simple_path::SimplePathIntegrator, Integrator},
    light::{diffuse_area::DiffuseAreaLight, point::PointLight, Light},
    material::{matte::Matte, MaterialsEnum},
    math::Transform,
    point2, point3,
    scene::{
        cameras::{BaseCameraConfig, OrthographicCamera, OrthographicCameraConfig},
        film::RGBFilm,
        primitives::{geometric::GeometricPrimitive, simple::SimplePrimitive, PrimitiveEnum},
    },
    shapes::{mesh::Triangle, sphere::Sphere},
    test_scenes::cornell_box,
    textures::{checkerboard::CheckerboardTexture, constant::ConstantTexture},
    vec3, Bounds2f, Point3f,
};

fn teapot_triangles() -> Vec<Triangle> {
    let obj = obj::Obj::load("./data/teapot.obj").unwrap();
    let vertices: Vec<Point3f> = obj.data.position.iter().map(|x| point3!(x[0], x[1], x[2])).collect();
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
            Transform::scale(0.25, 0.25, 0.25),
        ));
    }
    triangles
}

fn main() {
    env_logger::init();

    let camera = OrthographicCamera::new(OrthographicCameraConfig {
        base_config: BaseCameraConfig {
            transform: Transform::id().then_translate(vec3!(0., 0., -1.)),
            film: RGBFilm::new(300, 300),
        },
        screen_window: Bounds2f::from_points(point2!(-1., -1.), point2!(1., 1.)),
        lens_radius: 0.0,
        focal_distance: 0.0,
    });

    let const_gray = Arc::new(ConstantTexture {
        value: colors::LIGHT_GRAY,
    });

    let mat = Arc::new(
        MaterialsEnum::Matte(Matte {
            reflectance: Arc::new(
                // ConstantTexture {
                //     value: colors::LIGHT_GRAY,
                // },
                CheckerboardTexture {
                    light: colors::GREEN,
                    dark: colors::RED,
                    size: 0.1,
                },
            ),
        }), /* MaterialsEnum::Metal(Metal {
                 *          reflectance: const_gray.clone(),
                 *          eta: const_gray.clone(),
                 *          k: const_gray.clone(),
                 *      }), */

            /* MaterialsEnum::Glass(Glass {
             *      ior: 2.,
             *      spectrum: const_gray.clone(),
             *  }), */
    );

    let mut primitives: Vec<Arc<PrimitiveEnum>> = vec![
        PrimitiveEnum::Simple(SimplePrimitive {
            shape: Arc::new(Sphere {
                radius: 0.7,
                transform: Transform::id(),
            }),
            material: mat.clone(),
        }),
        PrimitiveEnum::Geometric(GeometricPrimitive {
            shape: Arc::new(Sphere {
                radius: 10.,
                transform: Transform::translate(vec3!(0., 12., 0.)),
            }),
            material: mat.clone(),
            light: Some(Arc::new(DiffuseAreaLight::new(
                colors::WHITE,
                1.,
                Transform::translate(vec3!(0., 0., 0.)),
                Arc::new(Sphere {
                    radius: 10.,
                    transform: Transform::id(),
                }),
            ))),
        }),
    ]
    .into_iter()
    .map(Arc::new)
    .collect();

    // let teapot_material = Arc::new(MaterialsEnum::Matte(Matte {
    //     reflectance: Arc::new(ConstantTexture {
    //         value: colors::LIGHT_GRAY,
    //     }),
    // }));
    //
    // let teapot: Vec<Arc<PrimitiveEnum>> = teapot_triangles()
    //     .into_iter()
    //     .map(|t| {
    //         Arc::new(PrimitiveEnum::Simple(SimplePrimitive {
    //             shape: Arc::new(t),
    //             material: teapot_material.clone(),
    //         }))
    //     })
    //     .collect();
    // primitives.extend(teapot);

    let lights: Vec<Arc<dyn Light>> = vec![
        Arc::new(PointLight::new(
            colors::WHITE,
            1.,
            Transform::translate(vec3!(0., 2., 0.)),
        )),
        // Arc::new(DiffuseAreaLight::new(
        //     colors::WHITE,
        //     1.,
        //     Transform::translate(vec3!(0., 3., 0.)),
        //     Arc::new(Sphere {
        //         radius: 1.0,
        //         transform: Transform::id(),
        //     }),
        // )),
    ];

    let objects = PrimitiveEnum::BVH(BVH::new(primitives, 8));

    // let scene = Scene {
    //     camera,
    //     objects,
    //     background_color: Rgb([0.25, 0.25, 0.25]),
    //     lights,
    // };

    let scene = cornell_box();
    // let mut integrator = DebugNormalIntegrator::new(scene);
    // let mut integrator = RandomWalkIntegrator::new(scene, 5, 2u32.pow(4));
    let mut integrator = SimplePathIntegrator::new(scene, 5, 2u32.pow(4));
    integrator.render();
}

#[cfg(test)]
mod tests {
    use std::cmp::min;

    use ndarray::{Array2, ArrayViewMut2, Axis};

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
