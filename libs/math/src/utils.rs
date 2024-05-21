use std::{fmt::Debug, ops::Deref};

use rand::{distributions::Standard, prelude::Distribution, random, Rng};
use strum_macros::EnumIter;

use crate::{dot, unit::Unit, Dot, Normal3, Normed, Number, Ray, Vec3, Vec3f};

#[derive(Copy, Clone, EnumIter, Debug)]
pub enum Axis3 {
    X,
    Y,
    Z,
}

impl Distribution<Axis3> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Axis3 {
        match random::<u32>() % 3 {
            0 => Axis3::X,
            1 => Axis3::Y,
            2 => Axis3::Z,
            _ => unreachable!(),
        }
    }
}

#[derive(Copy, Clone, EnumIter, Debug)]
pub enum Axis4 {
    X,
    Y,
    Z,
    W,
}

impl Distribution<Axis4> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Axis4 {
        match random::<u32>() % 4 {
            0 => Axis4::X,
            1 => Axis4::Y,
            2 => Axis4::Z,
            3 => Axis4::W,
            _ => unreachable!(),
        }
    }
}

pub fn random_unit() -> Unit<Vec3f> {
    loop {
        let rnd: Vec3f = random();
        if rnd.len() <= 1. {
            break rnd.to_unit();
        }
    }
}

pub fn random_on_hemisphere(normal: &Unit<Vec3f>) -> Unit<Vec3f> {
    let random = random_unit();
    if normal.dot(&random) >= 0. {
        random
    } else {
        -random
    }
}

pub fn random_in_unit_disk() -> Vec3f {
    loop {
        let rnd: Vec3f = random();
        if rnd.len_squared() <= 1. {
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

pub fn local_normal<T: Number>(normal: Vec3<T>, ray: &Ray<T>) -> Vec3<T> {
    if dot(&normal, &ray.dir) < T::zero() {
        normal
    } else {
        -normal
    }
}
