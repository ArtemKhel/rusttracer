use std::f32::consts::{FRAC_2_PI, FRAC_PI_4, PI};

use num_traits::Zero;

use crate::{
    math::{utils::spherical_coordinates::spherical_direction, Unit, Vec3},
    point2,
    spectra::{VISIBLE_MAX, VISIBLE_MIN},
    unit3_unchecked, vec2, Point2f, Vec3f,
};

// TODO: should check all math-y things and do them properly. Finding NaNs in random places isn't funny

/// Generate vectors to a uniformly distributed points on a unit sphere
pub fn sample_uniform_sphere(u: Point2f) -> Unit<Vec3<f32>> {
    // This method from PBRT generates uniformly distributed **spherical coordinates**.
    // When mapped to actual sphere, they are biased towards Z poles.
    // TODO: check other samplers

    // let z = (1. - 2. * u.x).clamp(-1., 1.);
    // let r = (1. - z.powi(2)).sqrt();
    // let phi = 2. * PI * u.y;
    // unit3_unchecked!(r * phi.cos(), r * phi.sin(), z)

    // from https://github.com/CorySimon/CorySimon.github.io/blob/master/_posts/articles/2015-02-27-uniformdistn-on-sphere.md
    let theta = 2. * PI * u.x;
    let phi = (1. - 2. * u.y).acos();
    let x = phi.sin() * theta.cos();
    let y = phi.sin() * theta.sin();
    let z = phi.cos();
    unit3_unchecked!(x, y, z)
}

pub const fn uniform_sphere_pdf() -> f32 { 1. / (4. * PI) }

pub fn sample_uniform_hemisphere(u: Point2f) -> Unit<Vec3<f32>> {
    let theta = 2. * PI * u.x;
    let phi = u.y.acos();
    let x = phi.sin() * theta.cos();
    let y = phi.sin() * theta.sin();
    let z = phi.cos();
    unit3_unchecked!(x, y, z)
}
pub const fn uniform_hemisphere_pdf() -> f32 { 1. / (2. * PI) }

pub fn sample_uniform_cone(u: Point2f, max_cos_theta: f32) -> Vec3f {
    let cos_theta = (1. - u.x) + u.x * max_cos_theta;
    let sin_theta = (1. - cos_theta.powi(2)).sqrt();
    let phi = u.y * 2. * PI;
    spherical_direction(sin_theta, cos_theta, phi)
}

pub fn sample_uniform_disk_concentric(u: Point2f) -> Point2f {
    let u_offset = 2. * *u - vec2!(1., 1.);

    if u_offset.x.is_zero() && u_offset.y.is_zero() {
        return point2!();
    }

    let (r, theta) = if u_offset.x.abs() > u_offset.y.abs() {
        (u_offset.x, FRAC_PI_4 * (u_offset.y / u_offset.x))
    } else {
        (u_offset.y, FRAC_2_PI - FRAC_PI_4 * (u_offset.x / u_offset.y))
    };

    point2!(theta.cos(), theta.sin()) * r
}

/// Samples visible wavelength according to it visual importance
pub fn sample_visible_wavelengths(rnd_c: f32) -> f32 { 538.0 - 138.888889 * (0.85691062 - 1.82750197 * rnd_c).atanh() }

/// PDF for a chosen in [sample_visible_wavelengths] wavelength
pub fn visible_wavelengths_pdf(lambda: f32) -> f32 {
    if (VISIBLE_MIN..VISIBLE_MAX).contains(&lambda) {
        0.0039398042 / ((0.0072 * (lambda - 538.)).cosh()).powi(2)
    } else {
        0.
    }
}
