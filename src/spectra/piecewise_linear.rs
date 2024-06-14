use std::ops::Deref;

use crate::{
    math::utils::lerp,
    spectra::{
        cie::{CIE, CIE_Y_INTEGRAL},
        inner_product, Spectrum, LAMBDA_MAX, LAMBDA_MIN,
    },
};

#[derive(Clone, Debug)]
pub struct PiecewiseLinearSpectrum {
    lambdas: Vec<f32>,
    values: Vec<f32>,
}

impl PiecewiseLinearSpectrum {
    pub fn new(lambdas: &[f32], values: &[f32]) -> Self {
        assert!(lambdas.windows(2).all(|x| x[0] < x[1]));
        PiecewiseLinearSpectrum {
            lambdas: Vec::from(lambdas),
            values: Vec::from(values),
        }
    }

    pub fn from_interleaved(from: &[f32], normalize: bool) -> PiecewiseLinearSpectrum {
        let len = from.len();
        assert_eq!(len % 2, 0);
        let half = len / 2;

        let mut lambdas = Vec::<f32>::with_capacity(half + 2);
        let mut values = Vec::<f32>::with_capacity(half + 2);

        if from[0] > LAMBDA_MIN {
            lambdas.push(from[0]);
            values.push(from[1]);
        }

        from.chunks_exact(2).for_each(|x| {
            lambdas.push(x[0]);
            values.push(x[1]);
        });

        if from[len - 2] < LAMBDA_MAX {
            lambdas.push(from[len - 2]);
            values.push(from[len - 1]);
        }

        debug_assert!(lambdas.windows(2).all(|x| x[0] <= x[1]));
        let mut spectrum = PiecewiseLinearSpectrum { lambdas, values };

        // Normalize to have luminance of 1.
        if (normalize) {
            spectrum.scale(CIE_Y_INTEGRAL / inner_product(&spectrum, CIE::Y.get()));
        }

        spectrum
    }

    fn scale(&mut self, factor: f32) { self.values.iter_mut().for_each(|x| *x *= factor); }
}

impl Spectrum for PiecewiseLinearSpectrum {
    fn value(&self, wavelength: f32) -> f32 {
        match self.lambdas.binary_search_by(|x| x.total_cmp(&wavelength)) {
            Ok(i) => self.values[i],
            Err(i) => {
                if i == 0 || i == self.values.len() {
                    0.
                } else {
                    let t = (wavelength - self.lambdas[i - 1]) / (self.lambdas[i] - self.lambdas[i - 1]);
                    let left = self.values[i - 1];
                    let right = self.values[i];
                    lerp(left, right, t)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spectra::{VISIBLE_MAX, VISIBLE_MIN};

    #[test]
    fn test() {
        let pwl = PiecewiseLinearSpectrum::new(&[VISIBLE_MIN, VISIBLE_MAX], &[1., 2.]);
        assert_eq!(pwl.value(360.0), 1.);
        assert_eq!(pwl.value(477.5), 1.25);
        assert_eq!(pwl.value(595.0), 1.5);
        assert_eq!(pwl.value(712.5), 1.75);
        assert_eq!(pwl.value(830.0), 2.);
    }
}
