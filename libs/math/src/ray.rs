use std::process::Output;

use derive_new::new;
use num_traits::Pow;

use crate::{point::Point3, unit_vec::UnitVec3, Number};

#[derive(Copy, Clone, Debug, PartialOrd, PartialEq, new)]
pub struct Ray<T: Number> {
    pub origin: Point3<T>,
    // TODO: use vec3 and normalize if needed?
    pub dir: UnitVec3<T>,
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
