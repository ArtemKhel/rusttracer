use std::time::Instant;

use image::{ImageBuffer, Pixel, Rgb};
use rand::distributions::Distribution;

use crate::intersection::Intersection;
use crate::primitive::Primitive;
use geometry::point::Point;
use geometry::ray::Ray;
use geometry::utils::random_unit;
use geometry::vec::Vec3;
use geometry::Object;

pub struct CameraConfig {
    // aspect_ratio: f32,
    width: u32,
    height: u32,
    focal_length: f32,
}

impl Default for CameraConfig {
    fn default() -> Self {
        let aspect_ratio = 16. / 9.;
        let width = 400;
        CameraConfig {
            // aspect_ratio,
            width,
            height: (width as f32 / aspect_ratio) as u32,
            focal_length: 1.,
        }
    }
}

pub struct Camera<'a> {
    position: Point,
    look_at: Point,
    world: &'a Vec<Box<Primitive>>,
    config: CameraConfig,
}

impl<'a> Camera<'a> {
    pub fn new(position: Point, look_at: Point, world: &'a Vec<Box<Primitive>>) -> Camera<'a> {
        Camera {
            position,
            look_at,
            world,
            config: CameraConfig::default(),
        }
    }

    pub fn render(&self) -> ImageBuffer<Rgb<f32>, Vec<f32>> {
        let reflection_depth = 4;
        let viewport_height = 2.;
        let viewport_width = viewport_height * (self.config.width as f32 / self.config.height as f32);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0., 0.);
        let viewport_v = Vec3::new(0., -viewport_height, 0.);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / self.config.width as f32;
        let pixel_delta_v = viewport_v / self.config.height as f32;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            self.position - Vec3::new(0., 0., self.config.focal_length) - viewport_u / 2. - viewport_v / 2.;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let mut image = ImageBuffer::new(self.config.width, self.config.height);

        let aliasing_offsets = [
            0.25 * (pixel_delta_u + pixel_delta_v),
            0.25 * (pixel_delta_u - pixel_delta_v),
            0.25 * (-pixel_delta_u + pixel_delta_v),
            0.25 * (-pixel_delta_u - pixel_delta_v),
        ];
        // let aliasing_offsets = [
        //     Vec3::default(),
        //     -pixel_delta_u - pixel_delta_v,
        //     -pixel_delta_u,
        //     -pixel_delta_u + pixel_delta_v,
        //     pixel_delta_v,
        //     pixel_delta_u - pixel_delta_v,
        //     pixel_delta_u,
        //     -pixel_delta_u + pixel_delta_v,
        //     -pixel_delta_v,
        // ].map(|x| x*0.25);

        let start = Instant::now();
        // let bar = indicatif::ProgressBar::new(self.config.width as u64);
        for (x, y, pixel) in image.enumerate_pixels_mut() {
            let mut color: [f32; 3] = [0., 0., 0.];
            for offset in aliasing_offsets {
                let pixel_center = pixel00_loc + (pixel_delta_u * x as f32) + (pixel_delta_v * y as f32) + offset;
                let ray_direction = self.position.unit_vector_to(pixel_center);
                let ray = Ray::new(self.position, ray_direction);
                color
                    .iter_mut()
                    .zip(self.ray_color(&ray, reflection_depth).0)
                    .for_each(|(c, x)| {
                        *c += x;
                    });
            }
            *pixel = Self::linear_to_gamma(Rgb(color.map(|x| x / aliasing_offsets.len() as f32)))
            // if self.config.width == 0 { bar.inc(1); }
        }
        let finish = Instant::now();
        println!("Render time: {:?}", finish - start);
        // bar.finish();
        // println!("{:?}", bar.elapsed());

        // image.save("./images/image.png").expect("");
        image
    }

    fn linear_to_gamma(linear: Rgb<f32>) -> Rgb<f32> {
        linear.map(|x| if x > 0. { x.sqrt() } else { x })
    }

    fn ray_color(&self, ray: &Ray, reflection_depth: u8) -> Rgb<f32> {
        if reflection_depth == 0 {
            return Rgb([0., 0., 0.]);
        }

        let closest_hit = self.world.iter().fold((None, f32::INFINITY), |closest, primitive| {
            if let Some(hit) = primitive.object.hit(ray) {
                let dist = ray.origin.distance_to(hit.point);
                if dist < closest.1 {
                    return (
                        Some(Intersection {
                            hit,
                            material: primitive.material,
                        }),
                        dist,
                    );
                }
            }
            closest
        });
        if let (Some(intersection), _) = closest_hit {
            // Note the third option: we could scatter with some fixed probability p and have attenuation be albedo/p
            let scatter_direction = if !intersection.material.metal{
                (intersection.hit.normal.vec + random_unit().vec).to_unit()
            } else{
                (ray.dir.reflect(intersection.hit.normal).vec + random_unit() * intersection.material.fuzz).to_unit()
            };
            let color = self.ray_color(
                &Ray::new(
                    intersection.hit.point + intersection.hit.normal * 0.01,
                    scatter_direction,
                ),
                reflection_depth - 1,
            );
            return intersection.material.color.map2(&color, |x, y| x * y);
        }
        let a = 0.5 * (ray.dir.vec.y + 1.0);
        lerp(a)
    }
}

fn lerp(/*a: Rgb<u8>, b: Rgb<u8>,*/ t: f32) -> Rgb<f32> {
    let a = Rgb([1.0, 1.0, 1.0]);
    let b = Rgb([0.5, 0.7, 1.0]);
    Rgb([
        (1. - t) * a.0[0] + t * b.0[0],
        (1. - t) * a.0[1] + t * b.0[1],
        (1. - t) * a.0[2] + t * b.0[2],
    ])
}
