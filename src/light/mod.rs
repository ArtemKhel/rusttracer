use image::Rgb;

pub trait Light {
    fn flux() -> Rgb<f32>;
    fn light_type(); // ?
}
