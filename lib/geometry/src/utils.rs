use crate::unit_vec::UnitVec;
use crate::vec::Vec3;
use crate::Dot;
use rand::{random, Rng};
// TODO: SmallRng

pub fn random_unit() -> UnitVec {
    loop {
        let rnd: Vec3 = random();
        if rnd.len() <= 1. {
            break rnd.to_unit();
        }
    }
}
pub fn random_on_hemisphere(normal: &UnitVec) -> UnitVec {
    let random = random_unit();
    if normal.dot(random) >= 0. {
        random
    } else {
        -random
    }
}
pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let rnd: Vec3 = random();
        if rnd.len() <= 1. {
            break rnd;
        }
    }
}
