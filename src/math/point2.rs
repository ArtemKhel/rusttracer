use derive_more::{Deref, Div, Mul};
use derive_new::new;

use crate::math::vec2::Vec2;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[derive(new, Div, Mul, Deref)] // Deref
pub struct Point2<T> {
    pub coords: Vec2<T>,
}
#[macro_export]
macro_rules! point2 {
    () => {
        $crate::math::Point2::default()
    };
    ($x:expr, $y:expr) => {
        $crate::math::Point2 {
            coords: $crate::math::Vec2 { x: $x, y: $y },
        }
    };
}
