use crate::{math::utils::lerp, spectra::Spectrum};

#[derive(Debug)]
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
