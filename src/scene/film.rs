use std::cmp::min;

use image::{buffer::ConvertBuffer, FlatSamples, ImageBuffer, ImageResult, Pixel, Rgb, RgbImage};
use itertools::{any, Itertools};
use log::{debug, warn};
use ndarray::Array2;
use num_traits::Signed;
use rand::random;

use crate::{
    colors,
    math::{Bounds2, Point2},
    point2, Point2u, Point2us,
};

pub trait Film {
    fn add_sample(&mut self, coord: Point2us, color: Rgb<f32>, weight: f32);
    // fn sample_bounds(&self);
    // fn resolution(&self);
    fn write_image(&self, path: &str) -> ImageResult<()>;
    fn tiles(&self, height: usize, width: usize) -> Vec<Bounds2<usize>>;
}

#[derive(Copy, Clone, Debug)]
pub struct RGBPixel {
    // TODO: f64?
    rgb: Rgb<f32>,
    weight: f32,
}

impl Default for RGBPixel {
    fn default() -> Self {
        RGBPixel {
            rgb: colors::BLACK,
            weight: 0.,
        }
    }
}

#[derive(Debug)]
pub struct RGBFilm {
    pub resolution: Point2us,
    pixels: Array2<RGBPixel>,
}

impl RGBFilm {
    pub fn new(width: usize, height: usize) -> Self {
        RGBFilm {
            resolution: point2!(width, height),
            pixels: Array2::from_elem((height, width), RGBPixel::default()),
        }
    }
}

impl Film for RGBFilm {
    fn add_sample(&mut self, coord: Point2us, color: Rgb<f32>, weight: f32) {
        if any(color.0, f32::is_nan) || weight.is_nan() {
            warn!("Trying to add NaN-valued pixel {color:?} or weight {weight} at {coord:?}, ignoring");
            return;
        }
        let width = coord.x;
        let height = coord.y;
        // let pixel = unsafe { self.pixels.uget_mut((width, height)) };
        if let Some(pixel) = self.pixels.get_mut((height, width)) {
            pixel.rgb.apply2(&color, |a, b| a + b);
            pixel.weight += weight
        } else {
            warn!(
                "Trying to access pixel ({height},{width}) out of {:?}",
                self.pixels.shape()
            )
        }
    }

    fn write_image(&self, path: &str) -> ImageResult<()> {
        // TODO: zero-copy
        // TODO: white balance / linear_to_gamma
        let raw_pixels: Vec<f32> = self
            .pixels
            .iter()
            .flat_map(|pix| pix.rgb.0.map(|x| x / pix.weight))
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
