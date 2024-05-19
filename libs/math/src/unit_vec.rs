use std::{
    ops::{Deref, Neg},
    panic,
};

use num_traits::{real::Real, Pow};

use crate::{dot, Dot, Number, Ray, Vec3};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct UnitVec3<T: Number> {
    vec: Vec3<T>,
}

pub type UnitVec3f = UnitVec3<f32>;

#[macro_export]
macro_rules! unit3 {
    ($x:expr, $y:expr, $z:expr) => {
        Vec3 { x: $x, y: $y, z: $z }.to_unit()
    };
}
#[macro_export]
macro_rules! unit3_unchecked {
    ($x:expr, $y:expr, $z:expr) => {
        UnitVec3::new_unchecked(vec3!($x, $y, $z))
    };
}

impl<T: Number> UnitVec3<T> {
    pub fn new_unchecked(vec: Vec3<T>) -> Self { Self { vec } }
}

impl<T: Number> Deref for UnitVec3<T> {
    type Target = Vec3<T>;

    fn deref(&self) -> &Self::Target { &self.vec }
}

pub fn reflect<T: Number>(vec: &Vec3<T>, normal: &Vec3<T>) -> UnitVec3<T> {
    (*vec - (*normal * vec.dot(normal) * (T::one() + T::one()))).into()
}

pub fn refract<T: Number>(ray: &Vec3<T>, normal: &UnitVec3<T>, refraction_coef_ratio: T) -> UnitVec3<T> {
    let mut cos_theta = ray.dot(normal);
    //TODO:
    let sign = cos_theta.signum();
    // cos_theta *= sign;
    let perpend = (*ray + **normal * cos_theta * sign) * refraction_coef_ratio;
    let parallel = **normal * -T::pow(T::abs(T::one() - perpend.len_squared()), 0.5);
    (perpend + parallel).to_unit()
}

pub fn local_normal<T: Number>(normal: UnitVec3<T>, ray: &Ray<T>) -> UnitVec3<T> {
    if dot(normal.deref(), &ray.dir) < T::zero() {
        normal
    } else {
        -normal
    }
}

impl<T: Number> Neg for UnitVec3<T> {
    type Output = Self;

    fn neg(self) -> Self::Output { UnitVec3 { vec: -self.vec } }
}
impl<T: Number> From<Vec3<T>> for UnitVec3<T> {
    fn from(value: Vec3<T>) -> Self {
        UnitVec3 {
            vec: value / value.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use num_traits::Inv;

    use super::*;
    use crate::{cross, dot, utils::Axis, vec3, Cross, Vec3f};

    #[test]
    fn test_macro() {
        let v = unit3!(5., 10., 0.);
        let len = v.len();
        let delta = 1e-5;
        assert!(-delta < len && len < 1. + delta)
    }

    #[test]
    fn test_dot_cross() {
        let u = unit3!(10., 0., 0.);
        let v = unit3!(0., 20., 0.);
        let r = cross(*u, *v);
        let r2 = dot(u.deref(), u.deref());

        assert_eq!(r, Vec3f::new(0., 0., 1.));
        assert_eq!(r2, 1.0);
    }

    #[test]
    fn test_reflect() {
        // TODO: almost eq
        let u = unit3!(-10., 0., 0.);
        let v = unit3!(1., 0., 0.);
        let r = reflect(&u, &v);

        assert_eq!(r, unit3!(1., 0., 0.))
    }
}
