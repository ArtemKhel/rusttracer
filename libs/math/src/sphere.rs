use std::{
    fmt::{Debug, Formatter},
    ops::Deref,
};

use num_traits::Pow;

use crate::{
    aabb::Aabb, dot, vec3, Bounded, BoundedIntersectable, Dot, Hit, Intersectable, Number, Point3, Ray, Shape,
    UnitVec3, Vec3,
};

#[derive(Default, Debug, Clone, Copy)]
pub struct Sphere<T: Number> {
    pub center: Point3<T>,
    pub radius: T,
}
impl<T: Number> Sphere<T> {
    pub fn new(center: Point3<T>, radius: T) -> Sphere<T> { Sphere { center, radius } }

    pub fn normal(&self, point: Point3<T>) -> UnitVec3<T> { (point - self.center).to_unit() }
}

impl<T: Number> Intersectable<T> for Sphere<T> {
    fn hit(&self, ray: &Ray<T>) -> Option<Hit<T>> {
        let o = ray.origin - self.center;
        let h = dot(ray.dir.deref(), &o);
        let c = o.len_squared() - self.radius.powi(2);
        let disc = h.powi(2) - c;
        if disc < T::zero() {
            return None;
        }

        let disc_sqrt = disc.sqrt();
        // Find the nearest root that lies in the acceptable range.
        let root = {
            let r = -h - disc_sqrt;
            let r2 = -h + disc_sqrt;
            if r >= T::zero() {
                Some(r)
            } else if r2 >= T::zero() {
                Some(r2)
            } else {
                None
            }
        };

        root.map(|root| {
            let point = ray.at(root);
            Hit::new(point, self.normal(point), root)
        })
    }
}

impl<T: Number> Bounded<T> for Sphere<T> {
    fn bound(&self) -> Aabb<T> {
        let vec = vec3!(self.radius);
        Aabb::from_points(Point3::from(self.center - vec), self.center + vec)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{point3, BoundedIntersectable};

    #[test]
    fn test_aabb() {
        let sphere = Sphere::new(Point3::default(), 1.0);
        let aabb = sphere.bound();
        let expected = Aabb::from_points(point3!(1., 1., 1.), point3!(-1., -1., -1.));

        assert_eq!(aabb, expected)
    }

    #[test]
    fn test() {
        let sphere = Sphere::new(Point3::default(), 1.0_f32);
        let boxed = Box::new(sphere);
        let boxed_dyn: Box<dyn BoundedIntersectable<_>> = Box::new(sphere);
    }
}
