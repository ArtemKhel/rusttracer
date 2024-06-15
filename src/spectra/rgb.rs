use std::sync::{Arc, LazyLock};

use approx::AbsDiffEq;
use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use derive_new::new;
use num_traits::Signed;

use crate::{
    math::Matrix3,
    point2,
    Point2f,
    spectra::{gamut::Gamut, LAMBDA_MAX, LAMBDA_MIN, named::NamedSpectra, SpectrumEnum, xyz::XYZ}, Vec3f,
};

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[derive(new)]
#[derive(Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign)]
pub struct RGB {
    r: f32,
    g: f32,
    b: f32,
}

#[rustfmt::skip]
impl RGB {
    pub const WHITE: RGB = RGB { r: 1., g: 1., b: 1. };
    pub const BLACK: RGB = RGB { r: 0., g: 0., b: 0. };
    pub const R: RGB = RGB { r: 1., g: 0., b: 0. };
    pub const G: RGB = RGB { r: 0., g: 1., b: 0. };
    pub const B: RGB = RGB { r: 0., g: 0., b: 1. };
    pub const LIGHT_GRAY: RGB = RGB { r: 0.73, g: 0.73, b: 0.73 };
    pub const DARK_GRAY: RGB = RGB { r: 0.4, g: 0.4, b: 0.4 };
    pub const RED: RGB = RGB { r: 0.65, g: 0.05, b: 0.05 };
    pub const GREEN: RGB = RGB { r: 0.12, g: 0.45, b: 0.15 };
    pub const LIGHT_BLUE: RGB = RGB { r: 0.5, g: 0.8, b: 0.95 };
    pub const DARK_BLUE: RGB = RGB { r: 0.2, g: 0.3, b: 0.5 };
}

impl RGB {
    pub fn has_nan(&self) -> bool { self.r.is_nan() || self.g.is_nan() || self.b.is_nan() }

    pub fn max(&self) -> f32 { self.r.max(self.g.max(self.b)) }
}

impl From<RGB> for Vec3f {
    fn from(value: RGB) -> Self { Vec3f::new(value.r, value.g, value.b) }
}

impl From<Vec3f> for RGB {
    fn from(value: Vec3f) -> Self { RGB::new(value.x, value.y, value.z) }
}

impl From<RGB> for [f32; 3] {
    fn from(value: RGB) -> Self { [value.r, value.g, value.b] }
}

#[derive(Copy, Clone, Debug)]
#[derive(new)]
pub struct RGBSigmoidPoly {
    c0: f32,
    c1: f32,
    c2: f32,
}

impl From<[f32; 3]> for RGBSigmoidPoly {
    fn from(value: [f32; 3]) -> Self { Self::new(value[0], value[1], value[2]) }
}

impl RGBSigmoidPoly {
    pub fn eval(&self, lambda: f32) -> f32 {
        let s = lambda.mul_add(lambda.mul_add(self.c0, self.c1), self.c2);
        match s {
            f32::INFINITY => 1.,
            f32::NEG_INFINITY => 0.,
            _ => 0.5 + s / (2. * (1. + s.powi(2)).sqrt()),
        }
    }

    pub fn max_value(&self) -> f32 {
        let result = f32::max(self.eval(LAMBDA_MIN), self.eval(LAMBDA_MAX));
        let lambda = -self.c1 / (2.0 * self.c0);
        if (LAMBDA_MIN..=LAMBDA_MAX).contains(&lambda) {
            f32::max(result, self.eval(lambda))
        } else {
            result
        }
    }
}

#[derive(Debug)]
pub struct RGBColorSpace {
    r: Point2f,
    g: Point2f,
    b: Point2f,
    whitepoint: Point2f,
    pub(super) illuminant: Arc<SpectrumEnum>,
    rgb_to_xyz: Matrix3<f32>,
    xyz_to_rgb: Matrix3<f32>,
    gamut: Gamut,
}

impl RGBColorSpace {
    #[allow(non_snake_case)]
    pub fn new(r: Point2f, g: Point2f, b: Point2f, illuminant: Arc<SpectrumEnum>, gamut: Gamut) -> Self {
        let w = XYZ::from(illuminant.as_ref());
        let whitepoint = w.xy();
        let xyz_r = XYZ::from_xy(r);
        let xyz_g = XYZ::from_xy(g);
        let xyz_b = XYZ::from_xy(b);

        #[rustfmt::skip]
            let rgb = Matrix3::from_elements(
            xyz_r.x, xyz_g.x, xyz_b.x,
            xyz_r.y, xyz_g.y, xyz_b.y,
            xyz_r.z, xyz_g.z, xyz_b.z,
        );

        let c = rgb.invert().unwrap() * Vec3f::from(w);
        let rgb_to_xyz = rgb * Matrix3::diag(c.x, c.y, c.z);
        let xyz_to_rgb = rgb_to_xyz.invert().unwrap();

        RGBColorSpace {
            r,
            g,
            b,
            whitepoint,
            illuminant,
            rgb_to_xyz,
            xyz_to_rgb,
            gamut,
        }
    }

    pub fn to_rgb_poly(&self, rgb: RGB) -> RGBSigmoidPoly {
        debug_assert!(rgb.r.is_positive() && rgb.g.is_positive() && rgb.b.is_positive());
        self.gamut.fetch_coefs(rgb)
    }

    pub fn xyz_to_rgb(&self, xyz: XYZ) -> RGB { RGB::from(self.xyz_to_rgb * Vec3f::from(xyz)) }

    pub fn rgb_to_xyz(&self, rgb: RGB) -> XYZ { XYZ::from(self.rgb_to_xyz * Vec3f::from(rgb)) }
}

impl AbsDiffEq for RGB {
    type Epsilon = f32;

    fn default_epsilon() -> Self::Epsilon { f32::EPSILON }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.r.abs_diff_eq(&other.r, epsilon)
            && self.g.abs_diff_eq(&other.g, epsilon)
            && self.b.abs_diff_eq(&other.b, epsilon)
    }
}

#[allow(non_upper_case_globals)]
pub static sRGB: LazyLock<Arc<RGBColorSpace>> = LazyLock::new(|| {
    Arc::new(RGBColorSpace::new(
        point2!(0.64, 0.33),
        point2!(0.3, 0.6),
        point2!(0.15, 0.06),
        NamedSpectra::IlluminantD65.get(),
        Gamut::sRGB,
    ))
});

#[cfg(test)]
mod tests {
    use std::io::Write;

    use approx::assert_abs_diff_eq;

    use crate::spectra::{RGBAlbedoSpectrum, RGBUnboundedSpectrum, Spectrum, VISIBLE_MIN};

    use super::*;

    #[test]
    fn from_xy_zero() {
        let point = point2!(1.0, 0.0);
        let res = XYZ::from_xy_Y(point, 0.5);
        let exp = XYZ::new(0.0, 0.0, 0.0);
        assert_eq!(res, exp);
    }

    #[test]
    fn test_() {
        let rgb = RGB::WHITE;
        let poly = RGBUnboundedSpectrum::new(&sRGB.clone(), rgb);
        let mut file = std::fs::File::create("./dump").unwrap();
        for i in 360..=830 {
            let v = poly.value(i as f32);
            file.write(format!("{i}-{v}\n").as_bytes());
        }
        file.flush();
    }

    #[test]
    fn test_srgb() {
        let color_space = sRGB.clone();

        let rgb = color_space.xyz_to_rgb(XYZ::new(1.0, 0.0, 0.0));
        assert_abs_diff_eq!(3.2406, rgb.r, epsilon = 0.01);
        assert_abs_diff_eq!(-0.9689, rgb.g, epsilon = 0.01);
        assert_abs_diff_eq!(0.0557, rgb.b, epsilon = 0.01);

        let rgb = color_space.xyz_to_rgb(XYZ::new(0.0, 1.0, 0.0));
        assert_abs_diff_eq!(-1.5372, rgb.r, epsilon = 0.01);
        assert_abs_diff_eq!(1.8758, rgb.g, epsilon = 0.01);
        assert_abs_diff_eq!(-0.2040, rgb.b, epsilon = 0.01);

        let rgb = color_space.xyz_to_rgb(XYZ::new(0.0, 0.0, 1.0));
        assert_abs_diff_eq!(-0.4986, rgb.r, epsilon = 0.01);
        assert_abs_diff_eq!(0.0415, rgb.g, epsilon = 0.01);
        assert_abs_diff_eq!(1.0570, rgb.b, epsilon = 0.01);
    }

    #[test]
    fn test_rgb_xyz_rgb() {
        let color_space = sRGB.clone();
        let rgbs = [
            RGB::WHITE,
            RGB::BLACK,
            RGB::R,
            RGB::G,
            RGB::B,
            RGB::RED,
            RGB::GREEN,
            RGB::LIGHT_BLUE,
        ];
        for rgb in rgbs {
            let xyz = color_space.rgb_to_xyz(rgb);
            let res = color_space.xyz_to_rgb(xyz);
            assert_abs_diff_eq!(res, rgb, epsilon = 1e-3)
        }
    }
}
