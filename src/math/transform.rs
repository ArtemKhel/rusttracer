use std::{fmt::Debug, iter::Iterator};

use itertools::Itertools;
use num_traits::Zero;
use strum::IntoEnumIterator;

use crate::{
    math::{
        axis::{Axis3, Axis4},
        dot,
        matrix4::Matrix4,
        Dot, Normed, Number, Vec3,
    },
    vec3, Vec3f,
};

#[derive(Debug, Clone, Copy)]
pub struct Transform<T> {
    pub mat: Matrix4<T>,
    pub inv: Matrix4<T>,
}

pub trait Transformable<T> {
    // TODO:
    fn transform(&self, trans: &Transform<T>) -> Self;
    fn inv_transform(&self, trans: &Transform<T>) -> Self;
}

impl<T: Number> Transform<T> {
    pub fn id() -> Self {
        Transform {
            mat: Matrix4::id(),
            inv: Matrix4::id(),
        }
    }

    pub fn from_matrix(mat: Matrix4<T>) -> Self {
        // TODO: unwraps
        Transform {
            inv: mat.invert().unwrap(),
            mat,
        }
    }

    pub fn invert(&self) -> Self {
        Transform {
            mat: self.inv,
            inv: self.mat,
        }
    }

    pub fn translate(vec: Vec3<T>) -> Self {
        let mut mat = Matrix4::id();
        mat.x.w = vec.x;
        mat.y.w = vec.y;
        mat.z.w = vec.z;
        Transform::from_matrix(mat)
    }

    pub fn then_translate(mut self, vec: Vec3<T>) -> Self {
        self = Transform::compose(self, Transform::translate(vec));
        self
    }

    pub fn scale(factor_x: T, factor_y: T, factor_z: T) -> Self {
        let mut mat = Matrix4::id();
        mat.x.x = factor_x;
        mat.y.y = factor_y;
        mat.z.z = factor_z;
        Transform::from_matrix(mat)
    }

    pub fn then_scale(mut self, factor_x: T, factor_y: T, factor_z: T) -> Self {
        self = Transform::compose(self, Transform::scale(factor_x, factor_y, factor_z));
        self
    }

    pub fn scale_uniform(factor: T) -> Self { Self::scale(factor, factor, factor) }

    pub fn then_scale_uniform(mut self, factor: T) -> Self {
        self = Transform::compose(self, Transform::scale_uniform(factor));
        self
    }

    /// Clockwise
    pub fn rotate(axis: Axis3, theta: T) -> Self {
        let mut mat = Matrix4::id();
        let (sin, cos) = theta.sin_cos();
        match axis {
            Axis3::X => {
                mat.y.y = cos;
                mat.y.z = -sin;
                mat.z.y = sin;
                mat.z.z = cos;
            }
            Axis3::Y => {
                mat.x.x = cos;
                mat.x.z = sin;
                mat.z.x = -sin;
                mat.z.z = cos;
            }
            Axis3::Z => {
                mat.x.x = cos;
                mat.x.y = -sin;
                mat.y.x = sin;
                mat.y.y = cos;
            }
        }
        Transform::from_matrix(mat)
    }

    /// Clockwise
    pub fn then_rotate(mut self, axis: Axis3, theta: T) -> Self {
        self = Transform::compose(self, Transform::rotate(axis, theta));
        self
    }

    /// Clockwise
    pub fn rotate_degrees(axis: Axis3, degrees: T) -> Self { Self::rotate(axis, degrees.to_radians()) }

    /// Clockwise
    pub fn then_rotate_degrees(mut self, axis: Axis3, degrees: T) -> Self {
        self = Transform::compose(self, Transform::rotate_degrees(axis, degrees));
        self
    }

    pub fn rotate_arbitrary_axis(axis: Vec3<T>, theta: T) -> Self {
        let sin = theta.sin();
        let cos = theta.cos();

        let axis = axis.to_unit();
        let mut mat = Matrix4::id();

        use Axis4::*;
        mat[X][X] = axis.x * axis.x + (T::one() - axis.x.powi(2)) * cos;
        mat[X][Y] = axis.x * axis.y * (T::one() - cos) - axis.z * sin;
        mat[X][Z] = axis.x * axis.z * (T::one() - cos) + axis.y * sin;

        mat[Y][X] = axis.y * axis.x * (T::one() - cos) + axis.z * sin;
        mat[Y][Y] = axis.y * axis.y + (T::one() - axis.y.powi(2)) * cos;
        mat[Y][Z] = axis.y * axis.z * (T::one() - cos) - axis.x * sin;

        mat[Z][X] = axis.z * axis.x * (T::one() - cos) - axis.y * sin;
        mat[Z][Y] = axis.z * axis.y * (T::one() - cos) + axis.x * sin;
        mat[Z][Z] = axis.z * axis.z + (T::one() - axis.z.powi(2)) * cos;

        Transform {
            inv: mat.transpose(),
            mat,
        }
    }

    pub fn then_rotate_arbitrary_axis(mut self, axis: Vec3<T>, theta: T) -> Self {
        self = Transform::compose(self, Transform::rotate_arbitrary_axis(axis, theta));
        self
    }

    pub fn orthographic(z_near: T, z_far: T) -> Self {
        Transform::compose(
            Transform::translate(vec3!(T::zero(), T::zero(), -z_near)),
            Transform::scale(T::one(), T::one(), (z_far - z_near).recip()),
        )
    }

    pub fn perspective(fov: T, z_near: T, z_far: T) -> Self {
        let _0 = T::zero();
        let _1 = T::one();
        let _2 = _1 + _1;

        #[rustfmt::skip]
        let perspective = Matrix4::from_elements(
            _1, _0, _0, _0,
            _0, _1, _0, _0,
            _0, _0, z_far / (z_far - z_near), -z_far * z_near / (z_far - z_near),
            _0, _0, _1, _0,
        );
        let scale = (fov.to_radians() / _2).tan().recip();
        Transform::compose(Transform::from_matrix(perspective), Transform::scale(scale, scale, _1))
    }

    pub fn compose(a: Transform<T>, b: Transform<T>) -> Self {
        // TODO: rotations
        Transform {
            mat: b.mat * a.mat,
            inv: a.inv * b.inv,
        }
    }

    pub fn compose_iter<Iterable, Iter>(it: Iterable) -> Self
    where
        Iterable: IntoIterator<Item = Transform<T>, IntoIter = Iter>,
        Iter: Iterator<Item = Transform<T>>, {
        it.into_iter().reduce(|acc, x| Self::compose(acc, x)).unwrap()
    }

    pub fn apply_to<R: Transformable<T>>(&self, x: R) -> R { x.transform(self) }

    pub fn apply_inv_to<R: Transformable<T>>(&self, x: R) -> R { x.inv_transform(self) }
}

impl Transform<f32> {
    pub fn rotate_from_to(from: &Vec3f, to: &Vec3f) -> Self {
        // Compute intermediate vector for vector reflection
        let reflection_vec = if (from.x.abs() < 0.72 && to.x.abs() < 0.72) {
            vec3!(1., 0., 0.)
        } else if (from.y.abs() < 0.72 && to.y.abs() < 0.72) {
            vec3!(0., 1., 0.)
        } else {
            vec3!(0., 0., 1.)
        };
        // Initialize matrix r for rotation
        let u = reflection_vec - *from;
        let v = reflection_vec - *to;
        let mut matrix = Matrix4::id();
        for (row, col) in Axis3::iter().cartesian_product(Axis3::iter()) {
            matrix[row.into()][col.into()] = (if row == col { 1. } else { 0. }
                - 2. / dot(&u, &u) * u[row] * u[col]
                - 2. / dot(&v, &v) * v[row] * v[col]
                + 4. * dot(&u, &v) / (dot(&u, &u) * dot(&v, &v)) * v[row] * u[col])
        }
        Transform::from_matrix(matrix)
    }
}

impl<T: Number> Default for Transform<T> {
    fn default() -> Self { Transform::id() }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::{FRAC_1_SQRT_2, FRAC_PI_2};

    use approx::assert_abs_diff_eq;

    use super::*;
    use crate::{point3, vec3, Vec3f};

    #[test]
    fn test_translate() {
        let v = vec3!(1., 2., 3.);
        let p = point3!(1., 2., 3.);
        let t = Transform::translate(vec3!(4., 5., 6.));
        let ev = vec3!(1., 2., 3.);
        let ep = point3!(5., 7., 9.);
        let eiv = vec3!(1., 2., 3.);
        let eip = point3!(-3., -3., -3.);

        assert_abs_diff_eq!(t.apply_to(v), ev);
        assert_abs_diff_eq!(t.apply_to(p), ep);
        assert_abs_diff_eq!(t.apply_inv_to(v), eiv);
        assert_abs_diff_eq!(t.apply_inv_to(p), eip);
    }

    #[test]
    fn test_scale() {
        let v = vec3!(1., 2., 3.);
        let p = point3!(1., 2., 3.);
        let t = Transform::scale(1., 2., 3.);
        let ev = vec3!(1., 4., 9.);
        let ep = point3!(1., 4., 9.);
        let eiv = vec3!(1., 1., 1.);
        let eip = point3!(1., 1., 1.);

        assert_abs_diff_eq!(t.apply_to(v), ev);
        assert_abs_diff_eq!(t.apply_to(p), ep);
        assert_abs_diff_eq!(t.apply_inv_to(v), eiv);
        assert_abs_diff_eq!(t.apply_inv_to(p), eip);
    }

    #[test]
    fn test_rotate() {
        let v = vec3!(1., 2., 3.);
        let p = point3!(1., 2., 3.);
        let t = Transform::rotate(Axis3::Z, FRAC_PI_2);
        let ev = vec3!(-2., 1., 3.);
        let ep = point3!(-2., 1., 3.);
        let eiv = vec3!(2., -1., 3.);
        let eip = point3!(2., -1., 3.);

        assert_abs_diff_eq!(t.apply_to(v), ev);
        assert_abs_diff_eq!(t.apply_to(p), ep);
        assert_abs_diff_eq!(t.apply_inv_to(v), eiv);
        assert_abs_diff_eq!(t.apply_inv_to(p), eip);
    }

    #[test]
    fn test_rotate2() {
        let vec = vec3!(0., 0., 1.);

        let rot_y = Transform::rotate_degrees(Axis3::Y, 225.);
        let exp_rot_y = vec3!(-FRAC_1_SQRT_2, 0., -FRAC_1_SQRT_2);

        let rot_x = Transform::rotate_degrees(Axis3::X, 45.);
        let exp_rot_x = vec3!(0., -FRAC_1_SQRT_2, FRAC_1_SQRT_2);

        let rot_x_y = Transform::compose(rot_x, rot_y);
        let exp_rot_x_y = vec3!(0., -FRAC_1_SQRT_2, FRAC_1_SQRT_2);

        assert_abs_diff_eq!(vec.transform(&rot_y), exp_rot_y, epsilon = 1e-5);
        assert_abs_diff_eq!(vec.transform(&rot_x), exp_rot_x, epsilon = 1e-5);
    }

    #[test]
    fn test_compose() {
        let p = point3!(0., 0., 0.);
        let t1 = Transform::translate(vec3!(1., 2., 3.));
        let t2 = Transform::scale(1., 2., 3.);
        let t3 = Transform::rotate(Axis3::Z, FRAC_PI_2);
        let comp = Transform::compose_iter([t1, t2, t3]);
        let ep = point3!(-4., 1., 9.);
        let eip = point3!(-1., -2., -3.);

        assert_abs_diff_eq!(comp.apply_to(p), ep, epsilon = 1e-5);
        assert_abs_diff_eq!(comp.apply_inv_to(p), eip);
    }

    #[test]
    fn test_orthographic() {
        let orthographic = Transform::orthographic(0., 10.);
        let v1 = vec3!(1., 2., 5.);
        let v2 = vec3!(1., 2., -5.);
        let v3 = vec3!(1., 2., 10.);
        let v4 = vec3!(1., 2., 15.);

        assert_eq!(v1.transform(&orthographic), vec3!(1., 2., 0.5));
        assert_eq!(v2.transform(&orthographic), vec3!(1., 2., -0.5));
        assert_eq!(v3.transform(&orthographic), vec3!(1., 2., 1.));
        assert_eq!(v4.transform(&orthographic), vec3!(1., 2., 1.5));
    }
}
