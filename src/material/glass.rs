use std::{marker::PhantomData, sync::Arc};

use image::Rgb;

use crate::{
    bxdf::{conductor::ConductorBxDF, dielectric::DielectricBxDF, BxDFEnum, DiffuseBxDF, BSDF},
    core::SurfaceInteraction,
    material::Material,
    textures::Texture,
};

#[derive(Debug)]
pub struct Glass<T> {
    // TODO: spectrum texture in pbrt
    pub ior: f32,
    // pub roughness: f32,
    pub spectrum: Arc<dyn Texture<T>>,
}

impl Material for Glass<Rgb<f32>> {
    type BxDF = BxDFEnum;

    fn get_bxdf(&self, surf_int: &SurfaceInteraction) -> Self::BxDF {
        BxDFEnum::Dielectric(DielectricBxDF::new(self.ior))
    }

    fn get_bsdf(&self, surf_int: &SurfaceInteraction) -> BSDF {
        // todo: non-constant IOR
        let bxdf = self.get_bxdf(surf_int);
        // todo: box
        BSDF::new(**surf_int.hit.normal, surf_int.dp_du, Box::new(bxdf))
    }
}
