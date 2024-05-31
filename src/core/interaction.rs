use std::cmp::Ordering;

use derive_new::new;

use crate::{F, math::{Normed, Transform, Transformable, Unit}, Normal3f, Point2f, Point3f, Vec3f};

#[derive(Debug, Clone, Copy)]
#[derive(new)]
pub struct Interaction {
    pub point: Point3f,
    pub normal: Unit<Normal3f>,
    pub t: f32,
    pub outgoing: Unit<Vec3f>,
    pub uv: Point2f,
    // pub medium: Option<M>,
}

impl Transformable<F> for Interaction {
    fn transform(&self, trans: &Transform<F>) -> Self {
        // TODO: don't normalize normals and outgoings?
        Interaction {
            point: self.point.transform(trans),
            normal: self.normal.transform(trans).to_unit(),
            t: self.t,
            outgoing: self.outgoing.transform(trans).to_unit(),
            uv: self.uv,
        }
    }

    fn inv_transform(&self, trans: &Transform<F>) -> Self {
        Interaction {
            point: self.point.inv_transform(trans),
            normal: self.normal.inv_transform(trans).to_unit(),
            t: self.t,
            outgoing: self.outgoing.transform(trans).to_unit(),
            uv: self.uv,
        }
    }
}

impl Eq for Interaction {}

impl PartialEq<Self> for Interaction {
    fn eq(&self, other: &Self) -> bool { self.t.eq(&other.t) }
}

impl PartialOrd for Interaction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.t.partial_cmp(&other.t) }
}

impl Ord for Interaction {
    fn cmp(&self, other: &Self) -> Ordering { self.t.total_cmp(&other.t) }
}
