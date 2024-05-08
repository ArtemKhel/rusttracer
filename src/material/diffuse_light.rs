use image::Rgb;
use crate::geometry::Ray;
use crate::material::{Material, Scatter};
use crate::scene::Intersection;

#[derive(Debug, Clone, Copy)]
pub struct DiffuseLight {
    pub color: Rgb<f32>
}

impl Material for DiffuseLight{
    fn emitted(&self) -> Option<Rgb<f32>> {
        // if intersection.hit.on_front_side(ray){
        //     None
        // } else {
            Some(self.color)
        // }
    }
}
