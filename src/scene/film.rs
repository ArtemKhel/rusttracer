use std::{cmp::min, sync::Arc};

use image::{
    buffer::ConvertBuffer, codecs::avif::ColorSpace, FlatSamples, ImageBuffer, ImageResult, Pixel, Rgb, RgbImage,
};
use itertools::{any, Itertools};
use log::{debug, warn};
use ndarray::Array2;
use num_traits::Signed;
use rand::random;

use crate::{
    math::{Bounds2, Point2},
    point2,
    spectra::rgb::{RGBColorSpace, RGB},
    utils::linear_to_gamma,
    Point2u, Point2us, SampledSpectrum, SampledWavelengths,
};

pub trait Film {
    fn add_sample(&mut self, coord: Point2us, spectrum: SampledSpectrum, wavelengths: SampledWavelengths, weight: f32);
    // fn sample_bounds(&self);
    // fn resolution(&self);
    fn sample_wavelengths(&self, rnd_c: f32) -> SampledWavelengths;
    fn write_image(&self, path: &str) -> ImageResult<()>;
    fn tiles(&self, height: usize, width: usize) -> Vec<Bounds2<usize>>;
}

#[derive(Copy, Clone, Debug, Default)]
pub struct RGBPixel {
    // TODO: f64?
    rgb: RGB,
    weight: f32,
}

#[derive(Debug)]
pub struct RGBFilm {
    pub resolution: Point2us,
    pixels: Array2<RGBPixel>,
    color_space: Arc<RGBColorSpace>,
}

impl RGBFilm {
    pub fn new(width: usize, height: usize, color_space: Arc<RGBColorSpace>) -> Self {
        RGBFilm {
            resolution: point2!(width, height),
            pixels: Array2::from_elem((height, width), RGBPixel::default()),
            color_space,
        }
    }
}

impl Film for RGBFilm {
    fn add_sample(&mut self, coord: Point2us, spectrum: SampledSpectrum, wavelengths: SampledWavelengths, weight: f32) {
        let rgb = spectrum.to_rgb(&wavelengths, &self.color_space);
        if rgb.has_nan() || weight.is_nan() {
            warn!("Trying to add NaN-valued pixel {rgb:?} or weight {weight} at {coord:?}, ignoring");
            return;
        }
        let width = coord.x;
        let height = coord.y;
        if let Some(pixel) = self.pixels.get_mut((height, width)) {
            pixel.rgb += rgb;
            pixel.weight += weight;
        } else {
            warn!(
                "Trying to access pixel ({height},{width}) out of {:?}",
                self.pixels.shape()
            )
        }
    }

    fn sample_wavelengths(&self, rnd_c: f32) -> SampledWavelengths { SampledWavelengths::sample_visible(rnd_c) }

    fn write_image(&self, path: &str) -> ImageResult<()> {
        // TODO: zero-copy
        // TODO: white balance / linear_to_gamma
        let raw_pixels: Vec<f32> = self
            .pixels
            .iter()
            // .flat_map(|pix| linear_to_gamma(pix.rgb.map(|x| x / pix.weight)).0)
            .flat_map(|pix| <[f32; 3]>::from(pix.rgb / pix.weight))
            .collect();
        let image =
            ImageBuffer::<Rgb<f32>, Vec<f32>>::from_vec(self.resolution.x as u32, self.resolution.y as u32, raw_pixels)
                .unwrap();
        let image: RgbImage = image.convert();
        image.save(path)
    }

    fn tiles(&self, width: usize, height: usize) -> Vec<Bounds2<usize>> {
        let rows = self.pixels.nrows();
        let cols = self.pixels.ncols();
        let mut chunks = Vec::new();

        for row_start in (0..rows).step_by(height) {
            let row_end = min(row_start + width, rows);
            for col_start in (0..cols).step_by(width) {
                let col_end = min(col_start + height, cols);
                chunks.push(Bounds2::new(point2!(col_start, row_start), point2!(col_end, row_end)));
            }
        }

        chunks
    }
}
