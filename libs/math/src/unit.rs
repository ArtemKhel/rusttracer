use std::{marker::PhantomData, ops::Div};

use derive_more::{Deref, DerefMut, Neg};

use crate::{vec3, Normed, Vec3};

#[derive(Debug, Default, Clone, Copy, PartialEq, Deref, DerefMut, Neg)]
pub struct Unit<Inner> {
    value: Inner,
}

impl<Inner: Normed> Unit<Inner> {
    pub fn from_unchecked(value: Inner) -> Self { Unit { value } }

    pub fn into_inner(self) -> Inner { self.value }

    pub fn lift<Outer>(self) -> Unit<Outer>
    where Outer: From<Inner> {
        Unit {
            value: self.value.into(),
        }
    }
}

#[macro_export]
macro_rules! unit3 {
    ($x:expr, $y:expr, $z:expr) => {
        Unit::from($crate::vec3!($x, $y, $z))
    };
}
#[macro_export]
macro_rules! unit3_unchecked {
    ($x:expr, $y:expr, $z:expr) => {
        Unit::from_unchecked($crate::vec3!($x, $y, $z))
    };
}

impl<Inner, T> From<Inner> for Unit<Inner>
where Inner: Normed<Output = T> + Div<T, Output = Inner> + Copy
{
    fn from(value: Inner) -> Self {
        Unit {
            value: value / value.len(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        cross, dot,
        utils::{reflect, Axis3},
        vec3, Cross, Vec3f,
    };

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
