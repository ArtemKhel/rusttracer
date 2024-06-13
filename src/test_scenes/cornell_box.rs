use std::sync::Arc;

use crate::{
    aggregates::BVH,
    light::{DiffuseAreaLight, LightEnum, PointLight},
    material::{glass::Glass, matte::Matte, metal::Metal, MaterialsEnum},
    math::{axis::Axis3, Transform},
    point2, point3,
    scene::{
        cameras::{BaseCameraConfig, CameraType, PerspectiveCamera, PerspectiveCameraConfig},
        film::RGBFilm,
        primitives::{geometric::GeometricPrimitive, simple::SimplePrimitive, PrimitiveEnum},
        Scene,
    },
    shapes::quad::Quad,
    spectra::{
        named::NamedSpectra,
        piecewise_linear::PiecewiseLinearSpectrum,
        rgb::{sRGB, RGB},
        RGBAlbedoSpectrum, SpectrumEnum, VISIBLE_MAX, VISIBLE_MIN,
    },
    test_scenes::teapot_triangles,
    textures::constant::ConstantSpectrumTexture,
    vec3, Bounds2f,
};

fn base_box(
    left_wall: &Arc<MaterialsEnum>,
    right_wall: &Arc<MaterialsEnum>,
    back_wall: &Arc<MaterialsEnum>,
    other_walls: &Arc<MaterialsEnum>,
) -> Vec<Arc<PrimitiveEnum>> {
    let mut walls = vec![
        // left wall
        SimplePrimitive {
            shape: Arc::new(Quad::new(
                point3!(1000., 0., 0.),
                vec3!(0., 1000., 0.),
                vec3!(0., 0., 1000.),
                Transform::id(),
            )),
            material: left_wall.clone(),
        },
        // right wall
        SimplePrimitive {
            shape: Arc::new(Quad::new(
                point3!(0., 0., 0.),
                vec3!(0., 1000., 0.),
                vec3!(0., 0., 1000.),
                Transform::id(),
            )),
            material: right_wall.clone(),
        },
        // floor
        SimplePrimitive {
            shape: Arc::new(Quad::new(
                point3!(0., 0., 0.),
                vec3!(1000., 0., 0.),
                vec3!(0., 0., 1000.),
                Transform::id(),
            )),
            material: other_walls.clone(),
        },
        // ceiling
        SimplePrimitive {
            shape: Arc::new(Quad::new(
                point3!(0., 1000., 0.),
                vec3!(1000., 0., 0.),
                vec3!(0., 0., 1000.),
                Transform::id(),
            )),
            material: other_walls.clone(),
        },
        // back wall
        SimplePrimitive {
            shape: Arc::new(Quad::new(
                point3!(0., 0., 1000.),
                vec3!(1000., 0., 0.),
                vec3!(0., 1000., 0.),
                Transform::id(),
            )),
            material: back_wall.clone(),
        },
    ];

    let mut base_box: Vec<Arc<PrimitiveEnum>> = walls.into_iter().map(|x| Arc::new(PrimitiveEnum::Simple(x))).collect();
    base_box
}

pub fn cornell_box() -> Scene {
    let camera: CameraType = PerspectiveCamera::new(PerspectiveCameraConfig {
        base_config: BaseCameraConfig {
            transform: Transform::id()
                .then_rotate_degrees(Axis3::Y, 180.)
                .then_translate(vec3!(500., 500., -1000.)),
            film: RGBFilm::new(400, 400, sRGB.clone()),
        },
        fov: 55.0,
        screen_window: Bounds2f::from_points(point2!(-1., -1.), point2!(1., 1.)),
        lens_radius: 5.0,
        focal_distance: 1500.0,
    })
    .into();

    let white = Arc::new(SpectrumEnum::RGBAlbedo(RGBAlbedoSpectrum::new(&sRGB, RGB::WHITE)));
    let light_gray = Arc::new(SpectrumEnum::RGBAlbedo(RGBAlbedoSpectrum::new(&sRGB, RGB::LIGHT_GRAY)));
    let green = Arc::new(SpectrumEnum::RGBAlbedo(RGBAlbedoSpectrum::new(&sRGB, RGB::GREEN)));
    let red = Arc::new(SpectrumEnum::RGBAlbedo(RGBAlbedoSpectrum::new(&sRGB, RGB::RED)));

    let const_white = Arc::new(ConstantSpectrumTexture { value: white });
    let const_gray = Arc::new(ConstantSpectrumTexture { value: light_gray });
    let const_green = Arc::new(ConstantSpectrumTexture { value: green });
    let const_red = Arc::new(ConstantSpectrumTexture { value: red });

    let matte_gray = Arc::new(MaterialsEnum::Matte(Matte {
        reflectance: const_gray.clone() as _,
    }));
    let matte_green = Arc::new(MaterialsEnum::Matte(Matte {
        reflectance: const_green.clone() as _,
    }));
    let matte_red = Arc::new(MaterialsEnum::Matte(Matte {
        reflectance: const_red.clone() as _,
    }));
    let metal = Arc::new(MaterialsEnum::Metal(Metal {
        reflectance: const_white.clone(),
        eta: const_white.clone(),
        k: const_white.clone(),
    }));
    let glass = Arc::new(MaterialsEnum::Glass(Glass {
        spectrum: const_gray.clone(),
        ior: SpectrumEnum::PiecewiseLinear(PiecewiseLinearSpectrum::new(&[VISIBLE_MIN, VISIBLE_MAX], &[2.5, 1.5])),
    }));

    // let mut cornell_box = base_box(&matte_green, &matte_red, &metal, &matte_gray, &glass);
    let mut cornell_box = base_box(&matte_green, &matte_red, &matte_gray, &matte_gray);

    let light_shape = Arc::new(Quad::new(
        point3!(250., 950., 250.),
        vec3!(50., 0., 0.),
        vec3!(0., 0., 500.),
        Transform::id(),
    ));
    let d65 = NamedSpectra::IlluminantD65.get();
    let light_source = Arc::new(LightEnum::DiffuseArea(DiffuseAreaLight::new(
        // Arc::new(SpectrumEnum::Constant(ConstantSpectrum::new(1.))),
        d65.clone(),
        1.5,
        Transform::id(),
        light_shape.clone(),
    )));
    let light = GeometricPrimitive {
        shape: light_shape.clone(),
        material: matte_gray.clone(),
        light: Some(light_source.clone()),
    };
    cornell_box.push(Arc::new(PrimitiveEnum::Geometric(light)));

    let tri = teapot_triangles(
        Transform::scale_uniform(2500.)
            .then_rotate_degrees(Axis3::X, 90.)
            .then_translate(vec3!(500., 500., 500.)),
    );
    let tri: Vec<Arc<PrimitiveEnum>> = tri
        .into_iter()
        .map(Arc::new)
        .map(|x| SimplePrimitive {
            shape: x,
            material: glass.clone(),
        })
        .map(PrimitiveEnum::Simple)
        .map(Arc::new)
        .collect();
    cornell_box.extend(tri);

    // cornell_box.push(Arc::new(PrimitiveEnum::Simple(SimplePrimitive {
    //     shape: Arc::new(Sphere {
    //         radius: 300.,
    //         transform: Transform::translate(vec3!(500., 400., 500.)),
    //     }),
    //     material: glass.clone(),
    // })));

    let objects = PrimitiveEnum::BVH(BVH::new(cornell_box, 8));
    let lights = vec![
        light_source as _,
        // Arc::new(LightEnum::Point(PointLight::new(
        //     d65.clone(),
        //     500_000.,
        //     Transform::translate(vec3!(200., 200., 500.)),
        // ))),
    ];
    // let lights = vec![Arc::new(LightEnum::Point(PointLight::new(
    //     NamedSpectra::IlluminantD65.get(),
    //     1.,
    //     Transform::translate(vec3!(500., 950., 500.)),
    // )))];

    Scene {
        camera,
        objects,
        lights,
    }
    // for side in Quad::quad_box(
    //     165.0,
    //     165.0,
    //     165.0,
    //     TransformBuilder::default()
    //         .rotate(Axis3::Y, FRAC_PI_8)
    //         .translate(vec3!(150.5, 82.5, 150.5))
    //         .build(),
    // )
    // .into_iter()
    // {
    //     world.push(Primitive {
    //         shape: Box::new(side),
    //         material: Box::new(Lambertian {
    //             albedo: colors::LIGHT_GRAY,
    //         }),
    //     })
    // }
    // let box2 = Quad::quad_box(
    //     165.0,
    //     330.0,
    //     165.0,
    //     TransformBuilder::default()
    //         .rotate(Axis3::Y, -FRAC_PI_8)
    //         .translate(vec3!(347.5, 165.1, 377.5))
    //         .build(),
    // )
    // .into_iter()
    // .map(|x: Quad| Box::new(x) as _)
    // .collect();
    // world.push(Primitive {
    //     shape: Box::new(Medium::new(Box::new(Composite { objects: box2 }), 0.015)),
    //     material: Box::new(Isotropic {
    //         color: Rgb([0.2, 0.2, 0.2]),
    //     }),
    // });
    //
    // let obj = obj::Obj::load("./data/buddha.obj").unwrap();
    // let vertices: Vec<Point3f> = obj.data.position.iter().map(|x| point3!(x[0], x[1], x[2])).collect();
    // let normals = obj.data.normal;
    // let group = obj.data.objects.first().unwrap().groups.first().unwrap();
    //
    // let mut triangles: Vec<Triangle> = vec![];
    // for x in group.polys.iter() {
    //     let a = x.0[0].0;
    //     let b = x.0[1].0;
    //     let c = x.0[2].0;
    //     // let d = x.0[3].0;
    //
    //     triangles.push(Triangle::new(
    //         vertices[a],
    //         vertices[b] - vertices[a],
    //         vertices[c] - vertices[a],
    //         Transform::compose_iter([
    //             Transform::scale_uniform(40.),
    //             Transform::rotate(Axis3::Y, PI),
    //             Transform::translate(vec3!(400.5, 0., 200.5)),
    //         ]),
    //         // &Transform::compose_iter([
    //         //     Transform::scale_uniform(150.),
    //         //     Transform::rotate(Axis3::Z, FRAC_PI_2),
    //         //     Transform::rotate(Axis3::Y, PI),
    //         //     Transform::rotate(Axis3::X, -FRAC_PI_3),
    //         //     Transform::translate(vec3!(400.5, 150., 200.5)),
    //         // ]),
    //     ));
    //     // triangles.push(Triangle::new(
    //     //     vertices[d],
    //     //     vertices[c] - vertices[d],
    //     //     vertices[a] - vertices[d],
    //     //     &Transform::compose_iter([
    //     //         Transform::scale_uniform(150.),
    //     //         Transform::rotate(Axis3::Z, FRAC_PI_2),
    //     //         Transform::rotate(Axis3::Y, PI),
    //     //         Transform::rotate(Axis3::X, -FRAC_PI_3),
    //     //         Transform::translate(vec3!(400.5, 150., 200.5)),
    //     //     ]
    //     //     ),
    //     // ));
    // }
    // for t in triangles {
    //     world.push(Primitive {
    //         shape: Box::new(t),
    //         material: Box::new(Metal {
    //             albedo: Rgb([0.8, 0.5, 0.2]),
    //             fuzz: 0.75,
    //             // attenuation: Rgb([0.95, 0.95, 0.95]),
    //             // refraction_index: 2.4,
    //         }),
    //     })
    // }
    //
    // let world = BVH::new(world.into_iter().map(Rc::new).collect(), 8);
    //
    // let camera = SimpleCamera::from(CameraConfig {
    //     transform: TransformBuilder::default()
    //         .rotate(Axis3::Y, PI)
    //         // .rotate(Axis3::Z, FRAC_PI_4)
    //         .translate(vec3!(278., 278., -750.))
    //         .build(),
    //     aspect_ratio: 1.0,
    //     vertical_fov: 40.0,
    //     defocus_angle: 1.,
    //     focus_dist: 1000.0,
    // });
    //
    // Scene {
    //     camera,
    //     objects: world,
    //     materials,
    //     background_color: colors::BLACK,
    // }
}
