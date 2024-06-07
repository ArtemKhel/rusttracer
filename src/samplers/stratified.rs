use std::hash::{DefaultHasher, Hash, Hasher};

use rand::prelude::*;
use rand_seeder::Seeder;

use crate::{samplers::Sampler, Point2f, Point2us};

#[derive(Clone, Debug)]
pub struct StratifiedSampler {
    samples_per_pixel: u32,
    x_samples: u32,
    y_samples: u32,
    jitter: bool,
    seed: u64,
    rng: SmallRng,
    current_pixel: Point2us,
    sample_index: u32,
    dimension: u32,
}

impl StratifiedSampler {
    pub(crate) fn new(x_samples: u32, y_samples: u32, jitter: bool, seed: u64) -> Self {
        let rng = SmallRng::seed_from_u64(seed);
        let samples_per_pixel = x_samples * y_samples;
        StratifiedSampler {
            samples_per_pixel,
            x_samples,
            y_samples,
            jitter,
            seed,
            rng,
            current_pixel: Default::default(),
            sample_index: 0,
            dimension: 0,
        }
    }

    fn permutation_element(&self) -> u32 {
        let mut hasher = DefaultHasher::new();
        (self.current_pixel, self.dimension, self.seed).hash(&mut hasher);
        let hash = hasher.finish();
        let mut rng = SmallRng::seed_from_u64(hash);
        let rnd_index = rng.gen_range((0..self.samples_per_pixel));
        (rnd_index + self.sample_index + hash as u32) % self.samples_per_pixel
    }

    fn permutation_element_2(&self) -> u32 {
        let mut i = self.sample_index as u64;
        let l = self.samples_per_pixel as u64;
        let mut hasher = DefaultHasher::new();
        (self.current_pixel, self.dimension, self.seed).hash(&mut hasher);
        let p = hasher.finish();
        let mut w = l - 1;
        w |= w >> 1;
        w |= w >> 2;
        w |= w >> 4;
        w |= w >> 8;
        w |= w >> 16;
        loop {
            i ^= p;
            i.overflowing_mul(0xe170893d);
            i ^= p >> 16;
            i ^= (i & w) >> 4;
            i ^= p >> 8;
            i.overflowing_mul(0x0929eb3f);
            i ^= p >> 23;
            i ^= (i & w) >> 1;
            i.overflowing_mul(1 | p >> 27);
            i.overflowing_mul(0x6935fa69);
            i ^= (i & w) >> 11;
            i.overflowing_mul(0x74dcb303);
            i ^= (i & w) >> 2;
            i.overflowing_mul(0x9e501cc3);
            i ^= (i & w) >> 2;
            i.overflowing_mul(0xc860a3df);
            i &= w;
            i ^= i >> 5;
            if i <= l {
                break;
            }
        }
        return ((i + p) % l) as u32;
    }
}

// TODO: somehow it is worse then independent. why?
impl Sampler for StratifiedSampler {
    fn samples_per_pixel(&self) -> u32 { self.samples_per_pixel }

    fn start_pixel_sample(&mut self, pixel: Point2us, sample_index: u32) {
        self.start_pixel_sample_with_dim(pixel, sample_index, 0);
    }

    fn start_pixel_sample_with_dim(&mut self, pixel: Point2us, sample_index: u32, dimension: u32) {
        self.dimension = dimension;
        self.current_pixel = pixel;
        self.sample_index = sample_index;
        self.rng = Seeder::from((pixel, sample_index, self.seed)).make_rng::<SmallRng>();
    }

    fn get_1d(&mut self) -> f32 {
        let stratum = self.permutation_element_2() as f32;
        self.dimension += 1;
        let delta = if self.jitter { self.rng.gen() } else { 0.5 };
        return (stratum + delta) / self.samples_per_pixel as f32;
    }

    fn get_2d(&mut self) -> Point2f {
        let stratum = self.permutation_element_2();
        self.dimension += 2;
        let x = stratum % self.x_samples;
        let y = stratum / self.x_samples;
        let dx = if self.jitter { self.rng.gen() } else { 0.5f32 };
        let dy = if self.jitter { self.rng.gen() } else { 0.5f32 };

        Point2f::new(
            (x as f32 + dx) / self.x_samples as f32,
            (y as f32 + dy) / self.y_samples as f32,
        )
    }

    fn get_pixel(&mut self) -> Point2f { self.get_2d() }
}

#[cfg(test)]
mod tests {
    use approx::{assert_abs_diff_eq, assert_abs_diff_ne};

    use super::*;
    use crate::point2;

    #[test]
    fn test_fn() {
        let seed = 42;
        let mut sampler = StratifiedSampler::new(3, 3, true, seed);
        dbg!("Run 1");
        sampler.start_pixel_sample(point2!(0, 0), 0);
        for _ in (0..9) {
            dbg!(&sampler.get_1d());
        }
        dbg!("Run 2");
        sampler.start_pixel_sample(point2!(0, 0), 1);
        for _ in (0..9) {
            dbg!(&sampler.get_1d());
        }
    }

    #[test]
    fn test_reproducibility() {
        let seed = 42;
        let mut sampler1 = StratifiedSampler::new(5, 5, true, seed);
        let mut sampler2 = StratifiedSampler::new(5, 5, true, seed);

        for p in (0..5) {
            for i in (0..5) {
                sampler2.start_pixel_sample(point2!(0, 3), i);
                sampler2.get_2d();
                sampler2.get_1d();

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
        let mut sampler1 = StratifiedSampler::new(5, 5, true, seed);
        let mut sampler2 = StratifiedSampler::new(5, 5, true, seed + 1);

        sampler1.start_pixel_sample(point2!(0, 0), 0);
        sampler2.start_pixel_sample(point2!(0, 0), 0);

        assert_abs_diff_ne!(sampler1.get_1d(), sampler2.get_1d());
        assert_abs_diff_ne!(sampler1.get_2d(), sampler2.get_2d());
        assert_abs_diff_ne!(sampler1.get_1d(), sampler2.get_1d());
        assert_abs_diff_ne!(sampler1.get_pixel(), sampler2.get_pixel());
    }
}
