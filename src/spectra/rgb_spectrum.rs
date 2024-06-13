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
    use crate::spectra::{LAMBDA_MAX, LAMBDA_MIN, rgb::sRGB, SampledSpectrum, SampledWavelengths, VISIBLE_MAX, VISIBLE_MIN};
    use crate::spectra::xyz::XYZ;

    #[test]
    fn test_albedo() {
        // TODO: at some point it started to fail :(
        let bl = RGB::BLACK;
        let wh = RGB::WHITE;
        let r = RGB::R;
        let g = RGB::G;
        let b = RGB::B;

        let color_space = sRGB.clone();

        for rgb in [bl, wh, r, g, b] {
            let lambda = SampledWavelengths::<470>::sample_visible(random());
            // let lambda = SampledWavelengths::<470>::sample_uniform(0., VISIBLE_MIN, VISIBLE_MAX);
            let albedo = RGBAlbedoSpectrum::new(&sRGB, rgb);
            // let albedo = RGBUnboundedSpectrum::new(&sRGB, rgb);
            // let spectrum = albedo.sample(&lambda);
            let spectrum = albedo.sample(&lambda) * color_space.illuminant.sample(&lambda);
            let xyz = spectrum.to_xyz(&lambda);
            let result = color_space.xyz_to_rgb(xyz);
            let result2 = spectrum.to_rgb(&lambda, &sRGB);

            assert_abs_diff_eq!(result, rgb, epsilon=0.01);
            assert_abs_diff_eq!(result2, rgb, epsilon=0.01);
        }
    }
}
