use std::sync::Arc;

use crate::spectra::{
    rgb::{RGBColorSpace, RGBSigmoidPoly, RGB},
    DenselySampledSpectrum, Spectrum, SpectrumEnum,
};

#[derive(Clone, Debug)]
pub struct RGBAlbedoSpectrum {
    rsp: RGBSigmoidPoly,
}

impl RGBAlbedoSpectrum {
    pub fn new(color_space: &RGBColorSpace, rgb: RGB) -> RGBAlbedoSpectrum {
        RGBAlbedoSpectrum {
            rsp: color_space.to_rgb_poly(rgb),
        }
    }
}

impl Spectrum for RGBAlbedoSpectrum {
    fn value(&self, wavelength: f32) -> f32 { self.rsp.eval(wavelength) }
}

#[derive(Clone, Debug)]
pub struct RGBUnboundedSpectrum {
    rsp: RGBSigmoidPoly,
    scale: f32,
}

impl RGBUnboundedSpectrum {
    pub(crate) fn new(color_space: &RGBColorSpace, rgb: RGB) -> RGBUnboundedSpectrum {
        let scale = match 2. * rgb.max() {
            0.0 => 1.,
            x => x,
        };
        RGBUnboundedSpectrum {
            rsp: color_space.to_rgb_poly(rgb / scale),
            scale,
        }
    }
}

impl Spectrum for RGBUnboundedSpectrum {
    fn value(&self, wavelength: f32) -> f32 { self.rsp.eval(wavelength) * self.scale }
}

#[derive(Clone, Debug)]
pub struct RGBIlluminantSpectrum {
    rsp: RGBSigmoidPoly,
    scale: f32,
    illuminant: Arc<SpectrumEnum>,
}

impl RGBIlluminantSpectrum {
    pub(crate) fn new(color_space: &RGBColorSpace, rgb: RGB) -> RGBIlluminantSpectrum {
        let scale = match 2. * rgb.max() {
            0.0 => 1.,
            x => x,
        };
        RGBIlluminantSpectrum {
            rsp: color_space.to_rgb_poly(rgb / scale),
            scale,
            illuminant: color_space.illuminant.clone(),
        }
    }
}

impl Spectrum for RGBIlluminantSpectrum {
    fn value(&self, wavelength: f32) -> f32 {
        self.rsp.eval(wavelength) * self.scale * self.illuminant.value(wavelength)
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;
    use rand::random;

    use super::*;
    use crate::spectra::{
        rgb::sRGB, xyz::XYZ, SampledSpectrum, SampledWavelengths, LAMBDA_MAX, LAMBDA_MIN, VISIBLE_MAX, VISIBLE_MIN,
    };

    #[test]
    fn test_albedo() {
        let bl = RGB::BLACK;
        let wh = RGB::WHITE;
        let r = RGB::R;
        let g = RGB::G;
        let b = RGB::B;

        let color_space = sRGB.clone();

        for rgb in [bl, wh, r, g, b] {
            let lambda = SampledWavelengths::<470>::sample_visible(random());
            let albedo = RGBAlbedoSpectrum::new(&sRGB, rgb);
            let spectrum = albedo.sample(&lambda) * color_space.illuminant.sample(&lambda);
            let result = spectrum.to_rgb(&lambda, &sRGB);

            assert_abs_diff_eq!(result, rgb, epsilon = 0.1);
        }
    }
}
