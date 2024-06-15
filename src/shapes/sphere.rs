use std::{default::Default, f32::consts::PI, fmt::Debug, ops::Deref};

use derive_new::new;
use num_traits::{real::Real, Pow};

use crate::{
    aggregates::{Aabb, Bounded},
    core::{Interaction, Ray, SurfaceInteraction},
    math::{
        cross, dot,
        utils::spherical_coordinates::{spherical_direction, spherical_phi},
        Dot, Frame, Normed, Number, Point3, Transform, Transformable, Unit,
    },
    point2, point3, ray,
    samplers::utils::{sample_uniform_cone, sample_uniform_sphere},
    shapes::{Intersectable, Samplable, ShapeSample},
    vec3, Normal3f, Point2f, Point3f, Vec3f,
};

#[derive(Default, Debug, Clone, Copy, new)]
pub struct Sphere {
    // pub center: Point3<T>,
    pub radius: f32,
    pub object_to_world: Transform<f32>,
}

impl Sphere {
    /// Checks if ray hits the sphere. Assume that ray is in object space.
    /// Returns [Interaction] that is also **in object space.**
    // TODO: obj space wrapper?
    fn basic_intersect(&self, ray: Ray, t_max: f32) -> Option<Interaction> {
        let o = *ray.origin;
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
            normal: point.to_normal().to_unit(),
            t: root,
            outgoing: -ray.dir,
            uv: point2!(),
        })
    }

    #[allow(non_snake_case)]
    fn calculate_surface_interaction(&self, mut interaction: Interaction) -> SurfaceInteraction {
        let point = interaction.point;

        // uv coordinates of interaction
        let phi = spherical_phi(*point);
        let cos_theta = point.z / self.radius;
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
        let EGF2 = (E * G) - (F * F);
        let inv_EGF2 = if (EGF2 == 0.) { 0. } else { EGF2.recip() };
        let dn_du = ((f * F - e * G) * inv_EGF2 * dp_du + (e * F - f * E) * inv_EGF2 * dp_dv).to_normal();
        let dn_dv = ((g * F - f * G) * inv_EGF2 * dp_du + (f * F - g * E) * inv_EGF2 * dp_dv).to_normal();

        SurfaceInteraction::new(interaction, dp_du, dp_dv, dn_du, dn_dv)
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray, t_max: f32) -> Option<SurfaceInteraction> {
        let ray = ray.inv_transform(&self.object_to_world);
        if let Some(interaction) = self.basic_intersect(ray, t_max) {
            let mut surf_int = self.calculate_surface_interaction(interaction);
            surf_int = surf_int.transform(&self.object_to_world);
            Some(surf_int)
        } else {
            None
        }
    }

    fn check_intersect(&self, ray: &Ray, t_max: f32) -> bool {
        let ray = ray.inv_transform(&self.object_to_world);
        self.basic_intersect(ray, t_max).is_some()
    }
}

impl Samplable for Sphere {
    fn sample(&self, rnd_p: Point2f) -> Option<ShapeSample> {
        let point_obj = point3!(self.radius * *sample_uniform_sphere(rnd_p));
        let normal = point_obj.to_normal().transform(&self.object_to_world).to_unit();

        let phi = spherical_phi(*point_obj);
        let theta = (point_obj.z / self.radius).acos();
        let u = phi / (2. * PI);
        let v = theta / PI;

        Some(ShapeSample {
            hit: Interaction {
                point: point_obj,
                normal,
                t: 0.,
                outgoing: Default::default(),
                uv: point2!(u, v),
            }
            .transform(&self.object_to_world),
            pdf: self.area().recip(),
        })
    }

    fn sample_from_point(&self, origin: Point3f, rnd_p: Point2f) -> Option<ShapeSample> {
        // TODO: doesn't work
        let point = origin.inv_transform(&self.object_to_world);
        if point.len_squared() < self.radius.powi(2) + 1e-4 {
            let mut ss = self.sample(rnd_p).unwrap();
            let incoming = ss.hit.point - point;
            ss.pdf /= dot(&ss.hit.normal, &-incoming).abs() / (point - ss.hit.point).len_squared();
            return if ss.pdf.is_infinite() { None } else { Some(ss) };
        }

        let max_sin_theta = self.radius / point.len();
        let max_cos_theta = (1. - max_sin_theta.powi(2)).sqrt();
        let sampled_dir = sample_uniform_cone(rnd_p, max_cos_theta).to_unit();
        let interaction = self
            .basic_intersect(ray!(point, sampled_dir), f32::INFINITY)
            .unwrap()
            .inv_transform(&self.object_to_world);
        Some(ShapeSample {
            hit: interaction,
            pdf: 1. / (2. * PI * (1. - max_cos_theta)),
        })

        // let p_center: Point3f = point3!().transform(&self.transform);
        // let p_origin: Point3f = origin;
        // // Sample uniformly on sphere if $\pt{}$ is inside it
        // if (p_origin-p_center).len_squared() <= self.radius.powi(2) {
        //     // Sample shape by area and compute incident direction _wi_
        //     let mut ss = self.sample(rnd_p).expect("Sphere sample() failed!");
        //     let wi = ss.hit.point - origin;
        //     if wi.len_squared() == 0.0 {
        //         return None;
        //     }
        //     let wi = wi.to_unit();
        //
        //     // Convert area sampling PDF in _ss_ to solid angle measure
        //     ss.pdf /= dot(&ss.hit.normal, &-wi) / (origin - ss.hit.point).len_squared();
        //     if ss.pdf.is_infinite() {
        //         return None;
        //     }
        //     return Some(ss);
        // }
        //
        // // Sample sphere uniformly inside subtended cone
        // // Compute quantities related to the $\theta_\roman{max}$ for cone
        // let sin_theta_max = self.radius / (origin - p_center).len();
        // let sin2_theta_max = (sin_theta_max).powi(2);
        // let cos_theta_max = (1.0 - sin2_theta_max).sqrt();
        // let mut one_minus_cos_theta_max = 1.0 - cos_theta_max;
        //
        // // Compute $\theta$ and $\phi$ values for sample in cone
        // let mut cos_theta = (cos_theta_max - 1.0) * rnd_p.x + 1.0;
        // let mut sin2_theta = 1.0 - cos_theta.powi(2);
        // /* sin^2(1.5 deg) */
        // if sin2_theta_max < 0.00068523
        // {
        //     // Compute cone sample via Taylor series expansion for small angles
        //     sin2_theta = sin2_theta_max * rnd_p.x;
        //     cos_theta = (1.0 - sin2_theta).sqrt();
        //     one_minus_cos_theta_max = sin2_theta_max / 2.0;
        // }
        //
        // // Compute angle $\alpha$ from center of sphere to sampled point on surface
        // let cos_alpha = sin2_theta / sin_theta_max
        //     + cos_theta * (1.0 - sin2_theta / sin_theta_max.powi(2)).sqrt();
        // let sin_alpha = (1.0 - cos_alpha.powi(2)).sqrt();
        //
        // // Compute surface normal and sampled point on sphere
        // let phi = rnd_p.y * 2.0 * PI;
        // let w = spherical_direction(sin_alpha, cos_alpha, phi);
        // let sampling_frame = Frame::from_z(p_center - origin);
        // let normal_sign = if /*self.reverse_orientation */ false { -1.0 } else { 1.0 };
        // let n = Normal3f::new(sampling_frame.from_local(-w)* normal_sign) ;
        // let p = p_center + vec3!(n.x, n.y, n.z) * self.radius;
        //
        // // Return _ShapeSample_ for sampled point on sphere
        // // Compute $(u,v)$ coordinates for sampled point on sphere
        // let p_obj = p.inv_transform(&self.transform);
        // let theta = (p_obj.z / self.radius).acos();
        // let mut sphere_phi = f32::atan2(p_obj.y, p_obj.x);
        // if sphere_phi < 0.0 {
        //     sphere_phi += 2.0 * PI;
        // }
        // let uv = Point2f::new(
        //     sphere_phi,
        //     theta,
        // );
        //
        // debug_assert_ne!(one_minus_cos_theta_max, 0.0); // very small far away sphere
        // let interaction = Interaction {
        //     point: p,
        //     t: 0.,
        //     outgoing: Unit::from_unchecked(Vec3f::default()),
        //     normal: n.to_unit(),
        //     uv,
        // };
        // Some(ShapeSample {
        //     hit: interaction,
        //     pdf: 1.0 / (2.0 * PI * one_minus_cos_theta_max),
        // })
    }

    fn pdf(&self, interaction: &Interaction) -> f32 { self.area().recip() }

    fn pdf_incoming(&self, interaction: &SurfaceInteraction, incoming: Unit<Vec3f>) -> f32 {
        let center = point3!().inv_transform(&self.object_to_world);
        let origin = interaction.hit.point;
        if (origin - center).len_squared() < self.radius.powi(2) {
            let ray = interaction.spawn_ray(incoming);
            let Some(int) = self.intersect(&ray, f32::INFINITY) else {
                return 0.;
            };
            let mut pdf = self.area().recip()
                / dot(&int.hit.normal, &-incoming).abs()
                / (interaction.hit.point - int.hit.point).len_squared();
            if pdf.is_infinite() {
                0.
            } else {
                pdf
            }
        } else {
            let sin2_theta_max = self.radius.powi(2) / (interaction.hit.point - center).len_squared();
            let cos_theta_max = (1. - sin2_theta_max).sqrt();
            let one_minus_cos_theta_max = 1. - cos_theta_max;
            // TODO: Compute more accurate oneMinusCosThetaMax for small solid angle
            (2. * PI * one_minus_cos_theta_max).recip()
        }
    }

    fn area(&self) -> f32 { 4. * PI * self.radius.powi(2) }
}

impl Bounded<f32> for Sphere {
    fn bound(&self) -> Aabb<f32> {
        let vec = vec3!(self.radius);
        Aabb::from_points(Point3::from(vec), Point3::from(-vec)).transform(&self.object_to_world)
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
