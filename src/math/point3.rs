use std::{
    cmp::{max, min},
    fmt::Debug,
    ops::{Add, Index, IndexMut},
};

use approx::AbsDiffEq;
use derive_more::{Deref, DerefMut, Div, From, Mul};
use derive_new::new;
use gen_ops::gen_ops;
use num_traits::{float::FloatCore, Float, One};

use crate::{
    impl_axis_index,
    math::{
        axis::Axis3,
        transform::{Transform, Transformable},
        Dot, Number, Point2, Vec3, Vec4,
    },
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[derive(new, Div, Mul, Deref, DerefMut, From)]
pub struct Point3<T> {
    pub coords: Vec3<T>,
}

#[macro_export]
macro_rules! point3 {
    () => {
        $crate::math::Point3::default()
    };
    ($vec:expr) => {
        $crate::math::Point3 { coords: $vec }
    };
    ($x:expr, $y:expr, $z:expr) => {
        $crate::math::Point3 {
            coords: $crate::math::Vec3 { x: $x, y: $y, z: $z },
        }
    };
}

impl<T: Number> Point3<T> {
    pub fn min_coords(lhs: Point3<T>, rhs: Point3<T>) -> Point3<T> {
        point3!(lhs.x.min(rhs.x), lhs.y.min(rhs.y), lhs.z.min(rhs.z))
    }

    pub fn max_coords(lhs: Point3<T>, rhs: Point3<T>) -> Point3<T> {
        point3!(lhs.x.max(rhs.x), lhs.y.max(rhs.y), lhs.z.max(rhs.z))
    }

    pub fn map<FN>(&self, f: FN) -> Point3<T>
    where FN: FnMut(T) -> T {
        point3!(self.coords.map(f))
    }
}

impl_axis_index!(Point3, Axis3, T, (X, x), (Y, y), (Z, z));

gen_ops!(
    <T>;
    types Point3<T>, Vec3<T> => Point3<T>;

    for + call |a: &Point3<T>, b: &Vec3<T>| {
        Point3{ coords: a.coords + *b }
    };

    where T: Number
);

gen_ops!(
    <T>;
    types Point3<T>, Vec3<T> => Vec3<T>;

    for - call |a: &Point3<T>, b: &Vec3<T>| {
        *a - *b
    };

    where T: Number
);

gen_ops!(
    <T>;
    types Point3<T>, Point3<T> => Vec3<T>;

    for - call |a: &Point3<T>, b: &Point3<T>| {
        a.coords - b.coords
    };

    where T: Number
);

impl<T: Number> From<Point2<T>> for Point3<T> {
    fn from(point: Point2<T>) -> Self {
        Point3 {
            coords: point.coords.into(),
        }
    }
}

impl<T: Float + AbsDiffEq<Epsilon = T>> AbsDiffEq for Point3<T> {
    type Epsilon = T;

    fn default_epsilon() -> Self::Epsilon { T::epsilon() }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool { self.deref().abs_diff_eq(other, epsilon) }
}

impl<T: Number> Transformable<T> for Point3<T> {
    fn transform(&self, trans: &Transform<T>) -> Self {
        let point = Vec4::from(*self);
        let px = point.dot(&trans.mat.x);
        let py = point.dot(&trans.mat.y);
        let pz = point.dot(&trans.mat.z);
        let pw = point.dot(&trans.mat.w);
        if pw == T::one() {
            point3!(px, py, pz)
        } else {
            point3!(px, py, pz) / pw
        }
    }

    fn inv_transform(&self, trans: &Transform<T>) -> Self {
        let point = Vec4::from(*self);
        let px = point.dot(&trans.inv.x);
        let py = point.dot(&trans.inv.y);
        let pz = point.dot(&trans.inv.z);
        let pw = point.dot(&trans.inv.w);
        if pw == T::one() {
            point3!(px, py, pz)
        } else {
            point3!(px, py, pz) / pw
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec3;

    #[test]
    fn test_from_add() {
        let a = point3!(0., 0., 0.);
        let v = vec3!(1., 2., 3.);
        let res = a + v;
        let expected = point3!(1., 2., 3.);
        assert_eq!(res, expected)
    }

    #[test]
    fn test_min() {
        let a = point3!(3., 2., 1.);
        let b = point3!(1., 2., 3.);
        let res = Point3::min_coords(a, b);
        let expected = point3!(1., 2., 1.);
        assert_eq!(res, expected)
    }
}
