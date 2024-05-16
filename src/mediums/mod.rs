use image::Rgb;
use log::debug;
use rand::random;

use crate::geometry::{Aabb, Bounded, BoundedIntersectable, Hit, Intersectable, Ray, UnitVec};

#[derive(Debug)]
pub struct Medium {
    shape: Box<dyn BoundedIntersectable>,
    inv_density: f32,
}

impl Medium {
    pub fn new(shape: Box<dyn BoundedIntersectable>, density: f32) -> Self {
        Medium {
            shape,
            inv_density: -1. / density,
        }
    }
}

impl Intersectable for Medium {
    fn hit(&self, ray: &Ray) -> Option<Hit> {
        // TODO: assuming convex shape and ray starting outside a medium
        match self.shape.hit(ray) {
            None => None,
            Some(first_hit) => {
                let inside_ray = Ray::new(ray.at(first_hit.t + 0.001), ray.dir);
                if let Some(second_hit) = self.shape.hit(&inside_ray) {
                    let dist_inside = (second_hit.point - first_hit.point).len();
                    let hit_distance = self.inv_density * f32::ln(random());
                    if hit_distance < dist_inside {
                        let hit_t = first_hit.t + hit_distance;
                        return Some(Hit {
                            t: hit_t,
                            point: ray.at(hit_t),
                            normal: UnitVec::new(1., 0., 0.),
                        });
                    }
                }
                None
            }
        }
    }
}

impl Bounded for Medium {
    fn bound(&self) -> Aabb { self.shape.bound() }
}
