use std::f32::consts::PI;

use image::{Pixel, Rgb};

pub fn lerp(a: Rgb<f32>, b: Rgb<f32>, t: f32) -> Rgb<f32> {
    Rgb([
        (1. - t) * a.0[0] + t * b.0[0],
        (1. - t) * a.0[1] + t * b.0[1],
        (1. - t) * a.0[2] + t * b.0[2],
    ])
}

pub(crate) fn linear_to_gamma(linear: Rgb<f32>) -> Rgb<f32> { linear.map(|x| if x > 0. { x.sqrt() } else { x }) }

pub(crate) fn degrees_to_radians(degrees: f32) -> f32 { PI * degrees / 180.0 }

#[macro_export]
macro_rules! breakpoint {
    () => {
        unsafe {
            std::intrinsics::breakpoint();
            #[allow(clippy::unused_unit)]
            ()
        }
    };
    ($expr:expr) => {
        if $expr {
            breakpoint!()
        }
    };
}
