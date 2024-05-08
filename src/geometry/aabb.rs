use std::{mem::swap, ops::Add};

use strum::IntoEnumIterator;

use crate::geometry::{utils::Axis, Point, Ray};

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
        Aabb {
            min: Point::min_coords(p1, p2),
            max: Point::max_coords(p1, p2),
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
            if t_min >= t_max {
                return false;
            }
        }
        true
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
