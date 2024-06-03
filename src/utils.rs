use std::f32::consts::PI;

use image::{Pixel, Rgb};

use crate::Float;

pub fn lerp(a: Rgb<f32>, b: Rgb<f32>, t: f32) -> Rgb<f32> { a.map2(&b, |a, b| (1. - t) * a + t * b) }

pub(crate) fn linear_to_gamma(linear: Rgb<f32>) -> Rgb<f32> { linear.map(|x| if x > 0. { x.sqrt() } else { x }) }

#[macro_export]
macro_rules! breakpoint {
    () => {
        #[cfg(debug_assertions)]
        unsafe {
            std::intrinsics::breakpoint();
            #[allow(clippy::unused_unit)]
            ();
            ();
        }
    };
    ($expr:expr) => {
        if $expr {
            breakpoint!()
        }
    };
}
