use std::{array, fmt::Debug, iter::zip};

use derive_more::{Deref, DerefMut, From};
use derive_new::new;
use gen_ops::{gen_ops, gen_ops_comm, gen_ops_ex};
use num_traits::{One, Zero};

use crate::spectra::{
    cie::{CIE, CIE_Y_INTEGRAL},
    rgb::{RGBColorSpace, RGB},
    sampled_wavelengths::SampledWavelengths,
    xyz::XYZ,
    Spectrum,
};

#[derive(Clone, Copy, Debug)]
#[derive(new)]
#[derive(From, Deref, DerefMut)]
pub struct SampledSpectrum<const N: usize> {
    // Sadly, ArrayVec in not Copy
    // pub values: ArrayVec<f32, N>,
    pub values: [f32; N],
}

impl<const N: usize> SampledSpectrum<N> {
    pub fn has_nan(&self) -> bool { self.iter().any(|x| x.is_nan()) }

    pub fn avg(&self) -> f32 { self.values.iter().sum::<f32>() / (self.values.len() as f32) }

    pub fn to_xyz(&self, lambda: &SampledWavelengths<N>) -> XYZ {
        let x = CIE::X.get().sample(lambda);
        let y = CIE::Y.get().sample(lambda);
        let z = CIE::Z.get().sample(lambda);
        let pdf = lambda.pdf();
        XYZ::new(
            (x * self / &pdf).avg(),
            (y * self / &pdf).avg(),
            (z * self / &pdf).avg(),
        ) / CIE_Y_INTEGRAL
    }

    pub fn to_rgb(&self, lambda: &SampledWavelengths<N>, color_space: &RGBColorSpace) -> RGB {
        let xyz = self.to_xyz(lambda);
        color_space.xyz_to_rgb(xyz)
    }

    pub fn clamp(mut self, min: f32, max: f32) -> Self {
        self.values.iter_mut().for_each(|x| {
            x.clamp(min, max);
        });
        self
    }
}

impl<const N: usize> From<f32> for SampledSpectrum<N> {
    fn from(value: f32) -> Self { SampledSpectrum { values: [value; N] } }
}

impl<const N: usize> Default for SampledSpectrum<N> {
    fn default() -> Self { SampledSpectrum::zero() }
}

impl<const N: usize> Zero for SampledSpectrum<N> {
    fn zero() -> Self { SampledSpectrum::from(0.) }

    fn is_zero(&self) -> bool { self.iter().all(f32::is_zero) }
}

impl<const N: usize> One for SampledSpectrum<N> {
    fn one() -> Self { SampledSpectrum::from(1.0) }
}

gen_ops_ex!(
    <|const N: usize>;
    types ref SampledSpectrum<N>, ref SampledSpectrum<N> => SampledSpectrum<N>;

    for + call |x: &SampledSpectrum<N>, y: &SampledSpectrum<N>|
        // SampledSpectrum::new(zip(x.iter(), y.iter()).map(|(x, y)| x + y).collect());
        SampledSpectrum::new(array::from_fn(|i| x[i] + y[i]));

    for - call |x: &SampledSpectrum<N>, y: &SampledSpectrum<N>|
        // SampledSpectrum::new(zip(x.iter(), y.iter()).map(|(x, y)| x - y).collect());
        SampledSpectrum::new(array::from_fn(|i| x[i] - y[i]));

    for * call |x: &SampledSpectrum<N>, y: &SampledSpectrum<N>|
        // SampledSpectrum::new(zip(x.iter(), y.iter()).map(|(x, y)| x * y).collect());
        SampledSpectrum::new(array::from_fn(|i| x[i] * y[i]));

    for / call |x: &SampledSpectrum<N>, y: &SampledSpectrum<N>|
        // SampledSpectrum::new(zip(x.iter(), y.iter()).map(|(x, y)| if *y != 0. { x / y } else { 0. }).collect());
        SampledSpectrum::new(array::from_fn(|i| if y[i] != 0.0 {x[i] / y[i]} else {0.}));
);

gen_ops_comm!(
    <|const N: usize>;
    types SampledSpectrum<N>, f32 => SampledSpectrum<N>;

    for * call |x: &SampledSpectrum<N>, y: &f32|
        // SampledSpectrum::new(x.iter().map(|x| x * y).collect());
        SampledSpectrum::new(array::from_fn(|i| x[i] * y));
);

gen_ops!(
    <|const N: usize>;
    types SampledSpectrum<N>, f32 => SampledSpectrum<N>;

    for / call |x: &SampledSpectrum<N>, y: &f32|
        // SampledSpectrum::new(x.iter().map(|x| if *y != 0. {x / y} else {0.}).collect());
        SampledSpectrum::new( if *y != 0.0 { array::from_fn(|i| { x[i] / y } )} else { [0.;N] });
);

gen_ops!(
    <|const N: usize>;
    types SampledSpectrum<N>, SampledSpectrum<N>;

    for += call |x: &mut SampledSpectrum<N>, y: &SampledSpectrum<N>| zip( x.iter_mut(), y.iter() ).for_each(|(x, y)| *x += y);
    for -= call |x: &mut SampledSpectrum<N>, y: &SampledSpectrum<N>| zip( x.iter_mut(), y.iter() ).for_each(|(x, y)| *x -= y);
    for *= call |x: &mut SampledSpectrum<N>, y: &SampledSpectrum<N>| zip( x.iter_mut(), y.iter() ).for_each(|(x, y)| *x *= y);
    for /= call |x: &mut SampledSpectrum<N>, y: &SampledSpectrum<N>| zip( x.iter_mut(), y.iter() ).for_each(|(x, y)| if *y != 0. { *x /= y } else { *x = 0. });
);

gen_ops!(
    <|const N: usize>;
    types SampledSpectrum<N>, f32;
    
    for *= call |x: &mut SampledSpectrum<N>, y: &f32| x.iter_mut().for_each(|x| *x *= y);
);