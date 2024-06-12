use std::{array, env::var};

use arrayvec::ArrayVec;
pub use constant::ConstantSpectrum;
pub use densely_sampled::DenselySampledSpectrum;
use named::NamedSpectra;
pub use rgb_spectrum::{RGBAlbedoSpectrum, RGBIlluminantSpectrum, RGBUnboundedSpectrum};
pub use sampled_spectrum::SampledSpectrum;
pub use sampled_wavelengths::SampledWavelengths;

use crate::spectra::{blackbody::BlackbodySpectrum, piecewise_linear::PiecewiseLinearSpectrum};

mod blackbody;
mod cie;
mod constant;
mod densely_sampled;
mod gamut;
pub mod named;
mod piecewise_linear;
pub mod rgb;
mod rgb_spectrum;
pub mod sampled_spectrum;
pub mod sampled_wavelengths;
mod xyz;

pub const LAMBDA_MIN: f32 = 360.;
pub const LAMBDA_MAX: f32 = 830.;
pub const VISIBLE_MIN: f32 = 360.;
pub const VISIBLE_MAX: f32 = 830.;

#[enum_delegate::register]
pub trait Spectrum {
    fn value(&self, wavelength: f32) -> f32;
    fn sample<const N: usize>(&self, lambda: &SampledWavelengths<N>) -> SampledSpectrum<N> {
        let values: [f32; N] = array::from_fn(|i| self.value(lambda[i]));
        SampledSpectrum::from(values)
    }
}

#[enum_delegate::implement(Spectrum)]
#[derive(Clone, Debug)]
pub enum SpectrumEnum {
    Constant(ConstantSpectrum),
    DenselySampled(DenselySampledSpectrum),
    PiecewiseLinear(PiecewiseLinearSpectrum),
    Blackbody(BlackbodySpectrum),
    RGBAlbedo(RGBAlbedoSpectrum),
    RGBUnbounded(RGBUnboundedSpectrum),
    RGBIlluminant(RGBIlluminantSpectrum),
}

fn inner_product<F: Spectrum, G: Spectrum>(f: &F, g: &G) -> f32 {
    (LAMBDA_MIN as i32..=LAMBDA_MAX as i32)
        .into_iter()
        .map(|x| f.value(x as f32) * g.value(x as f32))
        .sum()
}
