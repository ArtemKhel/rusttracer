use std::{
    cmp::max,
    fmt::Debug,
    ops::{AddAssign, Deref, DerefMut},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
    time::Instant,
};

use derive_new::new;
use itertools::{partition, Itertools};
use log::{debug, info};
use num_traits::{Float, Zero};
use rayon::join;

use crate::{
    aggregates::{aabb::Sign, Aabb, Bounded},
    breakpoint,
    core::{Ray, SurfaceInteraction},
    math::{axis::Axis3, Normed, Number, Point3},
    normal3, point3,
    scene::primitives::PrimitiveEnum,
    shapes::Intersectable,
    unit_normal3,
    utils::time_it,
    vec3, Pair,
};

// TODO: allocator

#[derive(Debug)]
pub struct BVH<T: Number> {
    primitives: Vec<Arc<PrimitiveEnum>>,
    nodes: Vec<BVHLinearNode<T>>,
    height: usize,
}

#[derive(Debug, new)]
enum BVHLinearNode<T: Number> {
    Interior {
        bounds: Aabb<T>,
        second_child_offset: usize,
        axis: Axis3,
    },
    Leaf {
        bounds: Aabb<T>,
        first_offset: usize,
        n_primitives: usize,
    },
}

#[derive(Debug)]
struct BVHPrimitiveInfo<T: Number> {
    index: usize,
    bounds: Aabb<T>,
    center: Point3<T>,
}

#[derive(Debug, new)]
enum BVHBuildNode<T: Number> {
    Interior {
        bounds: Aabb<T>,
        children: (Box<BVHBuildNode<T>>, Box<BVHBuildNode<T>>),
        // children: [Box<BVHBuildNode>; 2],
        axis: Axis3,
    },
    Leaf {
        bounds: Aabb<T>,
        first_offset: usize,
        n_primitives: usize,
    },
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Clone, Copy, Debug)]
enum SplitMethod {
    Middle,
    EqCounts,
    SAH,
}

#[derive(Copy, Clone, Debug, Default)]
struct BVHSplitBucket<T: Number> {
    count: usize,
    bounds: Aabb<T>,
}

impl BVH<f32> {
    pub fn new(primitives: Vec<Arc<PrimitiveEnum>>, mut max_in_node: usize) -> BVH<f32> {
        info!("Building BVH ...");
        let (bvh, time) = time_it(|| {
            let bounds = primitives.iter().fold(Aabb::default(), |acc, p| acc + p.bound());

            let mut primitives_info: Vec<BVHPrimitiveInfo<f32>> = Vec::with_capacity(primitives.len());
            for (index, p) in primitives.iter().enumerate() {
                let bounds = p.bound();
                primitives_info.push(BVHPrimitiveInfo {
                    index,
                    bounds,
                    center: bounds.center(),
                })
            }

            let mut ordered_primitives: Mutex<Vec<Arc<PrimitiveEnum>>> =
                Mutex::new(Vec::with_capacity(primitives.len()));
            let mut total_nodes = AtomicUsize::new(0);
            max_in_node = max_in_node.min(255);
            let root = Self::recursive_build(
                &primitives,
                &mut primitives_info,
                &mut ordered_primitives,
                &total_nodes,
                SplitMethod::SAH,
                max_in_node,
            );

            let height = Self::height(&root);
            let total_nodes = total_nodes.into_inner();
            let root = Self::flatten(root, total_nodes);
            info!("BVH height: {height}, {total_nodes} nodes");

            let ordered_primitives = ordered_primitives.into_inner().unwrap();
            BVH {
                primitives: ordered_primitives,
                nodes: root,
                height,
            }
        });

        info!("BVH built in {time:.3}s");
        bvh
    }

    fn recursive_build(
        primitives: &[Arc<PrimitiveEnum>],
        primitives_info: &mut [BVHPrimitiveInfo<f32>],
        ordered_primitives: &Mutex<Vec<Arc<PrimitiveEnum>>>,
        total_nodes: &AtomicUsize,
        split_method: SplitMethod,
        max_in_node: usize,
    ) -> BVHBuildNode<f32> {
        total_nodes.fetch_add(1, Ordering::Release);

        let bounds = primitives_info.iter().fold(Aabb::default(), |acc, p| acc + p.bounds);
        if primitives_info.len() == 1 {
            return Self::build_leaf(primitives, primitives_info, ordered_primitives, bounds);
        }

        let centroid_bounds = primitives_info
            .iter()
            .fold(Aabb::default(), |acc, p| acc.union(p.center));
        let axis = centroid_bounds.max_dimension();
        // all centers are in the same point
        if centroid_bounds.min == centroid_bounds.max {
            return Self::build_leaf(primitives, primitives_info, ordered_primitives, bounds);
        }

        if let Some((left, right)) = Self::partition(primitives_info, centroid_bounds, axis, split_method, max_in_node)
        {
            let children = join(
                || {
                    Box::new(Self::recursive_build(
                        primitives,
                        left,
                        ordered_primitives,
                        total_nodes,
                        split_method,
                        max_in_node,
                    ))
                },
                || {
                    Box::new(Self::recursive_build(
                        primitives,
                        right,
                        ordered_primitives,
                        total_nodes,
                        split_method,
                        max_in_node,
                    ))
                },
            );
            BVHBuildNode::new_interior(bounds, children, axis)
        } else {
            Self::build_leaf(primitives, primitives_info, ordered_primitives, bounds)
        }
    }

    fn partition(
        primitives_info: &mut [BVHPrimitiveInfo<f32>],
        centroid_bounds: Aabb<f32>,
        axis: Axis3,
        split_method: SplitMethod,
        max_in_node: usize,
    ) -> Option<Pair<&mut [BVHPrimitiveInfo<f32>]>> {
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
        // debug!("Split at {mid} out of {} using {split_method:?}",
        // primitives_info.len());
        Some(primitives_info.split_at_mut(mid))
    }

    fn build_leaf(
        primitives: &[Arc<PrimitiveEnum>],
        primitives_info: &[BVHPrimitiveInfo<f32>],
        ordered_primitives: &Mutex<Vec<Arc<PrimitiveEnum>>>,
        bounds: Aabb<f32>,
    ) -> BVHBuildNode<f32> {
        let mut vec = ordered_primitives.lock().unwrap();
        let first_offset = vec.len();
        for prim_info in primitives_info {
            vec.push(primitives[prim_info.index].clone())
        }
        BVHBuildNode::new_leaf(bounds, first_offset, primitives_info.len())
    }

    fn height(root: &BVHBuildNode<f32>) -> usize {
        match root {
            BVHBuildNode::Interior { children, .. } => {
                max(Self::height(children.0.as_ref()), Self::height(children.1.as_ref())) + 1
            }
            BVHBuildNode::Leaf { .. } => 1,
        }
    }

    fn flatten(root: BVHBuildNode<f32>, total_nodes: usize) -> Vec<BVHLinearNode<f32>> {
        fn rec(root: &BVHBuildNode<f32>, lin_root: &mut Vec<BVHLinearNode<f32>>) {
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

impl Intersectable for BVH<f32> {
    fn intersect(&self, ray: &Ray, t_max: f32) -> Option<SurfaceInteraction> {
        if self.nodes.is_empty() {
            return None;
        }
        let mut stack = Vec::with_capacity(self.height * 2);
        stack.push(0);

        let mut t_max = t_max;
        let mut closest: Option<SurfaceInteraction> = None;

        // let mut _dbg_counter = 0;
        // let mut _dbg_node_history: Vec<usize> = vec![];

        let inv_dir = ray.dir.map(f32::recip);
        let inv_bounds = ray.dir.map(Sign::from);

        while let Some(node_id) = stack.pop() {
            // #[cfg(debug_assertions)]
            // {
            //     _dbg_counter += 1;
            //     _dbg_node_history.push(node_id);
            //     breakpoint!(_dbg_counter >= 20);
            // }
            match self.nodes[node_id] {
                BVHLinearNode::Interior {
                    bounds,
                    axis,
                    second_child_offset,
                } => {
                    if bounds.hit_fast(ray, inv_dir, inv_bounds, t_max) {
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
                    if bounds.hit_fast(ray, inv_dir, inv_bounds, t_max) {
                        let curr_closest = self.primitives[first_offset..first_offset + n_primitives]
                            .iter()
                            .filter_map(|obj| obj.intersect(ray, t_max))
                            .min();

                        if let Some(cc) = &curr_closest
                            && cc.hit.t < t_max
                            && (closest.is_none() || curr_closest < closest)
                        {
                            t_max = cc.hit.t;
                            closest = curr_closest;
                        }
                    }
                }
            }
        }
        closest
    }

    fn check_intersect(&self, ray: &Ray, t_max: f32) -> bool {
        if self.nodes.is_empty() {
            return false;
        }
        let mut stack = Vec::with_capacity(self.height * 2);
        stack.push(0);

        let mut t_max = t_max;

        let inv_dir = ray.dir.map(f32::recip);
        let inv_bounds = ray.dir.map(Sign::from);

        while let Some(node_id) = stack.pop() {
            match self.nodes[node_id] {
                BVHLinearNode::Interior {
                    bounds,
                    axis,
                    second_child_offset,
                } => {
                    if bounds.hit_fast(ray, inv_dir, inv_bounds, t_max) {
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
                    if bounds.hit_fast(ray, inv_dir, inv_bounds, t_max) {
                        let curr_closest = self.primitives[first_offset..first_offset + n_primitives]
                            .iter()
                            .any(|obj| obj.check_intersect(ray, t_max));

                        if curr_closest {
                            return true;
                        }
                    }
                }
            }
        }
        false
    }
}

impl<T: Number> Bounded<T> for BVH<T> {
    fn bound(&self) -> Aabb<T> {
        match self.nodes.first().unwrap() {
            BVHLinearNode::Interior { bounds, .. } | BVHLinearNode::Leaf { bounds, .. } => *bounds,
        }
    }
}

#[cfg(test)]
mod tests {
    use test::Bencher;

    use super::*;
    use crate::{math::Transform, point3, ray, shapes::sphere::Sphere, unit3};

    extern crate test;

    #[bench]
    fn bench_hit(b: &mut Bencher) {
        let aabb = Sphere::new(1., Transform::id()).bound();
        let ray = ray!(point3!(10., 10., 10.), unit3!(-1., -1., -1.));
        b.iter(|| aabb.hit(&ray, f32::INFINITY));
    }

    #[bench]
    fn bench_hit_fast(b: &mut Bencher) {
        let aabb = Sphere::new(1., Transform::id()).bound();
        let ray = ray!(point3!(10., 10., 10.), unit3!(-1., -1., -1.));
        let inv_dir = ray.dir.map(f32::recip);
        let inv_bounds = ray.dir.map(Sign::from);
        b.iter(|| aabb.hit_fast(&ray, inv_dir, inv_bounds, f32::INFINITY));
    }

    // #[test]
    // fn test_bvh() {
    //     let mut world: Vec<Arc<Primitive>> = vec![
    //         Primitive {
    //             shape: Box::new(Sphere::new(point3!(0., 0., 0.), 1.0)),
    //             material: Box::new(Lambertian {
    //                 albedo: Rgb([0.4, 0.2, 0.1]),
    //             }),
    //         },
    //         Primitive {
    //             shape: Box::new(Sphere::new(point3!(4., 0., 0.), 1.0)),
    //             material: Box::new(Lambertian {
    //                 albedo: Rgb([0.4, 0.2, 0.1]),
    //             }),
    //         },
    //         Primitive {
    //             shape: Box::new(Sphere::new(point3!(0., 4., 0.), 1.0)),
    //             material: Box::new(Lambertian {
    //                 albedo: Rgb([0.4, 0.2, 0.1]),
    //             }),
    //         },
    //         Primitive {
    //             shape: Box::new(Sphere::new(point3!(4., 4., 0.), 1.0)),
    //             material: Box::new(Lambertian {
    //                 albedo: Rgb([0.4, 0.2, 0.1]),
    //             }),
    //         },
    //     ]
    //     .into_iter()
    //     .map(Arc::new)
    //     .collect();
    //
    //     let bvh = BVH::new(world, 1);
    //     bvh.hit(&Ray::new(Point3::default(), vec3!(1., 0., 0.).to_unit()));
    //     // dbg!(&bvh);
    // }
}
