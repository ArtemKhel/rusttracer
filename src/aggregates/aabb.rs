use std::{
    mem::swap,
    ops::{Add, AddAssign, Index, Not},
};

use approx::AbsDiffEq;
use derive_new::new;
use num_traits::{Float, Signed};
use strum::IntoEnumIterator;

use crate::{
    aggregates::Bounded,
    core::ray::Ray,
    math::{axis::Axis3, Number, Point3, Transform, Transformable, Vec3},
    point3,
};

#[derive(Copy, Clone, Debug, PartialEq)]
#[derive(new)]
pub struct Aabb<T> {
    pub min: Point3<T>,
    pub max: Point3<T>,
}

impl<T: Number> Aabb<T> {
    pub fn center(&self) -> Point3<T> { Point3::from(*self.min + (self.max - self.min) / (T::one() + T::one())) }

    pub fn max_dimension(&self) -> Axis3 {
        let diag = self.max - self.min;
        if diag.x >= diag.y && diag.x >= diag.z {
            Axis3::X
        } else if diag.y >= diag.z {
            Axis3::Y
        } else {
            Axis3::Z
        }
    }

    pub fn from_points(p1: Point3<T>, p2: Point3<T>) -> Self {
        let mut aabb = Aabb::new(Point3::min_coords(p1, p2), Point3::max_coords(p1, p2));
        aabb
    }

    pub fn union(&self, p: Point3<T>) -> Self {
        Aabb::new(Point3::min_coords(self.min, p), Point3::max_coords(self.max, p))
    }

    /// Offset of a point (in local coordinates) within this AABB
    pub fn offset(&self, point: Point3<T>) -> Vec3<T> {
        // let padding = T::from(Self::PADDING).unwrap();
        let mut offset = point - self.min;
        for axis in Axis3::iter() {
            let delta = self.max[axis] - self.min[axis];
            if delta == T::zero() {
                offset[axis] = T::zero();
            } else {
                offset[axis] /= delta;
            }
            debug_assert!((T::zero()..=T::one()).contains(&offset[axis]));
        }
        offset
    }

    pub fn hit(&self, ray: &Ray<T>, mut t_max: T) -> bool {
        let mut t_min = T::zero();
        for axis in Axis3::iter() {
            let inv_dir = ray.dir[axis].recip();
            let mut t0 = (self.min[axis] - ray.origin[axis]) * inv_dir;
            let mut t1 = (self.max[axis] - ray.origin[axis]) * inv_dir;
            if inv_dir < T::zero() {
                swap(&mut t0, &mut t1)
            }
            t_min = T::max(t0, t_min);
            t_max = T::min(t1, t_max);
            if t_min > t_max {
                return false;
            }
        }
        true
    }

    pub fn hit_fast(&self, ray: &Ray<T>, inv_dir: Vec3<T>, inv_bounds: Vec3<Sign>, ray_t_max: T) -> bool {
        let mut t_min = T::neg_infinity();
        let mut t_max = T::infinity();
        // CALLS.fetch_add(1, Relaxed);
        for axis in Axis3::iter() {
            let t0 = (self[inv_bounds[axis]][axis] - ray.origin[axis]) * inv_dir[axis];
            let t1 = (self[!inv_bounds[axis]][axis] - ray.origin[axis]) * inv_dir[axis];
            if t_min > t1 || t0 > t_max {
                // SKIP.fetch_add(1, Relaxed);
                return false;
            }
            t_min = T::max(t0, t_min);
            t_max = T::min(t1, t_max);
        }
        // if t_min > ray_t_max || t_max < T::zero() { SKIP.fetch_add(1, Relaxed); }
        t_min < ray_t_max && t_max > T::zero()
    }

    pub fn surface_area(&self) -> T {
        let diag = self.max - self.min;
        (diag.x * diag.y + diag.x * diag.z + diag.y * diag.z) * T::from(2).unwrap()
    }

    fn corners(&self) -> [Point3<T>; 8] {
        let diag = self.max - self.min;
        use Axis3::*;
        [
            self.min,
            self.min + diag.only(X),
            self.min + diag.only(Y),
            self.min + diag.only(Z),
            self.max,
            self.max + -diag.only(X),
            self.max + -diag.only(X),
            self.max + -diag.only(X),
        ]
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Sign {
    Neg,
    Pos,
}

impl From<f32> for Sign {
    fn from(value: f32) -> Self {
        if value.is_negative() {
            Sign::Neg
        } else {
            Sign::Pos
        }
    }
}

impl Not for Sign {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Sign::Neg => Sign::Pos,
            Sign::Pos => Sign::Neg,
        }
    }
}

impl<T: Number> Index<Sign> for Aabb<T> {
    type Output = Point3<T>;

    fn index(&self, index: Sign) -> &Self::Output {
        match index {
            Sign::Pos => &self.min,
            Sign::Neg => &self.max,
        }
    }
}

impl<T: Number> Bounded<T> for Aabb<T> {
    fn bound(&self) -> Aabb<T> { *self }
}

impl<T: Number, B: Bounded<T>> Add<B> for Aabb<T> {
    type Output = Aabb<T>;

    fn add(self, rhs: B) -> Self::Output {
        let rhs = rhs.bound();
        let min = Point3::min_coords(self.min, rhs.min);
        let max = Point3::max_coords(self.max, rhs.max);
        Aabb::new(min, max)
    }
}

impl<T: Number> AddAssign for Aabb<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.min = Point3::min_coords(self.min, rhs.min);
        self.max = Point3::max_coords(self.max, rhs.max);
    }
}

impl<T: Number> Default for Aabb<T> {
    fn default() -> Self {
        let max = T::max_value();
        let min = T::min_value();
        Aabb::new(point3!(max, max, max), point3!(min, min, min))
    }
}

impl<T: Number> Transformable<T> for Aabb<T> {
    fn transform(&self, trans: &Transform<T>) -> Self {
        self.corners()
            .iter()
            .map(|x| x.transform(trans))
            .fold(Aabb::default(), |aabb, x| aabb.union(x))
    }

    fn inv_transform(&self, trans: &Transform<T>) -> Self {
        self.corners()
            .iter()
            .map(|x| x.inv_transform(trans))
            .fold(Aabb::default(), |aabb, x| aabb.union(x))
    }
}

impl<T: Number + AbsDiffEq<Epsilon = T>> AbsDiffEq for Aabb<T> {
    type Epsilon = T;

    fn default_epsilon() -> Self::Epsilon { T::epsilon() }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.min.abs_diff_eq(&other.min, epsilon) && self.max.abs_diff_eq(&other.max, epsilon)
    }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::{FRAC_PI_4, SQRT_2};

    use approx::assert_abs_diff_eq;

    use super::*;
    use crate::{ray, unit3};

    #[test]
    fn test_aabb() {
        let bbox = Aabb::new(point3!(1., -1., -1.), point3!(2., 1., 1.));

        let ray = ray!(Point3::default(), unit3!(1., 0., 0.));
        assert!(bbox.hit(&ray, 10.));
        assert!(!bbox.hit(&ray, 0.5));

        let ray = ray!(Point3::default(), unit3!(1., 2., 0.));
        assert!(!bbox.hit(&ray, 10.));

        let ray = ray!(point3!(0., 1., 0.), unit3!(1., 0., 0.));
        assert!(bbox.hit(&ray, 10.));
    }

    #[test]
    fn test_rotate() {
        let bbox = Aabb::new(point3!(0., 0., 0.), point3!(1., 2., 3.));
        let t = Transform::rotate(Axis3::Z, -FRAC_PI_4);
        let expected = Aabb::new(point3!(0., -SQRT_2 / 2., 0.), point3!(SQRT_2 + SQRT_2 / 2., SQRT_2, 3.));

        assert_abs_diff_eq!(bbox.transform(&t), expected)
    }
}
