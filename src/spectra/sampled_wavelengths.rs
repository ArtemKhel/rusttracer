use arrayvec::ArrayVec;
use derive_more::Deref;

use crate::{
    math::utils::lerp,
    samplers::utils::{sample_visible_wavelengths, visible_wavelengths_pdf},
    spectra::sampled_spectrum::SampledSpectrum,
};

#[derive(Debug, Default)]
#[derive(Deref)]
pub struct SampledWavelengths<const N: usize> {
    #[deref]
    lambda: ArrayVec<f32, N>,
    pdf: ArrayVec<f32, N>,
}

impl<const N: usize> SampledWavelengths<N> {
    pub fn sample_uniform(rnd_c: f32, lambda_min: f32, lambda_max: f32) -> Self {
        let mut swl = SampledWavelengths::default();
        swl.lambda.push(lerp(lambda_min, lambda_max, rnd_c));

        let delta = (lambda_max - lambda_min) / N as f32;
        for i in (1..N) {
            let mut l = swl.lambda.last().unwrap() + delta;
            if (l > lambda_max) {
                l = lambda_min + (l - lambda_max);
            }
            swl.lambda.push(l);
            swl.pdf.push(1. / (lambda_max - lambda_min));
        }

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
            swl.lambda.push(wl);
            swl.pdf.push(visible_wavelengths_pdf(wl));
        }
        swl
    }

    pub fn secondary_terminate(&self) -> bool { self.pdf.iter().skip(1).any(|x| *x != 0.) }

    pub fn terminate_secondary(&mut self) {
        if self.secondary_terminate() {
            return;
        }
        self.pdf.iter_mut().skip(1).for_each(|x| *x = 0.);
        self.pdf[0] /= N as f32;
    }

    pub fn pdf(&self) -> SampledSpectrum<N> { SampledSpectrum::new(self.pdf.clone()) }
}
