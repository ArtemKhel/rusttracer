use std::{
    mem::swap,
    ops::{Add, AddAssign},
};

use num_traits::{real::Real, Bounded};
use strum::IntoEnumIterator;

use crate::{point3, utils::Axis3, Number, Point3, Ray, Vec3};

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Aabb<T: Number> {
    pub min: Point3<T>,
    pub max: Point3<T>,
}

impl<T: Number> Aabb<T> {
    // TODO:
    const PADDING: f32 = 1e-5;

    pub fn center(&self) -> Point3<T> {
        Point3::new(self.min.coords + (self.max.coords - self.min.coords) / T::from(2).unwrap())
    }

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
        let mut aabb = Aabb {
            min: Point3::min_coords(p1, p2),
            max: Point3::max_coords(p1, p2),
        };
        Self::pad(&mut aabb);
        aabb
    }

    fn pad(aabb: &mut Aabb<T>) {
        let padding = T::from(Self::PADDING).unwrap();
        for axis in Axis3::iter() {
            if aabb.max[axis] - aabb.min[axis] < padding {
                aabb.min[axis] -= padding;
                aabb.max[axis] += padding;
            }
        }
    }

    pub fn hit(&self, ray: &Ray<T>, min: T, max: T) -> bool {
        let mut t_min = min;
        let mut t_max = max;
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

    pub fn offset(&self, point: Point3<T>) -> Vec3<T> {
        let padding = T::from(Self::PADDING).unwrap();
        let mut offset = point - self.min;
        for axis in Axis3::iter() {
            let delta = (self.max[axis] - self.min[axis]).max(padding);
            offset[axis] /= delta;
            debug_assert!((T::zero()..=T::one()).contains(&offset[axis]));
        }
        offset
    }

    pub fn surface_area(&self) -> T {
        let diag = self.max - self.min;
        (diag.x * diag.y + diag.x * diag.z + diag.y * diag.z) * T::from(2).unwrap()
    }
}

impl<T: Number> Add for Aabb<T> {
    type Output = Aabb<T>;

    fn add(self, rhs: Self) -> Self::Output {
        let min = Point3::min_coords(self.min, rhs.min);
        let max = Point3::max_coords(self.max, rhs.max);
        Aabb { min, max }
    }
}

impl<T: Number> AddAssign for Aabb<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.min = Point3::min_coords(self.min, rhs.min);
        self.max = Point3::max_coords(self.max, rhs.max);
    }
}

impl<T: Number> Add<Point3<T>> for Aabb<T> {
    type Output = Aabb<T>;

    fn add(self, rhs: Point3<T>) -> Self::Output {
        let min = Point3::min_coords(self.min, rhs);
        let max = Point3::max_coords(self.max, rhs);
        Aabb { min, max }
    }
}

impl<T: Number> Default for Aabb<T> {
    fn default() -> Self {
        let max = T::max_value();
        let min = T::min_value();
        Aabb {
            min: point3!(max, max, max),
            max: point3!(min, min, min),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::unit3;

    #[test]
    fn test_aabb() {
        let bbox = Aabb {
            min: point3!(1., -1., -1.),
            max: point3!(2., 1., 1.),
        };

        let ray = Ray::new(Point3::default(), unit3!(1., 0., 0.));
        assert!(bbox.hit(&ray, 0., 10.));
        assert!(!bbox.hit(&ray, 0., 0.5));

        let ray = Ray::new(Point3::default(), unit3!(1., 2., 0.));
        assert!(!bbox.hit(&ray, 0., 10.));

        let ray = Ray::new(point3!(0., 1., 0.), unit3!(1., 0., 0.));
        assert!(bbox.hit(&ray, 0., 10.));
    }
}
