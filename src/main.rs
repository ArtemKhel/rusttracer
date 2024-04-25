use image::{ImageBuffer, Rgb};

use geometry::point::Point;
use geometry::ray::Ray;
use geometry::vec::Vec3;

fn lerp(/*a: Rgb<u8>, b: Rgb<u8>,*/ t: f32) -> Rgb<u8> {
    let a = Rgb([255,255,255]);
    let b = Rgb([128,192,255]);
    Rgb([
        ((1. - t) * a.0[0] as f32 + t * b.0[0] as f32) as u8,
        ((1. - t) * a.0[1] as f32 + t * b.0[1] as f32) as u8,
        ((1. - t) * a.0[2] as f32 + t * b.0[2] as f32) as u8,
    ])
}
fn ray_color(ray: &Ray) -> Rgb<u8> {
    let a = 0.5*(ray.dir.vec.y + 1.0);
    lerp(a)
}

fn main() {
    let aspect_ratio = 16. / 9.;
    let width = 500;
    let height = (width as f32 / aspect_ratio) as u32;

    // Camera
    let focal_length = 1.0;
    let viewport_height = 1.;
    let viewport_width = viewport_height * (width as f32 / height as f32);
    let camera_center = Point::new(0., 0., 0.);

    // Calculate the vectors across the horizontal and down the vertical viewport edges.
    let viewport_u = Vec3::new(viewport_width, 0., 0.);
    let viewport_v = Vec3::new(0., -viewport_height, 0.);

    // Calculate the horizontal and vertical delta vectors from pixel to pixel.
    let pixel_delta_u = viewport_u / width as f32;
    let pixel_delta_v = viewport_v / height as f32;

    // Calculate the location of the upper left pixel.
    let viewport_upper_left = camera_center - Vec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);


    let mut image = ImageBuffer::new(width, height);

    let bar = indicatif::ProgressBar::new(width as u64);
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let pixel_center = pixel00_loc + (pixel_delta_u * x as f32) + (pixel_delta_v * y as f32);
        let ray_direction = pixel_center.vector_to(camera_center);
        let ray = Ray::new(camera_center, ray_direction);
        *pixel = ray_color(&ray);

        if width == 0 {
            bar.inc(1);
        }
    }
    bar.finish();
    println!("{:?}", bar.elapsed());

    image.save("./images/image.png").expect("TODO: panic message");
}
