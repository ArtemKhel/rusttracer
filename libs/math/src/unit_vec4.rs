use std::{
    ops::{Deref, Neg},
    panic,
};

use num_traits::{real::Real, Pow};

use crate::{dot, Dot, Number, Ray, Vec4};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct UnitVec4<T> {
    vec: Vec4<T>,
}

pub type UnitVec4f = UnitVec4<f32>;

#[macro_export]
macro_rules! unit4 {
    ($x:expr, $y:expr, $z:expr) => {
        Vec4 { x: $x, y: $y, z: $z }.to_unit()
    };
}
#[macro_export]
macro_rules! unit4_unchecked {
    ($x:expr, $y:expr, $z:expr) => {
        UnitVec4::new_unchecked(vec4!($x, $y, $z))
    };
}

impl<T: Number> UnitVec4<T> {
    pub fn new_unchecked(vec: Vec4<T>) -> Self { Self { vec } }
}

impl<T: Number> Deref for UnitVec4<T> {
    type Target = Vec4<T>;

    fn deref(&self) -> &Self::Target { &self.vec }
}

// pub fn reflect<T: Number>(vec: &Vec4<T>, normal: &Vec4<T>) -> UnitVec4<T> {
//     (*vec - (*normal * vec.dot(normal) * (T::one() + T::one()))).into()
// }
//
// pub fn refract<T: Number>(ray: &Vec4<T>, normal: &UnitVec4<T>, refraction_coef_ratio: T) -> UnitVec4<T> {
//     let mut cos_theta = ray.dot(normal);
//     //TODO:
//     let sign = cos_theta.signum();
//     // cos_theta *= sign;
//     let perpend = (*ray + **normal * cos_theta * sign) * refraction_coef_ratio;
//     let parallel = **normal * -T::pow(T::abs(T::one() - perpend.len_squared()), 0.5);
//     (perpend + parallel).to_unit()
// }

// pub fn local_normal<T: Number>(normal: UnitVec4<T>, ray: &Ray<T>) -> UnitVec4<T> {
//     if dot(normal.deref(), &ray.dir) < T::zero() {
//         normal
//     } else {
//         -normal
//     }
// }

impl<T: Number> Neg for UnitVec4<T> {
    type Output = Self;

    fn neg(self) -> Self::Output { UnitVec4 { vec: -self.vec } }
}
impl<T: Number> From<Vec4<T>> for UnitVec4<T> {
    fn from(value: Vec4<T>) -> Self {
        UnitVec4 {
            vec: value / value.len(),
        }
    }
}
