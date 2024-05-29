use std::sync::Arc;

use image::{Pixel, Rgb};

use crate::{
    bxdf::{DiffuseBxDF, BSDF},
    core::SurfaceInteraction,
    material::Material,
    textures::Texture,
};

#[derive(Debug)]
pub struct Matte<T> {
    pub reflectance: Arc<dyn Texture<T>>,
}

// TODO: rgb for now, will need refactoring for spectrum
impl Matte<Rgb<f32>> {
    fn get_bxdf(&self, surf_int: &SurfaceInteraction) -> <Matte<Rgb<f32>> as Material>::BxDF {
        let reflectance = self.reflectance.evaluate(surf_int).map(|x| x.clamp(0., 1.));
        DiffuseBxDF::new(reflectance)
    }
}

impl Material for Matte<Rgb<f32>> {
    type BxDF = DiffuseBxDF;

    fn get_bsdf(&self, surf_int: &SurfaceInteraction) -> BSDF {
        let bxdf = self.get_bxdf(surf_int);
        let bsdf = BSDF::new(**surf_int.interaction.normal, surf_int.dp_du, Box::new(bxdf));
        bsdf
    }
}
