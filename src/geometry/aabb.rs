use std::{
    mem::swap,
    ops::{Add, AddAssign},
};

use strum::IntoEnumIterator;

use crate::geometry::{utils::Axis, Point, Ray, Vec3};

const PADDING: f32 = 0.000_1;

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq)]
pub struct Aabb {
    pub min: Point,
    pub max: Point,
}

impl Aabb {
    pub fn center(&self) -> Point {
        Point {
            radius_vector: (self.max.radius_vector + self.min.radius_vector) * 0.5,
        }
    }

    pub fn max_dimension(&self) -> Axis {
        let diag = self.max - self.min;
        if diag.x >= diag.y && diag.x >= diag.z {
            Axis::X
        } else if diag.y >= diag.z {
            Axis::Y
        } else {
            Axis::Z
        }
    }

    pub fn from_points(p1: Point, p2: Point) -> Self {
        let mut aabb = Aabb {
            min: Point::min_coords(p1, p2),
            max: Point::max_coords(p1, p2),
        };
        Self::pad(&mut aabb);
        aabb
    }

    fn pad(aabb: &mut Aabb) {
        for axis in Axis::iter() {
            if aabb.max[axis] - aabb.min[axis] < PADDING {
                aabb.min[axis] -= PADDING / 2.;
                aabb.max[axis] += PADDING / 2.;
            }
        }
    }

    pub fn hit(&self, ray: &Ray, min: f32, max: f32) -> bool {
        let mut t_min = min;
        let mut t_max = max;
        for axis in Axis::iter() {
            let inv_dir = ray.dir[axis].recip();
            let mut t0 = (self.min.radius_vector[axis] - ray.origin.radius_vector[axis]) * inv_dir;
            let mut t1 = (self.max.radius_vector[axis] - ray.origin.radius_vector[axis]) * inv_dir;
            if inv_dir < 0. {
                swap(&mut t0, &mut t1)
            }
            t_min = f32::max(t0, t_min);
            t_max = f32::min(t1, t_max);
            if t_min > t_max {
                return false;
            }
        }
        true
    }

    pub fn offset(&self, point: Point) -> Vec3 {
        let mut offset = point - self.min;
        for axis in Axis::iter() {
            let delta = (self.max[axis] - self.min[axis]).max(0.001);
            offset[axis] /= delta;
            debug_assert!((0.0..=1.0).contains(&offset[axis]));
        }
        offset
    }

    pub fn surface_area(&self) -> f32 {
        let diag = self.max - self.min;
        (diag.x * diag.y + diag.x * diag.z + diag.y * diag.z) * 2.0
    }
}

impl Add for Aabb {
    type Output = Aabb;

    fn add(self, rhs: Self) -> Self::Output {
        let min = Point::min_coords(self.min, rhs.min);
        let max = Point::max_coords(self.max, rhs.max);
        Aabb { min, max }
    }
}

impl AddAssign for Aabb {
    fn add_assign(&mut self, rhs: Self) {
        self.min = Point::min_coords(self.min, rhs.min);
        self.max = Point::max_coords(self.max, rhs.max);
    }
}

impl Add<Point> for Aabb {
    type Output = Aabb;

    fn add(self, rhs: Point) -> Self::Output {
        let min = Point::min_coords(self.min, rhs);
        let max = Point::max_coords(self.max, rhs);
        Aabb { min, max }
    }
}

impl Default for Aabb {
    fn default() -> Self {
        let max = f32::MAX;
        let min = f32::MIN;
        Aabb {
            min: Point::new(max, max, max),
            max: Point::new(min, min, min),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::UnitVec;

    #[test]
    fn test_aabb() {
        let bbox = Aabb {
            min: Point::new(1., -1., -1.),
            max: Point::new(2., 1., 1.),
        };

        let ray = Ray::new(Point::default(), UnitVec::new(1., 0., 0.));
        assert!(bbox.hit(&ray, 0., 10.));
        assert!(!bbox.hit(&ray, 0., 0.5));

        let ray = Ray::new(Point::default(), UnitVec::new(1., 2., 0.));
        assert!(!bbox.hit(&ray, 0., 10.));

        let ray = Ray::new(Point::new(0., 1., 0.), UnitVec::new(1., 0., 0.));
        assert!(bbox.hit(&ray, 0., 10.));
    }
}
