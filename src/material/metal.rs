use std::sync::Arc;

use image::Rgb;

use crate::{
    bxdf::{conductor::ConductorBxDF, BxDFEnum, DiffuseBxDF, BSDF},
    core::SurfaceInteraction,
    material::Material,
    textures::Texture,
};

#[derive(Debug)]
pub struct Metal<T> {
    pub reflectance: Arc<dyn Texture<T>>,
    pub eta: Arc<dyn Texture<T>>,
    pub k: Arc<dyn Texture<T>>,
}

impl Material for Metal<Rgb<f32>> {
    type BxDF = BxDFEnum;

    fn get_bxdf(&self, surf_int: &SurfaceInteraction) -> <Metal<Rgb<f32>> as Material>::BxDF {
        // let reflectance = ...
        let eta = self.eta.evaluate(surf_int);
        let k = self.k.evaluate(surf_int);
        BxDFEnum::Conductor(ConductorBxDF::new(eta, k))
    }

    fn get_bsdf(&self, surf_int: &SurfaceInteraction) -> BSDF {
        let bxdf = self.get_bxdf(surf_int);
        // todo: box
        BSDF::new(**surf_int.hit.normal, surf_int.dp_du, Box::new(bxdf))
    }
}
