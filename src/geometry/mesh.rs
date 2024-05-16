use crate::geometry::{
    unit_vec::local_normal, utils::Axis, Aabb, Bounded, Cross, Dot, Hit, Intersectable, Point, Ray, UnitVec, Vec3,
};

#[derive(Debug)]
pub struct Triangle {
    a: Point,
    ab: Vec3,
    ac: Vec3,
    normal: UnitVec,
    normals: [UnitVec; 3],
    d: f32,
    w: Vec3,
}

impl Triangle {
    pub fn new(a: Point, ab: Vec3, ac: Vec3) -> Self {
        let n = ab.cross(ac);
        let normal = n.to_unit();
        let d = normal.dot(a.radius_vector);
        let w = -n / n.dot(n);
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

    pub fn new_with_normals(a: Point, ab: Vec3, ac: Vec3, normals: [UnitVec; 3]) -> Self {
        let n = ab.cross(ac);
        let normal = n.to_unit();
        let d = normal.dot(a.radius_vector);
        let w = -n / n.dot(n);
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

impl Intersectable for Triangle {
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

        if (0.0..=1.0).contains(&alpha) && (0.0..=1.0).contains(&beta) && alpha + beta <= 1.0 {
            let an = 1.0 - alpha - beta;
            let n = (self.normals[0] * an + self.normals[1] * alpha + self.normals[2] * beta).to_unit();
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

impl Bounded for Triangle {
    fn bound(&self) -> Aabb {
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
