pub use aabb::Aabb;
pub use bvh::BVH;

mod aabb;
mod bvh;

pub trait Bounded<T> {
    fn bound(&self) -> Aabb<T>;
}
