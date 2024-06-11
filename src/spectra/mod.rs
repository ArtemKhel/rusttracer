use arrayvec::ArrayVec;
pub use constant::ConstantSpectrum;
pub use densely_sampled::DenselySampledSpectrum;
use named::NamedSpectra;

use crate::spectra::{
    blackbody::BlackbodySpectrum, piecewise_linear::PiecewiseLinearSpectrum, sampled_spectrum::SampledSpectrum,
    sampled_wavelengths::SampledWavelengths,
};
use crate::spectra::rgb_spectrum::RGBAlbedoSpectrum;

mod blackbody;
mod cie;
mod constant;
mod densely_sampled;
mod named;
mod piecewise_linear;
mod rgb;
mod sampled_spectrum;
mod sampled_wavelengths;
mod xyz;
mod rgb2spec;
mod rgb_spectrum;

const LAMBDA_MIN: f32 = 360.;
const LAMBDA_MAX: f32 = 830.;

#[enum_delegate::register]
pub trait Spectrum {
    fn value(&self, wavelength: f32) -> f32;
    fn sample<const N: usize>(&self, lambda: &SampledWavelengths<N>) -> SampledSpectrum<N> {
        SampledSpectrum::from(lambda.iter().map(|&l| self.value(l)).collect::<ArrayVec<_, N>>())
    }
}

#[enum_delegate::implement(Spectrum)]
#[derive(Clone, Debug)]
pub enum SpectrumEnum {
    Constant(ConstantSpectrum),
    DenselySampled(DenselySampledSpectrum),
    PiecewiseLinear(PiecewiseLinearSpectrum),
    Blackbody(BlackbodySpectrum),
    RGBAlbedo(RGBAlbedoSpectrum)
}

// todo
fn inner_product<F: Spectrum, G: Spectrum>(f: &F, g: &G) -> f32 {
    (LAMBDA_MIN as i32..=LAMBDA_MAX as i32)
        .into_iter()
        .map(|x| f.value(x as f32) + g.value(x as f32))
        .sum()
}
