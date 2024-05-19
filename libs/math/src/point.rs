use std::ops::{Deref, Index, IndexMut};

use derive_new::new;
use gen_ops::{gen_ops, gen_ops_comm};
use num_traits::{float::FloatCore, Float};

use crate::{unit_vec::UnitVec3, utils::Axis, Number, Vec3};

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, new)]
pub struct Point3<T: Number> {
    pub coords: Vec3<T>,
}

pub type Point3f = Point3<f32>;

#[macro_export]
macro_rules! point3 {
    () => {
        Point3::default()
    };
    ($x:expr, $y:expr, $z:expr) => {
        Point3 {
            coords: Vec3 { x: $x, y: $y, z: $z },
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

impl<T: Number> Index<Axis> for Point3<T> {
    type Output = T;

    fn index(&self, index: Axis) -> &Self::Output {
        match index {
            Axis::X => &self.coords.x,
            Axis::Y => &self.coords.y,
            Axis::Z => &self.coords.z,
        }
    }
}

impl<T: Number> IndexMut<Axis> for Point3<T> {
    fn index_mut(&mut self, index: Axis) -> &mut Self::Output {
        match index {
            Axis::X => &mut self.coords.x,
            Axis::Y => &mut self.coords.x,
            Axis::Z => &mut self.coords.z,
        }
    }
}
// impl<T: Number> Deref for Point<T> {
//     type Target = Vec3<T>;
//
//     fn deref(&self) -> &Self::Target { &self.coords }
// }

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
