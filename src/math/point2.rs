use std::ops::{Index, IndexMut};

use approx::AbsDiffEq;
use derive_more::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, From, Mul, MulAssign, Neg, Sub, SubAssign};
use derive_new::new;
use gen_ops::gen_ops;
use num_traits::Float;
use rand::{
    distributions::{uniform::SampleUniform, Standard},
    prelude::Distribution,
    Rng,
};

use crate::{
    impl_axis_index,
    math::{axis::Axis2, vec2::Vec2, Number, Point3},
    point3,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, Hash)]
#[derive(Deref, DerefMut, From)]
#[derive(Neg, Mul, Div, AddAssign, SubAssign, MulAssign, DivAssign)]
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
    pub fn new(x: T, y: T) -> Point2<T> {
        Point2 {
            coords: Vec2::new(x, y),
        }
    }

    pub fn min_coords(lhs: Point2<T>, rhs: Point2<T>) -> Point2<T> { point2!(lhs.x.min(rhs.x), lhs.y.min(rhs.y)) }

    pub fn max_coords(lhs: Point2<T>, rhs: Point2<T>) -> Point2<T> { point2!(lhs.x.max(rhs.x), lhs.y.max(rhs.y)) }
}

impl<T: Copy> Point2<T> {
    pub fn map<F, Out>(&self, f: F) -> Point2<Out>
    where F: FnMut(T) -> Out {
        Point2::from(self.coords.map(f))
    }
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

impl<T: Number + SampleUniform> Distribution<Point2<T>> for Standard
where Standard: Distribution<T>
{
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point2<T> { Point2::from(rng.gen::<Vec2<T>>()) }
}
impl<T: Float + AbsDiffEq<Epsilon = T>> AbsDiffEq for Point2<T> {
    type Epsilon = T;

    fn default_epsilon() -> Self::Epsilon { T::epsilon() }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool { self.deref().abs_diff_eq(other, epsilon) }
}
