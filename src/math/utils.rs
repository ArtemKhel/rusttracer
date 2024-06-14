use std::{
    fmt::Debug,
    ops::{Add, Deref, Mul, Sub},
};

use num_traits::{real::Real, One, Signed, ToPrimitive, Zero};
use rand::distributions::uniform::SampleUniform;

use crate::{
    core::Ray,
    math::{dot, unit::Unit, Dot, Normed, Number},
    Normal3f, Vec3f,
};

pub fn lerp<T>(a: T, b: T, t: T) -> T
where T: Copy + One + Add<Output = T> + Sub<Output = T> + Mul<Output = T> {
    (T::one() - t) * a + t * b
}

pub fn power_heuristic(n_f: usize, f_pdf: f32, n_g: usize, g_pdf: f32) -> f32 {
    let f = n_f as f32 * f_pdf;
    let g = n_g as f32 * g_pdf;
    f.powi(2) / (f.powi(2) + g.powi(2))
}

/// Computes refracted vector for a given both outward-facing vector and normal and a refractive index ratio
///
/// Returns `None` in case of total internal reflection, otherwise a pair of refracted vector and a relative refractive
/// index ratio
pub fn refract(incoming: Unit<Vec3f>, mut normal: Unit<Normal3f>, mut eta: f32) -> Option<(Vec3f, f32)> {
    let mut cos_theta_in = dot(&normal, &incoming);

    // Flip in case of leaving the object
    if cos_theta_in.is_negative() {
        eta = eta.recip();
        cos_theta_in = -cos_theta_in;
        normal = -normal;
    }

    // Compute cos of transmitted ray
    let sin_2_theta_in = 1. - cos_theta_in.powi(2);
    let sin_2_theta_tr = sin_2_theta_in / eta.powi(2);
    if sin_2_theta_tr >= 1. {
        return None;
    }
    let cos_theta_tr = (1. - sin_2_theta_tr).sqrt();

    let transmitted = -incoming / eta + **normal * (cos_theta_in / eta - cos_theta_tr);
    Some((transmitted, eta))
}

pub fn local_normal(normal: Vec3f, ray: &Ray) -> Vec3f {
    if dot(&normal, &ray.dir) < 0.0 {
        normal
    } else {
        -normal
    }
}

pub mod spherical_coordinates {
    use std::f32::consts::{FRAC_1_PI, PI};

    use crate::{math::Vec3, vec3, Vec3f};

    pub fn spherical_theta(vec: Vec3<f32>) -> f32 { vec.z.acos() * FRAC_1_PI }

    pub fn spherical_phi(vec: Vec3<f32>) -> f32 {
        let p = f32::atan2(vec.y, vec.x);
        if p < 0. {
            p + 2. * PI
        } else {
            p
        }
    }

    pub fn spherical_direction(sin_theta: f32, cos_theta: f32, phi: f32) -> Vec3f {
        // TODO: is clamp needed?
        vec3!(
            sin_theta.clamp(-1., 1.) * phi.cos(),
            sin_theta.clamp(-1., 1.) * phi.sin(),
            cos_theta.clamp(-1., 1.)
        )
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::{FRAC_1_SQRT_2, FRAC_1_SQRT_3, SQRT_2, SQRT_3};

    use approx::assert_abs_diff_eq;

    use super::*;
    use crate::vec3;

    #[test]
    fn test_refraction_eta_eq_one() {
        let incoming = vec3!(1., 1., 0.).to_unit();
        let normal = vec3!(0., 1., 0.).to_normal().to_unit();
        let eta = 1.;

        let expected_vec = vec3!(-FRAC_1_SQRT_2, -FRAC_1_SQRT_2, 0.);
        let expected_eta = 1.;

        let actual = refract(incoming, normal, eta);
        assert!(actual.is_some());

        let (actual_vec, actual_eta) = actual.unwrap();
        assert_abs_diff_eq!(expected_vec, actual_vec);
        assert_abs_diff_eq!(expected_eta, actual_eta);
    }

    #[test]
    fn test_refraction_eta_gt_one() {
        let incoming = vec3!(1., 1., 0.).to_unit();
        let normal = vec3!(0., 1., 0.).to_normal().to_unit();
        let eta = SQRT_2;

        let expected_vec = vec3!(-0.5, -SQRT_3 / 2., 0.);
        let expected_eta = SQRT_2;

        let actual = refract(incoming, normal, eta);
        assert!(actual.is_some());

        let (actual_vec, actual_eta) = actual.unwrap();
        assert_abs_diff_eq!(expected_vec, actual_vec);
        assert_abs_diff_eq!(expected_eta, actual_eta);
    }

    #[test]
    fn test_refraction_eta_lt_one() {
        let incoming = vec3!(0.5, SQRT_3 / 2., 0.).to_unit();
        let normal = vec3!(0., 1., 0.).to_normal().to_unit();
        let eta = FRAC_1_SQRT_2;

        let expected_vec = vec3!(-FRAC_1_SQRT_2, -FRAC_1_SQRT_2, 0.);
        let expected_eta = FRAC_1_SQRT_2;

        let actual = refract(incoming, normal, eta);
        assert!(actual.is_some());

        let (actual_vec, actual_eta) = actual.unwrap();
        assert_abs_diff_eq!(expected_vec, actual_vec);
        assert_abs_diff_eq!(expected_eta, actual_eta);
    }

    #[test]
    fn test_refraction_total_internal_reflection() {
        let incoming = vec3!(1., 1., 0.).to_unit();
        let normal = vec3!(0., 1., 0.).to_normal().to_unit();
        let eta = FRAC_1_SQRT_2;

        let expected_vec = vec3!(-FRAC_1_SQRT_2, -FRAC_1_SQRT_2, 0.);
        let expected_eta = FRAC_1_SQRT_2;

        let actual = refract(incoming, normal, eta);
        assert!(actual.is_none());
    }
}
