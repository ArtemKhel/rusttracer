use image::Rgb;

#[derive(Debug, Clone, Copy)]
pub struct Material {
    pub color: Rgb<f32>,
    pub metal: bool,
    pub fuzz: f32,
    // pub albedo: Rgb<f32>,
}

impl Material {
    pub fn new(color: Rgb<f32>) -> Self {
        Material { color, metal: false, fuzz: 0.}
    }
}
