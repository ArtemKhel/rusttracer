use crate::{hit::Hit, point::Point, ray::Ray, unit_vec::UnitVec, Dot, Intersect};

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
        // let o = ray.origin.vector_to(self.center);
        // let o = ray.origin - self.center;
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
}
