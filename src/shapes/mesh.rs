use num_traits::Pow;

use crate::{
    aggregates::Aabb,
    core::{Interaction, Ray, SurfaceInteraction},
    math::{cross, dot, utils::local_normal, Cross, Dot, Frame, Normed, Number, Transform, Transformable, Unit},
    shapes::{Bounded, Intersectable, Samplable, ShapeSample},
    Point2f, Point3f, Vec3f,
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

// TODO: .coords
impl Triangle {
    const PADDING: f32 = 1e-4;

    pub fn new(a: Point3f, ab: Vec3f, ac: Vec3f, trans: Transform<f32>) -> Self {
        let a = a.transform(&trans);
        let ab = ab.transform(&trans);
        let ac = ac.transform(&trans);

        let n = cross(&ab, &ac);
        let normal = n.to_unit();
        let d = normal.dot(&a);
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
        let n = cross(&ab, &ac);
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

impl Intersectable for Triangle {
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

        if (0.0..=1.0).contains(&alpha) && (0.0..=1.0).contains(&beta) && alpha + beta <= 1.0 {
            let an = 1.0 - alpha - beta;
            let normal = (*self.normals[0] * an + *self.normals[1] * alpha + *self.normals[2] * beta);

            let normal = local_normal(normal, ray).to_normal().to_unit();
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

        (0.0..=1.0).contains(&alpha) && (0.0..=1.0).contains(&beta) && alpha + beta <= 1.0
    }
}

// TODO: triangle sampling
impl Samplable for Triangle {
    fn sample(&self, rnd_p: Point2f) -> Option<ShapeSample> { todo!() }

    fn sample_from_point(&self, point: Point3f, rnd_p: Point2f) -> Option<ShapeSample> { todo!() }

    fn pdf(&self, interaction: &Interaction) -> f32 { todo!() }

    fn pdf_incoming(&self, interaction: &SurfaceInteraction, incoming: Unit<Vec3f>) -> f32 {
        // TODO: !!!
        let dir = interaction.hit.point - self.a;
        let cos = dot(&incoming, &dir).abs();
        let dist = dir.len_squared();
        self.area() * cos / dist
    }

    fn area(&self) -> f32 { todo!() }
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
