use std::{
    hash::{DefaultHasher, Hash, Hasher},
    ptr::hash,
};

use log::warn;
use rand::prelude::*;
use rand_seeder::Seeder;

use crate::{breakpoint, samplers::Sampler, Point2f, Point2us};

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
        // let mut rng = Seeder::from((self.current_pixel, self.dimension, self.seed)).make_rng::<SmallRng>();
        let mut hasher = DefaultHasher::new();
        (self.current_pixel, self.dimension, self.seed).hash(&mut hasher);
        let hash = hasher.finish();
        let mut rng = SmallRng::seed_from_u64(hash);
        let rnd_index = rng.gen_range((0..self.samples_per_pixel));
        (rnd_index + hash as u32) % self.samples_per_pixel
    }
}

impl Sampler for StratifiedSampler {
    fn samples_per_pixel(&self) -> u32 { self.samples_per_pixel }

    fn start_pixel_sample(&mut self, pixel: Point2us, sample_index: u32) {
        self.current_pixel = pixel;
        self.sample_index = sample_index;
        self.rng = Seeder::from((pixel, sample_index, self.seed)).make_rng::<SmallRng>();
    }

    fn get_1d(&mut self) -> f32 {
        let stratum = self.permutation_element() as f32;
        self.dimension += 1;
        let delta = if self.jitter { self.rng.gen() } else { 0.5 };
        return (stratum + delta) / self.samples_per_pixel as f32;
    }

    fn get_2d(&mut self) -> Point2f {
        let stratum = self.permutation_element();
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
