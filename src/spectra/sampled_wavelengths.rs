use arrayvec::ArrayVec;
use derive_more::Deref;

use crate::math::utils::lerp;

#[derive(Debug, Default)]
#[derive(Deref)]
pub struct SampledWavelengths<const N: usize> {
    #[deref]
    lambda: ArrayVec<f32, N>,
    pdf: ArrayVec<f32, N>,
}

impl<const N: usize> SampledWavelengths<N> {
    pub fn sample_uniform(sample_c: f32, lambda_min: f32, lambda_max: f32) -> Self {
        let mut swl = SampledWavelengths::default();
        swl.lambda[0] = lerp(lambda_min, lambda_max, sample_c);

        let delta = (lambda_max - lambda_min) / N as f32;
        for i in (1..N) {
            swl.lambda[i] = swl.lambda[i - 1] + delta;
            if (swl.lambda[i] > lambda_max) {
                swl.lambda[i] = lambda_min + (swl.lambda[i] - lambda_max);
            }
        }

        swl.pdf.iter_mut().for_each(|x| *x = 1. / (lambda_max - lambda_min));

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
}
