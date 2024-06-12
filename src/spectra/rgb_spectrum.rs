use std::sync::Arc;

use crate::spectra::{
    rgb::{RGBColorSpace, RGBSigmoidPoly, RGB},
    DenselySampledSpectrum, Spectrum,
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
    fn new(color_space: &RGBColorSpace, rgb: RGB) -> RGBUnboundedSpectrum {
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
    illuminant: Arc<DenselySampledSpectrum>,
}

impl RGBIlluminantSpectrum {
    fn new(color_space: &RGBColorSpace, rgb: RGB, illuminant: Arc<DenselySampledSpectrum>) -> RGBIlluminantSpectrum {
        let scale = match 2. * rgb.max() {
            0.0 => 1.,
            x => x,
        };
        RGBIlluminantSpectrum {
            rsp: color_space.to_rgb_poly(rgb / scale),
            scale,
            illuminant,
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
    use crate::spectra::{rgb::sRGB, SampledSpectrum, SampledWavelengths};

    #[test]
    fn test_albedo() {
        // TODO: it fails after switching SS/SW from ArrayVec to regular array
        let r = RGB::R;
        let g = RGB::G;
        let b = RGB::B;

        for rgb in [g, b] {
            let mut spectrum = SampledSpectrum::<100>::default();
            let lambda = SampledWavelengths::<100>::sample_visible(random());
            let a = RGBAlbedoSpectrum::new(&sRGB, rgb);
            let sample = a.sample(&lambda);
            spectrum += sample;
            let result = spectrum.to_rgb(&lambda, &sRGB);

            assert_abs_diff_eq!(result, rgb);
        }
    }
}
