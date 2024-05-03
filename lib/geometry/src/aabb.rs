use std::mem::swap;

use crate::{Point, Ray};

pub struct Interval {
    min: f32,
    max: f32,
}

#[derive(Default, Debug, PartialOrd, PartialEq)]
pub struct AABB {
    // bound: Bound
    min: Point,
    max: Point,
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

    pub fn hit(&self, ray: &Ray, interval: Interval) -> bool {
        let mut t_min = interval.min;
        let mut t_max = interval.max;
        for axis in 0..3 {
            let inv_dir = ray.dir[axis].recip();
            let mut t0 = (self.min.radius_vector[axis] - ray.origin.radius_vector[axis]) * inv_dir;
            let mut t1 = (self.max.radius_vector[axis] - ray.origin.radius_vector[axis]) * inv_dir;
            if inv_dir < 0. {
                swap(&mut t0, &mut t1)
            }
            t_min = if t0 > t_min { t0 } else { t_min };
            t_max = if t0 < t_max { t1 } else { t_max };
            if t_min >= t_max {
                return false;
            }
        }
        return true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::UnitVec;

    #[test]
    fn test_aabb() {
        let bbox = AABB {
            min: Point::new(1., -1., -1.),
            max: Point::new(2., 1., 1.),
        };

        let ray = Ray::new(Point::default(), UnitVec::new(1., 0., 0.));
        let interval = Interval { min: 0.0, max: 10.0 };
        assert!(bbox.hit(&ray, interval));

        let interval = Interval { min: 0.0, max: 0.5 };
        assert!(!bbox.hit(&ray, interval));

        let ray = Ray::new(Point::default(), UnitVec::new(1., 2., 0.));
        let interval = Interval { min: 0.0, max: 10.0 };
        assert!(!bbox.hit(&ray, interval));
    }
}
