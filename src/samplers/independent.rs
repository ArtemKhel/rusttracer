use std::hash::Hash;

use rand::{rngs::SmallRng, Rng, SeedableRng};
use rand_seeder::Seeder;

use crate::{samplers::Sampler, Point2f, Point2u, Point2us};

#[derive(Clone, Debug)]
pub struct IndependentSampler {
    samples_per_pixel: u32,
    seed: u64,
    rng: SmallRng,
}

impl IndependentSampler {
    pub(crate) fn new(samples_per_pixel: u32, seed: u64) -> Self {
        let rng = SmallRng::seed_from_u64(seed);
        IndependentSampler {
            samples_per_pixel,
            seed,
            rng,
        }
    }
}

impl Sampler for IndependentSampler {
    fn samples_per_pixel(&self) -> u32 { self.samples_per_pixel }

    fn start_pixel_sample(&mut self, pixel: Point2us, sample_index: u32) {
        self.rng = Seeder::from((pixel, sample_index, self.seed)).make_rng::<SmallRng>();
    }

    fn get_1d(&mut self) -> f32 { self.rng.gen() }

    fn get_2d(&mut self) -> Point2f { self.rng.gen() }

    fn get_pixel(&mut self) -> Point2f { self.rng.gen() }
}

#[cfg(test)]
mod tests {
    use approx::{assert_abs_diff_eq, assert_abs_diff_ne};

    use super::*;
    use crate::point2;

    #[test]
    fn test_reproducibility() {
        let seed = 42;
        let mut sampler1 = IndependentSampler::new(5, seed);
        let mut sampler2 = IndependentSampler::new(5, seed);

        for p in (0..5) {
            for i in (0..5) {
                sampler1.start_pixel_sample(point2!(0, p), i);
                sampler2.start_pixel_sample(point2!(0, p), i);

                assert_abs_diff_eq!(sampler1.get_1d(), sampler2.get_1d());
                assert_abs_diff_eq!(sampler1.get_2d(), sampler2.get_2d());
                assert_abs_diff_eq!(sampler1.get_1d(), sampler2.get_1d());
                assert_abs_diff_eq!(sampler1.get_pixel(), sampler2.get_pixel());
            }
        }
    }

    #[test]
    fn test_different_seeds() {
        let seed = 42;
        let mut sampler1 = IndependentSampler::new(5, seed);
        let mut sampler2 = IndependentSampler::new(5, seed + 1);

        sampler1.start_pixel_sample(point2!(0, 0), 0);
        sampler2.start_pixel_sample(point2!(0, 0), 0);

        assert_abs_diff_ne!(sampler1.get_1d(), sampler2.get_1d());
        assert_abs_diff_ne!(sampler1.get_2d(), sampler2.get_2d());
        assert_abs_diff_ne!(sampler1.get_1d(), sampler2.get_1d());
        assert_abs_diff_ne!(sampler1.get_pixel(), sampler2.get_pixel());
    }
}
