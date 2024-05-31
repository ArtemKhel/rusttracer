use std::f32::consts::PI;
use std::ops::Mul;
use image::{ImageBuffer, Pixel, Rgb};
use rand::{random, Rng};
use crate::core::Ray;
use crate::integrators::Integrator;
use crate::{breakpoint, colors, point2, Point2f, ray};
use crate::math::dot;
use crate::samplers::utils::sample_uniform_sphere;
use crate::scene::cameras::{Camera, CameraSample};
use crate::scene::Scene;
use crate::utils::linear_to_gamma;

pub struct RandomWalkIntegrator {
    pub scene: Scene,
    pub max_depth: u32,
}

impl RandomWalkIntegrator {
    fn random_walk(&self, ray: &Ray, depth: u32) -> Rgb<f32>{
        // TODO: emitted
        let closest_hit = self.scene.cast_ray(ray);
        if let Some(mut interaction) = closest_hit {
            if let Some(bsdf) = interaction.get_bsdf(ray, &self.scene.camera, 1){
                // TODO: sample here
                let incoming = sample_uniform_sphere(point2!(random::<f32>(), random::<f32>()));

                if depth > self.max_depth {
                    return colors::BLACK
                }

                let cos = dot(&incoming, &interaction.hit.normal).abs();
                let res_cos = bsdf.eval(*interaction.hit.outgoing, *incoming).map(|x| x* cos);
                if res_cos == colors::BLACK{
                    return res_cos
                }
                // TODO: SI.spawn_ray
                let incoming_ray = ray!(interaction.hit.point + **interaction.hit.normal * 1e-2, incoming);
                let incoming_radiance = self.random_walk(&incoming_ray, depth+1);

                res_cos.map2(&incoming_radiance, |x,y| {
                    x * y / (1. / (4. * PI))
                })
            }else {
                colors::BLACK
            }
        }else{
            self.scene.background_color
        }
    }
    fn ray_color(&self, ray: &Ray) -> Rgb<f32> {
        self.random_walk(ray, 0)
    }
}

unsafe impl Sync for RandomWalkIntegrator {}

unsafe impl Send for RandomWalkIntegrator {}

impl Integrator for RandomWalkIntegrator {
    fn render(&self) -> ImageBuffer<Rgb<f32>, Vec<f32>> {
        let resolution = self.scene.camera.get_film().resolution;
        let mut image = ImageBuffer::new(resolution.x, resolution.y);
        let mut rng = rand::thread_rng();

        image.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            // image.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            let mut color = Rgb([0., 0., 0.]);
            let p_film = point2!(x as f32, y as f32);
            let sample = CameraSample {
                p_film,
                p_lens: point2!(rng.gen::<f32>(), rng.gen::<f32>()),
            };

            breakpoint!(x==150 && y==150);

            let ray = self.scene.camera.generate_ray(sample);

            *pixel = linear_to_gamma(self.ray_color(&ray));
        });

        image
    }
}
