use std::sync::Arc;

use image::{Pixel, Rgb};

use crate::{
    bxdf::{BxDFEnum, DiffuseBxDF, BSDF},
    core::SurfaceInteraction,
    material::Material,
    textures::Texture,
};

#[derive(Debug)]
pub struct Matte<T> {
    pub reflectance: Arc<dyn Texture<T>>,
}

// TODO: rgb for now, will need refactoring for spectrum
impl Material for Matte<Rgb<f32>> {
    type BxDF = BxDFEnum;

    fn get_bxdf(&self, surf_int: &SurfaceInteraction) -> <Matte<Rgb<f32>> as Material>::BxDF {
        let reflectance = self.reflectance.evaluate(surf_int).map(|x| x.clamp(0., 1.));
        BxDFEnum::Diffuse(DiffuseBxDF::new(reflectance))
    }

    fn get_bsdf(&self, surf_int: &SurfaceInteraction) -> BSDF {
        let bxdf = self.get_bxdf(surf_int);
        // todo: box
        BSDF::new(**surf_int.hit.normal, surf_int.dp_du, Box::new(bxdf))
    }
}
