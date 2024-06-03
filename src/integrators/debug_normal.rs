use image::{buffer::ConvertBuffer, ImageBuffer, Rgb, RgbImage};
use rand::Rng;

use crate::{
    breakpoint,
    core::Ray,
    integrators::Integrator,
    point2,
    scene::{
        cameras::{Camera, CameraSample, PixelCoord},
        Scene,
    },
    utils::linear_to_gamma,
    Point2f,
};

pub struct DebugNormalIntegrator {
    pub scene: Scene,
}

impl DebugNormalIntegrator {
    fn ray_color(&self, ray: &Ray) -> Rgb<f32> {
        let closest_hit = self.scene.cast_ray(ray);
        if let Some(interaction) = closest_hit {
            return Rgb([
                interaction.hit.normal.x,
                interaction.hit.normal.y,
                interaction.hit.normal.z,
            ]);
        }
        self.scene.background_color
    }
}

unsafe impl Sync for DebugNormalIntegrator {}

unsafe impl Send for DebugNormalIntegrator {}

impl Integrator for DebugNormalIntegrator {
    fn render(&self) {
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
            // breakpoint!(x == 100 && y == 100);
            let ray = self.scene.camera.generate_ray(sample);

            if x == 150 && y == 150 {
                dbg!(&ray);
            }

            *pixel = linear_to_gamma(self.ray_color(&ray));
        });

        let image: RgbImage = image.convert();
        image.save("./images/_image.png").unwrap();
    }
}
