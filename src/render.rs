use std::time::Instant;

use image::{ImageBuffer, Pixel, Rgb};
use rand::random;

use geometry::Ray;

use crate::scene::{PixelCoord, Scene};
use crate::utils;
use crate::utils::linear_to_gamma;

pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

pub struct AntiAliasing {
    pub offsets: Vec<PixelCoord>,
}

pub enum AAType {
    Random(usize),
    RegularGrid(usize),
}

impl AntiAliasing {
    pub fn new(aa_type: AAType) -> Self {
        match aa_type {
            AAType::Random(n) => AntiAliasing {
                offsets: vec![[random::<f32>() - 1.0, random::<f32>() - 1.0]; n],
            },
            AAType::RegularGrid(n) => {
                todo!()
            }
        }
    }
}

pub trait Render {
    fn render(&self) -> ImageBuffer<Rgb<f32>, Vec<f32>>;
}

pub struct RayTracer {
    pub scene: Scene,
    pub resolution: Resolution,
    pub antialiasing: Option<AntiAliasing>,
    // TODO: from config
    pub max_reflections: u32,
}

impl RayTracer {
    fn ray_color(&self, ray: &Ray, reflection_depth: u32) -> Rgb<f32> {
        if reflection_depth == self.max_reflections {
            return Rgb([0., 0., 0.]);
        }

        let closest_hit = self.scene.cast_ray(ray);
        if let Some(intersection) = closest_hit {
            // Note the third option: we could scatter with some fixed probability p and have attenuation be albedo/p

            let scatter_direction = intersection.object.material.scatter(ray, &intersection);
            return if let Some(scatter) = scatter_direction {
                let color = self.ray_color(&scatter.ray, reflection_depth + 1);
                scatter.attenuation.map2(&color, |x, y| x * y)
            } else {
                Rgb([0., 0., 0.])
            };
        }
        let a = 0.5 * (ray.dir.vec.y + 1.0);
        utils::lerp(a)
    }
}

impl Render for RayTracer {
    fn render(&self) -> ImageBuffer<Rgb<f32>, Vec<f32>> {
        let mut image = ImageBuffer::new(self.resolution.width, self.resolution.height);

        // let mut rng = SmallRng::from_thread_rng();
        // TODO:
        // let aliasing_offsets = [random::<f32>() - 1.0; 40];
        // let aliasing_offsets = 4;
        let map_x = |x: f32| x / (self.resolution.width as f32 / 2.0) - 1.0;
        let map_y = |y: f32| y / (self.resolution.height as f32 / 2.0) - 1.0;

        let start = Instant::now();
        for (x, y, pixel) in image.enumerate_pixels_mut() {
            let mut color = Rgb([0., 0., 0.]);

            // TODO: None as AA with offsets [0,0]?
            // .unwrap_or_...?
            if let Some(aa) = self.antialiasing.as_ref() {
                for offset in aa.offsets.iter() {
                    let x = map_x(x as f32 + offset[0]);
                    let y = map_y(y as f32 + offset[1]);
                    let ray = self.scene.camera.create_ray(x, y);
                    color.apply2(&self.ray_color(&ray, 0), |x, y| x + y);
                }
                *pixel = linear_to_gamma(color.map(|x| x / aa.offsets.len() as f32))
            } else {
                let x = map_x(x as f32);
                let y = map_y(y as f32);
                let ray = self.scene.camera.create_ray(x, y);
                *pixel = linear_to_gamma(self.ray_color(&ray, 0));
            }
        }
        let finish = Instant::now();
        println!("Render time: {:?}", finish - start);
        // let bar = indicatif::ProgressBar::new(self.config.width as u64);
        // bar.inc(1);
        // bar.finish();
        // println!("{:?}", bar.elapsed());

        image
    }
}
// let aliasing_offsets = [
//     -2. * pixel_delta_u - 2. * pixel_delta_v,
//     -2. * pixel_delta_u - pixel_delta_v,
//     -2. * pixel_delta_u,
//     -2. * pixel_delta_u + pixel_delta_v,
//     -2. * pixel_delta_u + 2. * pixel_delta_v,
//     -pixel_delta_u - 2. * pixel_delta_v,
//     -pixel_delta_u - pixel_delta_v,
//     -pixel_delta_u,
//     -pixel_delta_u + pixel_delta_v,
//     -pixel_delta_u + 2. * pixel_delta_v,
//     -2. * pixel_delta_v,
//     -pixel_delta_v,
//     Vec3::default(),
//     pixel_delta_v,
//     2. * pixel_delta_v,
//     pixel_delta_u - 2. * pixel_delta_v,
//     pixel_delta_u - pixel_delta_v,
//     pixel_delta_u,
//     pixel_delta_u + pixel_delta_v,
//     pixel_delta_u + 2. * pixel_delta_v,
//     2. * pixel_delta_u - 2. * pixel_delta_v,
//     2. * pixel_delta_u - pixel_delta_v,
//     2. * pixel_delta_u,
//     2. * pixel_delta_u + pixel_delta_v,
//     2. * pixel_delta_u + 2. * pixel_delta_v,
// ].map(|x| x * 0.333 * 0.5);
