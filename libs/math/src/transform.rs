use std::fmt::Debug;

use crate::{matrix4::Matrix4, point3, utils::Axis3, vec3, Dot, Number, Vec3};

#[derive(Debug)]
pub struct Transform<T> {
    pub(crate) mat: Matrix4<T>,
    pub(crate) inv: Matrix4<T>,
}

pub trait Transformable<T> {
    fn transform(&self, trans: &Transform<T>) -> Self;
    fn inv_transform(&self, trans: &Transform<T>) -> Self;
}

impl<T: Number> Transform<T> {
    pub fn from_matrix(mat: Matrix4<T>) -> Transform<T> {
        // TODO: unwraps
        Transform {
            inv: mat.inverse().unwrap(),
            mat,
        }
    }

    pub fn translate(vec: Vec3<T>) -> Self {
        let mut mat = Matrix4::id();
        mat.x.w = vec.x;
        mat.y.w = vec.y;
        mat.z.w = vec.z;
        Transform {
            inv: mat.inverse().unwrap(),
            mat,
        }
    }

    pub fn scale(factor_x: T, factor_y: T, factor_z: T) -> Self {
        let mut mat = Matrix4::id();
        mat.x.x = factor_x;
        mat.y.y = factor_y;
        mat.z.z = factor_z;
        Transform {
            inv: mat.inverse().unwrap(),
            mat,
        }
    }

    /// Clockwise
    pub fn rotate(axis: Axis3, theta: T) -> Self {
        let mut mat = Matrix4::id();
        let sin = theta.sin();
        let cos = theta.cos();
        match axis {
            Axis3::X => {
                mat.y.y = cos;
                mat.y.z = sin;
                mat.z.y = -sin;
                mat.z.z = cos;
            }
            Axis3::Y => {
                mat.x.x = cos;
                mat.x.z = -sin;
                mat.z.x = sin;
                mat.z.z = cos;
            }
            Axis3::Z => {
                mat.x.x = cos;
                mat.x.y = sin;
                mat.y.x = -sin;
                mat.y.y = cos;
            }
        }
        Transform {
            inv: mat.inverse().unwrap(),
            mat,
        }
    }

    pub fn compose(a: Transform<T>, b: Transform<T>) -> Self {
        Transform {
            mat: a.mat * b.mat,
            inv: b.inv * a.inv,
        }
    }

    pub fn compose_iter<I: IntoIterator<Item = Transform<T>>>(it: I) -> Self {
        it.into_iter().reduce(|acc, x| Self::compose(acc, x)).unwrap()
    }

    pub fn apply_to<R: Transformable<T>>(&self, x: R) -> R { x.transform(self) }

    pub fn apply_inv_to<R: Transformable<T>>(&self, x: R) -> R { x.inv_transform(self) }
}

#[cfg(test)]
mod tests {
    use std::f32::consts::FRAC_PI_2;

    use approx::assert_abs_diff_eq;

    use super::*;
    use crate::{point3, vec3, Point3, Vec3};

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
        let ev = vec3!(2., -1., 3.);
        let ep = point3!(2., -1., 3.);
        let eiv = vec3!(-2., 1., 3.);
        let eip = point3!(-2., 1., 3.);

        assert_abs_diff_eq!(t.apply_to(v), ev);
        assert_abs_diff_eq!(t.apply_to(p), ep);
        assert_abs_diff_eq!(t.apply_inv_to(v), eiv);
        assert_abs_diff_eq!(t.apply_inv_to(p), eip);
    }

    #[test]
    fn test_compose() {
        let p = point3!(0., 0., 0.);
        let t1 = Transform::translate(vec3!(1., 2., 3.));
        let t2 = Transform::scale(1., 2., 3.);
        let t3 = Transform::rotate(Axis3::Z, FRAC_PI_2);
        let comp = Transform::compose_iter([t3, t2, t1]);
        let ep = point3!(4., -1., 9.);
        let eip = point3!(-1., -2., -3.);

        assert_abs_diff_eq!(comp.apply_to(p), ep);
        assert_abs_diff_eq!(comp.apply_inv_to(p), eip);
    }
}
