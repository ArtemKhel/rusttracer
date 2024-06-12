use derive_more::Deref;
use itertools::Itertools;

use crate::{
    math::utils::lerp,
    samplers::utils::{sample_visible_wavelengths, visible_wavelengths_pdf},
    spectra::sampled_spectrum::SampledSpectrum,
};

#[derive(Debug)]
#[derive(Deref)]
pub struct SampledWavelengths<const N: usize> {
    #[deref]
    lambda: [f32; N],
    pdf: [f32; N],
    // lambda: ArrayVec<f32, N>,
    // pdf: ArrayVec<f32, N>,
}

impl<const N: usize> SampledWavelengths<N> {
    pub fn sample_uniform(rnd_c: f32, lambda_min: f32, lambda_max: f32) -> Self {
        let mut swl = SampledWavelengths::default();
        swl.lambda[0] = lerp(lambda_min, lambda_max, rnd_c);

        let delta = (lambda_max - lambda_min) / N as f32;
        for i in (1..N) {
            let mut l = swl.lambda[i - 1] + delta;
            if (l > lambda_max) {
                l = lambda_min + (l - lambda_max);
            }
            swl.lambda[i] = l;
        }
        swl.pdf.iter_mut().for_each(|x| *x = (lambda_max - lambda_min).recip());

        swl
    }

    pub fn sample_visible(rnd_c: f32) -> Self {
        let mut swl = SampledWavelengths::default();
        for i in 0..N {
            let mut wl = rnd_c + (i as f32 / N as f32);
            if wl > 1. {
                wl -= 1.
            }
            wl = sample_visible_wavelengths(wl);
            swl.lambda[i] = wl;
            swl.pdf[i] = visible_wavelengths_pdf(wl);
        }
        swl
    }

    pub fn secondary_terminated(&self) -> bool { self.pdf.iter().skip(1).any(|x| *x != 0.) }

    pub fn terminate_secondary(&mut self) {
        if self.secondary_terminated() {
            return;
        }
        self.pdf.iter_mut().skip(1).for_each(|x| *x = 0.);
        self.pdf[0] /= N as f32;
    }

    pub fn pdf(&self) -> SampledSpectrum<N> { SampledSpectrum::new(self.pdf.clone()) }
}

impl<const N: usize> Default for SampledWavelengths<N> {
    fn default() -> Self {
        SampledWavelengths {
            pdf: [0.; N],
            lambda: [0.; N],
        }
    }
}
