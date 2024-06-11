use std::sync::Arc;

use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use derive_new::new;
use num_traits::Signed;

use crate::{
    math::Matrix3,
    Point2f,
    spectra::{SpectrumEnum, xyz::XYZ}, Vec3f,
};
use crate::spectra::{LAMBDA_MAX, LAMBDA_MIN};
use crate::spectra::rgb2spec::Gamut;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
#[derive(new)]
#[derive(Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign)]
pub struct RGB {
    r: f32,
    g: f32,
    b: f32,
}

impl RGB {
    pub fn has_nan(&self) -> bool { self.r.is_nan() || self.g.is_nan() || self.b.is_nan() }
    pub fn max(&self) -> f32 {self.r.max(self.g.max(self.b))}
}

impl From<RGB> for Vec3f {
    fn from(value: RGB) -> Self { Vec3f::new(value.r, value.g, value.b) }
}

impl From<Vec3f> for RGB {
    fn from(value: Vec3f) -> Self { RGB::new(value.x, value.y, value.z) }
}

impl From<RGB> for [f32; 3] { fn from(value: RGB) -> Self { [value.r, value.g, value.b] } }


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
        let s = lambda.mul_add(lambda.mul_add(self.c2, self.c1), self.c0);
        match s {
            f32::INFINITY => 1.,
            f32::NEG_INFINITY => 0.,
            _ => 0.5 + s / (2. * (1. + s.powi(2)).sqrt())
        }
    }
    pub fn max_value(&self) -> f32 {
        let result = f32::max(self.eval(LAMBDA_MIN), self.eval(LAMBDA_MAX));
        let lambda = -self.c1 / (2.0 * self.c0);
        if lambda >= LAMBDA_MIN && lambda <= LAMBDA_MAX {
            return f32::max(result, self.eval(lambda));
        } else {
            result
        }
    }
}


pub struct RGBColorSpace {
    r: Point2f,
    g: Point2f,
    b: Point2f,
    whitepoint: Point2f,
    illuminant: Arc<SpectrumEnum>,
    rgb_to_xyz: Matrix3<f32>,
    xyz_to_rgb: Matrix3<f32>,
    gamut: Gamut,
}

impl RGBColorSpace {
    #[allow(non_snake_case)]
    pub fn new(
        r: Point2f,
        g: Point2f,
        b: Point2f,
        illuminant: Arc<SpectrumEnum>,
        gamut: Gamut,
    ) -> Self {
        let W = XYZ::from(illuminant.as_ref());
        let whitepoint = W.xy();
        let xyz_r = XYZ::from_xy(r);
        let xyz_g = XYZ::from_xy(g);
        let xyz_b = XYZ::from_xy(b);

        let rgb = Matrix3::from_elements(
            xyz_r.x, xyz_g.x, xyz_b.x, xyz_r.y, xyz_g.y, xyz_b.y, xyz_r.z, xyz_g.z, xyz_b.z,
        );

        let C = rgb.invert().unwrap() * Vec3f::from(W);
        let rgb_to_xyz = rgb * Matrix3::diag(C.x, C.y, C.z);
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

    pub fn rgb_to_xyz(&self, rgb: RGB) -> XYZ { XYZ::from(self.xyz_to_rgb * Vec3f::from(rgb)) }
}
