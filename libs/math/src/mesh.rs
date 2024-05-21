use num_traits::Pow;

use crate::{
    aabb::Aabb, local_normal, Bounded, Cross, Dot, Hit, Intersectable, Number, Point3, Ray, Sphere, UnitVec3, Vec3,
};

#[derive(Debug)]
pub struct Triangle<T: Number> {
    a: Point3<T>,
    ab: Vec3<T>,
    ac: Vec3<T>,
    normal: UnitVec3<T>,
    normals: [UnitVec3<T>; 3],
    d: T,
    w: Vec3<T>,
}

impl<T: Number> Triangle<T> {
    const PADDING: f32 = 1e-5;

    pub fn new(a: Point3<T>, ab: Vec3<T>, ac: Vec3<T>) -> Self {
        let n = ab.cross(ac);
        let normal = n.to_unit();
        let d = normal.dot(&a.coords);
        let w = -n / n.len_squared();
        Triangle {
            a,
            ab,
            ac,
            normal,
            normals: [normal, normal, normal],
            d,
            w,
        }
    }

    pub fn new_with_normals(a: Point3<T>, ab: Vec3<T>, ac: Vec3<T>, normals: [UnitVec3<T>; 3]) -> Self {
        let n = ab.cross(ac);
        let normal = n.to_unit();
        let d = normal.dot(&a.coords);
        let w = -n / n.len_squared();
        Triangle {
            a,
            ab,
            ac,
            normal,
            normals,
            d,
            w,
        }
    }
}

impl<T: Number> Intersectable<T> for Triangle<T> {
    fn hit(&self, ray: &Ray<T>) -> Option<Hit<T>> {
        // let denom = ray.dir.dot(self.normal);
        let denom = self.normal.dot(&ray.dir);
        if T::abs(denom) < T::from(Self::PADDING).unwrap() {
            return None;
        }

        let t = (self.d - self.normal.dot(&ray.origin.coords)) / denom;
        if t < T::zero() {
            return None;
        }

        let hit_point = ray.at(t);
        let planar_hit_point = hit_point - self.a;
        let alpha = self.w.dot(&planar_hit_point.cross(self.ab));
        let beta = self.w.dot(&self.ac.cross(planar_hit_point));

        if (T::zero()..=T::one()).contains(&alpha) && (T::zero()..=T::one()).contains(&beta) && alpha + beta <= T::one()
        {
            let an = T::one() - alpha - beta;
            let n = (*self.normals[0] * an + *self.normals[1] * alpha + *self.normals[2] * beta).to_unit();
            Some(Hit {
                point: hit_point,
                normal: local_normal(n, ray),
                t,
            })
        } else {
            None
        }
    }
}

impl<T: Number> Bounded<T> for Triangle<T> {
    fn bound(&self) -> Aabb<T> {
        Aabb::from_points(self.a + self.ab, self.a + self.ac) + Aabb::from_points(self.a, self.a + (self.ac + self.ab))
    }
}

#[cfg(test)]
mod tests {
    use log::debug;
    use obj;

    use super::*;

    #[test]
    fn test() {}
}
