use std::{fmt::Debug, iter::zip};

use arrayvec::ArrayVec;
use derive_more::{Deref, DerefMut, From};
use derive_new::new;
use gen_ops::{gen_ops_comm, gen_ops_ex};
use num_traits::Zero;

use crate::spectra::{
    cie::{CIE, CIE_Y_INTEGRAL},
    rgb::{RGBColorSpace, RGB},
    sampled_wavelengths::SampledWavelengths,
    xyz::XYZ,
    Spectrum,
};

#[derive(Clone, Debug)]
#[derive(new)]
#[derive(From, Deref, DerefMut)]
pub struct SampledSpectrum<const N: usize> {
    pub values: ArrayVec<f32, N>,
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

    pub fn to_rgb(&self, lambda: &SampledWavelengths<N>, colorspace: RGBColorSpace) -> RGB {
        let xyz = self.to_xyz(lambda);
        colorspace.xyz_to_rgb(xyz)
    }
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

type SS<const N: usize> = SampledSpectrum<N>;

gen_ops_ex!(
    <|const N: usize>;
    types ref SS<N>, ref SS<N> => SS<N>;

    for + call |x: &SS<N>, y: &SS<N>|
        SampledSpectrum::new(zip(x.iter(), y.iter()).map(|(x, y)| x + y).collect());

    for - call |x: &SS<N>, y: &SS<N>|
        SampledSpectrum::new(zip(x.iter(), y.iter()).map(|(x, y)| x - y).collect());

    for * call |x: &SS<N>, y: &SS<N>|
        SampledSpectrum::new(zip(x.iter(), y.iter()).map(|(x, y)| x * y).collect());

    for / call |x: &SS<N>, y: &SS<N>|
        SampledSpectrum::new(zip(x.iter(), y.iter()).map(|(x, y)| if *y != 0. { x / y } else { 0. }).collect());
);

gen_ops_comm!(
    <|const N: usize>;
    types SS<N>, f32 => SS<N>;

    for * call |x: &SS<N>, y: &f32| SampledSpectrum::new(x.iter().map(|x| x * y).collect());
    for / call |x: &SS<N>, y: &f32| SampledSpectrum::new(x.iter().map(|x| if *y != 0. {x / y} else {0.}).collect());
);

// |x: &mut SS<N>, y: SS<N>| zip( x.iter_mut(), y.iter() ).for_each(|(x, y)| *x += y));
// |x: &mut SS<N>, y: SS<N>| zip( x.iter_mut(), y.iter() ).for_each(|(x, y)| *x -= y));
// |x: &mut SS<N>, y: SS<N>| zip( x.iter_mut(), y.iter() ).for_each(|(x, y)| *x *= y));
// |x: &mut SS<N>, y: SS<N>| zip( x.iter_mut(), y.iter() ).for_each(|(x, y)| if *y != 0. { *x /= y } else { *x = 0. }));
