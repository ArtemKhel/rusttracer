use std::{
    cmp::max,
    fmt::Debug,
    ops::{AddAssign, Deref, DerefMut},
    rc::Rc,
};

use derive_new::new;
use itertools::{partition, Itertools};
use log::debug;
use math::{utils::Axis3, Bounded, Intersectable};
use rayon::join;

use crate::{
    scene::{Intersection, Primitive},
    Aabb, Point, Ray,
};

#[derive(Debug)]
pub struct BVH {
    primitives: Vec<Rc<Primitive>>,
    nodes: Vec<BVHLinearNode>,
    height: usize,
}

#[derive(Debug, new)]
enum BVHLinearNode {
    Interior {
        bounds: Aabb,
        second_child_offset: usize,
        axis: Axis3,
    },
    Leaf {
        bounds: Aabb,
        first_offset: usize,
        n_primitives: usize,
    },
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
        children: (Box<BVHBuildNode>, Box<BVHBuildNode>),
        // children: [Box<BVHBuildNode>; 2],
        axis: Axis3,
    },
    Leaf {
        bounds: Aabb,
        first_offset: usize,
        n_primitives: usize,
    },
}

#[derive(Clone, Copy, Debug)]
enum SplitMethod {
    Middle,
    EqCounts,
    SAH,
}

#[derive(Copy, Clone, Debug, Default)]
struct BVHSplitBucket {
    count: usize,
    bounds: Aabb,
}

impl BVH {
    pub fn new(primitives: Vec<Rc<Primitive>>, mut max_in_node: usize) -> BVH {
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

        let mut ordered_primitives: Vec<Rc<Primitive>> = Vec::with_capacity(primitives.len());
        let mut total_nodes = 0_usize; // TODO: atomic (+parallel)
        max_in_node = max_in_node.min(255);
        let root = Self::recursive_build(
            &primitives,
            &mut primitives_info,
            &mut ordered_primitives,
            &mut total_nodes,
            SplitMethod::SAH,
            max_in_node,
        );

        let height = Self::height(&root);
        debug!("BVH height: {height}");
        let root = Self::flatten(root, total_nodes);

        BVH {
            primitives: ordered_primitives,
            nodes: root,
            height,
        }
    }

    fn recursive_build(
        primitives: &Vec<Rc<Primitive>>,
        primitives_info: &mut [BVHPrimitiveInfo],
        ordered_primitives: &mut Vec<Rc<Primitive>>,
        total_nodes: &mut usize,
        split_method: SplitMethod,
        max_in_node: usize,
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

        if let Some((left, right)) = Self::partition(primitives_info, centroid_bounds, axis, split_method, max_in_node)
        {
            // TODO: rayon::join
            let children = (
                Box::new(Self::recursive_build(
                    primitives,
                    left,
                    ordered_primitives,
                    total_nodes,
                    split_method,
                    max_in_node,
                )),
                Box::new(Self::recursive_build(
                    primitives,
                    right,
                    ordered_primitives,
                    total_nodes,
                    split_method,
                    max_in_node,
                )),
            );
            BVHBuildNode::new_interior(bounds, children, axis)
        } else {
            Self::build_leaf(primitives, primitives_info, ordered_primitives, bounds)
        }
    }

    fn partition(
        primitives_info: &mut [BVHPrimitiveInfo],
        centroid_bounds: Aabb,
        axis: Axis3,
        split_method: SplitMethod,
        max_in_node: usize,
    ) -> Option<(&mut [BVHPrimitiveInfo], &mut [BVHPrimitiveInfo])> {
        let mut mid = primitives_info.len() / 2;
        match split_method {
            SplitMethod::Middle => {
                let axis_mid = (centroid_bounds.min[axis] + centroid_bounds.max[axis]) / 2.;
                mid = partition(primitives_info.iter_mut(), |pi| pi.center[axis] < axis_mid);
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
                if primitives_info.len() <= 2 {
                    return Self::partition(
                        primitives_info,
                        centroid_bounds,
                        axis,
                        SplitMethod::EqCounts,
                        max_in_node,
                    );
                }
                const N_BUCKETS: usize = 12;
                let mut buckets = [BVHSplitBucket::default(); N_BUCKETS];
                for prim in primitives_info.iter() {
                    let b =
                        ((centroid_bounds.offset(prim.center)[axis] * N_BUCKETS as f32) as usize).min(N_BUCKETS - 1);
                    buckets[b].count += 1;
                    buckets[b].bounds += prim.bounds;
                }

                const N_SPLITS: usize = N_BUCKETS - 1;
                let mut costs = [0.0; N_SPLITS];
                let mut count_below = 0.0;
                let mut bound_below = Aabb::default();
                for i in 0..N_SPLITS {
                    bound_below += buckets[i].bounds;
                    count_below += 1.0;
                    costs[i] += count_below * bound_below.surface_area();
                }

                let mut count_above = 0.0;
                let mut bounds_above = Aabb::default();
                for i in (1..=N_SPLITS).rev() {
                    bounds_above += buckets[i].bounds;
                    count_above += 1.0;
                    costs[i - 1] += count_above * bounds_above.surface_area();
                }

                let min_cost_split_index = costs.iter().position_min_by(|x, y| x.total_cmp(y)).unwrap();
                let leaf_cost = primitives_info.len() as f32;
                let min_cost = 0.5 + costs[min_cost_split_index] / centroid_bounds.surface_area();

                if primitives_info.len() > max_in_node || min_cost < leaf_cost {
                    mid = partition(primitives_info.iter_mut(), |prim| {
                        let b = ((centroid_bounds.offset(prim.center)[axis] * N_BUCKETS as f32) as usize)
                            .min(N_BUCKETS - 1);
                        b <= min_cost_split_index
                    })
                } else {
                    return None;
                }
            }
        }
        // debug!("Split at {mid} out of {} using {split_method:?}", primitives_info.len());
        Some(primitives_info.split_at_mut(mid))
    }

    fn build_leaf(
        primitives: &[Rc<Primitive>],
        primitives_info: &[BVHPrimitiveInfo],
        ordered_primitives: &mut Vec<Rc<Primitive>>,
        bounds: Aabb,
    ) -> BVHBuildNode {
        let first_offset = ordered_primitives.len();
        for prim_info in primitives_info {
            ordered_primitives.push(primitives[prim_info.index].clone())
        }
        BVHBuildNode::new_leaf(bounds, first_offset, primitives_info.len())
    }

    fn height(root: &BVHBuildNode) -> usize {
        match root {
            BVHBuildNode::Interior { children, .. } => {
                max(Self::height(children.0.as_ref()), Self::height(children.1.as_ref())) + 1
            }
            BVHBuildNode::Leaf { .. } => 1,
        }
    }

    fn flatten(root: BVHBuildNode, total_nodes: usize) -> Vec<BVHLinearNode> {
        fn rec(root: &BVHBuildNode, lin_root: &mut Vec<BVHLinearNode>) {
            match root {
                BVHBuildNode::Interior { bounds, children, axis } => {
                    let idx = lin_root.len();
                    lin_root.push(BVHLinearNode::Interior {
                        bounds: *bounds,
                        second_child_offset: 0,
                        axis: *axis,
                    });
                    rec(children.0.as_ref(), lin_root);
                    let offset = lin_root.len();
                    match lin_root.get_mut(idx) {
                        Some(BVHLinearNode::Interior {
                            second_child_offset, ..
                        }) => *second_child_offset = offset,
                        _ => unreachable!(),
                    };
                    rec(children.1.as_ref(), lin_root);
                }
                BVHBuildNode::Leaf {
                    bounds,
                    first_offset,
                    n_primitives,
                } => lin_root.push(BVHLinearNode::Leaf {
                    bounds: *bounds,
                    first_offset: *first_offset,
                    n_primitives: *n_primitives,
                }),
            }
        }

        let mut lin_root = Vec::with_capacity(total_nodes);
        rec(&root, &mut lin_root);
        lin_root
    }
}

// impl Bounded for BVH {
//     fn bound(&self) -> Aabb {
//         match self.nodes.first() {
//             None => Aabb::default(),
//             Some(BVHLinearNode::Leaf { bounds, .. }) | Some(BVHLinearNode::Interior { bounds, .. }) => *bounds,
//         }
//     }
// }

impl BVH {
    pub fn hit(&self, ray: &Ray) -> Option<Intersection> {
        if self.nodes.is_empty() {
            return None;
        }
        let mut stack = Vec::with_capacity(self.height);
        stack.push(0);

        let mut t_max = f32::MAX;
        let mut closest: Option<Intersection> = None;
        while let Some(node_id) = stack.pop() {
            let node = &self.nodes[node_id];
            match *node {
                BVHLinearNode::Interior {
                    bounds,
                    axis,
                    second_child_offset,
                } => {
                    if bounds.hit(ray, 0., t_max) {
                        if ray.dir[axis] >= 0. {
                            stack.push(second_child_offset);
                            stack.push(node_id + 1);
                        } else {
                            stack.push(node_id + 1);
                            stack.push(second_child_offset);
                        }
                    }
                }
                BVHLinearNode::Leaf {
                    bounds,
                    first_offset,
                    n_primitives,
                } => {
                    if bounds.hit(ray, 0., t_max) {
                        let curr_closest = self.primitives[first_offset..first_offset + n_primitives]
                            .iter()
                            .filter_map(|obj| obj.hit(ray).map(|hit| Intersection { hit, object: obj }))
                            .min();

                        if curr_closest.is_some() && (closest.is_none() || curr_closest < closest) {
                            closest = curr_closest;
                            t_max = curr_closest.unwrap().hit.t;
                        }
                    }
                }
            }
        }
        closest
    }
}

#[cfg(test)]
mod tests {
    use image::Rgb;
    use math::{point3, vec3, Sphere};

    use super::*;
    use crate::{material::lambertian::Lambertian, scene::Primitive, Point, Vec3};

    #[test]
    fn test_bvh() {
        let mut world: Vec<Rc<Primitive>> = vec![
            Primitive {
                shape: Box::new(Sphere::new(point3!(0., 0., 0.), 1.0)),
                material: Box::new(Lambertian {
                    albedo: Rgb([0.4, 0.2, 0.1]),
                }),
            },
            Primitive {
                shape: Box::new(Sphere::new(point3!(4., 0., 0.), 1.0)),
                material: Box::new(Lambertian {
                    albedo: Rgb([0.4, 0.2, 0.1]),
                }),
            },
            Primitive {
                shape: Box::new(Sphere::new(point3!(0., 4., 0.), 1.0)),
                material: Box::new(Lambertian {
                    albedo: Rgb([0.4, 0.2, 0.1]),
                }),
            },
            Primitive {
                shape: Box::new(Sphere::new(point3!(4., 4., 0.), 1.0)),
                material: Box::new(Lambertian {
                    albedo: Rgb([0.4, 0.2, 0.1]),
                }),
            },
        ]
        .into_iter()
        .map(Rc::new)
        .collect();

        let bvh = BVH::new(world, 1);
        bvh.hit(&Ray::new(Point::default(), vec3!(1., 0., 0.).to_unit()));
        dbg!(&bvh);
    }
}
