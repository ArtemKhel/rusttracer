use std::sync::Arc;
use derive_more::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use derive_new::new;
use rgb2spec::RGB2Spec;

use crate::{
    math::Matrix3
    ,
    Point2f,
    spectra::SpectrumEnum, Vec3f,
};
use crate::spectra::xyz::XYZ;

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
}

impl From<RGB> for Vec3f {
    fn from(value: RGB) -> Self { Vec3f::new(value.r, value.g, value.b) }
}

impl From<Vec3f> for RGB {
    fn from(value: Vec3f) -> Self { RGB::new(value.x, value.y, value.z) }
}

pub struct RGBColorSpace {
    r: Point2f,
    g: Point2f,
    b: Point2f,
    whitepoint: Point2f,
    illuminant: Arc<SpectrumEnum>,
    rgb_to_xyz: Matrix3<f32>,
    xyz_to_rgb: Matrix3<f32>,
    // rgb2spec: RGB2Spec,
}

impl RGBColorSpace {
    #[allow(non_snake_case)]
    pub fn new(r: Point2f, g: Point2f, b: Point2f, illuminant: Arc<SpectrumEnum>/*, rgb2spec: RGB2Spec*/) -> Self {
        let W = XYZ::from(illuminant.as_ref());
        let whitepoint = W.xy();
        let xyz_r = XYZ::from_xy(r);
        let xyz_g = XYZ::from_xy(g);
        let xyz_b = XYZ::from_xy(b);

        let rgb = Matrix3::from_elements(xyz_r.x, xyz_g.x, xyz_b.x, xyz_r.y, xyz_g.y, xyz_b.y, xyz_r.z, xyz_g.z, xyz_b.z);

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
            // rgb2spec,
        }
    }

    pub fn xyz_to_rgb(&self, xyz: XYZ) -> RGB { RGB::from(self.xyz_to_rgb * Vec3f::from(xyz)) }

    pub fn rgb_to_xyz(&self, rgb: RGB) -> XYZ { XYZ::from(self.xyz_to_rgb * Vec3f::from(rgb)) }
}
