use std::ops::Deref;

use approx::AbsDiffEq;

use crate::{
    math::{cross, dot, Normed, Number, Unit, Vec3},
    vec3,
};

// Represents a rotation that aligns three orthonormal vectors in a coordinate
// system with the x, y, z axes.
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Frame<T> {
    x: Unit<Vec3<T>>,
    y: Unit<Vec3<T>>,
    z: Unit<Vec3<T>>,
}

#[allow(clippy::wrong_self_convention)]
impl<T: Number> Frame<T> {
    fn new(x: Vec3<T>, y: Vec3<T>, z: Vec3<T>) -> Self {
        // TODO
        // #[cfg(debug_assertions)]
        // {
        //     assert!(dot(&x, &y).abs() < T::epsilon());
        //     assert!(dot(&x, &z).abs() < T::epsilon());
        //     assert!(dot(&y, &z).abs() < T::epsilon());
        // }
        Frame {
            x: x.to_unit(),
            y: y.to_unit(),
            z: z.to_unit(),
        }
    }

    pub fn from_x_y(x: Vec3<T>, y: Vec3<T>) -> Self { Frame::new(x, y, cross(&x, &y)) }

    pub fn from_x_z(x: Vec3<T>, z: Vec3<T>) -> Self { Frame::new(x, cross(&x, &z), z) }

    pub fn from_y_z(y: Vec3<T>, z: Vec3<T>) -> Self { Frame::new(cross(&y, &z), y, z) }

    pub fn from_z(v1: Vec3<T>) -> Self {
        let sign = T::one().copysign(v1.z);
        let a = -T::one() / (sign + v1.z);
        let b = v1.x * v1.y * a;
        let v2 = vec3!(T::one() + sign * v1.x.powi(2) * a, sign * b, -sign * v1.x);
        let v3 = vec3!(b, sign + v1.y.powi(2) * a, -v1.y);
        Frame::new(v1, v2, v3)
    }

    pub fn to_local(&self, vec: Vec3<T>) -> Vec3<T> {
        vec3!(dot(&vec, &self.x), dot(&vec, &self.y), dot(&vec, &self.z))
    }

    pub fn from_local(&self, vec: Vec3<T>) -> Vec3<T> { *self.x * vec.x + *self.y * vec.y + *self.z * vec.z }

    pub fn to_local_wrap<R>(&self, vec: Vec3<T>) -> R
    where R: From<Vec3<T>> {
        R::from(vec3!(dot(&vec, &self.x), dot(&vec, &self.y), dot(&vec, &self.z)))
    }

    pub fn from_local_unwrap<R>(&self, vec: R) -> Vec3<T>
    where R: Deref<Target = Vec3<T>> {
        *self.x * vec.x + *self.y * vec.y + *self.z * vec.z
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{unit3, vec3};

    #[test]
    fn test() {
        let x = vec3!(2., 0., 0.);
        let y = vec3!(0., 3., 0.);
        let expected = Frame {
            x: unit3!(1., 0., 0.),
            y: unit3!(0., 1., 0.),
            z: unit3!(0., 0., 1.),
        };
        assert_eq!(Frame::from_x_y(x, y), expected)
    }
}
