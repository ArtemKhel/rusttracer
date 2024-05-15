use crate::geometry::{
    unit_vec::local_normal, utils::Axis, Aabb, Bounded, Cross, Dot, Hit, Intersectable, Point, Ray, UnitVec, Vec3,
};

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
        let n = ab.cross(ac);
        let normal = n.to_unit();
        let d = normal.dot(a.radius_vector);
        let w = -n / n.dot(n);
        Quad {
            a,
            ab,
            ac,
            normal,
            d,
            w,
        }
    }

    pub fn quad_box(a: Point, b: Point) -> Vec<Quad> {
        let mut sides = Vec::with_capacity(6);

        let (a, b) = (Point::min_coords(a, b), Point::max_coords(a, b));
        let diag = b - a;
        let px = Vec3::new(diag[Axis::X], 0., 0.);
        let py = Vec3::new(0., diag[Axis::Y], 0.);
        let pz = Vec3::new(0., 0., diag[Axis::Z]);

        sides.push(Quad::new(a, px, py));
        sides.push(Quad::new(a, px, pz));
        sides.push(Quad::new(a, py, pz));
        sides.push(Quad::new(b, -px, -py));
        sides.push(Quad::new(b, -px, -pz));
        sides.push(Quad::new(b, -py, -pz));

        sides
    }
}

impl Intersectable for Quad {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        // let denom = ray.dir.dot(self.normal);
        let denom = self.normal.dot(ray.dir);
        if f32::abs(denom) < 1e-05 {
            return None;
        }

        let t = (self.d - self.normal.dot(ray.origin.radius_vector)) / denom;
        if t < 0. {
            return None;
        }

        let hit_point = ray.at(t);
        let planar_hit_point = hit_point - self.a;
        let alpha = self.w.dot(planar_hit_point.cross(self.ab));
        let beta = self.w.dot(self.ac.cross(planar_hit_point));

        if (0.0..=1.0).contains(&alpha) && (0.0..=1.0).contains(&beta) {
            Some(Hit {
                point: hit_point,
                normal: local_normal(self.normal, ray),
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
