use std::{ops::Deref, process::exit};

use num_traits::Pow;

use crate::{
    aabb::Aabb,
    cross, dot,
    utils::{local_normal, Axis3},
    vec3, Bounded, Cross, Dot, Hit, Intersectable, Normed, Number, Point3, Ray, Triangle, Unit, Vec3,
};

#[derive(Debug)]
pub struct Quad<T: Number> {
    a: Point3<T>,
    ab: Vec3<T>,
    ac: Vec3<T>,
    normal: Unit<Vec3<T>>,
    d: T,
    w: Vec3<T>,
}

impl<T: Number> Quad<T> {
    const PADDING: f32 = 1e-4;

    pub fn new(a: Point3<T>, ab: Vec3<T>, ac: Vec3<T>) -> Self {
        let n = cross(ab, ac);
        let normal = n.to_unit();
        let d = dot(normal.deref(), &a.coords);
        let w = -n / n.len_squared();
        Quad {
            a,
            ab,
            ac,
            normal,
            d,
            w,
        }
    }

    pub fn quad_box(a: Point3<T>, b: Point3<T>) -> Vec<Quad<T>> {
        let mut sides = Vec::with_capacity(6);

        let (a, b) = (Point3::min_coords(a, b), Point3::max_coords(a, b));
        let diag = b - a;
        let px = diag.only(Axis3::X);
        let py = diag.only(Axis3::Y);
        let pz = diag.only(Axis3::Z);

        sides.push(Quad::new(a, px, py));
        sides.push(Quad::new(a, px, pz));
        sides.push(Quad::new(a, py, pz));
        sides.push(Quad::new(b, -px, -py));
        sides.push(Quad::new(b, -px, -pz));
        sides.push(Quad::new(b, -py, -pz));

        sides
    }
}

impl<T: Number> Intersectable<T> for Quad<T> {
    fn hit(&self, ray: &Ray<T>) -> Option<Hit<T>> {
        // let denom = ray.dir.dot(self.normal);
        let denom = dot(self.normal.deref(), ray.dir.deref());
        if T::abs(denom) < T::from(Self::PADDING).unwrap() {
            return None;
        }

        let t = (self.d - dot(self.normal.deref(), &ray.origin.coords)) / denom;
        if t < T::zero() {
            return None;
        }

        let hit_point = ray.at(t);
        let planar_hit_point = hit_point - self.a;
        let alpha = self.w.dot(&planar_hit_point.cross(self.ab));
        let beta = self.w.dot(&self.ac.cross(planar_hit_point));

        if (T::zero()..=T::one()).contains(&alpha) && (T::zero()..=T::one()).contains(&beta) {
            Some(Hit {
                point: hit_point,
                normal: local_normal(*self.normal, ray).to_normal().to_unit(),
                t,
            })
        } else {
            None
        }
    }
}

impl<T: Number> Bounded<T> for Quad<T> {
    fn bound(&self) -> Aabb<T> {
        Aabb::from_points(self.a + self.ab, self.a + self.ac) + Aabb::from_points(self.a, self.a + (self.ac + self.ab))
    }
}
