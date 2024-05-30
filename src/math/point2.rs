use derive_more::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use derive_new::new;

use crate::math::vec2::Vec2;

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[derive(Deref, DerefMut)]
#[derive(new, Neg, Add, Sub, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign)]
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
