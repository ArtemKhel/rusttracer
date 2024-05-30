use std::ops::{Index, IndexMut};

use derive_more::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};
use derive_new::new;
use gen_ops::gen_ops;

use crate::{
    impl_axis_index,
    math::{axis::Axis2, vec2::Vec2, Number, Point3},
    point3,
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[derive(Deref, DerefMut)]
#[derive(new, Neg, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign)]
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
impl<T: Number> Point2<T> {
    pub fn min_coords(lhs: Point2<T>, rhs: Point2<T>) -> Point2<T> { point2!(lhs.x.min(rhs.x), lhs.y.min(rhs.y)) }

    pub fn max_coords(lhs: Point2<T>, rhs: Point2<T>) -> Point2<T> { point2!(lhs.x.max(rhs.x), lhs.y.max(rhs.y)) }
}

impl_axis_index!(Point2, Axis2, T, (X, x), (Y, y));

gen_ops!(
    <T>;
    types Point2<T>, Vec2<T> => Point2<T>;

    for + call |a: &Point2<T>, b: &Vec2<T>| {
        Point2{ coords: a.coords + *b }
    };

    where T: Number
);

gen_ops!(
    <T>;
    types Point2<T>, Vec2<T> => Vec2<T>;

    for - call |a: &Point2<T>, b: &Vec2<T>| {
        *a - *b
    };

    where T: Number
);

gen_ops!(
    <T>;
    types Point2<T>, Point2<T> => Vec2<T>;

    for - call |a: &Point2<T>, b: &Point2<T>| {
        a.coords - b.coords
    };

    where T: Number
);
