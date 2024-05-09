use std::cmp::Ordering;

use crate::{geometry::Hit, scene::Primitive};

#[derive(Clone, Copy)]
pub struct Intersection<'a> {
    pub hit: Hit,
    pub object: &'a Primitive,
}

impl<'a> Eq for Intersection<'a> {}

impl<'a> PartialEq<Self> for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool { self.hit.t.eq(&other.hit.t) }
}

impl<'a> PartialOrd<Self> for Intersection<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.hit.cmp(&other.hit)) }
}

impl<'a> Ord for Intersection<'a> {
    fn cmp(&self, other: &Self) -> Ordering { self.hit.cmp(&other.hit) }
}
