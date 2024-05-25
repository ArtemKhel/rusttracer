use std::ops::Deref;

use crate::{
    aggregates::Aabb,
    core::{Hit, Ray},
    math::{
        cross, dot,
        utils::{local_normal, Axis3},
        Cross, Dot, Normed, Number, Point3, Transform, Transformable, Unit, Vec3,
    },
    point3,
    shapes::{Bounded, Intersectable},
    vec3, Point3f, Vec3f,
};

#[derive(Debug)]
pub struct Quad {
    a: Point3f,
    ab: Vec3f,
    ac: Vec3f,
    normal: Unit<Vec3f>,
    d: f32,
    w: Vec3f,
    transform: Transform<f32>,
}

impl Quad {
    const PADDING: f32 = 1e-4;

    pub fn new(a: Point3f, ab: Vec3f, ac: Vec3f, transform: Transform<f32>) -> Self {
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
            transform,
        }
    }

    pub fn quad_box(width: f32, height: f32, depth: f32, transform: Transform<f32>) -> Vec<Quad> {
        let mut sides = Vec::with_capacity(6);

        let a = Point3::new(Vec3::new(-width / 2., -height / 2., -depth / 2.));
        let b = Point3::new(Vec3::new(width / 2., height / 2., depth / 2.));
        let diag = b - a;
        let px = diag.only(Axis3::X);
        let py = diag.only(Axis3::Y);
        let pz = diag.only(Axis3::Z);

        sides.push(Quad::new(a, px, py, transform));
        sides.push(Quad::new(a, px, pz, transform));
        sides.push(Quad::new(a, py, pz, transform));
        sides.push(Quad::new(b, -px, -py, transform));
        sides.push(Quad::new(b, -px, -pz, transform));
        sides.push(Quad::new(b, -py, -pz, transform));

        sides
    }
}

impl Intersectable<f32> for Quad {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let ray = ray.inv_transform(&self.transform);
        // let denom = ray.dir.dot(self.normal);
        let denom = dot(self.normal.deref(), ray.dir.deref());
        if denom.abs() < Self::PADDING {
            return None;
        }

        let t = (self.d - dot(self.normal.deref(), &ray.origin.coords)) / denom;
        if t < 0.0 {
            return None;
        }

        let hit_point = ray.at(t);
        let planar_hit_point = hit_point - self.a;
        let alpha = self.w.dot(&planar_hit_point.cross(self.ab));
        let beta = self.w.dot(&self.ac.cross(planar_hit_point));

        if (0.0..=1.0).contains(&alpha) && (0.0..=1.0).contains(&beta) {
            Some(
                Hit {
                    point: hit_point,
                    normal: local_normal(*self.normal, &ray).to_normal().to_unit(),
                    t,
                }
                .transform(&self.transform),
            )
        } else {
            None
        }
    }
}

impl Bounded<f32> for Quad {
    fn bound(&self) -> Aabb<f32> {
        Aabb::from_points(self.a + self.ab, self.a + self.ac)
            + Aabb::from_points(self.a, self.a + (self.ac + self.ab)).transform(&self.transform)
    }
}
