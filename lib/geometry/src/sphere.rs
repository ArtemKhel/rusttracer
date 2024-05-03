use crate::{hit::Hit, point::Point, ray::Ray, unit_vec::UnitVec, Dot, Intersect, Vec3, AABB};

#[derive(Default, Debug, Clone, Copy)]
pub struct Sphere {
    pub center: Point,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Point, radius: f32) -> Sphere { Sphere { center, radius } }

    pub fn normal(&self, point: Point) -> UnitVec { self.center.unit_vector_to(point) }
}

impl Intersect for Sphere {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        let o = self.center.vector_to(ray.origin);
        let h = ray.dir.dot(o);
        let c = o.dot(o) - self.radius.powi(2);
        let disc = h.powi(2) - c;
        if disc < 0. {
            return None;
        }

        let disc_sqrt = disc.sqrt();
        // Find the nearest root that lies in the acceptable range.
        let root = {
            let r = -h - disc_sqrt;
            let r2 = -h + disc_sqrt;
            if r >= 0. {
                Some(r)
            } else if r2 >= 0. {
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

    fn bounding_box(&self) -> AABB {
        let vec = Vec3::new(self.radius, self.radius, self.radius);
        AABB::from_points(self.center - vec, self.center + vec)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_aabb() {
        let sphere = Sphere::new(Point::default(), 1.0);
        let aabb = sphere.bounding_box();
        let expected = AABB::from_points(Point::new(1., 1., 1.), Point::new(-1., -1., -1.));

        assert_eq!(aabb, expected)
    }
}
