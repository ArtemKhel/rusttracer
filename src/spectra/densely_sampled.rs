use arrayvec::ArrayVec;

use crate::spectra::{
    sampled_spectrum::SampledSpectrum, sampled_wavelengths::SampledWavelengths, Spectrum, SpectrumEnum, LAMBDA_MAX,
    LAMBDA_MIN,
};

/// Spectral distribution sampled at 1 nm intervals over a given range of integer wavelengths
#[derive(Clone, Debug)]
pub struct DenselySampledSpectrum {
    lambda_min: usize,
    lambda_max: usize,
    // TODO: stack allocated?
    values: Vec<f32>,
}

impl DenselySampledSpectrum {
    pub fn new(spectrum: SpectrumEnum) -> Self { Self::new_range(spectrum, LAMBDA_MIN as usize, LAMBDA_MAX as usize) }

    pub fn new_range(spectrum: SpectrumEnum, lambda_min: usize, lambda_max: usize) -> Self {
        assert!(lambda_min < lambda_max);
        let values = Vec::from_iter((lambda_min..=lambda_max).map(|i| spectrum.value(i as f32)));
        DenselySampledSpectrum {
            lambda_min,
            lambda_max,
            values,
        }
    }
}

impl Spectrum for DenselySampledSpectrum {
    fn value(&self, wavelength: f32) -> f32 {
        let mut offset = wavelength.round() as usize;
        if self.lambda_min <= offset || offset < self.lambda_max {
            self.values[offset - self.lambda_min]
        } else {
            0.
        }
    }
}
