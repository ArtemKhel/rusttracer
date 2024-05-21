use std::process::Output;

use derive_new::new;
use num_traits::Pow;

use crate::{
    point::Point3,
    transform::{Transform, Transformable},
    unit::Unit,
    Normed, Number, Vec3,
};

#[derive(Copy, Clone, Debug, PartialEq, new)]
pub struct Ray<T: Number> {
    pub origin: Point3<T>,
    // TODO: use vec3 and normalize if needed?
    pub dir: Unit<Vec3<T>>,
    // pub medium: Option<M>
}

#[macro_export]
macro_rules! ray {
    ($origin:expr, $dir:expr) => {
        Ray {
            origin: $origin,
            dir: $dir,
        }
    };
}

impl<T: Number> Ray<T> {
    pub fn at(&self, t: T) -> Point3<T> { self.origin + *self.dir * t }

    pub fn from_to(origin: Point3<T>, end: Point3<T>) -> Ray<T> { ray!(origin, (end - origin).to_unit()) }
}

impl<T: Number> Transformable<T> for Ray<T> {
    fn transform(&self, trans: &Transform<T>) -> Self {
        let origin = trans.apply_to(self.origin);
        let dir = trans.apply_to(*self.dir).to_unit();
        ray!(origin, dir)
    }

    fn inv_transform(&self, trans: &Transform<T>) -> Self {
        let origin = trans.apply_inv_to(self.origin);
        let dir = trans.apply_inv_to(*self.dir).to_unit();
        ray!(origin, dir)
    }
}
