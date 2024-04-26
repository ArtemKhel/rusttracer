use geometry::Object;
use image::{ImageBuffer, Rgb};

use geometry::point::Point;
use geometry::ray::Ray;
use geometry::sphere::Sphere;
use geometry::vec::Vec3;

fn lerp(/*a: Rgb<u8>, b: Rgb<u8>,*/ t: f32) -> Rgb<u8> {
    let a = Rgb([0, 0, 0]);
    let b = Rgb([255, 255, 255]);
    Rgb([
        ((1. - t) * a.0[0] as f32 + t * b.0[0] as f32) as u8,
        ((1. - t) * a.0[1] as f32 + t * b.0[1] as f32) as u8,
        ((1. - t) * a.0[2] as f32 + t * b.0[2] as f32) as u8,
    ])
}

// TODO: dyn
fn ray_color(ray: &Ray, world: &Vec<Box<dyn Object>>) -> Rgb<u8> {
    if let (Some(hit), _) = world.iter().fold((None, f32::INFINITY), |closest, obj| {
        if let Some(hit) = obj.hit(ray) {
            let dist = ray.origin.distance_to(hit.point);
            if dist < closest.1 {
                return (Some(hit), dist);
            }
        }
        closest
    }) {
        let normal = hit.normal;
        return Rgb([
            (255. * 0.5 * (normal.vec.x + 1.)) as u8,
            (255. * 0.5 * (normal.vec.y + 1.)) as u8,
            (255. * 0.5 * (normal.vec.z + 1.)) as u8,
        ]);
    }
    let a = 0.5 * (ray.dir.vec.y + 1.0);
    lerp(a)
}

fn main() {
    let aspect_ratio = 16. / 9.;
    let width = 500;
    let height = (width as f32 / aspect_ratio) as u32;

    // Camera
    let focal_length = 1.0;
    let viewport_height = 2.;
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

    let world: Vec<Box<dyn Object>> = vec![
        Box::new(Sphere::new(Point::new(2., 0., -3.), 1.0)),
        Box::new(Sphere::new(Point::new(-2., 0., -3.), 2.0)),
        Box::new(Sphere::new(Point::new(0., -101., 0.), 100.0)),
    ];

    let bar = indicatif::ProgressBar::new(width as u64);
    for (x, y, pixel) in image.enumerate_pixels_mut() {
        let pixel_center = pixel00_loc + (pixel_delta_u * x as f32) + (pixel_delta_v * y as f32);
        let ray_direction = camera_center.unit_vector_to(pixel_center);
        let ray = Ray::new(camera_center, ray_direction);
        *pixel = ray_color(&ray, &world);

        if width == 0 {
            bar.inc(1);
        }
    }
    bar.finish();
    println!("{:?}", bar.elapsed());

    image.save("./images/image.png").expect("TODO: panic message");
}
