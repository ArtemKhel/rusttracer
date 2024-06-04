use std::f32::consts::{FRAC_1_PI, FRAC_2_PI, FRAC_PI_4, PI};

use num_traits::{FloatConst, Zero};

use crate::{bxdf::bxdf::Shading, point2, samplers::utils::sample_uniform_disk_concentric, vec2, vec3, Point2f, Vec3f};

// TODO: should all of that be here?

pub(super) fn sample_cosine_hemisphere(u: Point2f) -> Shading<Vec3f> {
    // TODO: same problem as sample_uniform_sphere
    let p = sample_uniform_disk_concentric(u);
    let z = (1.0 - (p.x.powi(2)) - (p.y.powi(2))).sqrt();
    Shading::from(vec3!(p.x, p.y, z))
}

pub(super) fn cosine_hemisphere_pdf(cos_theta: f32) -> f32 { cos_theta * FRAC_1_PI }

pub(super) fn same_hemisphere(a: Shading<Vec3f>, b: Shading<Vec3f>) -> bool { a.z * a.z > 0.0 }

pub(super) fn cos_theta(vec: Shading<Vec3f>) -> f32 { vec.z }

pub(super) fn abs_cos_theta(vec: Shading<Vec3f>) -> f32 { f32::abs(cos_theta(vec)) }
