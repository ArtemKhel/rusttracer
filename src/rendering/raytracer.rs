use std::time::Instant;

use image::{ImageBuffer, Pixel, Rgb};
use rayon::prelude::*;

use crate::{
    geometry::Ray,
    rendering::{antialiasing::AntiAliasing, PixelCoord, Renderer, Resolution},
    scene::Scene,
    utils,
    utils::linear_to_gamma,
};

pub struct RayTracer {
    pub scene: Scene,
    pub resolution: Resolution,
    pub antialiasing: AntiAliasing,
    // TODO: from config
    pub max_reflections: u32,
}

unsafe impl Sync for RayTracer {}

unsafe impl Send for RayTracer {}

impl Renderer for RayTracer {
    fn render(&self) -> ImageBuffer<Rgb<f32>, Vec<f32>> {
        let mut image = ImageBuffer::new(self.resolution.width, self.resolution.height);

        let bar = indicatif::ProgressBar::new(self.resolution.height as _);
        image.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            let mut color = Rgb([0., 0., 0.]);

            for offset in self.antialiasing.offsets.iter() {
                let coord = self.map_pixel_coords(x, y, offset);
                let ray = self.scene.camera.create_ray(coord);
                color.apply2(&self.ray_color(&ray, 0), |x, y| x + y);
            }
            *pixel = linear_to_gamma(color.map(|x| x / self.antialiasing.offsets.len() as f32));

            if x == 0 {
                bar.inc(1);
            }
        });
        bar.finish();
        println!("Render time: {:?}", bar.elapsed());

        image
    }
}

impl RayTracer {
    fn map_pixel_coords(&self, x: u32, y: u32, offset: &PixelCoord) -> PixelCoord {
        [
            (x as f32 + offset[0]) / (self.resolution.width as f32 / 2.0) - 1.0,
            (y as f32 + offset[1]) / (self.resolution.height as f32 / 2.0) - 1.0,
        ]
    }

    fn ray_color(&self, ray: &Ray, reflection_depth: u32) -> Rgb<f32> {
        if reflection_depth == self.max_reflections {
            return Rgb([0., 0., 0.]);
        }

        let closest_hit = self.scene.cast_ray(ray);
        if let Some(intersection) = closest_hit {
            // TODO: Note the third option: we could scatter with some fixed probability p and have attenuation be albedo

            let emitted = intersection.object.material.emitted();
            let scatter_direction = intersection.object.material.scattered(ray, &intersection);
            return if let Some(scatter) = scatter_direction {
                let color = self.ray_color(&scatter.ray, reflection_depth + 1);
                scatter.attenuation.map2(&color, |x, y| x * y)
            } else if let Some(emitted) = emitted {
                emitted
            } else {
                Rgb([0., 0., 0.])
            };
        }
        // let a = 0.5 * (ray.dir.vec.y + 1.0);
        // utils::lerp(a)
        // Rgb([0., 0., 0.])
        Rgb([0.1, 0.1, 0.1])
    }
}
