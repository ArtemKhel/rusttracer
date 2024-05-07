use std::{cmp::max, mem::swap, ops::Add};

use strum::IntoEnumIterator;

use crate::geometry::{utils::Axis, Point, Ray};

#[derive(Copy, Clone, Default, Debug, PartialOrd, PartialEq)]
pub struct AABB {
    pub min: Point,
    pub max: Point,
}

impl AABB {
    pub fn from_points(p1: Point, p2: Point) -> Self {
        let min = Point::new(
            p1.radius_vector.x.min(p2.radius_vector.x),
            p1.radius_vector.y.min(p2.radius_vector.y),
            p1.radius_vector.z.min(p2.radius_vector.z),
        );
        let max = Point::new(
            p1.radius_vector.x.max(p2.radius_vector.x),
            p1.radius_vector.y.max(p2.radius_vector.y),
            p1.radius_vector.z.max(p2.radius_vector.z),
        );
        AABB { min, max }
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
        return true;
    }
}

impl Add for AABB {
    type Output = AABB;

    fn add(self, rhs: Self) -> Self::Output {
        let min = Point::new(
            self.min.radius_vector.x.min(rhs.min.radius_vector.x),
            self.min.radius_vector.y.min(rhs.min.radius_vector.y),
            self.min.radius_vector.z.min(rhs.min.radius_vector.z),
        );
        let max = Point::new(
            self.max.radius_vector.x.max(rhs.max.radius_vector.x),
            self.max.radius_vector.y.max(rhs.max.radius_vector.y),
            self.max.radius_vector.z.max(rhs.max.radius_vector.z),
        );
        AABB { min, max }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::geometry::UnitVec;

    #[test]
    fn test_aabb() {
        let bbox = AABB {
            min: Point::new(1., -1., -1.),
            max: Point::new(2., 1., 1.),
        };

        let ray = Ray::new(Point::default(), UnitVec::new(1., 0., 0.));
        assert!(bbox.hit(&ray, 0., 10.,));
        assert!(!bbox.hit(&ray, 0., 0.5));

        let ray = Ray::new(Point::default(), UnitVec::new(1., 2., 0.));
        assert!(!bbox.hit(&ray, 0., 10.,));

        let ray = Ray::new(Point::new(0., 1., 0.), UnitVec::new(1., 0., 0.));
        assert!(bbox.hit(&ray, 0., 10.,));
    }
}
