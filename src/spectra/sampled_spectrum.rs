use std::{
    fmt::Debug,
    iter::zip,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign},
};

use arrayvec::ArrayVec;
use derive_more::{Deref, DerefMut, From};
use derive_new::new;
use num_traits::Zero;

#[derive(Clone, Debug)]
#[derive(new)]
#[derive(From, Deref, DerefMut)]
pub struct SampledSpectrum<const N: usize> {
    pub values: ArrayVec<f32, N>,
}

impl<const N: usize> SampledSpectrum<N> {
    pub fn has_nan(&self) -> bool { self.iter().any(|x| x.is_nan()) }
}

impl<const N: usize> From<f32> for SampledSpectrum<N> {
    fn from(value: f32) -> Self {
        SampledSpectrum {
            values: ArrayVec::from([value; N]),
        }
    }
}

impl<const N: usize> Default for SampledSpectrum<N> {
    fn default() -> Self { SampledSpectrum::zero() }
}

impl<const N: usize> Zero for SampledSpectrum<N> {
    fn zero() -> Self { SampledSpectrum::from(0.) }

    fn is_zero(&self) -> bool { self.iter().all(f32::is_zero) }
}

macro_rules! gen_trait_impl {
    ($trait_:ident, $fun:ident, $rhs:ty, $op:expr) => {
        impl<const N: usize> $trait_<$rhs> for SampledSpectrum<N> {
            type Output = Self;

            fn $fun(self, rhs: $rhs) -> Self::Output {
                SampledSpectrum {
                    values: $op(self, rhs),
                }
            }
        }
    };
    ($trait_:ident, $fun:ident, $op:expr) => {
        impl<const N: usize> $trait_ for SampledSpectrum<N> {
            type Output = Self;

            fn $fun(self, rhs: Self) -> Self::Output {
                SampledSpectrum {
                    values: $op(self, rhs),
                }
            }
        }
    };
}
macro_rules! gen_trait_assign_impl {
    ($trait_:ident, $fun:ident, $rhs:ty, $op:expr) => {
        impl<const N: usize> $trait_<$rhs> for SampledSpectrum<N> {
            fn $fun(&mut self, rhs: $rhs) { $op(self, rhs) }
        }
    };
    ($trait_:ident, $fun:ident, $op:expr) => {
        impl<const N: usize> $trait_ for SampledSpectrum<N> {
            fn $fun(&mut self, rhs: Self) { $op(self, rhs) }
        }
    };
}

type SS<const N: usize> = SampledSpectrum<N>;

gen_trait_impl!(Add, add, |x: SS<N>, y: SS<N>| zip(x.iter(), y.iter())
    .map(|(x, y)| x + y)
    .collect());
gen_trait_impl!(Sub, sub, |x: SS<N>, y: SS<N>| zip(x.iter(), y.iter())
    .map(|(x, y)| x - y)
    .collect());
gen_trait_impl!(Mul, mul, |x: SS<N>, y: SS<N>| zip(x.iter(), y.iter())
    .map(|(x, y)| x * y)
    .collect());
gen_trait_impl!(Div, div, |x: SS<N>, y: SS<N>| zip(x.iter(), y.iter())
    .map(|(x, y)| { x / y })
    .collect());

gen_trait_assign_impl!(AddAssign, add_assign, |x: &mut SS<N>, y: SS<N>| zip(
    x.iter_mut(),
    y.iter()
)
.for_each(|(x, y)| *x += y));
gen_trait_assign_impl!(SubAssign, sub_assign, |x: &mut SS<N>, y: SS<N>| zip(
    x.iter_mut(),
    y.iter()
)
.for_each(|(x, y)| *x -= y));
gen_trait_assign_impl!(MulAssign, mul_assign, |x: &mut SS<N>, y: SS<N>| zip(
    x.iter_mut(),
    y.iter()
)
.for_each(|(x, y)| *x *= y));
gen_trait_assign_impl!(DivAssign, div_assign, |x: &mut SS<N>, y: SS<N>| zip(
    x.iter_mut(),
    y.iter()
)
.for_each(|(x, y)| *x /= y));

gen_trait_impl!(Mul, mul, f32, |x: SS<N>, y: f32| x.iter().map(|x| x * y).collect());
gen_trait_impl!(Div, div, f32, |x: SS<N>, y: f32| x.iter().map(|x| { x / y }).collect());
gen_trait_assign_impl!(MulAssign, mul_assign, f32, |x: &mut SS<N>, y: f32| x
    .iter_mut()
    .for_each(|x| *x *= y));
gen_trait_assign_impl!(DivAssign, div_assign, f32, |x: &mut SS<N>, y: f32| x
    .iter_mut()
    .for_each(|x| *x /= y));
