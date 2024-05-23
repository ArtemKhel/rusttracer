use std::ops::{Index, IndexMut};

use approx::AbsDiffEq;
use derive_more::{Deref, Div, Mul};
use derive_new::new;
use gen_ops::gen_ops;
use num_traits::{float::FloatCore, Float, One};

use crate::math::{
    transform::{Transform, Transformable},
    utils::Axis3,
    Dot, Number, Vec3, Vec4,
};

#[derive(Debug, Default, Clone, Copy, PartialEq, new, Div, Mul)] // Deref
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
        point3!(
            lhs.coords.x.min(rhs.coords.x),
            lhs.coords.y.min(rhs.coords.y),
            lhs.coords.z.min(rhs.coords.z)
        )
    }

    pub fn max_coords(lhs: Point3<T>, rhs: Point3<T>) -> Point3<T> {
        point3!(
            lhs.coords.x.max(rhs.coords.x),
            lhs.coords.y.max(rhs.coords.y),
            lhs.coords.z.max(rhs.coords.z)
        )
    }
}

impl<T: Number> Index<Axis3> for Point3<T> {
    type Output = T;

    fn index(&self, index: Axis3) -> &Self::Output {
        match index {
            Axis3::X => &self.coords.x,
            Axis3::Y => &self.coords.y,
            Axis3::Z => &self.coords.z,
        }
    }
}

impl<T: Number> IndexMut<Axis3> for Point3<T> {
    fn index_mut(&mut self, index: Axis3) -> &mut Self::Output {
        match index {
            Axis3::X => &mut self.coords.x,
            Axis3::Y => &mut self.coords.x,
            Axis3::Z => &mut self.coords.z,
        }
    }
}

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
        a.coords - *b
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

impl<T: Number> From<Vec3<T>> for Point3<T> {
    fn from(coords: Vec3<T>) -> Self { Point3 { coords } }
}

impl<T: Float + AbsDiffEq<Epsilon = T>> AbsDiffEq for Point3<T> {
    type Epsilon = T;

    fn default_epsilon() -> Self::Epsilon { T::epsilon() }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.coords.abs_diff_eq(&other.coords, epsilon)
    }
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

// gen_ops!(
//     <T>;
//     types Point3<T>, T => Point3<T>;
//
//     for * call |a: &Point3<T>, b: &T| {
//         point3!(a.coords * *b)
//     };
//
//     for / call |a: &Point3<T>, b: &T| {
//         point3!(a.coords / *b)
//     };
//
//     where T:Number
// );

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
