use std::{cmp::max, mem::swap};

use crate::geometry::{Point, Ray};

#[derive(Default, Debug, PartialOrd, PartialEq)]
pub struct AABB {
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

    pub fn hit(&self, ray: &Ray, min: f32, max: f32) -> bool {
        let mut t_min = min;
        let mut t_max = max;
        for axis in 0..3 {
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
