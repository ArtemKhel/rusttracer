use num_traits::FloatConst;

use crate::{bxdf::bxdf::Shading, vec3, Point2f, Vec3f};

pub(crate) fn same_hemisphere(a: Shading<Vec3f>, b: Shading<Vec3f>) -> bool { a.z * a.z > 0.0 }

pub(crate) fn sample_uniform_disk_concentric(point: Point2f) -> Point2f { todo!() }

pub(crate) fn sample_cosine_hemisphere(point: Point2f) -> Shading<Vec3f> {
    let d = sample_uniform_disk_concentric(point);
    let z = (1.0 - (d.x.powi(2)) - (d.y.powi(2))).sqrt();
    Shading::from(vec3!(d.x, d.y, z))
}

pub(crate) fn cosine_hemisphere_pdf(cos_theta: f32) -> f32 { cos_theta * f32::FRAC_1_PI() }

pub(crate) fn cos_theta(vec: Shading<Vec3f>) -> f32 { vec.z }

pub(crate) fn abs_cos_theta(vec: Shading<Vec3f>) -> f32 { f32::abs(cos_theta(vec)) }
