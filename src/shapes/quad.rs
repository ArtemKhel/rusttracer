use std::ops::Deref;

use crate::{
    aggregates::Aabb,
    core::{Interaction, Ray, SurfaceInteraction},
    math::{axis::Axis3, cross, dot, utils::local_normal, Cross, Dot, Frame, Normed, Transform, Transformable, Unit},
    point3,
    shapes::{Bounded, Intersectable, Samplable, ShapeSample},
    unit_normal3_unchecked, Point2f, Point3f, Vec3f,
};

#[derive(Debug)]
pub struct Quad {
    a: Point3f,
    ab: Vec3f,
    ac: Vec3f,
    normal: Unit<Vec3f>,
    d: f32,
    w: Vec3f,
}

impl Quad {
    const PADDING: f32 = 1e-4;

    pub fn new(a: Point3f, ab: Vec3f, ac: Vec3f, transform: Transform<f32>) -> Self {
        let a = a.transform(&transform);
        let ab = ab.transform(&transform);
        let ac = ac.transform(&transform);

        let n = cross(&ab, &ac);
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

    pub fn quad_box(width: f32, height: f32, depth: f32, transform: Transform<f32>) -> Vec<Quad> {
        let mut sides = Vec::with_capacity(6);

        let a = point3!(-width / 2., -height / 2., -depth / 2.);
        let b = point3!(width / 2., height / 2., depth / 2.);
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

impl Intersectable for Quad {
    fn intersect(&self, ray: &Ray, t_max: f32) -> Option<SurfaceInteraction> {
        let denom = self.normal.dot(&ray.dir);
        if denom.abs() < Self::PADDING {
            return None;
        }

        let t = (self.d - self.normal.dot(&ray.origin.coords)) / denom;
        if t < 0.0 {
            return None;
        }

        let hit_point = ray.at(t);
        let planar_hit_point = hit_point - self.a;
        let alpha = dot(&self.w, &cross(&planar_hit_point, &self.ab));
        let beta = dot(&self.w, &cross(&self.ac, &planar_hit_point));

        if (0.0..=1.0).contains(&alpha) && (0.0..=1.0).contains(&beta) {
            let an = 1.0 - alpha - beta;

            let normal = unit_normal3_unchecked!(local_normal(*self.normal, ray));
            let si = SurfaceInteraction::new(
                Interaction {
                    point: hit_point,
                    normal,
                    t,
                    outgoing: -ray.dir,
                    uv: Point2f::default(),
                },
                self.ab,
                self.ac,
                Default::default(),
                Default::default(),
            );
            Some(si)
        } else {
            None
        }
    }

    fn check_intersect(&self, ray: &Ray, t_max: f32) -> bool {
        let denom = self.normal.dot(&ray.dir);
        if denom.abs() < Self::PADDING {
            return false;
        }

        let t = (self.d - self.normal.dot(&ray.origin.coords)) / denom;
        if t < 0.0 {
            return false;
        }

        let hit_point = ray.at(t);
        let planar_hit_point = hit_point - self.a;
        let alpha = dot(&self.w, &cross(&planar_hit_point, &self.ab));
        let beta = dot(&self.w, &cross(&self.ac, &planar_hit_point));

        if (0.0..=1.0).contains(&alpha) && (0.0..=1.0).contains(&beta) {
            true
        } else {
            false
        }
    }
}

impl Bounded<f32> for Quad {
    fn bound(&self) -> Aabb<f32> {
        Aabb::from_points(self.a + self.ab, self.a + self.ac) + Aabb::from_points(self.a, self.a + (self.ac + self.ab))
    }
}

impl Samplable for Quad {
    fn sample(&self, rnd_p: Point2f) -> Option<ShapeSample> {
        // TODO: uv,pdf
        let point = self.a + rnd_p.x * self.ab + rnd_p.y * self.ac;
        Some(ShapeSample {
            hit: Interaction {
                point,
                normal: self.normal.cast(),
                t: 0.0,
                outgoing: Default::default(),
                uv: Default::default(),
            },
            // pdf: self.area().recip(),
            pdf: 1.,
        })
    }

    fn sample_from_point(&self, point: Point3f, rnd_p: Point2f) -> Option<ShapeSample> {
        // TODO: this
        // let mut sample = self.sample(rnd_p).unwrap();
        // let cos = dot(&((self.a + 0.5 * (self.ab + self.ac)) - point).to_unit(), &self.normal).abs();
        // sample.pdf /= cos;
        // Some(sample)
        self.sample(rnd_p)
    }

    fn pdf(&self, interaction: &Interaction) -> f32 { todo!() }

    fn pdf_incoming(&self, interaction: &SurfaceInteraction, incoming: Unit<Vec3f>) -> f32 {
        // TODO: !!!
        let dir = interaction.hit.point - self.a;
        let cos = dot(&incoming, &dir).abs();
        let dist = dir.len_squared();
        self.area() * cos / dist
    }

    fn area(&self) -> f32 { cross(&self.ab, &self.ac).len() }
}
