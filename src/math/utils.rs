use std::{fmt::Debug, ops::Deref};

use rand::{distributions::uniform::SampleUniform, random};

use crate::{
    core::Ray,
    math::{dot, unit::Unit, Dot, Normal3, Normed, Number, Vec3},
    Vec3f,
};

pub fn random_unit<T: Number + SampleUniform>() -> Unit<Vec3<T>> {
    loop {
        let rnd: Vec3<T> = random();
        if rnd.len() <= T::one() {
            break rnd.to_unit();
        }
    }
}

pub fn random_on_hemisphere<T: Number + SampleUniform>(normal: &Unit<Vec3<T>>) -> Unit<Vec3<T>> {
    let random = random_unit();
    if normal.dot(&random) >= T::zero() {
        random
    } else {
        -random
    }
}

pub fn random_in_unit_disk<T: Number + SampleUniform>() -> Vec3<T> {
    loop {
        let rnd: Vec3<T> = random();
        if rnd.len_squared() <= T::one() {
            break rnd;
        }
    }
}

pub fn reflect<T: Number>(vec: &Vec3<T>, normal: &Vec3<T>) -> Unit<Vec3<T>> {
    (*vec - (*normal * vec.dot(normal) * (T::one() + T::one()))).into()
}

pub fn refract<T: Number>(ray: &Unit<Vec3<T>>, normal: &Unit<Normal3<T>>, refraction_coef_ratio: T) -> Unit<Vec3<T>> {
    //TODO: assuming normalized normal and ray dir
    let mut cos_theta = dot(ray.deref(), normal.deref());
    let sign = cos_theta.signum();
    // cos_theta *= sign;
    // Derefs for the deref god!
    let perpend = (**ray + ***normal * cos_theta * sign) * refraction_coef_ratio;
    let parallel = ***normal * -T::pow(T::abs(T::one() - perpend.len_squared()), 0.5);
    (perpend + parallel).to_unit()
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
