use std::cmp::Ordering;

use geometry::Hit;

use crate::scene::Object;

#[derive(Clone, Copy)]
pub struct Intersection<'a> {
    pub hit: Hit,
    pub object: &'a Object,
}

impl<'a> Eq for Intersection<'a> {}

impl<'a> PartialEq<Self> for Intersection<'a> {
    fn eq(&self, other: &Self) -> bool { self.hit.t.eq(&other.hit.t) }
}

impl<'a> PartialOrd<Self> for Intersection<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { self.hit.t.partial_cmp(&other.hit.t) }
}

impl<'a> Ord for Intersection<'a> {
    fn cmp(&self, other: &Self) -> Ordering { self.hit.cmp(&other.hit) }
}
