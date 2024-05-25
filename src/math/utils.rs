use std::{fmt::Debug, ops::Deref};

use rand::{
    distributions::{uniform::SampleUniform, Standard},
    prelude::Distribution,
    random, Rng,
};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::{
    core::Ray,
    math::{dot, unit::Unit, Dot, Normal3, Normed, Number, Vec3},
    Vec3f,
};

macro_rules! gen_axis_enums {
    ($name:ident { $( $axis:ident ),* }) => {
        #[derive(Copy, Clone, Debug)]
        #[derive(EnumIter)]
        pub enum $name {
            $( $axis ),*
        }
    }
}

gen_axis_enums!(Axis2 { X, Y });
gen_axis_enums!(Axis3 { X, Y, Z });
gen_axis_enums!(Axis4 { X, Y, Z, W });

impl Distribution<Axis2> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Axis2 {
        match random::<u32>() % 2 {
            0 => Axis2::X,
            1 => Axis2::Y,
            _ => unreachable!(),
        }
    }
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
