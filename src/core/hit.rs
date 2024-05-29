use std::cmp::Ordering;

use derive_new::new;

use crate::{
    core::ray::Ray,
    math::{dot, Dot, Normal3, Normed, Number, Point3, Transform, Transformable, Unit},
    Vec3f,
};

#[derive(Debug, Clone, Copy, PartialEq, new)]
pub struct Hit<T: Number> {
    pub point: Point3<T>,
    pub normal: Unit<Normal3<T>>,
    pub t: T,
    // pub medium: Option<M>,
    // pub outgoing: Vec3f,
}

impl<T: Number> Hit<T> {
    pub fn on_front_side(&self, ray: &Ray<T>) -> bool { dot(&self.normal, &ray.dir) < T::zero() }
}

impl<T: Number> Eq for Hit<T> {}

impl<T: Number> PartialOrd for Hit<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

impl<T: Number> Ord for Hit<T> {
    fn cmp(&self, other: &Self) -> Ordering { self.t.partial_cmp(&other.t).unwrap() }
}

impl<T: Number> Transformable<T> for Hit<T> {
    fn transform(&self, trans: &Transform<T>) -> Self {
        Hit {
            point: self.point.transform(trans),
            normal: self.normal.transform(trans).to_unit(),
            t: self.t,
        }
    }

    fn inv_transform(&self, trans: &Transform<T>) -> Self {
        // TODO: don't normalize normals?
        Hit {
            point: self.point.inv_transform(trans),
            normal: self.normal.inv_transform(trans).to_unit(),
            t: self.t,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{point3, unit_normal3};

    #[test]
    fn test() {
        let hit = Hit::new(point3!(0., 0., 0.), unit_normal3!(1., 0., 0.), 2.);
        let hit2 = Hit::new(point3!(1., 0., 0.), unit_normal3!(1., 2., 0.), 1.);

        assert!(hit > hit2);
    }
}
