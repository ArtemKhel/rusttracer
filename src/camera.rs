use image::{ImageBuffer, Pixel, Rgb};
use geometry::Object;
use geometry::point::Point;
use geometry::ray::Ray;
use geometry::sphere::Sphere;
use geometry::vec::Vec3;


pub struct CameraConfig{
    // aspect_ratio: f32,
    width: u32,
    height: u32,
    focal_length:f32,
}
impl Default for CameraConfig{
    fn default() -> Self {
        let aspect_ratio= 16./9.;
        let width=500;
        CameraConfig{
            // aspect_ratio,
            width,
            height: (width as f32 / aspect_ratio) as u32,
            focal_length:1.,
        }
    }
}
pub struct Camera<'a>{
    position:Point,
    look_at: Point,
    world: &'a Vec<Box<dyn Object>>,
    config: CameraConfig,
}

impl<'a> Camera<'a>{
    pub fn new(position: Point, look_at: Point, objects: &'a Vec<Box<dyn Object>>) -> Camera {
        Camera{position, look_at, world: objects, config:CameraConfig::default()}
    }

    pub fn render(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let viewport_height = 2.;
        let viewport_width = viewport_height * (self.config.width as f32 / self.config.height as f32);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0., 0.);
        let viewport_v = Vec3::new(0., -viewport_height, 0.);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / self.config.width as f32;
        let pixel_delta_v = viewport_v / self.config.height as f32;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left = self.position - Vec3::new(0., 0., self.config.focal_length) - viewport_u / 2. - viewport_v / 2.;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        let mut image = ImageBuffer::new(self.config.width, self.config.height);

        // let bar = indicatif::ProgressBar::new(self.config.width as u64);
        let aliasing_offsets = [
            0.25 * (pixel_delta_u + pixel_delta_v),
            0.25 * (pixel_delta_u - pixel_delta_v),
            0.25 * (-pixel_delta_u + pixel_delta_v),
            0.25 * (-pixel_delta_u - pixel_delta_v),
        ];
        for (x, y, pixel) in image.enumerate_pixels_mut() {
            let mut color:[u16;3] = [0,0,0];
            for offset in aliasing_offsets{
                let pixel_center = pixel00_loc + (pixel_delta_u * x as f32) + (pixel_delta_v * y as f32) + offset;
                let ray_direction = self.position.unit_vector_to(pixel_center);
                let ray = Ray::new(self.position, ray_direction);
                color.iter_mut().zip(self.ray_color(&ray).0).for_each(|(c,x)| {*c += x as u16;});
            }
            // *pixel = self.ray_color(&ray);
            *pixel = Rgb(color.map(|x| (x/aliasing_offsets.len() as u16) as u8))
            // if self.config.width == 0 { bar.inc(1); }
        }
        // bar.finish();
        // println!("{:?}", bar.elapsed());

        // image.save("./images/image.png").expect("");
        image
    }

    // TODO: dyn
    fn ray_color(&self, ray: &Ray) -> Rgb<u8> {
        if let (Some(hit), _) = self.world.iter().fold((None, f32::INFINITY), |closest, obj| {
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
}

fn lerp(/*a: Rgb<u8>, b: Rgb<u8>,*/ t: f32) -> Rgb<u8> {
    let a = Rgb([0, 0, 0]);
    let b = Rgb([255, 255, 255]);
    Rgb([
        ((1. - t) * a.0[0] as f32 + t * b.0[0] as f32) as u8,
        ((1. - t) * a.0[1] as f32 + t * b.0[1] as f32) as u8,
        ((1. - t) * a.0[2] as f32 + t * b.0[2] as f32) as u8,
    ])
}
