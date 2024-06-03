use std::{cmp::Ordering, sync::Arc};

use crate::{
    bxdf::BSDF,
    core::{interaction::Interaction, Ray},
    material::{Material, MaterialsEnum},
    math::{dot, Dot, Transform, Transformable},
    samplers::SamplerType,
    scene::cameras::Camera,
    Normal3f, Vec3f,
};

#[derive(Debug)]
pub struct SurfaceInteraction {
    // TODO: wrap some of them into structs?
    pub hit: Interaction,
    // pub uv: Point2f,
    pub dp_du: Vec3f,
    pub dp_dv: Vec3f,

    pub dn_du: Normal3f,
    pub dn_dv: Normal3f,

    pub dp_dx: Vec3f,
    pub dp_dy: Vec3f,

    pub du_dx: f32,
    pub dv_dx: f32,
    pub du_dy: f32,
    pub dv_dy: f32,
    pub material: Option<Arc<MaterialsEnum>>,
}

impl SurfaceInteraction {
    pub fn new(
        interaction: Interaction,
        // uv: Point2f,
        dp_du: Vec3f,
        dp_dv: Vec3f,
        dn_du: Normal3f,
        dn_dv: Normal3f,
    ) -> Self {
        SurfaceInteraction {
            hit: interaction,
            // uv,
            dp_du,
            dp_dv,
            //
            dn_du,
            dn_dv,
            //
            dp_dx: Vec3f::default(),
            dp_dy: Vec3f::default(),
            //
            du_dx: 0.,
            dv_dx: 0.,
            du_dy: 0.,
            dv_dy: 0.,
            material: None,
        }
    }

    pub fn get_bsdf(&mut self, ray: &Ray, camera: &dyn Camera, sampler: &mut SamplerType) -> Option<BSDF> {
        // TODO: spp, dyn
        // FIXME: not needed for now
        // self.calculate_differentials(ray, camera, 1)
        if let Some(material) = self.material.as_ref().map(|arc| arc.as_ref()) {
            let bsdf: BSDF = material.get_bsdf(self);
            Some(bsdf)
        } else {
            None
        }
    }

    pub fn set_material_properties(&mut self, material: Arc<MaterialsEnum>) { self.material = Some(material.clone()) }

    pub fn calculate_differentials(&mut self, ray: &Ray, camera: &dyn Camera, samples_per_pixel: u32) {
        if let Some(diff) = ray.diff {
            if dot(&self.hit.normal, &diff.rx_direction) != 0.0 && dot(&self.hit.normal, &diff.ry_direction) != 0.0 {
                // Estimate screen-space change in intersection point using ray differentials

                // Compute auxiliary intersection points with plane
                let d = -dot(&self.hit.normal, &self.hit.point);
                let tx = (-dot(&self.hit.normal, &diff.rx_origin) - d) / dot(&self.hit.normal, &diff.rx_direction);
                let px = diff.rx_origin + tx * *diff.rx_direction;
                let ty = (-dot(&self.hit.normal, &diff.ry_origin) - d) / dot(&self.hit.normal, &diff.ry_direction);
                let py = diff.ry_origin + ty * *diff.ry_direction;

                self.dp_dx = px - self.hit.point;
                self.dp_dy = py - self.hit.point;
            }
        } else {
            let approx = camera.approximate_dp_dxy(self.hit.point, *self.hit.normal, samples_per_pixel);
            self.dp_dx = approx.0;
            self.dp_dy = approx.1;
        }

        // Copy - paste from the book.
        // TODO: actually understand this
        // Estimate screen-space change in u, v
        // Compute A^T*A and its determinant
        let ata00 = dot(&self.dp_du, &self.dp_du);
        let ata01 = dot(&self.dp_du, &self.dp_dv);
        let ata11 = dot(&self.dp_dv, &self.dp_dv);
        let mut inv_det = 1. / (ata00 * ata11 - ata01 * ata01);
        if !inv_det.is_infinite() {
            inv_det = 0.0
        };

        // Compute A^T*b for x and y
        let atb0x = dot(&self.dp_du, &self.dp_dx);
        let atb1x = dot(&self.dp_dv, &self.dp_dx);
        let atb0y = dot(&self.dp_du, &self.dp_dy);
        let atb1y = dot(&self.dp_dv, &self.dp_dy);

        // Compute u and v derivatives with respect to x and y
        self.du_dx = (ata11 * atb0x - ata01 * atb1x) * inv_det;
        self.dv_dx = (ata00 * atb1x - ata01 * atb0x) * inv_det;
        self.du_dy = (ata11 * atb0y - ata01 * atb1y) * inv_det;
        self.dv_dy = (ata00 * atb1y - ata01 * atb0y) * inv_det;

        // Clamp derivatives of u and v to reasonable values
        self.du_dx = if self.du_dx.is_finite() {
            (self.du_dx.clamp(-1e8, 1e8))
        } else {
            0.
        };
        self.dv_dx = if self.dv_dx.is_finite() {
            (self.dv_dx.clamp(-1e8, 1e8))
        } else {
            0.
        };
        self.du_dy = if self.du_dy.is_finite() {
            (self.du_dy.clamp(-1e8, 1e8))
        } else {
            0.
        };
        self.dv_dy = if self.dv_dy.is_finite() {
            (self.dv_dy.clamp(-1e8, 1e8))
        } else {
            0.
        };
    }

    /// MaterialsEnum may return an unset BSDF to indicate an interface between
    /// two scattering media that does not itself scatter light.
    /// In this case, it is necessary to spawn a new ray in the same direction,
    /// but past the intersection on the surface.
    pub fn skip_interaction(ray: Ray, t: f32) { todo!() }
}

impl Transformable<f32> for SurfaceInteraction {
    fn transform(&self, trans: &Transform<f32>) -> Self {
        SurfaceInteraction {
            hit: self.hit.transform(trans),
            dp_du: self.dp_du.transform(trans),
            dp_dv: self.dp_dv.transform(trans),

            dn_du: self.dn_du.transform(trans),
            dn_dv: self.dn_dv.transform(trans),

            dp_dx: self.dp_dx.transform(trans),
            dp_dy: self.dp_dx.transform(trans),

            du_dx: self.du_dx,
            dv_dx: self.dv_dx,
            du_dy: self.du_dy,
            dv_dy: self.dv_dy,
            material: self.material.clone(),
        }
    }

    fn inv_transform(&self, trans: &Transform<f32>) -> Self { todo!() }
}

impl Eq for SurfaceInteraction {}

impl PartialEq<Self> for SurfaceInteraction {
    fn eq(&self, other: &Self) -> bool { self.hit.eq(&other.hit) }
}

impl PartialOrd<Self> for SurfaceInteraction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.hit.partial_cmp(&other.hit) }
}

impl Ord for SurfaceInteraction {
    fn cmp(&self, other: &Self) -> Ordering { self.hit.cmp(&other.hit) }
}
