use std::cmp::Ordering;

use derive_new::new;

use crate::{Dot, Number, Point3, Ray, UnitVec3, Vec3};

#[derive(Debug, Clone, Copy, PartialEq, new)]
pub struct Hit<T: Number> {
    pub point: Point3<T>,
    pub normal: UnitVec3<T>,
    pub t: T,
}

impl<T: Number> Hit<T> {
    pub fn on_front_side(&self, ray: &Ray<T>) -> bool { self.normal.dot(&ray.dir) < T::zero() }
}

impl<T: Number> Eq for Hit<T> {}

impl<T: Number> PartialOrd for Hit<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}
impl<T: Number> Ord for Hit<T> {
    fn cmp(&self, other: &Self) -> Ordering { self.t.partial_cmp(&other.t).unwrap() }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{point3, unit3};

    #[test]
    fn test() {
        let hit = Hit::new(point3!(0., 0., 0.), unit3!(1., 0., 0.), 2.);
        let hit2 = Hit::new(point3!(1., 0., 0.), unit3!(1., 2., 0.), 1.);

        assert!(hit > hit2);
    }
}
