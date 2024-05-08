use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
    rc::Rc,
};

use derive_new::new;
use itertools::partition;

use crate::{
    geometry::{Aabb, Bounded, BoundedIntersectable, Point, utils::Axis},
    scene::Primitive,
};

#[derive(Debug)]
pub struct BVH<T>
where
    T: Bounded,
{
    primitives: Vec<Rc<T>>,
    max_in_node: usize,
    nodes: BVHLinearNode,
}

#[derive(Debug,new)]
enum BVHLinearNode {
    Interior{
        bounds: Aabb,
        second_child_offset: usize,
        axis: Axis,
    },
    Leaf{
        bounds: Aabb,
        primitives_offset: usize,
        n_primitives: usize
    }
}

#[derive(Debug)]
struct BVHPrimitiveInfo {
    index: usize,
    bounds: Aabb,
    center: Point,
}

#[derive(Debug, new)]
enum BVHBuildNode {
    Interior {
        bounds: Aabb,
        children: [Box<BVHBuildNode>; 2],
        axis: Axis,
    },
    Leaf {
        bounds: Aabb,
        first_offset: usize,
        n_primitives: usize,
    },
}

enum SplitMethod {
    Middle,
    EqCounts,
    SAH,
}

impl<T> BVH<T>
where
    T: Bounded + Debug, // TODO: tmp
{
    fn flatten(){

    }

    pub fn new(primitives: Vec<Rc<T>>, max_in_node: usize) -> BVHBuildNode {
        // TODO: allocators
        let bounds = primitives.iter().fold(Aabb::default(), |acc, p| acc + p.bound());

        let mut primitives_info: Vec<BVHPrimitiveInfo> = Vec::with_capacity(primitives.len());
        for (index, p) in primitives.iter().enumerate() {
            let bounds = p.bound();
            primitives_info.push(BVHPrimitiveInfo {
                index,
                bounds,
                center: bounds.center(),
            })
        }

        let mut ordered_primitives: Vec<Rc<T>> = Vec::with_capacity(primitives.len());
        let mut total_nodes = 0_usize; // TODO: atomic (+parallel)
        let root = Self::recursive_build(
            &primitives,
            &mut primitives_info,
            &mut ordered_primitives,
            &mut total_nodes,
            SplitMethod::Middle,
        );

        dbg!(&root);
        dbg!(&ordered_primitives);
        dbg!(&primitives_info);
        todo!()
    }

    fn recursive_build(
        primitives: &Vec<Rc<T>>,
        primitives_info: &mut [BVHPrimitiveInfo],
        ordered_primitives: &mut Vec<Rc<T>>,
        total_nodes: &mut usize,
        split_method: SplitMethod,
    ) -> BVHBuildNode {
        *total_nodes += 1;

        let bounds = primitives_info.iter().fold(Aabb::default(), |acc, p| acc + p.bounds);
        if primitives_info.len() == 1 {
            return Self::build_leaf(primitives, primitives_info, ordered_primitives, bounds);
        }

        let centroid_bounds = primitives_info.iter().fold(Aabb::default(), |acc, p| acc + p.center);
        let axis = centroid_bounds.max_dimension();
        // all centers are in the same point
        if centroid_bounds.min == centroid_bounds.max {
            return Self::build_leaf(primitives, primitives_info, ordered_primitives, bounds);
        }

        let mut mid = primitives_info.len() / 2;
        match split_method {
            SplitMethod::Middle => {
                let axis_mid = (centroid_bounds.min[axis] + centroid_bounds.max[axis]) / 2.;
                let split_index = partition(primitives_info.iter_mut(), |pi| pi.center[axis] < axis_mid);
                mid = split_index;
                // fallback to EqCounts
                if mid == 0 || mid == primitives_info.len() {
                    primitives_info.select_nth_unstable_by(mid, |p1, p2| p1.center[axis].total_cmp(&p2.center[axis]));
                    mid = primitives_info.len() / 2
                }
            }
            SplitMethod::EqCounts => {
                primitives_info.select_nth_unstable_by(mid, |p1, p2| p1.center[axis].total_cmp(&p2.center[axis]));
            }
            SplitMethod::SAH => {
                todo!()
            }
        }
        let (left, right) = primitives_info.split_at_mut(mid);
        // TODO: parallel
        let children = [
            Box::new(Self::recursive_build(
                primitives,
                left,
                ordered_primitives,
                total_nodes,
                SplitMethod::Middle,
            )),
            Box::new(Self::recursive_build(
                primitives,
                right,
                ordered_primitives,
                total_nodes,
                SplitMethod::Middle,
            )),
        ];
        BVHBuildNode::new_interior(bounds, children, axis)
    }

    fn build_leaf(
        primitives: &Vec<Rc<T>>,
        primitives_info: &[BVHPrimitiveInfo],
        ordered_primitives: &mut Vec<Rc<T>>,
        bounds: Aabb,
    ) -> BVHBuildNode {
        let first_offset = ordered_primitives.len();
        for prim_info in primitives_info {
            ordered_primitives.push(primitives[prim_info.index].clone())
        }
        return BVHBuildNode::new_leaf(bounds, first_offset, primitives_info.len());
    }
}

#[cfg(test)]
mod tests {
    use image::Rgb;

    use crate::{
        geometry::{Point, Sphere},
        material::lambertian::Lambertian,
        scene::Primitive,
    };

    use super::*;

    #[test]
    fn test_bvh() {
        let mut world: Vec<Rc<Primitive>> = vec![
            Primitive {
                shape: Box::new(Sphere::new(Point::new(0., 0., 0.), 1.0)),
                material: Box::new(Lambertian {
                    albedo: Rgb([0.4, 0.2, 0.1]),
                }),
            },
            Primitive {
                shape: Box::new(Sphere::new(Point::new(4., 0., 0.), 1.0)),
                material: Box::new(Lambertian {
                    albedo: Rgb([0.4, 0.2, 0.1]),
                }),
            },
            Primitive {
                shape: Box::new(Sphere::new(Point::new(0., 4., 0.), 1.0)),
                material: Box::new(Lambertian {
                    albedo: Rgb([0.4, 0.2, 0.1]),
                }),
            },
            Primitive {
                shape: Box::new(Sphere::new(Point::new(4., 4., 0.), 1.0)),
                material: Box::new(Lambertian {
                    albedo: Rgb([0.4, 0.2, 0.1]),
                }),
            },
            Primitive {
                shape: Box::new(Sphere::new(Point::new(0., 0., 4.), 1.0)),
                material: Box::new(Lambertian {
                    albedo: Rgb([0.4, 0.2, 0.1]),
                }),
            },
            Primitive {
                shape: Box::new(Sphere::new(Point::new(4., 0., 4.), 1.0)),
                material: Box::new(Lambertian {
                    albedo: Rgb([0.4, 0.2, 0.1]),
                }),
            },
            Primitive {
                shape: Box::new(Sphere::new(Point::new(0., 4., 4.), 1.0)),
                material: Box::new(Lambertian {
                    albedo: Rgb([0.4, 0.2, 0.1]),
                }),
            },
            Primitive {
                shape: Box::new(Sphere::new(Point::new(4., 4., 4.), 1.0)),
                material: Box::new(Lambertian {
                    albedo: Rgb([0.4, 0.2, 0.1]),
                }),
            },
        ]
            .into_iter()
            .map(|x| Rc::new(x))
            .collect();

        let bvh = BVH::new(world, 1);
        dbg!(&bvh);
        panic!()
    }
}
