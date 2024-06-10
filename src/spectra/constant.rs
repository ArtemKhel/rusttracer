use derive_new::new;

use crate::spectra::{sampled_spectrum::SampledSpectrum, sampled_wavelengths::SampledWavelengths, Spectrum};

/// Constant spectral distribution over all wavelengths
#[derive(Copy, Clone, Debug)]
#[derive(new)]
pub struct ConstantSpectrum {
    c: f32,
}

impl Spectrum for ConstantSpectrum {
    fn value(&self, wavelength: f32) -> f32 { self.c }

    fn sample<const N: usize>(&self, lambda: &SampledWavelengths<N>) -> SampledSpectrum<N> {
        SampledSpectrum::from(self.c)
    }
}
