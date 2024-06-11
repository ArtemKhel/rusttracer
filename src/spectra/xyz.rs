use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use derive_new::new;

use crate::{
    point2,
    spectra::{
        cie::{CIE, CIE_Y_INTEGRAL},
        inner_product, Spectrum, SpectrumEnum,
    },
    Point2f, Vec3f,
};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[derive(new)]
#[derive(Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign)]
pub struct XYZ {
    pub(super) x: f32,
    pub(super) y: f32,
    pub(super) z: f32,
}

impl XYZ {
    pub fn has_nan(&self) -> bool { self.x.is_nan() || self.y.is_nan() || self.z.is_nan() }

    pub fn xy(&self) -> Point2f { point2!(self.x / (self.x + self.y + self.z), self.y / (self.x + self.y + self.z)) }

    #[allow(non_snake_case)]
    pub fn from_xy_Y(xy: Point2f, Y: f32) -> Self {
        if (xy.y == 0.) {
            XYZ::default()
        } else {
            XYZ {
                x: xy.x * Y / xy.y,
                y: Y,
                z: (1. - xy.x - xy.y) * Y / xy.y,
            }
        }
    }

    pub fn from_xy(xy: Point2f) -> Self { Self::from_xy_Y(xy, 1.) }
}

impl From<XYZ> for Vec3f {
    fn from(value: XYZ) -> Self { Vec3f::new(value.x, value.y, value.z) }
}

impl From<Vec3f> for XYZ {
    fn from(value: Vec3f) -> Self { XYZ::new(value.x, value.y, value.z) }
}

impl From<&SpectrumEnum> for XYZ {
    fn from(value: &SpectrumEnum) -> Self {
        XYZ {
            x: inner_product(CIE::X.get(), value),
            y: inner_product(CIE::Y.get(), value),
            z: inner_product(CIE::Z.get(), value),
        } / CIE_Y_INTEGRAL
    }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;
    use rand::random;

    use super::*;
    use crate::{
        samplers::{
            utils::{sample_visible_wavelengths, visible_wavelengths_pdf},
            Sampler, StratifiedSampler,
        },
        spectra::{ConstantSpectrum, SampledWavelengths, VISIBLE_MAX, VISIBLE_MIN},
    };

    #[test]
    fn test_xyz() {
        // XYZ of a constant spectrum is one
        let const_spec = ConstantSpectrum::new(1.);
        let mut sampler = StratifiedSampler::new(1, 1, true, 42);
        let mut xyz = XYZ::default();
        let n = 100;
        for i in 0..n {
            let lambda = SampledWavelengths::<10>::sample_uniform(sampler.get_1d(), VISIBLE_MIN, VISIBLE_MAX);
            xyz += const_spec.sample(&lambda).to_xyz(&lambda);
        }
        xyz /= n as f32;
        assert_abs_diff_eq!(xyz.x, 1.0, epsilon = 0.1);
        assert_abs_diff_eq!(xyz.y, 1.0, epsilon = 0.1);
        assert_abs_diff_eq!(xyz.z, 1.0, epsilon = 0.1);
    }

    #[test]
    fn test_y_integral() {
        let mut sampler = StratifiedSampler::new(1, 1, true, 42);
        let mut y = 0.;
        let n = 1000;
        for i in 0..n {
            let lambda = sample_visible_wavelengths(sampler.get_1d());
            let pdf = visible_wavelengths_pdf(lambda);
            y += CIE::Y.get().value(lambda) / pdf
        }
        y /= n as f32;
        assert_abs_diff_eq!(y, CIE_Y_INTEGRAL, epsilon = 1.0);
    }
}
