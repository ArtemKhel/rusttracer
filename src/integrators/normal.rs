use image::{ImageBuffer, Rgb};
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

pub struct NormalIntegrator {
    // camera: SimpleCamera,
    // primitives: PrimitiveEnum,
    pub scene: Scene,
}

const W: u32 = 300;
const H: u32 = 300;

impl NormalIntegrator {
    fn map_pixel_coords(&self, x: u32, y: u32) -> PixelCoord {
        [(x as f32) / (W as f32 / 2.0) - 1.0, (y as f32) / (H as f32 / 2.0) - 1.0]
    }

    fn map_pixel_coords_2(&self, x: u32, y: u32) -> Point2f {
        point2!((x as f32) / (W as f32), (y as f32) / (H as f32))
    }

    fn ray_color(&self, ray: &Ray) -> Rgb<f32> {
        let closest_hit = self.scene.cast_ray(ray);
        if let Some(intersection) = closest_hit {
            return Rgb([
                intersection.interaction.normal.x,
                intersection.interaction.normal.y,
                intersection.interaction.normal.z,
            ]);
        }
        self.scene.background_color
    }
}

unsafe impl Sync for NormalIntegrator {}

unsafe impl Send for NormalIntegrator {}

impl Integrator for NormalIntegrator {
    fn render(&self) -> ImageBuffer<Rgb<f32>, Vec<f32>> {
        let mut image = ImageBuffer::new(W, H);
        let mut rng = rand::thread_rng();

        image.enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            // breakpoint!(x==150 && y==150);
            // image.par_enumerate_pixels_mut().for_each(|(x, y, pixel)| {
            let mut color = Rgb([0., 0., 0.]);
            // TODO: why x inverted?
            let p_film = point2!(300. - x as f32, y as f32);
            let sample = CameraSample {
                p_film,
                p_lens: point2!(rng.gen::<f32>(), rng.gen::<f32>()),
            };
            let ray = self.scene.camera.generate_ray(sample);

            *pixel = linear_to_gamma(self.ray_color(&ray));
        });

        image
    }
}
