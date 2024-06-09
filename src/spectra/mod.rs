use arrayvec::ArrayVec;
pub use constant::ConstantSpectrum;
pub use densely_sampled::DenselySampledSpectrum;

use crate::spectra::{
    blackbody::BlackbodySpectrum, named::NamedSpectrum, piecewise_linear::PiecewiseLinearSpectrum,
    sampled_spectrum::SampledSpectrum, sampled_wavelengths::SampledWavelengths,
};

mod blackbody;
mod constant;
mod densely_sampled;
mod named;
mod piecewise_linear;
mod sampled_spectrum;
mod sampled_wavelengths;

const LAMBDA_MIN: f32 = 360.;
const LAMBDA_MAX: f32 = 830.;

#[enum_delegate::register]
pub trait Spectrum {
    fn value(&self, wavelength: f32) -> f32;
    fn sample<const N: usize>(&self, lambda: SampledWavelengths<N>) -> SampledSpectrum<N> {
        SampledSpectrum::from(lambda.iter().map(|&l| self.value(l)).collect::<ArrayVec<_, N>>())
    }
}

#[enum_delegate::implement(Spectrum)]
enum SpectrumEnum {
    Constant(ConstantSpectrum),
    DenselySampled(DenselySampledSpectrum),
    PiecewiseLinear(PiecewiseLinearSpectrum),
    Blackbody(BlackbodySpectrum),
    Named(NamedSpectrum),
}
