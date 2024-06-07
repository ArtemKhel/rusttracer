use approx::AbsDiffEq;
use derive_more::{Add, AddAssign, Deref, DerefMut, Div, DivAssign, From, Mul, MulAssign, Neg, Sub, SubAssign};
use derive_new::new;
use num_traits::Float;

use crate::math::{
    transform::{Transform, Transformable},
    Dot, Normed, Number, Vec3, Vec4,
};

#[derive(Debug, Default, Clone, Copy, PartialEq)]
#[derive(Neg)]
#[derive(new, Deref, DerefMut, From)]
pub struct Normal3<T> {
    pub value: Vec3<T>,
}
#[macro_export]
macro_rules! normal3 {
    ($x:expr, $y:expr, $z:expr) => {
        $crate::math::Normal3::from($crate::vec3!($x, $y, $z))
    };
}
#[macro_export]
macro_rules! unit_normal3 {
    ($x:expr, $y:expr, $z:expr) => {
        $crate::math::Unit::from($crate::vec3!($x, $y, $z)).cast::<$crate::math::Normal3<_>>()
    };
}

#[macro_export]
macro_rules! unit_normal3_unchecked {
    ($vec:expr) => {
        $crate::math::Unit::from_unchecked($vec).cast::<$crate::math::Normal3<_>>()
    };
    ($x:expr, $y:expr, $z:expr) => {
        $crate::math::Unit::from_unchecked($crate::vec3!($x, $y, $z)).cast::<$crate::math::Normal3<_>>()
    };
}
impl<T: Number> Transformable<T> for Normal3<T> {
    fn transform(&self, trans: &Transform<T>) -> Self {
        let vec = Vec4::from(self.value);
        let px = vec.dot(&trans.inv.x);
        let py = vec.dot(&trans.inv.y);
        let pz = vec.dot(&trans.inv.z);
        normal3!(px, py, pz)
    }

    fn inv_transform(&self, trans: &Transform<T>) -> Self {
        let vec = Vec4::from(self.value);
        let mat = trans.mat.transpose();
        let px = vec.dot(&mat.x);
        let py = vec.dot(&mat.y);
        let pz = vec.dot(&mat.z);
        normal3!(px, py, pz)
    }
}

impl<T: Float + AbsDiffEq<Epsilon = T>> AbsDiffEq for Normal3<T> {
    type Epsilon = T;

    fn default_epsilon() -> Self::Epsilon { T::epsilon() }

    fn abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool { self.deref().abs_diff_eq(other, epsilon) }
}

#[cfg(test)]
mod tests {
    use approx::assert_abs_diff_eq;

    use super::*;

    #[test]
    fn test_transform() {
        let n = normal3!(1., 1., 1.);
        let t = Transform::scale(0.5, 1., 1.);
        let expected = normal3!(2., 1., 1.);

        assert_abs_diff_eq!(n.transform(&t), expected)
    }
}
