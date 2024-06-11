#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

use std::sync::LazyLock;

use rgb2spec::RGB2Spec;

use crate::spectra::rgb::{RGBSigmoidPoly, RGB};

#[derive(Debug)]
pub enum Gamut {
    sRGB,
}

impl Gamut {
    pub fn fetch_coefs(&self, rgb: RGB) -> RGBSigmoidPoly {
        match self {
            Gamut::sRGB => sRGB.fetch(rgb.into()).into(),
        }
    }
}

static sRGB: LazyLock<RGB2Spec> =
    LazyLock::new(|| RGB2Spec::load("./data/srgb.spec").expect("Failed to load Rgb2spec model"));
