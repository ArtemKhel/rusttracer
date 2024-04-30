use std::time::Instant;

use image::{ImageBuffer, Pixel, Rgb};
use rand::random;

use geometry::Cross;
use geometry::point::Point;
use geometry::ray::Ray;
use geometry::utils::random_in_unit_disk;
use geometry::vec::Vec3;

use crate::scene::Scene;
use crate::utils;
use crate::utils::{degrees_to_radians, linear_to_gamma};

pub struct Resolution {
    pub width: u32,
    pub height: u32,
}

pub trait Render {
    fn render(&self) -> ImageBuffer<Rgb<f32>, Vec<f32>>;
}

pub struct RayTracer {
    pub scene: Scene,
    pub resolution: Resolution,
    pub antialiasing: u32,
    // TODO: <-
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

    fn defocus_disk_sample(&self, u: Vec3, v: Vec3) -> Point {
        let rnd = random_in_unit_disk();
        self.scene.camera.position + u * rnd.x + v * rnd.y
    }
}

impl Render for RayTracer {
    fn render(&self) -> ImageBuffer<Rgb<f32>, Vec<f32>> {
        let defocus_angle = 0.25;
        let focus_dist = 10.;
        // let focal_length = (self.scene.camera.look_at - self.scene.camera.position).len();

        let theta = utils::degrees_to_radians(self.scene.camera.vertical_fov);
        let h = f32::tan(theta / 2.0);

        let viewport_height = 2. * h * focus_dist;
        let viewport_width = viewport_height * (self.resolution.width as f32 / self.resolution.height as f32);

        let w = (self.scene.camera.position - self.scene.camera.look_at).to_unit();
        let u = self.scene.camera.up.cross(w.vec).to_unit();
        let v = w.vec.cross(u.vec).to_unit();

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = u * viewport_width;// Vec3::new(viewport_width, 0., 0.);
        let viewport_v = v * -viewport_height;// Vec3::new(0., -viewport_height, 0.);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / self.resolution.width as f32;
        let pixel_delta_v = viewport_v / self.resolution.height as f32;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = self.scene.camera.position
            - focus_dist * w
            - viewport_u / 2.
            - viewport_v / 2.;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let defocus_radius = focus_dist * f32::tan(degrees_to_radians(defocus_angle / 2.));
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        let mut image = ImageBuffer::new(self.resolution.width, self.resolution.height);

        // let aliasing_offsets = [0.5 * (pixel_delta_u * random::<f32>() + pixel_delta_v * random::<f32>()); 128];
        let aliasing_offsets = [0.5 * (pixel_delta_u * random::<f32>() + pixel_delta_v * random::<f32>()); 4];
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

        let start = Instant::now();
        for (x, y, pixel) in image.enumerate_pixels_mut() {
            let mut color = Rgb([0., 0., 0.]);

            for offset in aliasing_offsets {
                let pixel_center = pixel00_loc + (pixel_delta_u * x as f32) + (pixel_delta_v * y as f32) + offset;
                let ray_origin = if defocus_angle <= 0. { self.scene.camera.position } else { self.defocus_disk_sample(defocus_disk_u, defocus_disk_v) };
                let ray_direction = self.scene.camera.position.unit_vector_to(pixel_center);
                let ray = Ray::new(ray_origin, ray_direction);
                color.apply2(&self.ray_color(&ray, 0), |x, y| x + y);
            }

            *pixel = linear_to_gamma(color.map(|x| x / aliasing_offsets.len() as f32))
        }
        let finish = Instant::now();
        println!("Render time: {:?}", finish - start);
        // let bar = indicatif::ProgressBar::new(self.config.width as u64);
        // bar.finish();
        // bar.inc(1);
        // println!("{:?}", bar.elapsed());

        image
    }
}
