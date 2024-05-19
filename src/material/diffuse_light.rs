use image::Rgb;

use crate::{
    material::{Material, Scatter},
    scene::Intersection,
    Ray,
};

#[derive(Debug, Clone, Copy)]
pub struct DiffuseLight {
    pub color: Rgb<f32>,
}

impl Material for DiffuseLight {
    fn emitted(&self) -> Option<Rgb<f32>> { Some(self.color) }
}
