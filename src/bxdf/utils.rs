use num_traits::{FloatConst, One, Zero};

use crate::{bxdf::bxdf::Shading, vec3, Point2f, Vec3f, F};

pub(crate) fn same_hemisphere(a: Shading<Vec3f>, b: Shading<Vec3f>) -> bool { a.z * a.z > F::zero() }

pub(crate) fn sample_uniform_disk_concentric(point: Point2f) -> Point2f { todo!() }

pub(crate) fn sample_cosine_hemisphere(point: Point2f) -> Shading<Vec3f> {
    let d = sample_uniform_disk_concentric(point);
    let z = (F::one() - (d.x.powi(2)) - (d.y.powi(2))).sqrt();
    Shading::from(vec3!(d.x, d.y, z))
}

pub(crate) fn cosine_hemisphere_pdf(cos_theta: F) -> F { cos_theta * F::FRAC_1_PI() }

pub(crate) fn cos_theta(vec: Shading<Vec3f>) -> F { vec.z }
pub(crate) fn abs_cos_theta(vec: Shading<Vec3f>) -> F { F::abs(cos_theta(vec)) }
