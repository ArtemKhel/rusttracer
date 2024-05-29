use std::{f32::consts::PI, fmt::Debug, ops::Deref};

use derive_new::new;
use num_traits::Pow;

use crate::{
    aggregates::{Aabb, Bounded},
    core::{Interaction, Ray, SurfaceInteraction},
    math::{
        cross, dot, utils::spherical_coordinates::spherical_phi, Dot, Normed, Number, Point3, Transform, Transformable,
        Unit,
    },
    point2, point3, ray,
    samplers::utils::{sample_uniform_cone, sample_uniform_sphere},
    shapes::{Intersectable, Samplable, ShapeSample},
    vec3, Point2f, Point3f,
};

#[derive(Default, Debug, Clone, Copy, new)]
pub struct Sphere {
    // pub center: Point3<T>,
    pub radius: f32,
    pub transform: Transform<f32>,
}

impl Sphere {
    /// Checks if ray hits the sphere. Assume that ray is in object space.
    /// Returns [Interaction] that is also in object space.
    fn basic_intersect(&self, ray: Ray, t_max: f32) -> Option<Interaction> {
        let o = ray.origin.coords;
        let h = dot(ray.dir.deref(), &o);
        let c = o.len_squared() - self.radius.powi(2);
        let disc = h.powi(2) - c;
        if disc < 0.0 {
            return None;
        }

        let disc_sqrt = disc.sqrt();
        // Find the nearest root that lies in the acceptable range.
        let root = {
            let r = -h - disc_sqrt;
            let r2 = -h + disc_sqrt;
            if r >= 0.0 {
                r
            } else if r2 >= 0.0 {
                r2
            } else {
                return None;
            }
        };

        let point = ray.at(root);
        Some(Interaction {
            point,
            normal: point.coords.to_normal().to_unit(),
            t: root,
            outgoing: -ray.dir,
            uv: point2!(),
        })
    }

    #[allow(non_snake_case)]
    fn calculate_surface_interaction(&self, mut interaction: Interaction) -> SurfaceInteraction {
        let point = interaction.point;

        // uv coordinates of interaction
        let phi = spherical_phi(point.coords);
        let cos_theta = point.coords.z / self.radius;
        let theta = cos_theta.acos();
        let u = phi / (2. * PI);
        let v = theta / PI;
        interaction.uv = point2!(u, v);

        // dp_du and dp_dv
        let z_radius = (point.x.powi(2) + point.y.powi(2)).sqrt();
        let cos_phi = point.x / z_radius;
        let sin_phi = point.y / z_radius;
        let dp_du = vec3!(-2. * PI * point.y, 2. * PI * point.x, 0.);
        let sin_theta = (1. - cos_phi.powi(2)).sqrt();
        let dp_dv = 2. * PI * vec3!(point.z * cos_phi, point.z * sin_phi, -self.radius * sin_theta);

        // dn_du and dn_dv
        let d2p_duu = -(2. * PI).powi(2) * vec3!(point.x, point.y, 0.);
        let d2p_duv = (2. * PI).powi(2) * point.z * vec3!(-sin_phi, cos_phi, 0.);
        let d2p_dvv = -(2. * PI).powi(2) * vec3!(point.x, point.y, point.z);

        // Compute coefficients for fundamental forms
        let E = dot(&dp_du, &dp_du);
        let F = dot(&dp_du, &dp_dv);
        let G = dot(&dp_dv, &dp_dv);
        let n = cross(&dp_du, &dp_dv).to_unit();
        let e = dot(&n, &d2p_duu);
        let f = dot(&n, &d2p_duv);
        let g = dot(&n, &d2p_dvv);

        // Compute  and  from fundamental form coefficients
        let EGF2 = (E * G) - (F - F);
        let inv_EGF2 = if (EGF2 == 0.) { 0. } else { EGF2.recip() };
        let dn_du = ((f * F - e * G) * inv_EGF2 * dp_du + (e * F - f * E) * inv_EGF2 * dp_dv).to_normal();
        let dn_dv = ((g * F - f * G) * inv_EGF2 * dp_du + (f * F - g * E) * inv_EGF2 * dp_dv).to_normal();

        // TODO: if something won't work, it would be this
        SurfaceInteraction::new(interaction, dp_du, dp_dv, dn_du, dn_dv)
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray, t_max: f32) -> Option<SurfaceInteraction> {
        let ray = ray.inv_transform(&self.transform);
        if let Some(interaction) = self.basic_intersect(ray, t_max) {
            let surf_inter = self.calculate_surface_interaction(interaction);
            Some(surf_inter)
        } else {
            None
        }
    }
}

impl Samplable for Sphere {
    fn sample(&self, sample: Point2f) -> Option<ShapeSample> {
        let point_obj = point3!(self.radius * sample_uniform_sphere(sample));
        let normal = point_obj.coords.to_normal().transform(&self.transform).to_unit();

        let phi = spherical_phi(point_obj.coords);
        let theta = (point_obj.coords.z / self.radius).acos();
        let u = phi / (2. * PI);
        let v = theta / PI;

        Some(ShapeSample {
            interaction: Interaction {
                point: point_obj,
                normal,
                t: 0.,
                outgoing: Unit::default(),
                uv: point2!(u, v),
            }
            .inv_transform(&self.transform),
            pdf: 1. / self.area(),
        })
    }

    fn sample_from_point(&self, point: Point3f, sample: Point2f) -> Option<ShapeSample> {
        if point.coords.len_squared() < self.radius.powi(2) + 1e-4 {
            // todo: corner cases, rounding error
            let mut ss = self.sample(sample).unwrap();
            let incoming = ss.interaction.point - point;
            ss.pdf /= dot(&ss.interaction.normal, &-incoming).abs() / (point - ss.interaction.point).len_squared();
            return if ss.pdf.is_infinite() { None } else { Some(ss) };
        }

        let max_sin_theta = self.radius / point.coords.len();
        let max_cos_theta = (1. - max_sin_theta.powi(2)).sqrt();
        let sampled_dir = sample_uniform_cone(sample, max_cos_theta).to_unit();
        let interaction = self.basic_intersect(ray!(point, sampled_dir), f32::INFINITY).unwrap();
        Some(ShapeSample {
            interaction,
            pdf: 1. / (2. * PI * (1. - max_cos_theta)),
        })
    }

    fn pdf(&self, interaction: &Interaction) -> f32 { self.area().recip() }

    fn area(&self) -> f32 { 4. * PI * self.radius.powi(2) }
}

impl Bounded<f32> for Sphere {
    fn bound(&self) -> Aabb<f32> {
        let vec = vec3!(self.radius);
        Aabb::from_points(Point3::from(vec), Point3::from(-vec)).transform(&self.transform)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::point3;

    #[test]
    fn test_aabb() {
        let sphere = Sphere::new(1.0, Transform::id());
        let aabb = sphere.bound();
        let expected = Aabb::from_points(point3!(1., 1., 1.), point3!(-1., -1., -1.));

        assert_eq!(aabb, expected)
    }

    #[test]
    fn test_aabb_translated() {
        let sphere = Sphere::new(1.0, Transform::translate(vec3!(1., 2., 3.)));
        let aabb = sphere.bound();
        let expected = Aabb::from_points(point3!(0., 1., 2.), point3!(2., 3., 4.));

        assert_eq!(aabb, expected)
    }

    #[test]
    fn test_aabb_translated_scaled() {
        let sphere = Sphere::new(
            1.0,
            Transform::compose(Transform::scale(1., 1., 2.), Transform::translate(vec3!(1., 2., 3.))),
        );
        let aabb = sphere.bound();
        let expected = Aabb::from_points(point3!(0., 1., 1.), point3!(2., 3., 5.));

        assert_eq!(aabb, expected)
    }
}
