use std::{
    mem::swap,
    ops::{Add, AddAssign, Index, Not},
};

use approx::AbsDiffEq;
use derive_new::new;
use num_traits::{Float, Signed};
use strum::IntoEnumIterator;

use crate::{
    aggregates::Bounded,
    core::ray::Ray,
    math::{
        axis::{Axis2, Axis3},
        Number, Point2, Transform, Transformable, Vec2, Vec3,
    },
    point2,
};

#[derive(Copy, Clone, Debug, PartialEq)]
#[derive(new)]
pub struct Bounds2<T> {
    pub min: Point2<T>,
    pub max: Point2<T>,
}

impl<T: Number> Bounds2<T> {
    pub fn center(&self) -> Point2<T> { Point2::from(*self.min + (self.max - self.min) / (T::one() + T::one())) }

    pub fn max_dimension(&self) -> Axis2 {
        let diag = self.max - self.min;
        if diag.x >= diag.y {
            Axis2::X
        } else {
            Axis2::Y
        }
    }

    pub fn from_points(p1: Point2<T>, p2: Point2<T>) -> Self {
        let mut bounds = Bounds2::new(Point2::min_coords(p1, p2), Point2::max_coords(p1, p2));
        bounds
    }

    pub fn union(&self, p: Point2<T>) -> Self {
        Bounds2::new(Point2::min_coords(self.min, p), Point2::max_coords(self.max, p))
    }
}

impl<T: Number> Default for Bounds2<T> {
    fn default() -> Self {
        let max = T::max_value();
        let min = T::min_value();
        Bounds2::new(point2!(max, max), point2!(min, min))
    }
}
