use std::f32::consts::{FRAC_2_PI, FRAC_PI_4, PI};

use num_traits::Zero;

use crate::{math::utils::spherical_coordinates::spherical_direction, point2, vec2, vec3, Point2f, Vec3f, unit3_unchecked};
use crate::math::{Unit, Vec3};

pub fn sample_uniform_sphere(u: Point2f) -> Unit<Vec3<f32>> {
    let z = 1. - 2. * u.x;
    let r = (1. - z.powi(2)).sqrt();
    let phi = 2. * PI * u.y;
    unit3_unchecked!(r * phi.cos(), r * phi.sin(), z)
}

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
