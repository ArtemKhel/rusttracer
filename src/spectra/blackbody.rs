#![allow(clippy::excessive_precision)]
use arrayvec::ArrayVec;

use crate::spectra::{sampled_spectrum::SampledSpectrum, sampled_wavelengths::SampledWavelengths, Spectrum};

#[derive(Debug)]
pub struct BlackbodySpectrum {
    temp: f32,
    norm_factor: f32,
}

impl BlackbodySpectrum {
    pub fn new(temp: f32) -> Self {
        let lambda_max = 2.897_772_1e-3 / temp;
        let norm_factor = 1. / Self::blackbody(lambda_max * 1e9, temp);
        BlackbodySpectrum { temp, norm_factor }
    }

    #[allow(non_upper_case_globals)]
    fn blackbody(lambda: f32, temp: f32) -> f32 {
        if (temp <= 0.) {
            return 0.;
        };
        const c: f32 = 299_792_458.;
        const h: f32 = 6.626_069_57e-34;
        const kb: f32 = 1.380_328_8e-23;
        let l = lambda * 1e-9;

        (2. * h * c.powi(2)) / (l.powi(5) * (((h * c) / (l * kb * temp)).exp() - 1.))
    }
}

impl Spectrum for BlackbodySpectrum {
    fn value(&self, wavelength: f32) -> f32 { Self::blackbody(wavelength, self.temp) * self.norm_factor }
}
