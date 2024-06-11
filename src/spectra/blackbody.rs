#![allow(clippy::excessive_precision)]
use arrayvec::ArrayVec;

use crate::spectra::{sampled_spectrum::SampledSpectrum, sampled_wavelengths::SampledWavelengths, Spectrum};

#[derive(Copy, Clone, Debug)]
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

#[cfg(test)]
mod tests {
    use approx::{assert_abs_diff_eq, assert_relative_eq};
    use itertools::izip;

    use super::*;

    #[test]
    fn test_blackbody() {
        let temps = [6000., 6000., 3700., 4500.];
        let lambdas = [483., 600., 500., 600.];
        let expected = [3.1811e13, 2.86772e13, 1.59845e12, 7.46497e12];

        for (t, l, e) in izip!(temps, lambdas, expected) {
            assert_relative_eq!(BlackbodySpectrum::blackbody(l, t), e, max_relative = 0.01);
        }

        // Use Wien's displacement law to compute maximum wavelength for a few
        // temperatures, then confirm that the value returned by Blackbody() is
        // consistent with this.
        for t in [2700., 3000., 4500., 5600., 6000.] {
            let lambda_max = 2.8977721e-3 / t * 1e9;
            assert!(BlackbodySpectrum::blackbody(lambda_max * 0.99, t) < BlackbodySpectrum::blackbody(lambda_max, t));
            assert!(BlackbodySpectrum::blackbody(lambda_max * 1.01, t) < BlackbodySpectrum::blackbody(lambda_max, t));
        }
    }
}
