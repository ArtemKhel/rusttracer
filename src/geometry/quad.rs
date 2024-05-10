use crate::geometry::{Aabb, Bounded, Cross, Dot, Hit, Intersectable, Point, Ray, UnitVec, Vec3};

#[derive(Debug)]
pub struct Quad {
    a: Point,
    ab: Vec3,
    ac: Vec3,
    normal: UnitVec,
    d: f32,
    w: Vec3,
}

impl Quad {
    pub fn new(a: Point, ab: Vec3, ac: Vec3) -> Self {
        let n = -ab.cross(ac);
        let normal = n.to_unit();
        let d = normal.dot(a.radius_vector);
        let w = n / n.dot(n);
        Quad {
            a,
            ab,
            ac,
            normal,
            d,
            w,
        }
    }

    pub fn quad_box() -> Vec<Quad> { todo!() }
}

impl Intersectable for Quad {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        // let denom = ray.dir.dot(self.normal);
        // let normal = if self.normal.dot(ray.dir) < 0. {self.normal} else {-self.normal};
        let denom = self.normal.dot(ray.dir);
        // if f32::abs(denom) < 1e-06{
        //     return None
        // }

        let t = (self.d - self.normal.dot(ray.origin.radius_vector)) / denom;
        if t < 0. {
            return None;
        }

        let hit_point = ray.at(t);
        let planar_hit_point = hit_point - self.a;
        let alpha = self.w.dot(planar_hit_point.cross(self.ab));
        let beta = self.w.dot(self.ac.cross(planar_hit_point));

        if 0. <= alpha && alpha <= 1. && 0. <= beta && beta <= 1. {
            Some(Hit {
                point: hit_point,
                normal: self.normal,
                t,
            })
        } else {
            None
        }
    }
}

impl Bounded for Quad {
    fn bound(&self) -> Aabb {
        Aabb::from_points(self.a + self.ab, self.a + self.ac) + Aabb::from_points(self.a, self.a + (self.ac + self.ab))
    }
}
