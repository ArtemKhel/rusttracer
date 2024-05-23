use std::{fmt::Debug, ops::Deref};

use derive_new::new;
use num_traits::Pow;

use crate::{
    aggregates::Aabb,
    core::{Hit, Ray},
    math::{dot, Dot, Normal3, Normed, Number, Point3, Transform, Transformable, Unit},
    shapes::{Bounded, Intersectable},
    vec3,
};

#[derive(Default, Debug, Clone, Copy, new)]
pub struct Sphere<T: Number> {
    // pub center: Point3<T>,
    pub radius: T,
    pub transform: Transform<T>,
}

impl<T: Number> Sphere<T> {
    pub fn normal(&self, point: Point3<T>) -> Unit<Normal3<T>> { point.coords.to_normal().to_unit() }
}

impl<T: Number> Intersectable<T> for Sphere<T> {
    fn hit(&self, ray: &Ray<T>) -> Option<Hit<T>> {
        let ray = ray.inv_transform(&self.transform);
        let o = ray.origin.coords;
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
            Hit::new(point, self.normal(point), root).transform(&self.transform)
        })
    }
}

impl<T: Number> Bounded<T> for Sphere<T> {
    fn bound(&self) -> Aabb<T> {
        let vec = vec3!(self.radius);
        Aabb::from_points(Point3::from(vec), Point3::from(-vec)).transform(&self.transform)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point3;

    #[test]
    fn test_aabb() {
        let sphere = Sphere::new(1.0, Transform::id());
        let aabb = sphere.bound();
        let expected = Aabb::from_points(point3!(1., 1., 1.), point3!(-1., -1., -1.));

        assert_eq!(aabb, expected)
    }

    #[test]
    fn test_aabb_translated() {
        let sphere = Sphere::new(1.0, Transform::translate(vec3!(1., 2., 3.)));
        let aabb = sphere.bound();
        let expected = Aabb::from_points(point3!(0., 1., 2.), point3!(2., 3., 4.));

        assert_eq!(aabb, expected)
    }

    #[test]
    fn test_aabb_translated_scaled() {
        let sphere = Sphere::new(
            1.0,
            Transform::compose(Transform::translate(vec3!(1., 2., 3.)), Transform::scale(1., 1., 2.)),
        );
        let aabb = sphere.bound();
        let expected = Aabb::from_points(point3!(0., 1., 1.), point3!(2., 3., 5.));

        assert_eq!(aabb, expected)
    }
}
