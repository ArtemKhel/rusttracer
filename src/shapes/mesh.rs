use num_traits::Pow;

use crate::{
    aggregates::Aabb,
    core::{Hit, Ray},
    math::{utils::local_normal, Cross, Dot, Normed, Number, Point3, Transform, Transformable, Unit, Vec3},
    shapes::{Bounded, Intersectable},
    Point3f, Vec3f,
};

#[derive(Debug)]
pub struct Triangle {
    a: Point3f,
    ab: Vec3f,
    ac: Vec3f,
    normal: Unit<Vec3f>,
    normals: [Unit<Vec3f>; 3],
    d: f32,
    w: Vec3f,
}

impl Triangle {
    const PADDING: f32 = 1e-4;

    pub fn new(a: Point3f, ab: Vec3f, ac: Vec3f, trans: &Transform<f32>) -> Self {
        let a = a.transform(trans);
        let ab = ab.transform(trans);
        let ac = ac.transform(trans);

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

    pub fn new_with_normals(a: Point3f, ab: Vec3f, ac: Vec3f, normals: [Unit<Vec3f>; 3]) -> Self {
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

impl Intersectable<f32> for Triangle {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        // let denom = ray.dir.dot(self.normal);
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
        let alpha = self.w.dot(&planar_hit_point.cross(self.ab));
        let beta = self.w.dot(&self.ac.cross(planar_hit_point));

        if (0.0..=1.0).contains(&alpha) && (0.0..=1.0).contains(&beta) && alpha + beta <= 1.0 {
            let an = 1.0 - alpha - beta;
            let normal = (*self.normals[0] * an + *self.normals[1] * alpha + *self.normals[2] * beta);
            Some(Hit {
                point: hit_point,
                normal: local_normal(normal, ray).to_normal().to_unit(),
                t,
            })
        } else {
            None
        }
    }
}

impl Bounded<f32> for Triangle {
    fn bound(&self) -> Aabb<f32> {
        Aabb::from_points(self.a + self.ab, self.a + self.ac) + Aabb::from_points(self.a, self.a + (self.ac + self.ab))
    }
}

#[cfg(test)]
mod tests {
    // TODO:
    use obj;

    #[test]
    fn test() {}
}
